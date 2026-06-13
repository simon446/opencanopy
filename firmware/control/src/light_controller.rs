//! Light controller: photoperiod schedule, 30-min intensity ramps, RTC-fallback schedule, and LED
//! thermal derating. Spec §9.5, §9.4 (RTC fallback), §5.2.

use crate::calibration::Calibration;
use crate::hal::WallTime;
use crate::plant_profile::Setpoints;

/// Default lights-on local time: 06:00 (§9.5).
const DEFAULT_ON_SECONDS: u32 = 6 * 3600;
/// Intensity ramp duration at lights-on and lights-off (§9.5).
const RAMP_SECONDS: u32 = 30 * 60;
/// Conservative RTC-invalid fallback: 16 h on / 8 h off from boot (§9.4).
const FALLBACK_ON_SECONDS: u32 = 16 * 3600;
const FALLBACK_CYCLE_SECONDS: u32 = 24 * 3600;

/// Where in the photoperiod we are, independent of which schedule produced it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phase {
    pub on: bool,
    /// Seconds since lights-on (0 when off).
    pub seconds_into_on: u32,
    /// Length of the on-period in seconds.
    pub on_secs: u32,
    /// True when running the RTC-invalid safe fallback (§9.4).
    pub fallback: bool,
}

impl Phase {
    /// Fraction of the light period elapsed, 0.0 at lights-on → 1.0 at lights-off. Used by the
    /// irrigation watering-window logic (watering-model §5).
    pub fn fraction(&self) -> f32 {
        if !self.on || self.on_secs == 0 {
            0.0
        } else {
            self.seconds_into_on as f32 / self.on_secs as f32
        }
    }

    /// Hours remaining until lights-off (watering-model §5).
    pub fn hours_to_off(&self) -> f32 {
        if !self.on {
            0.0
        } else {
            (self.on_secs.saturating_sub(self.seconds_into_on)) as f32 / 3600.0
        }
    }
}

/// Resolve the current photoperiod phase from the RTC (preferred) or the boot-relative fallback.
pub fn phase(sp: &Setpoints, rtc: WallTime, utc_offset_s: i32, boot_ms: u64, now_ms: u64) -> Phase {
    if rtc.valid {
        let sod = rtc.local_seconds_of_day(utc_offset_s);
        let on_secs = sp.photoperiod_h as u32 * 3600;
        // Relative position within the [on_start, on_start+on_secs) window, mod a day.
        let rel = (sod + 86_400 - DEFAULT_ON_SECONDS) % 86_400;
        if rel < on_secs {
            Phase {
                on: true,
                seconds_into_on: rel,
                on_secs,
                fallback: false,
            }
        } else {
            Phase {
                on: false,
                seconds_into_on: 0,
                on_secs,
                fallback: false,
            }
        }
    } else {
        // Fallback: fixed 16/8 from boot (§9.4), regardless of stage photoperiod.
        let elapsed_s = (now_ms.saturating_sub(boot_ms) / 1000) as u32;
        let into_cycle = elapsed_s % FALLBACK_CYCLE_SECONDS;
        if into_cycle < FALLBACK_ON_SECONDS {
            Phase {
                on: true,
                seconds_into_on: into_cycle,
                on_secs: FALLBACK_ON_SECONDS,
                fallback: true,
            }
        } else {
            Phase {
                on: false,
                seconds_into_on: 0,
                on_secs: FALLBACK_ON_SECONDS,
                fallback: true,
            }
        }
    }
}

/// Linear ramp factor in [0,1]: ramps up over the first [`RAMP_SECONDS`] after lights-on and down
/// over the last [`RAMP_SECONDS`] before lights-off (§9.5). 0 when lights are off.
pub fn ramp_factor(ph: &Phase) -> f32 {
    if !ph.on {
        return 0.0;
    }
    let into = ph.seconds_into_on as f32;
    let remaining = ph.on_secs.saturating_sub(ph.seconds_into_on) as f32;
    let r = RAMP_SECONDS as f32;
    let up = (into / r).min(1.0);
    let down = (remaining / r).min(1.0);
    up.min(down).max(0.0)
}

/// Result of the thermal-derate ladders (§9.5).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Derate {
    /// Multiplier on commanded LED power, [0,1].
    pub factor: f32,
    pub derated: bool,
    /// Air temp in the 30–60 % derate band (>32 °C) — climate fault (§9.5).
    pub climate_fault: bool,
    /// Air temp >35 °C — LED off/min, critical fault.
    pub critical: bool,
    /// LED heat-sink >80 °C — LED off, driver fault.
    pub led_fault: bool,
}

