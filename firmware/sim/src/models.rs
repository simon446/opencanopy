//! Plant/environment models for the simulator (§10.3), **passive watering** (ECO-003).
//! **Engineering estimates only** — per WI-FW-09 and §23 (DR-02) these must be re-parameterized from
//! measured bench data ([WI-QA-09]) before the sim is trusted to gate a live-plant grow loop.
//! Passing scenarios proves the *control logic* (monitor + warn), not that reality matches the model.
//!
//! V1 has **no pump**: water reaches the substrate by capillary wicking from the base reservoir. The
//! model has two regimes — a healthy wick holds the substrate near an equilibrium while the reservoir
//! has water; once the reservoir empties (or the wick fails) the substrate dries out.
//!
//! Equations are intentionally simple and documented in `sim/models/README.md`.

use control::climate_controller::vpd_kpa;

/// Reservoir level (mL) at/below which the firmware should warn LOW_WATER (refill).
pub const RESERVOIR_LOW_ML: f32 = 500.0;
/// LED self-heating: +°C at 100% power. (No fan to disperse it; no pump to add load.)
pub const LED_HEAT_GAIN_C: f32 = 4.0;

/// Transpiration / water uptake (mL/min) — what the plant pulls from the substrate, drawn from the
/// reservoir through the wick. Higher under light and high VPD. Used to drain the reservoir.
pub const ET_LIGHT_ML_PER_MIN: f32 = 0.30;
pub const ET_DARK_ML_PER_MIN: f32 = 0.10;

/// Substrate-moisture dynamics (normalized %/tick). With a working wick and water in the reservoir
/// the substrate **dries through the day** (transpiration outruns the slow wick) and **rehydrates at
/// night** — a gentle diurnal sawtooth that stays in-band and, crucially, never plateaus (so it is
/// never mistaken for a stuck probe). When the wick fails or the reservoir runs dry there is no
/// replenishment and the substrate dries steadily at [`NOWATER_DROP_PCT_PER_TICK`].
pub const DAY_DRY_PCT_PER_TICK: f32 = 0.02;
pub const NIGHT_WET_PCT_PER_TICK: f32 = 0.04;
pub const NOWATER_DROP_PCT_PER_TICK: f32 = 0.05;

/// Water drawn from the reservoir over `dt_min` minutes (transpiration; §10.3).
pub fn transpiration_ml(dt_min: f32, light_on: bool, vpd: f32) -> f32 {
    let base = if light_on {
        ET_LIGHT_ML_PER_MIN
    } else {
        ET_DARK_ML_PER_MIN
    };
    let vpd_factor = if vpd > 1.2 { 1.5 } else { 1.0 };
    base * vpd_factor * dt_min
}

/// Air temperature seen by the sensor: room ambient + LED self-heating (§10.3: "LED increases
/// heat"). V1 has no circulation fan, so there is no dispersion term — the only way the device sheds
/// heat is by derating/cutting the LED.
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
