//! Status-LED mapping: system/subsystem state → (color, pattern) for the 4 front LEDs.
//! Spec §9.8, §3.5, §7.11; ECO-003 (5→4 LEDs — the Climate LED is dropped, climate warnings fold
//! into System). Colorblind-safe: every warning/fault is distinguishable by **position + pattern**,
//! never color alone (WI-FW-08 acceptance).

use crate::hal::LedId;
use crate::safety_controller::SystemState;

/// LED color. `Off` is a valid state (night mode / unused subsystem).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedColor {
    Off,
    Green,
    Amber,
    Red,
}

/// Blink pattern (§9.8). Position + pattern together encode meaning so color is never required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedPattern {
    /// Solid on — OK.
    Steady,
    /// Slow pulse — warning.
    SlowPulse,
    /// Fast blink — user action needed.
    FastBlink,
    /// Double blink — sensor fault (distinct from a generic fast blink).
    DoubleBlink,
    /// Off.
    Off,
}

/// One LED's commanded appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LedState {
    pub color: LedColor,
    pub pattern: LedPattern,
}

impl LedState {
    const OFF: LedState = LedState {
        color: LedColor::Off,
        pattern: LedPattern::Off,
    };
    const fn new(color: LedColor, pattern: LedPattern) -> LedState {
        LedState { color, pattern }
    }
}

/// Per-subsystem health the LED panel renders. These are computed by the individual monitors and the
/// safety machine, then mapped to LEDs here so the mapping has one home and one test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaterLevel {
    Ok,
    LowSoon,
    /// Reservoir empty, or a leak/flood — the water subsystem needs attention.
    Empty,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoistureHealth {
    InTarget,
    DrySoonOrWetHigh,
    FaultOrCriticalOrWaterlogged,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightHealth {
    Normal,
    ThermalDimOrUncertain,
    FaultOrOverTemp,
}

/// Full panel input. The System LED is derived from the top-level [`SystemState`]; the three
/// subsystem LEDs come from the monitors. Climate has no LED of its own (ECO-003) — a climate
/// warning instead tints the System LED amber via [`PanelInputs::climate_warn`].
#[derive(Debug, Clone, Copy)]
pub struct PanelInputs {
    pub state: SystemState,
    pub water: WaterLevel,
    pub moisture: MoistureHealth,
    pub light: LightHealth,
    /// True between lights-off: dim non-critical LEDs, keep a System heartbeat (§9.8).
    pub night_mode: bool,
    /// Maintenance/calibration due → System amber (§9.8).
    pub maintenance_due: bool,
    /// RTC invalid → running the safe-schedule fallback → System amber pulse (§9.4).
    pub rtc_fallback: bool,
    /// Climate (temp/RH/VPD) outside the preferred band — there is no Climate LED, so it surfaces as
    /// a System amber warning (ECO-003).
    pub climate_warn: bool,
}

/// The four LEDs' commanded states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Panel {
    pub water: LedState,
    pub moisture: LedState,
    pub light: LedState,
    pub system: LedState,
}

impl Panel {
    /// Index by LED id (used by the HAL push and by tests).
    pub fn get(&self, id: LedId) -> LedState {
        match id {
            LedId::Water => self.water,
            LedId::Moisture => self.moisture,
            LedId::Light => self.light,
            LedId::System => self.system,
        }
    }
}

/// Map the System LED from the top-level state (§9.8 "System" row). The System LED is special: it
/// keeps a heartbeat even at night so the user always knows the controller is alive.
fn system_led(
    state: SystemState,
    maintenance_due: bool,
    night_mode: bool,
    rtc_fallback: bool,
    climate_warn: bool,
) -> LedState {
    use SystemState::*;
    match state {
        Boot | SelfTest => LedState::new(LedColor::Red, LedPattern::FastBlink), // self-test / fatal
        SafeShutdown => LedState::new(LedColor::Red, LedPattern::Steady),
        // Flood is a fatal-class fault: System red, fast blink (urgent).
        LeakDetected => LedState::new(LedColor::Red, LedPattern::FastBlink),
        // Over-temp: LED is being cut — red slow pulse.
        OverTempLed => LedState::new(LedColor::Red, LedPattern::SlowPulse),
        Maintenance => LedState::new(LedColor::Amber, LedPattern::SlowPulse),
        // Warnings surface amber on System while the subsystem LED carries the detail.
        LowWater | MoistureLow | MoistureHigh | SensorFault => {
            LedState::new(LedColor::Amber, LedPattern::SlowPulse)
        }
        // Heartbeat: dim slow pulse at night, steady green by day; amber if anything wants attention.
        Normal => {
            if maintenance_due || rtc_fallback || climate_warn {
                LedState::new(LedColor::Amber, LedPattern::SlowPulse)
            } else if night_mode {
                LedState::new(LedColor::Green, LedPattern::SlowPulse) // dim heartbeat
            } else {
                LedState::new(LedColor::Green, LedPattern::Steady)
            }
        }
    }
}

fn water_led(w: WaterLevel) -> LedState {
    match w {
        WaterLevel::Ok => LedState::new(LedColor::Green, LedPattern::Steady),
        WaterLevel::LowSoon => LedState::new(LedColor::Amber, LedPattern::SlowPulse),
        WaterLevel::Empty => LedState::new(LedColor::Red, LedPattern::FastBlink),
    }
}

fn moisture_led(m: MoistureHealth) -> LedState {
    match m {
        MoistureHealth::InTarget => LedState::new(LedColor::Green, LedPattern::Steady),
        MoistureHealth::DrySoonOrWetHigh => LedState::new(LedColor::Amber, LedPattern::SlowPulse),
        // Sensor fault gets the distinctive double-blink (§9.8) so it is told apart from "critical dry".
        MoistureHealth::FaultOrCriticalOrWaterlogged => {
            LedState::new(LedColor::Red, LedPattern::DoubleBlink)
        }
    }
}