/// Air-temperature + optional LED-heatsink derating (§9.5). The two ladders combine by taking the
/// more aggressive (lower) factor.
pub fn derate(air_temp_c: f32, led_heat_c: Option<f32>) -> Derate {
    // Air-temp ladder.
    let (mut factor, mut derated, climate_fault, critical) = if air_temp_c > 35.0 {
        (0.0, true, true, true)
    } else if air_temp_c > 32.0 {
        (0.5, true, true, false) // reduce 30–60% → 50%
    } else if air_temp_c > 30.0 {
        (0.8, true, false, false) // reduce up to 20%
    } else {
        (1.0, false, false, false)
    };

    // Optional LED heat-sink ladder (§9.5).
    let mut led_fault = false;
    if let Some(h) = led_heat_c {
        let hf = if h > 80.0 {
            led_fault = true;
            0.0
        } else if h > 70.0 {
            derated = true;
            0.7
        } else if h > 60.0 {
            derated = true;
            0.9 // dim slightly / fan high
        } else {
            1.0
        };
        if hf < factor {
            factor = hf;
        }
    }
    if factor < 1.0 {
        derated = true;
    }
    Derate {
        factor,
        derated,
        climate_fault,
        critical,
        led_fault,
    }
}

/// Inputs for one light-control tick.
#[derive(Debug, Clone, Copy)]
pub struct LightInputs<'a> {
    pub sp: &'a Setpoints,
    pub cal: &'a Calibration,
    pub rtc: WallTime,
    pub utc_offset_s: i32,
    pub boot_ms: u64,
    pub now_ms: u64,
    pub air_temp_c: f32,
    pub led_heat_c: Option<f32>,
}

/// Commanded LED output + status for one tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightOutput {
    pub on: bool,
    /// Final commanded LED power, percent 0..=100 (schedule × ramp × derate).
    pub commanded_pct: u8,
    /// The stage PPFD target this aims at (midpoint of the stage band).
    pub target_ppfd: u16,
    pub phase: Phase,
    pub ramping: bool,
    pub derate: Derate,
    /// True while running the RTC-invalid fallback schedule (drives System amber, §9.4).
    pub rtc_fallback: bool,
}

/// Stage PPFD setpoint: midpoint of the band (the controller aims for the middle of the target).
fn target_ppfd(sp: &Setpoints) -> u16 {
    ((sp.ppfd_min as u32 + sp.ppfd_max as u32) / 2) as u16
}

