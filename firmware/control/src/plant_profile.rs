//! Hot-pepper lifecycle profile: grow-cycle age (days) → stage → setpoints. Spec §5.1, §5.2.
//!
//! Every number here is transcribed from the plant-science single source of truth and must not be
//! changed independently of it:
//!   - stages / age windows ........ `docs/plant-profile-hot-pepper.md` §1
//!   - PPFD / DLI / photoperiod ..... `docs/plant-profile-hot-pepper.md` §2, `docs/dli-targets.md`
//!   - RH / VPD bands ............... `docs/vpd-climate-model.md`
//!   - moisture thresholds / pulses / daily caps / recheck ... `docs/watering-model.md` §3,§4
//!
//! "If a target there is wrong, fix it there first, then re-derive this table — never the other
//! way around." (plant-profile-hot-pepper.md.)

/// Growth stage. Age-based selection only (V1 has no camera). `docs/plant-profile-hot-pepper.md` §1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// S0 Germination, day [0, 21).
    Germination,
    /// S1 Seedling, day [21, 56).
    Seedling,
    /// S2 Vegetative, day [56, 100).
    Vegetative,
    /// S3 Flowering, day [100, 140).
    Flowering,
    /// S4 Fruiting/ripening, day [140, ∞).
    Fruiting,
    /// S5 Maintenance/overwinter. **Never** entered by the age timer — dev/overwinter only.
    Maintenance,
}

impl Stage {
    /// Short stage code for logs/LEDs.
    pub const fn code(self) -> &'static str {
        match self {
            Stage::Germination => "S0",
            Stage::Seedling => "S1",
            Stage::Vegetative => "S2",
            Stage::Flowering => "S3",
            Stage::Fruiting => "S4",
            Stage::Maintenance => "S5",
        }
    }
}

/// Inclusive-low / exclusive-high day boundaries (plant-profile §1). Selecting a stage is a single
/// deterministic lookup on `age_days`.
const S1_START: u32 = 21;
const S2_START: u32 = 56;
const S3_START: u32 = 100;
const S4_START: u32 = 140;

/// `TRANSPLANT_PROFILE` build flag (§5.1). When a builder always starts from a purchased
/// transplant, the grow-cycle age is initialized to the S1→S2 boundary so S0/S1 are skipped while
/// all downstream age math is unchanged. Compile-time, not a runtime user setting.
#[cfg(feature = "transplant_profile")]
pub const TRANSPLANT_PROFILE: bool = true;
#[cfg(not(feature = "transplant_profile"))]
pub const TRANSPLANT_PROFILE: bool = false;

/// The age (days) a fresh grow-cycle reset starts at, honoring `TRANSPLANT_PROFILE`.
pub const fn reset_age_days() -> u32 {
    if TRANSPLANT_PROFILE {
        S2_START // start at day 56 = S2 Vegetative
    } else {
        0
    }
}

/// Map grow-cycle age in days to its stage (S5 is never returned here — it is a manual dev state).
pub fn stage_for_age(age_days: u32) -> Stage {
    if age_days < S1_START {
        Stage::Germination
    } else if age_days < S2_START {
        Stage::Seedling
    } else if age_days < S3_START {
        Stage::Vegetative
    } else if age_days < S4_START {
        Stage::Flowering
    } else {
        Stage::Fruiting
    }
}

/// Per-stage setpoints consumed by the light, irrigation and climate controllers. All fields are
/// `const`-table values from the plant-science docs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Setpoints {
    pub stage: Stage,
    // --- Light (plant-profile §2 / dli-targets.md) ---
    pub photoperiod_h: u8,
    pub ppfd_min: u16,
    pub ppfd_max: u16,
    // --- Climate (vpd-climate-model.md) ---
    pub rh_min_pct: f32,
    pub rh_max_pct: f32,
    pub vpd_min_kpa: f32,
    pub vpd_max_kpa: f32,
    // --- Irrigation (watering-model.md §3,§4) — normalized calibration %, NOT raw ADC ---
    /// Below this: dry, dose during the watering window.
    pub moisture_dry_pct: f32,
    /// Above this: too wet, block watering.
    pub moisture_wet_pct: f32,
    /// Well below dry: emergency watering allowed any time (design ← watering-model §2).
    pub moisture_critical_pct: f32,
    /// Representative normal pulse, mL (mid of the doc's per-stage range).
    pub normal_pulse_ml: u16,
    /// Emergency pulse when critically dry, mL (conservative — bottom of the range).
    pub emergency_pulse_ml: u16,
    /// Daily safety cap, mL/day (watering-model §4) — a ceiling, not a target.
    pub daily_max_ml: u16,
    /// Minutes to wait after a pulse before remeasuring (watering-model §3, mid of range).
    pub recheck_delay_min: u16,
}

