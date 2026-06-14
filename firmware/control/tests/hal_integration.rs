//! End-to-end integration over the `hal.rs` trait seam (WI-FW-02 acceptance). This mirrors what
//! `controller/` and `sim/` do at their bindings: read the mock sensors, assemble a `SensorFrame`,
//! run the real `App`, then push the results out through the actuator + status-LED traits. It
//! proves the control stack depends only on the traits and that the mocks can inject the faults the
//! §10.3 scenarios need (leak, stuck sensor) — all on the host with no hardware.
//!
//! V1 is passive (no pump) and fan-less: the grow LED is the only actuator. The seam carries the
//! LED command + 4 status LEDs; watering is monitored and warned, never actuated.

use control::app_state::{App, AppConfig, Commands, SensorFrame};
use control::calibration::Calibration;
use control::hal::{
    Clock, GrowLed, LeakSensor, MoistureSensor, ReservoirSensor, Rtc, TempRhSensor, WallTime,
};
use control::led_status::{self, LedColor};
use control::safety_controller::SystemState;
use control::testkit::{
    MockClock, MockLeak, MockLed, MockMoisture, MockReservoir, MockRtc, MockStatusLeds, MockTempRh,
};

fn cal() -> Calibration {
    Calibration {
        version: 4,
        moisture_raw_dry: 1000,
        moisture_raw_wet: 3000,
        led_ppfd_25: 120,
        led_ppfd_50: 240,
        led_ppfd_75: 360,
        led_ppfd_100: 480,
        reservoir_low_adc: 600,
    }
}

/// A little "binding" that owns the mock peripherals and runs one control tick through the traits,
/// exactly as the on-target binding would. Reading is done via the trait methods (`&mut self`),
/// proving the seam is the only coupling.
struct Bench {
    app: App,
    clock: MockClock,
    rtc: MockRtc,
    temp: MockTempRh,
    moist: MockMoisture,
    res: MockReservoir,
    leak: MockLeak,
    led: MockLed,
    leds: MockStatusLeds,
}

impl Bench {
    fn new() -> Bench {
        let bytes = cal().encode();
        let rtc = MockRtc {
            time: WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
        }; // 08:00
        let app = App::boot(
            AppConfig::default(),
            Some(&bytes),
            60,
            0,
            rtc.wall_time(),
            true,
        );
        Bench {
            app,
            clock: MockClock::new(),
            rtc,
            temp: MockTempRh::default(),
            moist: MockMoisture {
                raw: 2000,
                fault: None,
            }, // ~50%
            res: MockReservoir {
                adc: 1500,
                low: false,
                fault: None,
            },
            leak: MockLeak::default(),
            led: MockLed::default(),
            leds: MockStatusLeds::default(),
        }
    }

    fn tick(&mut self) -> Commands {
        // Read sensors *through the traits*.
        let frame = SensorFrame {
            now_ms: self.clock.now_ms(),
            rtc: self.rtc.wall_time(),
            temp_rh: self.temp.read(),
            moisture_raw: self.moist.read_raw(),
            reservoir_low: self.res.low || self.res.read_adc().map(|a| a < 700).unwrap_or(true),
            leak: self.leak.is_wet(),
            led_heat_c: None,
        };
        let cmd = self.app.step(&frame);
        // Drive the only actuator + the status LEDs *through the traits*.
        self.led.set_power(cmd.led_pct);
        led_status::drive(&mut self.leds, &cmd.panel);
        // Advance the simulated clock and RTC.
        self.clock.advance_ms(5 * 60_000);
        self.rtc.time.unix_s += 5 * 60;
        cmd
    }
}

#[test]
fn full_stack_runs_through_trait_seam() {
    let mut b = Bench::new();
    // Drive the moisture probe dry so the monitor has to warn.
    b.moist.raw = 1400; // ~20% — below the vegetative dry band
    let mut warned = false;
    for _ in 0..12 {
        let cmd = b.tick();
        assert_eq!(cmd.stage, control::plant_profile::Stage::Vegetative);
        if cmd.state == SystemState::MoistureLow {
            warned = true;
        }
        // Passive: a dry warning must never cut the grow light driven through the GrowLed trait.
        assert!(
            b.led.power_pct > 0,
            "light must stay on under a moisture warning"
        );
    }
    assert!(warned, "a dry pot should surface MOISTURE_LOW via the seam");
    // Status LEDs were populated through the StatusLeds trait.
    assert_ne!(b.leds.get(control::hal::LedId::System).0, LedColor::Off);
}

#[test]
fn injected_leak_warns_and_reds_water_system() {
    let mut b = Bench::new();
    b.leak.wet = true; // inject leak via the mock
    for _ in 0..6 {
        let cmd = b.tick();
        assert_eq!(cmd.state, SystemState::LeakDetected);
    }
    // Water + System LEDs are red (flood warning).
    assert_eq!(b.leds.get(control::hal::LedId::Water).0, LedColor::Red);
    assert_eq!(b.leds.get(control::hal::LedId::System).0, LedColor::Red);
}

#[test]
fn injected_moisture_fault_raises_sensor_fault() {
    let mut b = Bench::new();
    b.moist.fault = Some(control::hal::SensorError::Bus); // inject sensor failure
    let cmd = b.tick();
    assert_eq!(cmd.state, SystemState::SensorFault);
    assert!(cmd.moisture_pct.is_none());
}
