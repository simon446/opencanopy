//! Climate monitor: VPD calculation and temp/humidity health flags. Spec §9.7, §5.3, §5.4.
//! Implements `docs/vpd-climate-model.md`.
//!
//! V1 has **no circulation fan** (removed from the mechanical/electrical design). This module
//! therefore commands no actuator of its own — it is pure observation: it computes VPD, classifies
//! the air against the stage's preferred band for the Climate status LED, and (the one thing it can
//! still influence) **requests an LED derate** when the air runs hot, since the grow LED is now the
//! only heat source — and only lever — the device controls.

use crate::hal::TempRh;
use crate::math::{clampf, exp};
use crate::plant_profile::Setpoints;

/// Saturation vapour pressure via the Tetens equation, kPa, `T` in °C (vpd-climate-model §1).
pub fn svp_kpa(temp_c: f32) -> f32 {
    let t = temp_c as f64;
    (0.6108 * exp(17.27 * t / (t + 237.3))) as f32
}

/// Air vapour-pressure deficit, kPa (vpd-climate-model §1). RH is clamped to `[0, 100]`; an
/// out-of-range RH is a sensor fault upstream (§7.6), not a valid VPD input.
pub fn vpd_kpa(temp_c: f32, rh_pct: f32) -> f32 {
    let rh = clampf(rh_pct, 0.0, 100.0);
    svp_kpa(temp_c) * (1.0 - rh / 100.0)
}

/// VPD interpretation bands (vpd-climate-model §2). `[0.4,0.5)` is treated as the bottom edge of
/// the normal band (no humid-specific action), per the doc's note.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpdBand {
    /// < 0.4 — air too humid / low transpiration.
    TooHumid,
    /// 0.4–1.2 — normal productive range.
    Normal,
    /// 1.2–1.6 — dry air / high transpiration.
    DryAir,
    /// > 1.6 — stress risk.
    Stress,
}

pub fn vpd_band(vpd: f32) -> VpdBand {
    if vpd < 0.4 {
        VpdBand::TooHumid
    } else if vpd < 1.2 {
        VpdBand::Normal
    } else if vpd <= 1.6 {
        VpdBand::DryAir
    } else {
        VpdBand::Stress
    }
}

/// Output of the climate monitor for one tick. No actuator command — V1 has no fan.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClimateOutput {
    pub vpd_kpa: f32,
    pub vpd_band: VpdBand,
    /// Climate LED should be amber (outside preferred range) — §9.8.
    pub climate_amber: bool,
    /// Climate LED should be red (critical temp/humidity) — §9.8.
    pub climate_red: bool,
    /// Request the light controller derate the LED (temp >32 °C) — §9.7, §9.5. With no fan, cutting
    /// the LED is the only way to shed heat.
    pub request_led_derate: bool,
}

/// Inputs needed beyond the air reading.
#[derive(Debug, Clone, Copy)]
pub struct ClimateInputs<'a> {
    pub air: TempRh,
    pub sp: &'a Setpoints,
    /// Whether the grow lights are on — affects only the "sustained warm at night" amber nuance.
    pub lights_on: bool,
}

