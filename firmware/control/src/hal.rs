//! Hardware-abstraction layer: the **seam** between platform-agnostic control logic and the
//! concrete world (`controller/` = esp-hal peripherals, `sim/` = simulated models). Spec §10.1.
//!
//! Every controller in this crate is generic over these traits and depends on *nothing* else —
//! no `esp-hal`, no concrete driver. That is what lets the whole §10.2 unit-test matrix and the
//! §10.3 simulator run on the host. We prefer plain project traits with `&mut self` reads over
//! `embedded-hal` here for two reasons: (1) our sensors return domain values (°C, %RH, raw ADC),
//! not just bus transactions, and (2) staying dep-free keeps host tests offline. Where a standard
//! `embedded-hal` trait fits a concrete driver (e.g. PWM, ADC one-shot), `controller/` adapts it
//! to these traits at the binding layer.
//!
//! All traits are object-safe-agnostic: controllers take them by generic `&mut T: Trait`, so the
//! on-target build monomorphizes with no heap and no `dyn` (WI-FW-02 note).

/// Reason a sensor read could not produce a trustworthy value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorError {
    /// Bus/IO failure talking to the device.
    Bus,
    /// Device responded but the value is outside the physically plausible range (§7.6).
    OutOfRange,
    /// Reading has not changed across the plausibility window — likely stuck/disconnected (§10.3).
    Stuck,
    /// No device is present / not wired (optional sensors).
    NotPresent,
}

/// A temperature + relative-humidity sample from the SHT-class air sensor (§7.5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TempRh {
    pub temp_c: f32,
    /// Relative humidity, percent 0..=100.
    pub rh_pct: f32,
}

/// Authoritative wall-clock reading from the battery-backed RTC (§16.1). `valid == false` means
/// the RTC lost time (dead coin cell / first boot) and the safe-schedule fallback applies (§9.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WallTime {
    pub valid: bool,
    /// Seconds since the Unix epoch (only meaningful when `valid`).
    pub unix_s: u64,
}

impl WallTime {
    pub const INVALID: WallTime = WallTime {
        valid: false,
        unix_s: 0,
    };

    /// Local seconds-since-midnight, given a fixed UTC offset in seconds. V1 has no DST handling;
    /// the light schedule only needs a stable local time-of-day.
    pub fn local_seconds_of_day(&self, utc_offset_s: i32) -> u32 {
        let local = (self.unix_s as i64 + utc_offset_s as i64).rem_euclid(86_400);
        local as u32
    }
}

// ---- Time ----------------------------------------------------------------------------------

/// Monotonic millisecond clock since boot. Injected so schedules and timeouts are deterministic
/// in tests (WI-FW-02). Never goes backwards; unaffected by RTC validity.
pub trait Clock {
    fn now_ms(&self) -> u64;
}

/// Battery-backed real-time clock providing wall-clock time for the photoperiod and log
/// timestamps (§16.1, §9.10). Separate from [`Clock`]: the RTC can be invalid while the
/// monotonic clock keeps running.
pub trait Rtc {
    fn wall_time(&self) -> WallTime;
}

// ---- Sensors -------------------------------------------------------------------------------

pub trait TempRhSensor {
    fn read(&mut self) -> Result<TempRh, SensorError>;
}

/// Capacitive substrate-moisture probe. Returns a **raw** ADC count; normalization to 0..100 is a
/// calibration concern (§9.9, [`crate::calibration`]) — the firmware must never treat raw counts
/// as moisture percent (watering-model.md §3).
pub trait MoistureSensor {
    fn read_raw(&mut self) -> Result<u16, SensorError>;
}

/// Reservoir level. Coarse low/not-low is the safety-critical signal; the raw ADC supports the
/// `reservoir_low_adc` calibration point (§9.9).
pub trait ReservoirSensor {
    fn read_adc(&mut self) -> Result<u16, SensorError>;
}

/// Leak / spill sensor in the catch tray. Conservative: any wet reading latches a leak (§11.4).
pub trait LeakSensor {
    fn is_wet(&mut self) -> bool;
}

/// Optional NTC on the LED heat-sink for the thermal-derate ladder (§9.5). `None` = not fitted.
pub trait LedHeatSensor {
    fn temp_c(&mut self) -> Option<f32>;
}

// ---- Actuators -----------------------------------------------------------------------------

// V1 has no pump (ECO-003: passive self-watering) and no fan (ECO-001). The grow LED is the only
// actuator; watering is monitored and warned, never actuated.

/// Dimmable grow LED. Percent is *commanded* power; the PPFD it yields comes from the
/// `led_ppfd_map` calibration (§9.9).
pub trait GrowLed {
    fn set_power(&mut self, pct: u8);
}

/// Identity of each of the 4 front status LEDs (§3.5, §9.8; ECO-003 dropped the Climate LED).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedId {
    Water,
    Moisture,
    Light,
    System,
}

/// Colorblind-safe status LED driver. Color is *augmented* by position + pattern, never relied on
/// alone (§7.11, §9.8).
pub trait StatusLeds {
    fn set(
        &mut self,
        id: LedId,
        color: crate::led_status::LedColor,
        pattern: crate::led_status::LedPattern,
    );
}
