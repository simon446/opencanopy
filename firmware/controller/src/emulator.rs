//! Emulator smoke-test entry point (built only with `--features emulator`).
//!
//! This runs the **real** `control::app_state::App` and the genuine 5-min control loop on the
//! emulated ESP32-S3 (QEMU/Wokwi), driven by canned-but-varying sensor values and emitting serial
//! telemetry. It is the on-silicon complement to the host `sim/` crate: where `sim/` validates the
//! control *logic*, this proves the actual firmware **binary boots, links esp-hal, and runs the
//! loop without panicking on emulated hardware**, with observable UART output for CI assertions.
//!
//! It does NOT touch real GPIO/ADC/I2C peripherals (those are bring-up bindings, WI-EE-08); the
//! sensor frame is synthesized in-loop. Full analog signal fidelity stays with HIL.
//!
//! STATUS: like all on-target code here, this is verified in CI/Wokwi, not on the host (the host
//! has no Xtensa toolchain). The Wokwi project + CI job live under `controller/wokwi/`.

use control::app_state::{App, AppConfig, SensorFrame};
use control::calibration::Calibration;
use control::hal::{TempRh, WallTime};
use esp_hal::delay::Delay;
use esp_println::println;

/// Simulated control-loop period (minutes). The loop advances a *virtual* clock so a full day of
/// control runs in a few seconds of wall time.
const TICK_MIN: u64 = 5;
/// Number of ticks to run (~1 simulated day at 5-min ticks).
const TICKS: u64 = 288;
/// Virtual wall-clock start: 06:00 (lights-on), so the light schedule exercises on/ramp/off.
const START_UNIX_S: u64 = 6 * 3600;

fn emulator_calibration() -> Calibration {
    Calibration {
        version: 1,
        moisture_raw_dry: 1000,
        moisture_raw_wet: 3000,
        pump_ml_per_sec: 3.8,
        fan_min_pwm: 28,
        led_ppfd_25: 120,
        led_ppfd_50: 240,
        led_ppfd_75: 360,
        led_ppfd_100: 480,
        reservoir_low_adc: 600,
    }
}

fn raw_for(pct: f32, cal: &Calibration) -> u16 {
    let span = (cal.moisture_raw_wet - cal.moisture_raw_dry) as f32;
    (cal.moisture_raw_dry as f32 + (pct / 100.0) * span) as u16
}

/// Run the smoke test, then idle. Diverges (`-> !`).
pub fn run() -> ! {
    let cal = emulator_calibration();
    let cal_bytes = cal.encode();

    // Boot the real controller with a valid calibration so the full watering path is exercised.
    let mut app = App::boot(
        AppConfig { utc_offset_s: 0 },
        Some(&cal_bytes),
        60, // start vegetative
        0,
        WallTime { valid: true, unix_s: START_UNIX_S },
        true,
    );
    println!("boot: state={} calibration=valid age=60d", app.state().name());

    let delay = Delay::new();
    // Crude environment feedback so the loop visibly reacts (dries out → waters → recovers).
    let mut moisture_pct = 50.0_f32;

    for i in 0..TICKS {
        let now_ms = i * TICK_MIN * 60_000;
        let rtc = WallTime { valid: true, unix_s: START_UNIX_S + now_ms / 1000 };

        let frame = SensorFrame {
            now_ms,
            rtc,
            temp_rh: Ok(TempRh { temp_c: 24.0, rh_pct: 60.0 }),
            moisture_raw: Ok(raw_for(moisture_pct, &cal)),
            reservoir_low: false,
            leak: false,
            led_heat_c: None,
            fan_tach_rpm: Some(1500),
        };

        let cmd = app.step(&frame);

        // Environment response: pump adds water, otherwise it slowly dries.
        if cmd.pump_on {
            moisture_pct += 6.0;
        }
        moisture_pct = (moisture_pct - 0.4).clamp(5.0, 95.0);

        // Periodic telemetry over UART (every hour of simulated time).
        if i % 12 == 0 {
            println!(
                "[t={:>4}m] stage={} state={} light={} led={}% fan={}% moist={} pump={}",
                now_ms / 60_000,
                cmd.stage.code(),
                cmd.state.name(),
                cmd.light_on as u8,
                cmd.led_pct,
                cmd.fan_pct,
                cmd.moisture_pct.map(|m| m as i32).unwrap_or(-1),
                cmd.pump_on as u8,
            );
        }

        delay.delay_millis(15);
    }

    println!(
        "EMULATOR SMOKE TEST COMPLETE: ran {} ticks, final state={}",
        TICKS,
        app.state().name()
    );

    loop {
        delay.delay_millis(1000);
    }
}
