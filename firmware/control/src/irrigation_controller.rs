//! Irrigation controller: the safety-first watering decision loop. Spec §9.6, §5.6.
//! Implements `docs/watering-model.md` (thresholds, pulses, windows, daily caps).
//!
//! Design invariants (WI-FW-05 acceptance):
//!   - The pump can **never** enable while leak or reservoir-low is asserted — checked first,
//!     before any moisture logic.
//!   - Dosing is always **pulsed** with a remeasure; never continuous.
//!   - Every dose is bounded by single-run ≤30 s, ≤3 pulses/hour, and the per-stage daily cap.

use crate::calibration::Calibration;
use crate::hal::SensorError;
use crate::plant_profile::Setpoints;

/// Minimum normalized-% rise expected after a pulse soaks in; below this counts as "no response".
/// Deliberately small: it only needs to confirm water is *moving* (a working pump raises moisture,
/// a disconnected/clogged one or a stuck probe does not). It must stay below the smallest per-stage
/// pulse's expected effect so healthy watering never false-faults — the seedling 20–50 mL pulse is
/// the binding case.
const MIN_RISE_PCT: f32 = 1.0;
/// Consecutive no-rise pulses that latch a PUMP_FAULT (§9.6).
const NO_RISE_PULSES: u8 = 3;
/// Absolute pump-safety bounds (§9.6).
const MAX_PULSES_PER_HOUR: u8 = 3;
const MAX_RUN_SECONDS: f32 = 30.0;
const HOUR_MS: u64 = 3_600_000;
/// How long a moisture reading may be perfectly unchanged before it is deemed stuck (§10.3
/// "sensor fault after plausibility window"). In a live grow, moisture always drifts.
const STUCK_WINDOW_MS: u64 = 6 * HOUR_MS;
const PULSE_RING: usize = 8;

/// What the controller decided this tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaterReason {
    /// Nothing to do — moisture in band.
    Idle,
    /// Routine pulse started inside the watering window.
    Dosing,
    /// Emergency pulse (critically dry) — bypasses the window, day or night.
    EmergencyDosing,
    /// Waiting out the post-pulse recheck delay before remeasuring.
    AwaitingRecheck,
    /// Too wet — watering blocked.
    BlockedTooWet,
    /// Dry but outside the allowed watering window.
    BlockedWindow,
    /// Would exceed the per-stage daily cap.
    BlockedDailyMax,
    /// Hit the ≤3 pulses/hour rate limit.
    BlockedRate,
    /// Leak asserted — hard interlock.
    BlockedLeak,
    /// Reservoir low — hard interlock.
    BlockedReservoir,
    /// Moisture sensor invalid/uncalibrated — auto-watering disabled (§7.6).
    BlockedSensor,
    /// No-rise pump fault latched.
    PumpFault,
}

/// Fault to escalate to the safety machine, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrrigationFault {
    PumpFault,
    /// Daily watering limit reached (§9.6 PUMP_FAULT_OR_WATERING_LIMIT).
    WateringLimit,
    SensorFault,
}

/// The controller's decision for one tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decision {
    pub pump_on: bool,
    pub dose_ml: u16,
    pub run_seconds: f32,
    pub reason: WaterReason,
    pub fault: Option<IrrigationFault>,
    /// True only on a tick where the pump actually runs (drives the WATERING state).
    pub watering_active: bool,
}

impl Decision {
    fn idle(reason: WaterReason) -> Decision {
        Decision {
            pump_on: false,
            dose_ml: 0,
            run_seconds: 0.0,
            reason,
            fault: None,
            watering_active: false,
        }
    }
    fn fault(reason: WaterReason, fault: IrrigationFault) -> Decision {
        Decision {
            pump_on: false,
            dose_ml: 0,
            run_seconds: 0.0,
            reason,
            fault: Some(fault),
            watering_active: false,
        }
    }
}

/// Inputs for one irrigation tick.
#[derive(Debug, Clone, Copy)]
pub struct Inputs<'a> {
    pub sp: &'a Setpoints,
    pub now_ms: u64,
    /// Normalized moisture, or `None` if the sensor is invalid/uncalibrated.
    pub moisture: Option<f32>,
    /// Calibrated pump flow.
    pub ml_per_sec: f32,
    pub reservoir_low: bool,
    pub leak: bool,
    /// Lights currently on (watering-window input).
    pub light_on: bool,
    /// Fraction of the light period elapsed, 0..1 (watering-window input).
    pub light_fraction: f32,
    /// Hours until lights-off (watering-window input).
    pub hours_to_off: f32,
    /// Wall-clock (or boot-relative) day index, for the daily-cap reset.
    pub day_index: u32,
}

