//! Climate / fan controller: VPD calculation and the open-frame fan/LED response. Spec §9.7,
//! §5.3, §5.4. Implements `docs/vpd-climate-model.md`.
//!
//! This is a **monitor-and-nudge** system, not an HVAC: it never tries to cool below ambient. The
//! only heat source it controls is the LED (derate request handed to the light controller); the
//! fan provides circulation and heat dispersion, not active cooling.

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

/// Output of the climate controller for one tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClimateOutput {
    /// Commanded fan duty, percent 0..=100 (before any safety force-high).
    pub fan_pct: u8,
    pub vpd_kpa: f32,
    pub vpd_band: VpdBand,
    /// Climate LED should be amber (outside preferred range) — §9.8.
    pub climate_amber: bool,
    /// Climate LED should be red (critical temp/humidity) — §9.8.
    pub climate_red: bool,
    /// Request the light controller derate the LED (temp >32 °C) — §9.7.
    pub request_led_derate: bool,
    /// Fan tach missing while commanded on → FAN_FAULT (§9.7).
    pub fan_fault: bool,
}

/// Stage fan minimum for the current lighting condition (§9.7). During lights-off the fan runs
/// periodically: on for `fan_off_min_per_hour` minutes each hour. `minute_of_hour` selects the
/// phase deterministically so the behavior is testable without wall-clock drift.
pub fn fan_minimum(sp: &Setpoints, lights_on: bool, minute_of_hour: u8) -> u8 {
    if lights_on {
        sp.fan_min_on_pct
    } else if minute_of_hour < sp.fan_off_min_per_hour {
        sp.fan_min_off_pct
    } else {
        0
    }
}

/// Inputs needed beyond the air reading.
#[derive(Debug, Clone, Copy)]
pub struct ClimateInputs<'a> {
    pub air: TempRh,
    pub sp: &'a Setpoints,
    pub lights_on: bool,
    pub minute_of_hour: u8,
    /// Fan tach reading; `None` while commanded on → fault. `Some` only meaningful when fan runs.
    pub tach_rpm: Option<u16>,
}

