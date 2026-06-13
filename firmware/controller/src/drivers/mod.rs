//! Platform plumbing: the [`Platform`] aggregate that owns every peripheral binding, plus the
//! clock, RTC, watchdog and flash-backed calibration/age store. Spec §9.4, §9.9, §16.1.
//!
//! NOTE: the bodies below sketch the esp-hal 0.23 binding shape. Exact peripheral construction and
//! the pin map are finalized during electronics bring-up (WI-EE-08); this file fixes the *seam*
//! between those peripherals and the verified `control` logic.

use control::calibration;
use control::hal::{Clock, Rtc, WallTime};

use crate::actuators::{GrowLedPwm, PumpGpio, StatusLedDriver, VentFan};
use crate::sensors::{LeakGpio, MoistureAdc, ReservoirAdc, ShtAirSensor};

/// Monotonic millisecond clock backed by the ESP32-S3 system timer.
pub struct SystemClock {
    // held: esp_hal::timer::systimer::SystemTimer / Instant source
}
impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        // esp_hal::time::now() → Instant → millis since boot.
        esp_hal::time::now().duration_since_epoch().to_millis()
    }
}

/// Battery-backed RTC (DS3231/RV-3028-class, §16.1) read over I2C. Reports `valid == false` when
/// the oscillator-stop / power-loss flag is set, triggering the §9.4 safe-schedule fallback.
pub struct ExternalRtc {
    // held: shared I2C bus handle
}
impl Rtc for ExternalRtc {
    fn wall_time(&self) -> WallTime {
        // Read seconds + the oscillator-stop flag; convert BCD → unix seconds.
        // On bus error or OSF set → WallTime::INVALID.
        WallTime::INVALID // TODO(WI-EE-08): real DS3231 read
    }
}

/// Flash-backed calibration + grow-cycle-age store (esp-storage + sequential-storage, no NVS).
/// The codec/validation lives in `control::calibration`; this only does the flash IO.
pub struct CalibrationStore {
    // held: esp_storage::FlashStorage + partition offsets
}
impl CalibrationStore {
    /// Read the raw calibration record (`None` if the partition is empty/erased).
    pub fn load_raw(&self) -> Option<heapless::Vec<u8, { calibration::RECORD_LEN }>> {
        None // TODO(WI-EE-08): read RECORD_LEN bytes from the calibration partition
    }
    /// Persisted grow-cycle age (days) for §9.4 restore.
    pub fn load_age_days(&self) -> Option<u32> {
        None
    }
    pub fn store_age_days(&mut self, _age: u32) {
        // append-only write of the latest age
    }
}

/// Everything the control loop touches, constructed once at boot.
pub struct Platform {
    pub clock: SystemClock,
    pub rtc: ExternalRtc,
    pub calibration_store: CalibrationStore,
    pub temp_rh: ShtAirSensor,
    pub moisture: MoistureAdc,
    pub reservoir: ReservoirAdc,
    pub leak: LeakGpio,
    pub pump: PumpGpio,
    pub fan: VentFan,
    pub grow_led: GrowLedPwm,
    pub status_leds: StatusLedDriver,
    pub reservoir_low_adc: u16,
    last_age_persist: Option<u32>,
    #[cfg(feature = "telemetry")]
    pub telemetry: crate::drivers::telemetry::Telemetry,
}

impl Platform {
    /// Build all bindings from the raw peripherals. Pump/fan/LED are constructed OFF/min (§9.4).
    pub fn new(_peripherals: esp_hal::peripherals::Peripherals) -> Platform {
        unimplemented!("WI-EE-08 bring-up: construct peripherals from the finalized pin map")
    }

    /// Arm the RTC watchdog (RWDT). Feeding it each loop guarantees a hung loop reboots, on which
    /// the pump pull-down forces water off.
    pub fn enable_watchdog(&mut self) {}
    pub fn feed_watchdog(&mut self) {}

    /// Power-on self-test (§9.4 step 2): sanity-check sensor ranges, LED/fan/pump drivers.
    pub fn self_test(&mut self) -> bool {
        true
    }

    /// Optional LED heat-sink temperature (§9.5 NTC ladder); `None` if not fitted.
    pub fn led_heat(&mut self) -> Option<f32> {
        None
    }

    /// Light-sleep until the next tick to save power without losing the monotonic clock.
    pub fn idle_until(&mut self, _deadline_ms: u64) {}

    /// Persist grow-cycle age when it advances a day (avoids needless flash wear).
    pub fn persist_age_if_due(&mut self, age_days: u32) {
        if self.last_age_persist != Some(age_days) {
            self.calibration_store.store_age_days(age_days);
            self.last_age_persist = Some(age_days);
        }
    }
}

#[cfg(feature = "telemetry")]
pub mod telemetry {
    //! Optional Wi-Fi/MQTT/Home-Assistant telemetry + NTP (§9.11, §23 DR-05). DEFAULT-OFF and
    //! never on a control path: a network hiccup must not perturb watering or lighting. NTP is used
    //! only to discipline the RTC *when present*; offline operation uses the RTC/fallback alone.
    use control::app_state::Commands;
    use control::logging::OnboardLog;

    pub struct Telemetry {
        // held: esp-wifi controller + MQTT client
    }
    impl Telemetry {
        pub fn publish(&mut self, _cmd: &Commands, _log: &OnboardLog) {}
    }
}
