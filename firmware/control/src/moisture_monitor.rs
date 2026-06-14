//! Moisture monitor (V1 passive watering, ECO-003). Spec §9.6, §5.6; `docs/watering-model.md`.
//!
//! V1 has **no pump** — watering is passive (reservoir + wick + slotted insert). This module
//! therefore does **not** actuate anything. It (1) validates the raw capacitive reading into a
//! trustworthy normalized value or a sensor fault, and (2) classifies that value against the stage's
//! moisture band so the controller can **warn** the user (Moisture LED / state):
//!
//!   - below critical → `CriticalLow` (the passive supply is failing — check reservoir/wick),
//!   - below dry      → `Low`,
//!   - in band        → `Ok`,
//!   - above wet      → `High` (water-logging risk; an insert with an air gap should prevent this),
//!   - invalid/stuck  → `Fault` (SENSOR_FAULT).
//!
//! It replaces the former pulse-dosing `irrigation_controller` (pump removed in ECO-003).

use crate::calibration::Calibration;
use crate::hal::SensorError;
use crate::plant_profile::Setpoints;

const HOUR_MS: u64 = 3_600_000;
/// How long a moisture reading may be perfectly unchanged before it is deemed stuck (§10.3
/// "sensor fault after plausibility window"). In a live grow, moisture always drifts.
const STUCK_WINDOW_MS: u64 = 6 * HOUR_MS;

/// Health of the substrate moisture, for the warning logic. Ordered worst→best is not implied; the
/// caller maps each variant to a state/LED.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoistureStatus {
    /// Sensor invalid / stuck / uncalibrated → SENSOR_FAULT (no value to trust).
    Fault,
    /// Below the critical threshold — the passive supply is not keeping the root zone wet.
    CriticalLow,
    /// Below the dry threshold — drying out; warn before it becomes critical.
    Low,
    /// Within the target band.
    Ok,
    /// Above the wet threshold — water-logging risk.
    High,
}

impl MoistureStatus {
    /// True for any state the user should be warned about (anything but `Ok`).
    pub fn is_warning(self) -> bool {
        !matches!(self, MoistureStatus::Ok)
    }
}

/// Classify a (already validated) moisture value against the stage band. `None` → `Fault`.
pub fn classify(moisture: Option<f32>, sp: &Setpoints) -> MoistureStatus {
    match moisture {
        None => MoistureStatus::Fault,
        Some(m) if m < sp.moisture_critical_pct => MoistureStatus::CriticalLow,
        Some(m) if m < sp.moisture_dry_pct => MoistureStatus::Low,
        Some(m) if m > sp.moisture_wet_pct => MoistureStatus::High,
        Some(_) => MoistureStatus::Ok,
    }
}

/// Validates raw moisture readings into a trustworthy normalized value, or `None` (= sensor fault).
/// Catches three §7.6/§10.3 conditions: bus/range errors, an uncalibrated/implausible normalize,
/// and a **stuck** reading that has not moved across the plausibility window.
#[derive(Debug, Clone, Copy)]
pub struct MoistureValidator {
    last_raw: Option<u16>,
    unchanged_since_ms: u64,
    window_ms: u64,
}

impl Default for MoistureValidator {
    fn default() -> Self {
        Self::new(STUCK_WINDOW_MS)
    }
}

impl MoistureValidator {
    pub const fn new(window_ms: u64) -> Self {
        MoistureValidator {
            last_raw: None,
            unchanged_since_ms: 0,
            window_ms,
        }
    }

    /// Process a raw read. `moisture_trusted` reflects calibration validity (§7.6): if false,
    /// moisture is never trusted regardless of the reading (no valid dry/wet span to normalize
    /// against).
    pub fn validate(
        &mut self,
        now_ms: u64,
        read: Result<u16, SensorError>,
        cal: &Calibration,
        moisture_trusted: bool,
    ) -> Option<f32> {
        if !moisture_trusted {
            return None;
        }
        let raw = read.ok()?;
        match self.last_raw {
            Some(prev) if prev == raw => {
                if now_ms.saturating_sub(self.unchanged_since_ms) >= self.window_ms {
                    return None; // stuck: unchanged across the plausibility window
                }
            }
            _ => {
                self.last_raw = Some(raw);
                self.unchanged_since_ms = now_ms;
            }
        }
        cal.normalize_moisture(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plant_profile::{setpoints, Stage};

    fn veg() -> Setpoints {
        setpoints(Stage::Vegetative)
    }

    // §10.2 "Moisture monitor — band classification".
    #[test]
    fn classify_bands() {
        let sp = veg(); // dry 30, wet 55, critical 17
        assert_eq!(classify(None, &sp), MoistureStatus::Fault);
        assert_eq!(classify(Some(10.0), &sp), MoistureStatus::CriticalLow);
        assert_eq!(classify(Some(25.0), &sp), MoistureStatus::Low);
        assert_eq!(classify(Some(45.0), &sp), MoistureStatus::Ok);
        assert_eq!(classify(Some(60.0), &sp), MoistureStatus::High);
    }

    #[test]
    fn only_ok_is_not_a_warning() {
        let sp = veg();
        assert!(!classify(Some(45.0), &sp).is_warning());
        assert!(classify(Some(25.0), &sp).is_warning());
        assert!(classify(None, &sp).is_warning());
    }

    fn good_cal() -> Calibration {
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

    #[test]
    fn untrusted_calibration_yields_no_value() {
        let cal = good_cal();
        let mut v = MoistureValidator::default();
        // moisture_trusted = false → always None regardless of reading.
        assert!(v.validate(0, Ok(2000), &cal, false).is_none());
    }

    #[test]
    fn bus_error_is_none() {
        let cal = good_cal();
        let mut v = MoistureValidator::default();
        assert!(v.validate(0, Err(SensorError::Bus), &cal, true).is_none());
    }

    #[test]
    fn stuck_reading_flagged_after_window() {
        let cal = good_cal();
        let mut v = MoistureValidator::new(HOUR_MS);
        assert!(v.validate(0, Ok(2000), &cal, true).is_some());
        assert!(v.validate(HOUR_MS / 2, Ok(2000), &cal, true).is_some());
        // Past the window with no change → stuck → None.
        assert!(v.validate(HOUR_MS + 1, Ok(2000), &cal, true).is_none());
    }

    #[test]
    fn drifting_reading_stays_valid() {
        let cal = good_cal();
        let mut v = MoistureValidator::new(HOUR_MS);
        assert!(v.validate(0, Ok(2000), &cal, true).is_some());
        // A changed reading resets the stuck timer, so it never trips while drifting.
        assert!(v.validate(HOUR_MS + 1, Ok(2100), &cal, true).is_some());
        assert!(v.validate(2 * HOUR_MS + 2, Ok(2200), &cal, true).is_some());
    }
}
