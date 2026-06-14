//! Plant/environment models for the simulator (§10.3). **Engineering estimates only** — per
//! WI-FW-09 and §23 (DR-02) these must be re-parameterized from measured bench data
//! ([WI-QA-09]) before the sim is trusted to gate a live-plant grow loop. Passing scenarios proves
//! the *control logic*, not that reality matches the model.
//!
//! Equations are intentionally simple and documented in `sim/models/README.md`.

use control::climate_controller::vpd_kpa;

/// Physical/behavioral constants (estimates).
pub const POT_ML_PER_PCT: f32 = 15.0; // mL of water to move normalized moisture by 1%
pub const SOAK_MS: u64 = 8 * 60_000; // water takes ~8 min to register on the probe
pub const RESERVOIR_LOW_ML: f32 = 300.0; // low-water lockout level
/// Moisture decline rate (normalized %/min) with lights on, at nominal VPD.
pub const DECLINE_LIGHT_PCT_PER_MIN: f32 = 0.012;
/// Moisture decline rate with lights off.
pub const DECLINE_DARK_PCT_PER_MIN: f32 = 0.004;
/// LED self-heating: +°C at 100% power.
pub const LED_HEAT_GAIN_C: f32 = 4.0;

/// Moisture decline over `dt_min` minutes (§10.3: faster under light and high VPD).
pub fn moisture_decline(dt_min: f32, light_on: bool, vpd: f32) -> f32 {
    let base = if light_on {
        DECLINE_LIGHT_PCT_PER_MIN
    } else {
        DECLINE_DARK_PCT_PER_MIN
    };
    let vpd_factor = if vpd > 1.2 { 1.5 } else { 1.0 };
    base * vpd_factor * dt_min
}

/// Air temperature seen by the sensor: room ambient + LED self-heating (§10.3: "LED increases
/// heat"). V1 has no circulation fan, so there is no dispersion term — the only way the device
/// sheds heat is by derating/cutting the LED.
pub fn air_temp(room_temp_c: f32, led_pct: u8) -> f32 {
    room_temp_c + (led_pct as f32 / 100.0) * LED_HEAT_GAIN_C
}

/// Relative humidity seen by the sensor: room ambient (no fan to disturb it in V1).
pub fn air_rh(room_rh_pct: f32) -> f32 {
    room_rh_pct.clamp(0.0, 100.0)
}

/// Convenience: VPD from the modeled air temp/RH (matches the controller's own formula).
pub fn modeled_vpd(room_temp_c: f32, room_rh_pct: f32, led_pct: u8) -> f32 {
    vpd_kpa(air_temp(room_temp_c, led_pct), air_rh(room_rh_pct))
}