/// Watering-window gate (watering-model §5). Critically-dry bypasses all window rules (but not the
/// hard interlocks, which are checked earlier).
pub fn within_watering_window(
    critically_dry: bool,
    f: f32,
    hours_to_off: f32,
    light_on: bool,
) -> bool {
    if critically_dry {
        return true; // emergency: any time, day or night
    }
    if !light_on {
        return false; // routine watering only during the light period
    }
    if f > 0.70 {
        return false; // past first 70% of the light period
    }
    if hours_to_off < 2.0 {
        return false; // last 2 h before lights-off
    }
    true
}

/// Persistent irrigation state across ticks.
#[derive(Debug, Clone, Copy)]
pub struct IrrigationController {
    day_index: u32,
    daily_watered_ml: u32,
    /// Recent pulse start times (monotonic ms). `None` = empty slot. Using `Option` rather than a
    /// `0` sentinel matters: the very first pulse can occur at `now_ms == 0`, which a `0` sentinel
    /// would silently drop from the rate-limit window.
    pulse_times: [Option<u64>; PULSE_RING],
    pulse_head: usize,
    pulses_since_rise: u8,
    pump_fault_latched: bool,
    /// After a pulse we wait this long, then remeasure (set when a pulse starts).
    recheck_at_ms: Option<u64>,
    moisture_before_pulse: Option<f32>,
}

impl Default for IrrigationController {
    fn default() -> Self {
        Self::new()
    }
}

impl IrrigationController {
    pub const fn new() -> Self {
        IrrigationController {
            day_index: u32::MAX,
            daily_watered_ml: 0,
            pulse_times: [None; PULSE_RING],
            pulse_head: 0,
            pulses_since_rise: 0,
            pump_fault_latched: false,
            recheck_at_ms: None,
            moisture_before_pulse: None,
        }
    }

    pub fn daily_watered_ml(&self) -> u32 {
        self.daily_watered_ml
    }
    pub fn pump_fault_latched(&self) -> bool {
        self.pump_fault_latched
    }

    /// Clear a latched pump fault (service action).
    pub fn clear_pump_fault(&mut self) {
        self.pump_fault_latched = false;
        self.pulses_since_rise = 0;
    }

    fn pulses_in_last_hour(&self, now: u64) -> u8 {
        let mut n = 0;
        for t in self.pulse_times.iter().flatten() {
            if now.saturating_sub(*t) < HOUR_MS {
                n += 1;
            }
        }
        n
    }

    fn record_pulse(&mut self, now: u64) {
        self.pulse_times[self.pulse_head] = Some(now);
        self.pulse_head = (self.pulse_head + 1) % PULSE_RING;
    }