fn light_led(l: LightHealth) -> LedState {
    match l {
        LightHealth::Normal => LedState::new(LedColor::Green, LedPattern::Steady),
        LightHealth::ThermalDimOrUncertain => LedState::new(LedColor::Amber, LedPattern::SlowPulse),
        LightHealth::FaultOrOverTemp => LedState::new(LedColor::Red, LedPattern::FastBlink),
    }
}

/// Build the full panel. At night, non-critical (green) subsystem LEDs are turned off to avoid a
/// glowing appliance, but **warnings/faults (amber/red) stay visible** and the System heartbeat is
/// preserved (§9.8).
pub fn render(inp: &PanelInputs) -> Panel {
    let dim_if_night = |s: LedState| -> LedState {
        if inp.night_mode && s.color == LedColor::Green {
            LedState::OFF
        } else {
            s
        }
    };
    Panel {
        water: dim_if_night(water_led(inp.water)),
        moisture: dim_if_night(moisture_led(inp.moisture)),
        light: dim_if_night(light_led(inp.light)),
        // System LED is never fully dark in normal operation — heartbeat preserved.
        system: system_led(
            inp.state,
            inp.maintenance_due,
            inp.night_mode,
            inp.rtc_fallback,
            inp.climate_warn,
        ),
    }
}

/// Push a rendered panel to the hardware (§9.8). Generic over the [`crate::hal::StatusLeds`] trait.
pub fn drive<L: crate::hal::StatusLeds>(leds: &mut L, panel: &Panel) {
    leds.set(LedId::Water, panel.water.color, panel.water.pattern);
    leds.set(
        LedId::Moisture,
        panel.moisture.color,
        panel.moisture.pattern,
    );
    leds.set(LedId::Light, panel.light.color, panel.light.pattern);
    leds.set(LedId::System, panel.system.color, panel.system.pattern);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> PanelInputs {
        PanelInputs {
            state: SystemState::Normal,
            water: WaterLevel::Ok,
            moisture: MoistureHealth::InTarget,
            light: LightHealth::Normal,
            night_mode: false,
            maintenance_due: false,
            rtc_fallback: false,
            climate_warn: false,
        }
    }

    #[test]
    fn all_ok_is_all_green_steady() {
        let p = render(&base());
        for s in [p.water, p.moisture, p.light, p.system] {
            assert_eq!(s.color, LedColor::Green);
            assert_eq!(s.pattern, LedPattern::Steady);
        }
    }

    #[test]
    fn leak_lights_water_red_and_system_red() {
        let mut i = base();
        i.state = SystemState::LeakDetected;
        i.water = WaterLevel::Empty; // flood path also reds the water LED
        let p = render(&i);
        assert_eq!(p.system.color, LedColor::Red);
        assert_eq!(p.water.color, LedColor::Red);
    }

    #[test]
    fn sensor_fault_is_distinct_double_blink() {
        // §9.8: sensor faults use double-blink — distinguishable from critical-dry's red without color.
        let mut i = base();
        i.moisture = MoistureHealth::FaultOrCriticalOrWaterlogged;
        let p = render(&i);
        assert_eq!(p.moisture.pattern, LedPattern::DoubleBlink);
    }

    #[test]
    fn night_mode_dims_greens_but_keeps_system_heartbeat() {
        let mut i = base();
        i.night_mode = true;
        let p = render(&i);
        // Green subsystem LEDs go off at night...
        assert_eq!(p.water, LedState::OFF);
        assert_eq!(p.moisture, LedState::OFF);
        // ...but the System LED keeps a (dim) heartbeat, never fully off.
        assert_ne!(p.system.pattern, LedPattern::Off);
        assert_eq!(p.system.color, LedColor::Green);
    }

    #[test]
    fn night_mode_keeps_warnings_visible() {
        let mut i = base();
        i.night_mode = true;
        i.light = LightHealth::FaultOrOverTemp;
        let p = render(&i);
        // A red fault must NOT be dimmed away at night.
        assert_eq!(p.light.color, LedColor::Red);
    }

    #[test]
    fn climate_warning_tints_system_amber() {
        // No Climate LED (ECO-003): a climate warning surfaces on System instead.
        let mut i = base();
        i.climate_warn = true;
        let p = render(&i);
        assert_eq!(p.system.color, LedColor::Amber);
        assert_eq!(p.system.pattern, LedPattern::SlowPulse);
    }

    #[test]
    fn every_subsystem_warning_distinguishable_without_color() {
        // Position (which LED) + pattern must disambiguate even for a colorblind user.
        let mut i = base();
        i.water = WaterLevel::LowSoon;
        i.moisture = MoistureHealth::FaultOrCriticalOrWaterlogged;
        i.light = LightHealth::ThermalDimOrUncertain;
        let p = render(&i);
        assert_ne!(p.water.pattern, LedPattern::Steady);
        assert_eq!(p.moisture.pattern, LedPattern::DoubleBlink);
        assert_ne!(p.light.pattern, LedPattern::Steady);
    }

    #[test]
    fn maintenance_due_shows_amber_system() {
        let mut i = base();
        i.maintenance_due = true;
        let p = render(&i);
        assert_eq!(p.system.color, LedColor::Amber);
    }

    #[test]
    fn rtc_fallback_shows_amber_system_pulse() {
        // §9.4: RTC invalid → System amber pulse.
        let mut i = base();
        i.rtc_fallback = true;
        let p = render(&i);
        assert_eq!(p.system.color, LedColor::Amber);
        assert_eq!(p.system.pattern, LedPattern::SlowPulse);
    }
}