/// Evaluate the light controller for one tick.
pub fn evaluate(inp: &LightInputs) -> LightOutput {
    let ph = phase(inp.sp, inp.rtc, inp.utc_offset_s, inp.boot_ms, inp.now_ms);
    let ramp = ramp_factor(&ph);
    let der = derate(inp.air_temp_c, inp.led_heat_c);

    let ppfd = target_ppfd(inp.sp);
    let base_pct = inp.cal.percent_for_ppfd(ppfd) as f32;
    let commanded = if ph.on {
        crate::math::clampf(base_pct * ramp * der.factor, 0.0, 100.0)
    } else {
        0.0
    };

    LightOutput {
        on: ph.on,
        commanded_pct: commanded as u8,
        target_ppfd: ppfd,
        phase: ph,
        ramping: ph.on && (ramp < 1.0),
        derate: der,
        rtc_fallback: ph.fallback,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plant_profile::{setpoints, Stage};

    fn cal() -> Calibration {
        Calibration {
            version: 1,
            moisture_raw_dry: 1234,
            moisture_raw_wet: 2870,
            pump_ml_per_sec: 3.8,
            fan_min_pwm: 28,
            led_ppfd_25: 120,
            led_ppfd_50: 240,
            led_ppfd_75: 360,
            led_ppfd_100: 480,
            reservoir_low_adc: 600,
        }
    }

    fn at(hhmm_s: u32) -> WallTime {
        // Build a wall time whose local seconds-of-day == hhmm_s, with zero offset.
        WallTime {
            valid: true,
            unix_s: hhmm_s as u64,
        }
    }

    // §10.2 "Light scheduler — on/off/ramp behavior".
    #[test]
    fn schedule_on_between_0600_and_2200() {
        let sp = setpoints(Stage::Vegetative);
        let off_night = phase(&sp, at(3 * 3600), 0, 0, 0); // 03:00
        assert!(!off_night.on);
        let midday = phase(&sp, at(12 * 3600), 0, 0, 0); // 12:00
        assert!(midday.on);
        let just_off = phase(&sp, at(22 * 3600 + 60), 0, 0, 0); // 22:01
        assert!(!just_off.on);
        let just_on = phase(&sp, at(6 * 3600 + 60), 0, 0, 0); // 06:01
        assert!(just_on.on);
    }

    #[test]
    fn ramp_rises_over_first_30_min_and_falls_over_last() {
        let sp = setpoints(Stage::Vegetative);
        // At lights-on exactly → 0.
        let p0 = phase(&sp, at(6 * 3600), 0, 0, 0);
        assert_eq!(ramp_factor(&p0), 0.0);
        // 15 min in → 0.5.
        let p15 = phase(&sp, at(6 * 3600 + 15 * 60), 0, 0, 0);
        assert!((ramp_factor(&p15) - 0.5).abs() < 0.01);
        // 30 min in → full.
        let p30 = phase(&sp, at(6 * 3600 + 30 * 60), 0, 0, 0);
        assert!((ramp_factor(&p30) - 1.0).abs() < 0.01);
        // Midday → full.
        let pmid = phase(&sp, at(13 * 3600), 0, 0, 0);
        assert_eq!(ramp_factor(&pmid), 1.0);
        // 15 min before off (21:45) → 0.5.
        let pdown = phase(&sp, at(21 * 3600 + 45 * 60), 0, 0, 0);
        assert!((ramp_factor(&pdown) - 0.5).abs() < 0.01);
    }

    #[test]
    fn rtc_invalid_uses_16_8_fallback_from_boot() {
        let sp = setpoints(Stage::Vegetative);
        let boot = 1_000;
        // 1 hour after boot → on.
        let p1h = phase(&sp, WallTime::INVALID, 0, boot, boot + 3_600_000);
        assert!(p1h.on && p1h.fallback);
        // 17 hours after boot → off (past the 16h on-period).
        let p17h = phase(&sp, WallTime::INVALID, 0, boot, boot + 17 * 3_600_000);
        assert!(!p17h.on && p17h.fallback);
        // 25 hours after boot → back on (cycle wraps at 24h).
        let p25h = phase(&sp, WallTime::INVALID, 0, boot, boot + 25 * 3_600_000);
        assert!(p25h.on && p25h.fallback);
    }

    // §10.2 "LED derating — derate thresholds".
    #[test]
    fn air_temp_derate_thresholds() {
        assert_eq!(derate(29.0, None).factor, 1.0); // <30: none
        assert_eq!(derate(31.0, None).factor, 0.8); // 30–32: −20%
        let d33 = derate(33.0, None); // >32: −50% + climate fault
        assert_eq!(d33.factor, 0.5);
        assert!(d33.climate_fault);
        let d36 = derate(36.0, None); // >35: off + critical
        assert_eq!(d36.factor, 0.0);
        assert!(d36.critical);
    }

    #[test]
    fn led_heatsink_ladder() {
        assert_eq!(derate(24.0, Some(50.0)).factor, 1.0);
        assert_eq!(derate(24.0, Some(65.0)).factor, 0.9);
        assert_eq!(derate(24.0, Some(75.0)).factor, 0.7);
        let f = derate(24.0, Some(85.0));
        assert_eq!(f.factor, 0.0);
        assert!(f.led_fault);
        // The more aggressive ladder wins: hot air (0.5) + warm sink (0.9) → 0.5.
        assert_eq!(derate(33.0, Some(65.0)).factor, 0.5);
    }

    #[test]
    fn commanded_power_combines_schedule_ramp_and_derate() {
        let sp = setpoints(Stage::Vegetative); // target PPFD ~297 → ~62%
        let c = cal();
        // Midday, cool: full base.
        let day = evaluate(&LightInputs {
            sp: &sp,
            cal: &c,
            rtc: at(13 * 3600),
            utc_offset_s: 0,
            boot_ms: 0,
            now_ms: 0,
            air_temp_c: 24.0,
            led_heat_c: None,
        });
        assert!(day.on && !day.ramping);
        let full = day.commanded_pct;
        assert!((60..=64).contains(&full), "expected ~62%, got {full}");
        // Same moment but 33°C → halved by derate.
        let hot = evaluate(&LightInputs {
            sp: &sp,
            cal: &c,
            rtc: at(13 * 3600),
            utc_offset_s: 0,
            boot_ms: 0,
            now_ms: 0,
            air_temp_c: 33.0,
            led_heat_c: None,
        });
        assert!(hot.derate.climate_fault);
        assert!((hot.commanded_pct as i32 - (full as i32) / 2).abs() <= 1);
        // Night → off.
        let night = evaluate(&LightInputs {
            sp: &sp,
            cal: &c,
            rtc: at(2 * 3600),
            utc_offset_s: 0,
            boot_ms: 0,
            now_ms: 0,
            air_temp_c: 24.0,
            led_heat_c: None,
        });
        assert!(!night.on && night.commanded_pct == 0);
    }

    #[test]
    fn fallback_does_not_block_watering_window_data() {
        // RTC fallback still yields a usable fraction/hours-to-off for the irrigation window.
        let sp = setpoints(Stage::Vegetative);
        let ph = phase(&sp, WallTime::INVALID, 0, 0, 8 * 3_600_000); // 8h into fallback on-period
        assert!(ph.on);
        assert!(ph.fraction() > 0.0 && ph.fraction() < 1.0);
        assert!(ph.hours_to_off() > 0.0);
    }
}