    /// Evaluate one tick of the §9.6 decision loop. Returns the pump command + reason + any fault.
    pub fn tick(&mut self, inp: &Inputs<'_>) -> Decision {
        // Daily cap reset on day rollover.
        if inp.day_index != self.day_index {
            self.day_index = inp.day_index;
            self.daily_watered_ml = 0;
        }

        // --- Hard interlocks, in the §9.6 precedence order. Pump off, no further logic. ---
        if inp.leak {
            return Decision::idle(WaterReason::BlockedLeak);
        }
        if inp.reservoir_low {
            return Decision::idle(WaterReason::BlockedReservoir);
        }
        let m = match inp.moisture {
            None => {
                return Decision::fault(WaterReason::BlockedSensor, IrrigationFault::SensorFault)
            }
            Some(v) => v,
        };
        if self.pump_fault_latched {
            return Decision::fault(WaterReason::PumpFault, IrrigationFault::PumpFault);
        }

        // --- Pending recheck: wait out the delay, then judge whether the last pulse worked. ---
        if let Some(at) = self.recheck_at_ms {
            if inp.now_ms < at {
                return Decision::idle(WaterReason::AwaitingRecheck);
            }
            // Remeasure window elapsed: did moisture rise enough?
            let before = self.moisture_before_pulse.unwrap_or(m);
            if m >= before + MIN_RISE_PCT {
                self.pulses_since_rise = 0;
            } else {
                self.pulses_since_rise = self.pulses_since_rise.saturating_add(1);
                if self.pulses_since_rise >= NO_RISE_PULSES {
                    self.pump_fault_latched = true;
                    return Decision::fault(WaterReason::PumpFault, IrrigationFault::PumpFault);
                }
            }
            self.recheck_at_ms = None;
        }

        // --- Classify moisture. ---
        let critically_dry = m < inp.sp.moisture_critical_pct;
        let dry = m < inp.sp.moisture_dry_pct;
        let too_wet = m > inp.sp.moisture_wet_pct;

        let (dose, emergency) = if critically_dry {
            (inp.sp.emergency_pulse_ml, true)
        } else if dry {
            if within_watering_window(false, inp.light_fraction, inp.hours_to_off, inp.light_on) {
                (inp.sp.normal_pulse_ml, false)
            } else {
                return Decision::idle(WaterReason::BlockedWindow);
            }
        } else if too_wet {
            return Decision::idle(WaterReason::BlockedTooWet);
        } else {
            return Decision::idle(WaterReason::Idle); // in band
        };

        // --- Rate limit: ≤3 pulses/hour (applies even to emergency — bounds worst case). ---
        if self.pulses_in_last_hour(inp.now_ms) >= MAX_PULSES_PER_HOUR {
            return Decision::idle(WaterReason::BlockedRate);
        }

        // --- Daily cap (§9.6): withhold and flag, never exceed. ---
        if self.daily_watered_ml + dose as u32 > inp.sp.daily_max_ml as u32 {
            return Decision::fault(WaterReason::BlockedDailyMax, IrrigationFault::WateringLimit);
        }

        // --- Run the pulse. ---
        let mut run_seconds = if inp.ml_per_sec > 0.0 {
            dose as f32 / inp.ml_per_sec
        } else {
            0.0
        };
        if run_seconds > MAX_RUN_SECONDS {
            run_seconds = MAX_RUN_SECONDS; // single-run cap
        }
        let actual_ml = (run_seconds * inp.ml_per_sec) as u32;
        self.daily_watered_ml += actual_ml;
        self.record_pulse(inp.now_ms);
        self.recheck_at_ms = Some(inp.now_ms + inp.sp.recheck_delay_min as u64 * 60_000);
        self.moisture_before_pulse = Some(m);

        Decision {
            pump_on: true,
            dose_ml: dose,
            run_seconds,
            reason: if emergency {
                WaterReason::EmergencyDosing
            } else {
                WaterReason::Dosing
            },
            fault: None,
            watering_active: true,
        }
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

    /// Process a raw read. `auto_watering_enabled` reflects calibration validity (§7.6): if false,
    /// moisture is never trusted regardless of the reading.
    pub fn validate(
        &mut self,
        now_ms: u64,
        read: Result<u16, SensorError>,
        cal: &Calibration,
        auto_watering_enabled: bool,
    ) -> Option<f32> {
        if !auto_watering_enabled {
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

    fn base_inputs(sp: &Setpoints, now: u64, moisture: f32) -> Inputs<'_> {
        Inputs {
            sp,
            now_ms: now,
            moisture: Some(moisture),
            ml_per_sec: 3.8,
            reservoir_low: false,
            leak: false,
            light_on: true,
            light_fraction: 0.2, // early in the day → in window
            hours_to_off: 10.0,
            day_index: 1,
        }
    }

    // §10.2 "Pump safety — leak lockout".
    #[test]
    fn leak_blocks_pump_even_when_bone_dry() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 5.0); // critically dry
        inp.leak = true;
        let d = c.tick(&inp);
        assert!(!d.pump_on);
        assert_eq!(d.reason, WaterReason::BlockedLeak);
    }