/// Classify the air and decide the climate health flags + LED-derate request (§9.7, vpd-climate-
/// model §4,§5). With the fan gone there is no duty to compute; temperature tiers now only raise
/// the amber/red health flags and, past 32 °C, ask the light controller to derate.
pub fn evaluate(inp: &ClimateInputs) -> ClimateOutput {
    let t = inp.air.temp_c;
    let rh = inp.air.rh_pct;
    let vpd = vpd_kpa(t, rh);
    let band = vpd_band(vpd);

    let mut amber = false;
    let mut red = false;
    let mut derate = false;

    // --- RH health ---
    if rh > 85.0 {
        amber = true; // §9.7 "amber climate"
    }
    // RH > 90 sustained leans toward red disease-risk.
    if rh > 90.0 {
        red = true;
    }

    // --- VPD ---
    if band == VpdBand::Stress {
        amber = true; // persistent stress risk — alert (vpd-climate-model §2)
    }

    // --- Temperature tiers (highest applicable, escalating) ---
    if t > 32.0 {
        // >32 °C (and the >35 °C critical case handled by safety as OverTemp): red + LED derate,
        // the only heat lever left without a fan (§9.7, §9.5).
        red = true;
        derate = true;
    } else if t > 30.0 {
        amber = true;
    } else if t > 28.0 && !inp.lights_on {
        // sustained warm at night still warrants attention
        amber = true;
    } else if t < 16.0 {
        // cold: climate amber (vpd-climate-model §4).
        amber = true;
    }

    // Outside the stage's preferred RH band (but not yet red) → amber.
    if !red && (rh < inp.sp.rh_min_pct || rh > inp.sp.rh_max_pct) {
        amber = true;
    }

    ClimateOutput {
        vpd_kpa: vpd,
        vpd_band: band,
        climate_amber: amber && !red,
        climate_red: red,
        request_led_derate: derate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plant_profile::{setpoints, Stage};

    fn round4(v: f32) -> f32 {
        (v * 10_000.0).round() / 10_000.0
    }

    // §10.2 "VPD calculator — temp/RH to kPa". Reference vectors from vpd-climate-model §1.1,
    // compared to 4 decimal places.
    #[test]
    fn vpd_reference_vectors() {
        let cases = [
            (24.0, 90.0, 0.2984),
            (24.0, 85.0, 0.4476),
            (18.0, 75.0, 0.5160),
            (20.0, 60.0, 0.9353),
            (24.0, 65.0, 1.0444),
            (22.0, 60.0, 1.0576),
            (22.0, 55.0, 1.1898),
            (25.0, 60.0, 1.2671),
            (26.0, 55.0, 1.5126),
            (25.0, 50.0, 1.5839),
            (30.0, 40.0, 2.5458),
        ];
        for (t, rh, want) in cases {
            assert_eq!(round4(vpd_kpa(t, rh)), want, "VPD({t},{rh})");
        }
    }

    #[test]
    fn svp_anchors() {
        assert_eq!(round4(svp_kpa(20.0)), 2.3383);
        assert_eq!(round4(svp_kpa(25.0)), 3.1678);
        assert_eq!(round4(svp_kpa(30.0)), 4.2431);
    }

    #[test]
    fn vpd_clamps_out_of_range_rh() {
        // RH>100 would otherwise give negative VPD; clamp keeps it at 0.
        assert_eq!(vpd_kpa(24.0, 120.0), 0.0);
    }

    #[test]
    fn vpd_bands() {
        assert_eq!(vpd_band(0.30), VpdBand::TooHumid);
        assert_eq!(vpd_band(0.45), VpdBand::Normal); // [0.4,0.5) is bottom of normal
        assert_eq!(vpd_band(1.0), VpdBand::Normal);
        assert_eq!(vpd_band(1.4), VpdBand::DryAir);
        assert_eq!(vpd_band(2.0), VpdBand::Stress);
    }

    fn eval(t: f32, rh: f32, lights_on: bool) -> ClimateOutput {
        let sp = setpoints(Stage::Vegetative);
        evaluate(&ClimateInputs {
            air: TempRh {
                temp_c: t,
                rh_pct: rh,
            },
            sp: &sp,
            lights_on,
        })
    }

    // §10.2 "Climate monitor — temp/RH/VPD health classification".
    #[test]
    fn nominal_air_is_neither_amber_nor_red() {
        let out = eval(23.0, 60.0, true);
        assert!(!out.climate_amber && !out.climate_red);
        assert!(!out.request_led_derate);
    }

    #[test]
    fn high_rh_ambers_and_very_high_reds() {
        // RH>85 → amber climate.
        assert!(eval(24.0, 86.0, true).climate_amber);
        // RH>90 → red (disease risk); red supersedes amber.
        let out = eval(24.0, 92.0, true);
        assert!(out.climate_red);
        assert!(!out.climate_amber);
    }

    #[test]
    fn low_vpd_is_humid_band() {
        // 24°C/90% → VPD ~0.30 (<0.4): too-humid band, and RH 90 just under the red line.
        let out = eval(24.0, 90.0, true);
        assert!(out.vpd_kpa < 0.4);
        assert_eq!(out.vpd_band, VpdBand::TooHumid);
    }

    #[test]
    fn temp_escalation_thresholds() {
        // <16 cold → amber; 28–30 by day → no amber; 28–30 at night → amber; >30 → amber;
        // >32 → red + LED derate (the only heat lever without a fan). RH 72 % keeps VPD in the
        // Normal band and inside the preferred RH window, isolating the temperature behavior.
        assert!(eval(15.0, 60.0, true).climate_amber); // cold
        assert!(!eval(29.0, 72.0, true).climate_amber); // warm, lights on: not yet flagged
        assert!(eval(29.0, 72.0, false).climate_amber); // warm at night
        assert!(eval(31.0, 72.0, true).climate_amber); // >30
        let hot = eval(33.0, 60.0, true); // >32
        assert!(hot.request_led_derate);
        assert!(hot.climate_red);
        let crit = eval(36.0, 60.0, true); // >35 (safety also raises OverTemp)
        assert!(crit.request_led_derate && crit.climate_red);
    }

    #[test]
    fn hot_air_only_requests_derate_no_actuator() {
        // The monitor's sole influence is the LED-derate request — there is no fan to spin up. A hot
        // reading asks the light controller to shed heat and reds the climate LED; nothing more.
        let out = eval(34.0, 50.0, true);
        assert!(out.request_led_derate);
        assert!(out.climate_red);
    }

    #[test]
    fn outside_preferred_rh_band_ambers() {
        // Vegetative band is 55–75 % RH; 50 % is below it → amber even when temp is fine.
        let out = eval(23.0, 50.0, true);
        assert!(out.climate_amber);
    }
}
