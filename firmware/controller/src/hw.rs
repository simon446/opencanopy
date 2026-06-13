//! Real ESP32-S3 peripheral bindings (esp-hal 0.23) wired to the committed pin map
//! (`electronics/analysis/pin-map.csv`). These do the actual bus/ADC/GPIO/PWM transactions and
//! hand the bytes to the host-tested protocol logic in `control::i2c_devices`.
//!
//! The whole control loop runs here against the real drivers. Under the `emulator` feature it runs
//! with fast simulated timing + serial telemetry so the Wokwi smoke test exercises this exact code
//! (with mock I2C chips on the bus); without it, it runs the production 5-minute cadence.
//!
//! WS2812 status LEDs (RMT) are intentionally not driven here yet: esp-hal-smartled has no release
//! for esp-hal 0.23 (0.14→0.22, 0.15→1.0-beta). The status-LED *logic* is host-tested in
//! `control::led_status`; wiring the RMT driver is a follow-up (tracked for WI-EE-08 / esp-hal bump).

use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, Pull};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::ledc::channel::{self, ChannelIFace};
use esp_hal::ledc::timer::{self, TimerIFace};
use esp_hal::ledc::{LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::peripherals::Peripherals;
use esp_hal::time::RateExtU32;
use esp_println::println;

use control::app_state::{App, AppConfig, SensorFrame};
use control::calibration::Calibration;
use control::hal::SensorError;
use control::i2c_devices::{self as dev, DeviceError};

const UTC_OFFSET_S: i32 = 0;
/// INA219 current LSB (µA/bit) and shunt — chosen so a ~1 A pump reads with headroom.
const INA219_CURRENT_LSB_UA: u32 = 100;
const INA219_SHUNT_MOHM: u32 = 100;

// Thin adapters binding esp-hal's concrete types to control's host-tested driver traits. All the
// I2C transaction *logic* lives in `control::i2c_devices` (host-tested); these only forward calls.
// (Newtypes are required by the orphan rule — both trait and esp-hal type are foreign to this crate.)
struct I2cAdapter<'a>(I2c<'a, esp_hal::Blocking>);
impl dev::I2cBus for I2cAdapter<'_> {
    type Error = esp_hal::i2c::master::Error;
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.write(addr, bytes)
    }
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.0.read(addr, buf)
    }
    fn write_read(&mut self, addr: u8, bytes: &[u8], buf: &mut [u8]) -> Result<(), Self::Error> {
        self.0.write_read(addr, bytes, buf)
    }
}

struct DelayAdapter(Delay);
impl dev::DelayMs for DelayAdapter {
    fn delay_ms(&mut self, ms: u32) {
        self.0.delay_millis(ms);
    }
}

/// Map a device-driver error to the control crate's sensor-fault type.
fn map_dev_err(e: DeviceError) -> SensorError {
    match e {
        DeviceError::Bus => SensorError::Bus,
        DeviceError::Protocol => SensorError::OutOfRange,
    }
}

/// Demo calibration used so the loop exercises the full watering path (real dev values come from
/// WI-EE-08 bring-up; without a valid calibration the firmware fails safe and won't auto-water).
fn demo_calibration() -> Calibration {
    Calibration {
        version: 1,
        moisture_raw_dry: 1200,
        moisture_raw_wet: 2600,
        pump_ml_per_sec: 3.8,
        fan_min_pwm: 28,
        led_ppfd_25: 120,
        led_ppfd_50: 240,
        led_ppfd_75: 360,
        led_ppfd_100: 480,
        reservoir_low_adc: 600,
    }
}