// S0 has no automated watering row (kept warm/moist by hand until emergence, watering-model §3);
// its moisture fields mirror the seedling row so a mis-set age can never *increase* dosing.
const GERMINATION: Setpoints = Setpoints {
    stage: Stage::Germination,
    photoperiod_h: 16,
    ppfd_min: 0,
    ppfd_max: 100,
    rh_min_pct: 65.0,
    rh_max_pct: 80.0,
    vpd_min_kpa: 0.0,
    vpd_max_kpa: 0.9,
    moisture_dry_pct: 35.0,
    moisture_wet_pct: 55.0,
    moisture_critical_pct: 22.0,
    normal_pulse_ml: 30,
    emergency_pulse_ml: 20,
    daily_max_ml: 250,
    recheck_delay_min: 18,
};

const SEEDLING: Setpoints = Setpoints {
    stage: Stage::Seedling,
    photoperiod_h: 16,
    ppfd_min: 140,
    ppfd_max: 210,
    rh_min_pct: 60.0,
    rh_max_pct: 75.0,
    vpd_min_kpa: 0.5,
    vpd_max_kpa: 0.9,
    moisture_dry_pct: 35.0,
    moisture_wet_pct: 55.0,
    moisture_critical_pct: 22.0,
    normal_pulse_ml: 35, // 20–50 mL range
    emergency_pulse_ml: 20,
    daily_max_ml: 250,
    recheck_delay_min: 18, // 15–20 min
};

const VEGETATIVE: Setpoints = Setpoints {
    stage: Stage::Vegetative,
    photoperiod_h: 16,
    ppfd_min: 245,
    ppfd_max: 350,
    rh_min_pct: 55.0,
    rh_max_pct: 75.0,
    vpd_min_kpa: 0.7,
    vpd_max_kpa: 1.2,
    moisture_dry_pct: 30.0,
    moisture_wet_pct: 55.0,
    moisture_critical_pct: 17.0,
    normal_pulse_ml: 100, // 50–150 mL
    emergency_pulse_ml: 50,
    daily_max_ml: 800,
    recheck_delay_min: 25, // 20–30 min
};

const FLOWERING: Setpoints = Setpoints {
    stage: Stage::Flowering,
    photoperiod_h: 16,
    ppfd_min: 315,
    ppfd_max: 420,
    rh_min_pct: 55.0,
    rh_max_pct: 70.0,
    vpd_min_kpa: 0.8,
    vpd_max_kpa: 1.2,
    moisture_dry_pct: 35.0,
    moisture_wet_pct: 60.0,
    moisture_critical_pct: 22.0,
    normal_pulse_ml: 140, // 75–200 mL
    emergency_pulse_ml: 75,
    daily_max_ml: 1200,
    recheck_delay_min: 25,
};

const FRUITING: Setpoints = Setpoints {
    stage: Stage::Fruiting,
    photoperiod_h: 16,
    ppfd_min: 350,
    ppfd_max: 435,
    rh_min_pct: 55.0,
    rh_max_pct: 70.0,
    vpd_min_kpa: 0.8,
    vpd_max_kpa: 1.3,
    moisture_dry_pct: 35.0,
    moisture_wet_pct: 60.0,
    moisture_critical_pct: 22.0,
    normal_pulse_ml: 175, // 100–250 mL
    emergency_pulse_ml: 100,
    daily_max_ml: 1800,
    recheck_delay_min: 25,
};

// S5 Maintenance: lower light/water survival mode (design ← R4). Conservative, low caps.
const MAINTENANCE: Setpoints = Setpoints {
    stage: Stage::Maintenance,
    photoperiod_h: 12,
    ppfd_min: 100,
    ppfd_max: 200,
    rh_min_pct: 50.0,
    rh_max_pct: 70.0,
    vpd_min_kpa: 0.7,
    vpd_max_kpa: 1.3,
    moisture_dry_pct: 28.0,
    moisture_wet_pct: 50.0,
    moisture_critical_pct: 15.0,
    normal_pulse_ml: 60,
    emergency_pulse_ml: 40,
    daily_max_ml: 400,
    recheck_delay_min: 30,
};

