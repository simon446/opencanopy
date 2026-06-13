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
use control::hal::{SensorError, TempRh, WallTime};
use control::i2c_devices as dev;

const UTC_OFFSET_S: i32 = 0;
/// INA219 current LSB (µA/bit) and shunt — chosen so a ~1 A pump reads with headroom.
const INA219_CURRENT_LSB_UA: u32 = 100;
const INA219_SHUNT_MOHM: u32 = 100;

/// Read SHT40 temp/RH over I2C: write the measure command, wait, read 6 bytes, parse+CRC-check.
fn read_sht40(i2c: &mut I2c<'_, esp_hal::Blocking>, delay: &Delay) -> Result<TempRh, SensorError> {
    i2c.write(dev::SHT40_ADDR, &[dev::SHT40_CMD_MEASURE_HIGH])
        .map_err(|_| SensorError::Bus)?;
    delay.delay_millis(10); // high-precision conversion time
    let mut buf = [0u8; 6];
    i2c.read(dev::SHT40_ADDR, &mut buf).map_err(|_| SensorError::Bus)?;
    dev::sht40_parse(&buf).ok_or(SensorError::OutOfRange)
}

/// Read the DS3231 wall clock: 7 timekeeping registers from 0x00 + the status register (OSF).
fn read_ds3231(i2c: &mut I2c<'_, esp_hal::Blocking>) -> WallTime {
    let mut t = [0u8; 7];
    if i2c.write_read(dev::DS3231_ADDR, &[dev::DS3231_REG_SECONDS], &mut t).is_err() {
        return WallTime::INVALID;
    }
    let mut status = [0u8; 1];
    if i2c.write_read(dev::DS3231_ADDR, &[dev::DS3231_REG_STATUS], &mut status).is_err() {
        return WallTime::INVALID;
    }
    dev::ds3231_parse(&t, status[0])
}

/// Read the INA219 CURRENT register → mA (for the pump dry-run / clog fault path).
fn read_ina219_ma(i2c: &mut I2c<'_, esp_hal::Blocking>) -> Option<i32> {
    let mut buf = [0u8; 2];
    i2c.write_read(dev::INA219_ADDR, &[dev::INA219_REG_CURRENT], &mut buf).ok()?;
    let raw = i16::from_be_bytes(buf);
    Some(dev::ina219_current_ma(raw, INA219_CURRENT_LSB_UA))
}

/// Program the INA219 calibration register (must be set before current reads are meaningful).
fn init_ina219(i2c: &mut I2c<'_, esp_hal::Blocking>) {
    let cal = dev::ina219_calibration(INA219_CURRENT_LSB_UA, INA219_SHUNT_MOHM);
    let bytes = cal.to_be_bytes();
    let _ = i2c.write(dev::INA219_ADDR, &[dev::INA219_REG_CALIBRATION, bytes[0], bytes[1]]);
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

    // ---- I2C0: SHT40 (0x44) + DS3231 (0x68) + INA219 (0x40), GPIO8=SDA, GPIO9=SCL ----
    let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("i2c init")
        .with_sda(peripherals.GPIO8)
        .with_scl(peripherals.GPIO9);
    init_ina219(&mut i2c);

    // ---- ADC1: capacitive moisture probe on GPIO4 (CH3) ----
    let mut adc_cfg = AdcConfig::new();
    let mut moisture_pin = adc_cfg.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc_cfg);

    // ---- Digital inputs: leak (GPIO7, active-high) + reservoir float (GPIO5, pull-up, closed=low) ----
    let leak_in = Input::new(peripherals.GPIO7, Pull::Down);
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
    let boot_rtc = read_ds3231(&mut i2c);
    let mut app = App::boot(AppConfig { utc_offset_s: UTC_OFFSET_S }, Some(&cal_bytes), 60, 0, boot_rtc, true);
    println!("=== OpenCanopy real-driver loop (ESP32-S3) ===");
    println!("boot: state={} rtc_valid={}", app.state().name(), boot_rtc.valid);

    // Timing: fast virtual ticks under `emulator` (for Wokwi), real 5-min cadence otherwise.
    #[cfg(feature = "emulator")]
    let (tick_ms, total_ticks, real_delay_ms) = (5u64 * 60_000, 288u64, 15u64);
    #[cfg(not(feature = "emulator"))]
    let (tick_ms, total_ticks, real_delay_ms) = (5u64 * 60_000, u64::MAX, 5u64 * 60_000);

    let mut now_ms = 0u64;
    let mut tick = 0u64;
    loop {
        // --- Read every sensor through the REAL drivers ---
        let temp_rh = read_sht40(&mut i2c, &delay);
        let rtc = read_ds3231(&mut i2c);
        let moisture_raw: Result<u16, SensorError> =
            nb::block!(adc1.read_oneshot(&mut moisture_pin)).map_err(|_| SensorError::Bus);
        let pump_ma = read_ina219_ma(&mut i2c);
        let reservoir_low = res_float_in.is_low(); // closed=low=full; open=high → low water
        let leak = leak_in.is_high();

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

        if tick % 12 == 0 {
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