/// Set up every peripheral from the pin map and run the control loop against the real drivers.
pub fn run(peripherals: Peripherals) -> ! {
    let delay = Delay::new();
    let mut delayer = DelayAdapter(delay);

    // ---- I2C0: SHT40 (0x44) + DS3231 (0x68) + INA219 (0x40), GPIO8=SDA, GPIO9=SCL ----
    let mut i2c = I2cAdapter(
        I2c::new(peripherals.I2C0, I2cConfig::default())
            .expect("i2c init")
            .with_sda(peripherals.GPIO8)
            .with_scl(peripherals.GPIO9),
    );
    let _ = dev::init_ina219(&mut i2c, INA219_CURRENT_LSB_UA, INA219_SHUNT_MOHM);

    // ---- ADC1: capacitive moisture probe on GPIO4 (CH3) ----
    let mut adc_cfg = AdcConfig::new();
    let mut moisture_pin = adc_cfg.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_cfg);

    // ---- Digital inputs ----
    // Leak (GPIO7): pin map "active-high = leak"; pull-down so an unconnected input reads "no leak".
    let leak_in = Input::new(peripherals.GPIO7, Pull::Down);
    // Reservoir level (GPIO5): pin map net RES_LOW_SW, "internal pullup; closed=low". The switch
    // is wired so it closes (pin LOW) when the reservoir is LOW; open (pulled HIGH) = not low.
    // ASSUMPTION to confirm at WI-EE-08 bring-up: the float's NO/NC orientation matches this.
    let res_float_in = Input::new(peripherals.GPIO5, Pull::Up);

    // ---- LEDC PWM: pump (GPIO10), fan (GPIO12, 25 kHz), grow LED (GPIO14) ----
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut pwm_timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    pwm_timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 25_000u32.Hz(),
        })
        .expect("ledc timer");
    let mut pump_pwm = ledc.channel(channel::Number::Channel0, peripherals.GPIO10);
    pump_pwm
        .configure(channel::config::Config { timer: &pwm_timer, duty_pct: 0, pin_config: channel::config::PinConfig::PushPull })
        .expect("pump ch");
    let mut fan_pwm = ledc.channel(channel::Number::Channel1, peripherals.GPIO12);
    fan_pwm
        .configure(channel::config::Config { timer: &pwm_timer, duty_pct: 0, pin_config: channel::config::PinConfig::PushPull })
        .expect("fan ch");
    let mut grow_pwm = ledc.channel(channel::Number::Channel2, peripherals.GPIO14);
    grow_pwm
        .configure(channel::config::Config { timer: &pwm_timer, duty_pct: 0, pin_config: channel::config::PinConfig::PushPull })
        .expect("grow ch");

    // ---- Boot the controller ----
    let cal = demo_calibration();
    let cal_bytes = cal.encode();
    let boot_rtc = dev::read_ds3231(&mut i2c);
    let mut app = App::boot(AppConfig { utc_offset_s: UTC_OFFSET_S }, Some(&cal_bytes), 60, 0, boot_rtc, true);
    println!("=== OpenCanopy real-driver loop (ESP32-S3) ===");
    println!("boot: state={} rtc_valid={}", app.state().name(), boot_rtc.valid);

    // Timing: a short fast run under `emulator` (for the Wokwi/CI smoke test — absent-device I2C
    // NAKs are slow under emulation, so a full simulated day won't finish in the CI timeout; a
    // handful of ticks proves boot + drivers + loop). Real 5-min cadence otherwise.
    #[cfg(feature = "emulator")]
    let (tick_ms, total_ticks, real_delay_ms) = (5u64 * 60_000, 8u64, 5u64);
    #[cfg(not(feature = "emulator"))]
    let (tick_ms, total_ticks, real_delay_ms) = (5u64 * 60_000, u64::MAX, 5u64 * 60_000);

    let mut now_ms = 0u64;
    let mut tick = 0u64;
    loop {
        // --- Read every sensor through the REAL drivers (logic in host-tested control crate) ---
        let temp_rh = dev::read_sht40(&mut i2c, &mut delayer).map_err(map_dev_err);
        let rtc = dev::read_ds3231(&mut i2c);
        let moisture_raw: Result<u16, SensorError> =
            nb::block!(adc1.read_oneshot(&mut moisture_pin)).map_err(|_| SensorError::Bus);
        let pump_ma = dev::read_ina219_ma(&mut i2c, INA219_CURRENT_LSB_UA);
        let reservoir_low = res_float_in.is_low(); // RES_LOW_SW closes (low) when reservoir is low
        let leak = leak_in.is_high(); // active-high = leak

        let frame = SensorFrame {
            now_ms,
            rtc,
            temp_rh,
            moisture_raw,
            reservoir_low,
            leak,
            led_heat_c: None,
            fan_tach_rpm: None,
        };
        let cmd = app.step(&frame);

        // --- Drive actuators through the REAL PWM channels ---
        let _ = pump_pwm.set_duty(if cmd.pump_on { 100 } else { 0 });
        let _ = fan_pwm.set_duty(cmd.fan_pct);
        let _ = grow_pwm.set_duty(cmd.led_pct);

        {
            println!(
                "[t={}m] stage={} state={} light={} led={}% fan={}% pump={} moist={} temp={} pump_mA={}",
                now_ms / 60_000,
                cmd.stage.code(),
                cmd.state.name(),
                cmd.light_on as u8,
                cmd.led_pct,
                cmd.fan_pct,
                cmd.pump_on as u8,
                cmd.moisture_pct.map(|m| m as i32).unwrap_or(-1),
                temp_rh.map(|t| t.temp_c as i32).unwrap_or(-99),
                pump_ma.unwrap_or(-1),
            );
        }

        now_ms += tick_ms;
        tick += 1;
        if tick >= total_ticks {
            break;
        }
        delay.delay_millis(real_delay_ms as u32);
    }

    println!("REAL-DRIVER SMOKE TEST COMPLETE: ran {} ticks, final state={}", tick, app.state().name());
    loop {
        delay.delay_millis(1000);
    }
}