/// Look up the immutable setpoints for a stage.
pub const fn setpoints(stage: Stage) -> Setpoints {
    match stage {
        Stage::Germination => GERMINATION,
        Stage::Seedling => SEEDLING,
        Stage::Vegetative => VEGETATIVE,
        Stage::Flowering => FLOWERING,
        Stage::Fruiting => FRUITING,
        Stage::Maintenance => MAINTENANCE,
    }
}

/// Convenience: setpoints for a grow-cycle age in days.
pub fn setpoints_for_age(age_days: u32) -> Setpoints {
    setpoints(stage_for_age(age_days))
}

#[cfg(test)]
mod tests {
    use super::*;

    // §10.2 "Plant profile — stage selection by age": exercise every boundary, inclusive-low /
    // exclusive-high (plant-profile §1).
    #[test]
    fn stage_boundaries_are_inclusive_low_exclusive_high() {
        assert_eq!(stage_for_age(0), Stage::Germination);
        assert_eq!(stage_for_age(20), Stage::Germination);
        assert_eq!(stage_for_age(21), Stage::Seedling); // S0->S1 boundary
        assert_eq!(stage_for_age(55), Stage::Seedling);
        assert_eq!(stage_for_age(56), Stage::Vegetative); // S1->S2
        assert_eq!(stage_for_age(99), Stage::Vegetative);
        assert_eq!(stage_for_age(100), Stage::Flowering); // S2->S3
        assert_eq!(stage_for_age(139), Stage::Flowering);
        assert_eq!(stage_for_age(140), Stage::Fruiting); // S3->S4
        assert_eq!(stage_for_age(365), Stage::Fruiting);
        assert_eq!(stage_for_age(10_000), Stage::Fruiting);
    }

    #[test]
    fn maintenance_is_never_age_selected() {
        // S5 must never come from the age timer (plant-profile §1).
        for age in (0..400).step_by(7) {
            assert_ne!(stage_for_age(age), Stage::Maintenance);
        }
    }

    #[test]
    fn transplant_flag_default_starts_at_day_zero() {
        // Default public build (transplant_profile feature off) includes all stages and starts at
        // day 0 / S0. `reset_age_days()` reflects the flag, so asserting on it covers both builds.
        assert_eq!(
            reset_age_days(),
            if TRANSPLANT_PROFILE { S2_START } else { 0 }
        );
        #[cfg(not(feature = "transplant_profile"))]
        {
            assert_eq!(reset_age_days(), 0);
            assert_eq!(stage_for_age(reset_age_days()), Stage::Germination);
        }
    }

    #[test]
    fn setpoints_match_plant_science_tables() {
        // Spot-check the irrigation + light numbers against the docs (single source of truth).
        let veg = setpoints(Stage::Vegetative);
        assert_eq!(veg.ppfd_min, 245);
        assert_eq!(veg.ppfd_max, 350);
        assert_eq!(veg.daily_max_ml, 800);
        assert_eq!(veg.moisture_dry_pct, 30.0);

        let fruit = setpoints(Stage::Fruiting);
        assert_eq!(fruit.daily_max_ml, 1800);
        assert_eq!(fruit.ppfd_max, 435);
        assert_eq!(fruit.photoperiod_h, 16);

        let seed = setpoints(Stage::Seedling);
        assert_eq!(seed.daily_max_ml, 250);
        assert_eq!(seed.moisture_wet_pct, 55.0);
    }

    #[test]
    fn critical_is_below_dry_for_every_stage() {
        for s in [
            Stage::Germination,
            Stage::Seedling,
            Stage::Vegetative,
            Stage::Flowering,
            Stage::Fruiting,
            Stage::Maintenance,
        ] {
            let sp = setpoints(s);
            assert!(
                sp.moisture_critical_pct < sp.moisture_dry_pct,
                "critical must be below dry for {}",
                s.code()
            );
            assert!(sp.moisture_dry_pct < sp.moisture_wet_pct);
        }
    }
}