    // §10.2 "Pump safety — low water".
    #[test]
    fn reservoir_low_blocks_pump() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 5.0);
        inp.reservoir_low = true;
        assert!(!c.tick(&inp).pump_on);
        assert_eq!(c.tick(&inp).reason, WaterReason::BlockedReservoir);
    }

    #[test]
    fn leak_precedes_reservoir_precedes_sensor() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 10.0);
        inp.leak = true;
        inp.reservoir_low = true;
        inp.moisture = None;
        assert_eq!(c.tick(&inp).reason, WaterReason::BlockedLeak);
        inp.leak = false;
        assert_eq!(c.tick(&inp).reason, WaterReason::BlockedReservoir);
        inp.reservoir_low = false;
        assert_eq!(c.tick(&inp).reason, WaterReason::BlockedSensor);
    }

    #[test]
    fn invalid_moisture_raises_sensor_fault_no_pump() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 0.0);
        inp.moisture = None;
        let d = c.tick(&inp);
        assert!(!d.pump_on);
        assert_eq!(d.fault, Some(IrrigationFault::SensorFault));
    }

    #[test]
    fn waters_when_dry_inside_window() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let inp = base_inputs(&sp, 0, 20.0); // below dry (30), above critical (17)
        let d = c.tick(&inp);
        assert!(d.pump_on);
        assert_eq!(d.reason, WaterReason::Dosing);
        assert_eq!(d.dose_ml, sp.normal_pulse_ml);
        // run time = 100 mL / 3.8 ≈ 26.3 s, under the 30 s cap.
        assert!(d.run_seconds < MAX_RUN_SECONDS && d.run_seconds > 20.0);
    }

    #[test]
    fn dry_but_outside_window_does_not_water() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 20.0);
        inp.light_fraction = 0.9; // past 70% of the day
        assert_eq!(c.tick(&inp).reason, WaterReason::BlockedWindow);
        // lights off, not critical → still blocked
        let mut inp2 = base_inputs(&sp, 0, 20.0);
        inp2.light_on = false;
        assert_eq!(c.tick(&inp2).reason, WaterReason::BlockedWindow);
    }

    #[test]
    fn critically_dry_waters_at_night_bypassing_window() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 10.0); // below critical (17)
        inp.light_on = false;
        inp.light_fraction = 0.0;
        let d = c.tick(&inp);
        assert!(d.pump_on);
        assert_eq!(d.reason, WaterReason::EmergencyDosing);
        assert_eq!(d.dose_ml, sp.emergency_pulse_ml);
    }

    #[test]
    fn too_wet_blocks_watering() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let inp = base_inputs(&sp, 0, 70.0); // above wet (55)
        assert_eq!(c.tick(&inp).reason, WaterReason::BlockedTooWet);
    }

    #[test]
    fn single_run_capped_at_30s() {
        let sp = setpoints(Stage::Fruiting); // 175 mL pulse
        let mut c = IrrigationController::new();
        let mut inp = base_inputs(&sp, 0, 20.0);
        inp.ml_per_sec = 1.0; // 175 mL / 1.0 = 175 s → must cap at 30 s
        let d = c.tick(&inp);
        assert_eq!(d.run_seconds, MAX_RUN_SECONDS);
    }

    // §10.2 "Pump safety — daily max".
    #[test]
    fn daily_max_withholds_and_flags() {
        let sp = veg(); // 800 mL/day cap, 100 mL pulses
        let mut c = IrrigationController::new();
        // Pre-load near the cap.
        c.daily_watered_ml = 750;
        c.day_index = 1;
        let inp = base_inputs(&sp, 0, 20.0);
        let d = c.tick(&inp);
        assert!(!d.pump_on);
        assert_eq!(d.reason, WaterReason::BlockedDailyMax);
        assert_eq!(d.fault, Some(IrrigationFault::WateringLimit));
    }

    #[test]
    fn rate_limited_to_three_pulses_per_hour() {
        // Seedling recheck is 18 min, so three pulses land at 0/18/36 min; a 4th attempt at 54 min
        // still sees all three inside the trailing hour → rate-limited. Moisture rises a little at
        // each recheck (so the no-rise fault never trips) yet stays below the dry threshold (35).
        let sp = setpoints(Stage::Seedling);
        let mut c = IrrigationController::new();
        let r = sp.recheck_delay_min as u64 * 60_000;
        let steps = [
            (0u64, 20.0f32),
            (r + 1, 24.0),
            (2 * r + 2, 28.0),
            (3 * r + 3, 32.0),
        ];
        let mut last = WaterReason::Idle;
        let mut pulses = 0;
        for (t, m) in steps {
            let d = c.tick(&base_inputs(&sp, t, m));
            if d.pump_on {
                pulses += 1;
            }
            last = d.reason;
        }
        assert_eq!(
            pulses, 3,
            "exactly three pulses should fire before the limit"
        );
        assert_eq!(last, WaterReason::BlockedRate);
    }

    // §10.2 "Pump safety — no-rise → PUMP_FAULT" (and §10.3 stuck-dry / pump-disconnected).
    #[test]
    fn no_rise_after_n_pulses_is_pump_fault() {
        let sp = veg();
        let mut c = IrrigationController::new();
        // Feed a constant dry reading that never rises (stuck dry / pump disconnected). Tick once a
        // minute; the controller pulses, waits out the recheck, sees no rise, and after
        // NO_RISE_PULSES rechecks latches a PUMP_FAULT.
        let mut now = 0u64;
        let mut latched = false;
        for _ in 0..4000 {
            c.tick(&base_inputs(&sp, now, 20.0));
            if c.pump_fault_latched() {
                latched = true;
                break;
            }
            now += 60_000;
        }
        assert!(latched, "no-rise pump fault should latch");
        let d = c.tick(&base_inputs(&sp, now, 20.0));
        assert_eq!(d.fault, Some(IrrigationFault::PumpFault));
        assert!(!d.pump_on);
    }

    #[test]
    fn recheck_delay_blocks_back_to_back_pulses() {
        let sp = veg();
        let mut c = IrrigationController::new();
        let d1 = c.tick(&base_inputs(&sp, 0, 20.0));
        assert!(d1.pump_on);
        // Immediately after, still within recheck delay → awaiting, pump off.
        let d2 = c.tick(&base_inputs(&sp, 60_000, 20.0));
        assert!(!d2.pump_on);
        assert_eq!(d2.reason, WaterReason::AwaitingRecheck);
    }

    #[test]
    fn window_logic_matches_doc() {
        // critically dry → always true
        assert!(within_watering_window(true, 0.99, 0.1, false));
        // routine: lights off → false
        assert!(!within_watering_window(false, 0.1, 10.0, false));
        // past 70% → false
        assert!(!within_watering_window(false, 0.71, 10.0, true));
        // last 2h → false
        assert!(!within_watering_window(false, 0.3, 1.5, true));
        // good window → true
        assert!(within_watering_window(false, 0.3, 5.0, true));
    }

    #[test]
    fn validator_flags_stuck_reading() {
        let cal = Calibration {
            version: 1,
            moisture_raw_dry: 1000,
            moisture_raw_wet: 3000,
            pump_ml_per_sec: 3.8,
            led_ppfd_25: 120,
            led_ppfd_50: 240,
            led_ppfd_75: 360,
            led_ppfd_100: 480,
            reservoir_low_adc: 600,
        };
        let mut v = MoistureValidator::new(HOUR_MS);
        // Same raw the whole time.
        assert!(v.validate(0, Ok(2000), &cal, true).is_some());
        assert!(v.validate(HOUR_MS / 2, Ok(2000), &cal, true).is_some());
        // Past the window with no change → stuck → None.
        assert!(v.validate(HOUR_MS + 1, Ok(2000), &cal, true).is_none());
    }

    #[test]
    fn validator_respects_calibration_gate() {
        let cal = Calibration::DEFAULTS;
        let mut v = MoistureValidator::default();
        // auto-watering disabled → always None regardless of reading.
        assert!(v.validate(0, Ok(2000), &cal, false).is_none());
    }

    #[test]
    fn validator_passes_through_bus_error_as_none() {
        let cal = Calibration {
            version: 1,
            moisture_raw_dry: 1000,
            moisture_raw_wet: 3000,
            pump_ml_per_sec: 3.8,
            led_ppfd_25: 120,
            led_ppfd_50: 240,
            led_ppfd_75: 360,
            led_ppfd_100: 480,
            reservoir_low_adc: 600,
        };
        let mut v = MoistureValidator::default();
        assert!(v.validate(0, Err(SensorError::Bus), &cal, true).is_none());
    }

    #[test]
    fn daily_counter_resets_on_new_day() {
        let sp = veg();
        let mut c = IrrigationController::new();
        c.daily_watered_ml = 700;
        c.day_index = 1;
        let mut inp = base_inputs(&sp, 0, 20.0);
        inp.day_index = 2; // new day
        let d = c.tick(&inp);
        assert!(d.pump_on); // counter reset, dose allowed again
        assert_eq!(
            c.daily_watered_ml(),
            (d.run_seconds * inp.ml_per_sec) as u32
        );
    }
}