/// Compute the fan duty and climate flags (§9.7, vpd-climate-model §4,§5).
///
/// Boosts are additive on the stage minimum. RH and temperature use the **highest applicable
/// tier** (not cumulative across tiers) so a humid, hot reading doesn't double-count; the VPD
/// boost is independent. Temp >32 °C pins the fan to max and requests an LED derate.
pub fn evaluate(inp: &ClimateInputs) -> ClimateOutput {
    let t = inp.air.temp_c;
    let rh = inp.air.rh_pct;
    let vpd = vpd_kpa(t, rh);
    let band = vpd_band(vpd);

    let base = fan_minimum(inp.sp, inp.lights_on, inp.minute_of_hour) as i32;
    let mut duty = base;
    let mut amber = false;
    let mut red = false;
    let mut derate = false;

    // --- RH tiers (highest applicable) ---
    if rh > 85.0 {
        duty += 30;
        amber = true; // §9.7 "amber climate"
    } else if rh > 75.0 && inp.lights_on {
        duty += 15;
    }
    // RH > 85 sustained leans toward red disease-risk; we surface red at the very high end.
    if rh > 90.0 {
        red = true;
    }

    // --- VPD boost (independent) ---
    if vpd < 0.5 {
        duty += 20;
    }
    if band == VpdBand::Stress {
        amber = true; // persistent stress risk — alert (vpd-climate-model §2)
    }

    // --- Temperature tiers (highest applicable, escalating) ---
    if t > 35.0 {
        duty = 100;
        red = true;
        derate = true; // critical handled by safety as OverTemp; still drive fan max + derate
    } else if t > 32.0 {
        duty = 100; // max fan
        red = true;
        derate = true; // LED derate (§9.7, §9.5)
    } else if t > 30.0 {
        duty += 40;
        amber = true;
    } else if t > 28.0 {
        duty += 20;
        if !inp.lights_on {
            // sustained warm at night still warrants attention
            amber = true;
        }
    } else if t < 16.0 {
        // cold: fan minimum only, climate amber (vpd-climate-model §4).
        amber = true;
    }

    // Outside the stage's preferred RH band (but not yet red) → amber.
    if !red && (rh < inp.sp.rh_min_pct || rh > inp.sp.rh_max_pct) {
        amber = true;
    }

    let fan_pct = clampf(duty as f32, 0.0, 100.0) as u8;

    // Fan-tach fault: commanded to spin (duty > 0) but no tach pulses.
    let fan_fault = fan_pct > 0 && inp.tach_rpm == Some(0);

    ClimateOutput {
        fan_pct,
        vpd_kpa: vpd,
        vpd_band: band,
        climate_amber: amber && !red,
        climate_red: red,
        request_led_derate: derate,
        fan_fault,
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

    fn veg_inputs(t: f32, rh: f32, _lights_on: bool) -> (crate::plant_profile::Setpoints, TempRh) {
        (
            setpoints(Stage::Vegetative),
            TempRh {
                temp_c: t,
                rh_pct: rh,
            },
        )
    }

    // §10.2 "Fan controller — temp/RH/VPD duty behavior".
    #[test]
    fn fan_at_stage_minimum_when_nominal() {
        let (sp, air) = veg_inputs(23.0, 60.0, true);
        let out = evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: true,
            minute_of_hour: 0,
            tach_rpm: Some(1200),
        });
        assert_eq!(out.fan_pct, sp.fan_min_on_pct); // 28%, no boosts
        assert!(!out.climate_amber && !out.climate_red);
    }

    #[test]
    fn high_rh_boosts_and_ambers() {
        let (sp, air) = veg_inputs(24.0, 80.0, true);
        let out = evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: true,
            minute_of_hour: 0,
            tach_rpm: Some(1200),
        });
        // 28 base + 15 (RH>75 lights on) = 43
        assert_eq!(out.fan_pct, 43);
        let (sp, air) = veg_inputs(24.0, 86.0, true);
        let out = evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: true,
            minute_of_hour: 0,
            tach_rpm: Some(1200),
        });
        // RH>85 tier (+30) AND VPD 0.42<0.5 (+20): 24°C/86% is humid enough to trip both.
        assert_eq!(out.fan_pct, 28 + 30 + 20);
        assert!(out.climate_amber);
    }

    #[test]
    fn low_vpd_adds_twenty() {
        // 24°C/90% → VPD 0.298 (<0.5): +20. RH 90 → red. Duty pinned by RH30 + VPD20.
        let (sp, air) = veg_inputs(24.0, 90.0, true);
        let out = evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: true,
            minute_of_hour: 0,
            tach_rpm: Some(1200),
        });
        assert_eq!(out.fan_pct, 28 + 30 + 20);
        assert!(out.vpd_kpa < 0.5);
    }

    #[test]
    fn temp_escalation_thresholds() {
        let sp = setpoints(Stage::Vegetative);
        let mk = |t: f32| {
            evaluate(&ClimateInputs {
                air: TempRh {
                    temp_c: t,
                    rh_pct: 60.0,
                },
                sp: &sp,
                lights_on: true,
                minute_of_hour: 0,
                tach_rpm: Some(1200),
            })
        };
        assert_eq!(mk(29.0).fan_pct, 28 + 20); // >28
        assert_eq!(mk(31.0).fan_pct, 28 + 40); // >30
        let hot = mk(33.0); // >32
        assert_eq!(hot.fan_pct, 100);
        assert!(hot.request_led_derate);
        assert!(hot.climate_red);
        let crit = mk(36.0); // >35
        assert_eq!(crit.fan_pct, 100);
        assert!(crit.request_led_derate && crit.climate_red);
    }

    #[test]
    fn fan_tach_missing_is_fault() {
        let (sp, air) = veg_inputs(23.0, 60.0, true);
        let out = evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: true,
            minute_of_hour: 0,
            tach_rpm: Some(0),
        });
        assert!(out.fan_fault);
    }

    #[test]
    fn lights_off_fan_is_periodic() {
        let sp = setpoints(Stage::Vegetative); // 8 min/hour off-period circulation
                                               // In the first 8 minutes of the hour the fan runs at the off-duty...
        assert_eq!(fan_minimum(&sp, false, 3), sp.fan_min_off_pct);
        // ...and is otherwise off.
        assert_eq!(fan_minimum(&sp, false, 30), 0);
    }

    #[test]
    fn never_cools_below_ambient_only_circulates() {
        // The controller never returns a "cooling" command; its only outputs are fan duty and a
        // derate *request*. Confirm a hot reading maxes the fan and asks for derate — no magic cooling.
        let sp = setpoints(Stage::Fruiting);
        let out = evaluate(&ClimateInputs {
            air: TempRh {
                temp_c: 34.0,
                rh_pct: 50.0,
            },
            sp: &sp,
            lights_on: true,
            minute_of_hour: 0,
            tach_rpm: Some(1500),
        });
        assert_eq!(out.fan_pct, 100);
        assert!(out.request_led_derate);
    }
}
