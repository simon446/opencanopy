//! OpenCanopy ESP32-S3 firmware binary. Binds `control`'s `hal.rs` traits to real esp-hal
//! peripherals and runs the deterministic control loop (§9.2).
//!
//! ARCHITECTURE: all policy lives in the platform-agnostic `control` crate (host-tested, §10.2).
//! This binary is *only* glue: initialize peripherals, construct the binding objects in
//! [`sensors`]/[`actuators`]/[`drivers`], then on each control tick read sensors into a
//! `SensorFrame`, call `App::step`, and apply `Commands` to the actuators. Pump fails safe to OFF
//! on reset/brownout via a hardware gate pull-down (WI-EE-03) and the RWDT/brownout detector
//! enabled below.
//!
//! Build with `--features emulator` to run the [`emulator`] smoke test instead of touching real
//! peripherals — that path boots the genuine `App` + control loop on emulated silicon (Wokwi/QEMU)
//! and prints serial telemetry for CI to assert on (`controller/wokwi/`).
//!
//! STATUS: the esp-hal peripheral calls here require the Espressif toolchain (espup) and on-board
//! bring-up to verify against the real pin map (WI-EE-08). Pin assignments are owned by the
//! electronics track; the structure and the control-loop sequencing are what this file pins down.

#![no_std]
#![no_main]

// The real peripheral bindings exist only in the on-hardware build; the emulator build doesn't use
// them (it synthesizes sensors), so gating them off keeps that build warning-clean.
#[cfg(not(feature = "emulator"))]
mod actuators;
#[cfg(not(feature = "emulator"))]
mod drivers;
#[cfg(feature = "emulator")]
mod emulator;
#[cfg(not(feature = "emulator"))]
mod sensors;

use control::app_state::FIRMWARE_VERSION;
use esp_backtrace as _;
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    // --- Peripheral init (§9.4 step 1) ---
    // Use the default clock config: forcing CpuClock::max() reconfigures the PLL, which the Wokwi
    // emulator doesn't fully model and can hang boot. The default clock is fine for this controller.
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Boot banner over UART — the first observable sign of life (and the Wokwi smoke-test anchor).
    println!("=== OpenCanopy firmware v{FIRMWARE_VERSION} (ESP32-S3) ===");

    #[cfg(feature = "emulator")]
    {
        // Emulator smoke test: run the real control loop with synthesized sensors. Diverges.
        let _ = peripherals;
        emulator::run();
    }

    #[cfg(not(feature = "emulator"))]
    run_on_hardware(peripherals)
}

/// The real on-hardware control loop (§9.4 boot + 5-min control cycle).
#[cfg(not(feature = "emulator"))]
fn run_on_hardware(peripherals: esp_hal::peripherals::Peripherals) -> ! {
    use control::app_state::{App, AppConfig, SensorFrame};
    use control::hal::{
        Clock as _, Fan as _, GrowLed as _, LeakSensor as _, MoistureSensor as _, Pump as _,
        ReservoirSensor as _, Rtc as _, TempRhSensor as _,
    };
    use control::led_status;

    /// Control-loop period (§9.6: checks every 5 minutes).
    const TICK_MS: u64 = 5 * 60_000;
    /// Fixed local UTC offset (configured at manufacture; V1 has no DST).
    const UTC_OFFSET_S: i32 = 0;

    // Brownout + watchdog: pump must die safe on power sag / hang (§9.4, §11.4). esp-hal enables
    // the brownout detector by default; arm the RWDT and feed it each loop.
    let mut platform = drivers::Platform::new(peripherals);
    platform.enable_watchdog();

    // --- Boot sequence (§9.4) ---
    // 3) read calibration, 5) restore grow-cycle age, 7) ensure pump off (done in actuator ctor).
    let stored_cal = platform.calibration_store.load_raw();
    let restored_age_days = platform.calibration_store.load_age_days().unwrap_or(0);
    let rtc_time = platform.rtc.wall_time();
    let self_test_passed = platform.self_test();

    let mut app = App::boot(
        AppConfig { utc_offset_s: UTC_OFFSET_S },
        stored_cal.as_deref(),
        restored_age_days,
        platform.clock.now_ms(),
        rtc_time,
        self_test_passed,
    );

    // --- Control loop ---
    let mut next_tick_ms = platform.clock.now_ms();
    loop {
        platform.feed_watchdog();
        let now = platform.clock.now_ms();
        if now < next_tick_ms {
            platform.idle_until(next_tick_ms);
            continue;
        }
        next_tick_ms = now + TICK_MS;

        // Read sensors through the hal.rs traits.
        let frame = SensorFrame {
            now_ms: now,
            rtc: platform.rtc.wall_time(),
            temp_rh: platform.temp_rh.read(),
            moisture_raw: platform.moisture.read_raw(),
            reservoir_low: platform
                .reservoir
                .read_adc()
                .map(|adc| adc < platform.reservoir_low_adc)
                .unwrap_or(true),
            leak: platform.leak.is_wet(),
            led_heat_c: platform.led_heat(),
            fan_tach_rpm: platform.fan.tach_rpm(),
        };

        // Run the verified control logic.
        let cmd = app.step(&frame);

        // Apply commands through the actuator traits. Pump is hard-gated by the safety state inside
        // `App::step`, so `cmd.pump_on` is already safe to forward.
        platform.pump.set(cmd.pump_on);
        platform.fan.set_duty(cmd.fan_pct);
        platform.grow_led.set_power(cmd.led_pct);
        led_status::drive(&mut platform.status_leds, &cmd.panel);

        // Persist grow-cycle age periodically so a power cycle resumes the right stage (§9.4/§9.10).
        platform.persist_age_if_due(app.age_days(now));

        // Optional telemetry — DEFAULT-OFF, never required for control (§9.11).
        #[cfg(feature = "telemetry")]
        platform.telemetry.publish(&cmd, app.log());
    }
}
