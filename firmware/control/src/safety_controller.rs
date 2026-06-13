//! Top-level state machine and fault-priority arbitration. Spec §9.3, §9.4, §11.4.
//!
//! This module is the authority: when a safety state is active it **overrides** the individual
//! controllers (pump off, LED off/min) regardless of what light/irrigation/climate proposed. The
//! priority order is a total ordering, so the highest-priority active condition always wins
//! (WI-FW-07 acceptance).

/// The required firmware states (§9.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    Boot,
    SelfTest,
    Normal,
    Watering,
    LowWater,
    LeakDetected,
    SensorFault,
    PumpFault,
    FanFault,
    LedFault,
    OverTemp,
    Maintenance,
    SafeShutdown,
}

impl SystemState {
    pub const fn name(self) -> &'static str {
        match self {
            SystemState::Boot => "BOOT",
            SystemState::SelfTest => "SELF_TEST",
            SystemState::Normal => "NORMAL",
            SystemState::Watering => "WATERING",
            SystemState::LowWater => "LOW_WATER",
            SystemState::LeakDetected => "LEAK_DETECTED",
            SystemState::SensorFault => "SENSOR_FAULT",
            SystemState::PumpFault => "PUMP_FAULT",
            SystemState::FanFault => "FAN_FAULT",
            SystemState::LedFault => "LED_FAULT",
            SystemState::OverTemp => "OVER_TEMP",
            SystemState::Maintenance => "MAINTENANCE",
            SystemState::SafeShutdown => "SAFE_SHUTDOWN",
        }
    }
}

/// Raw fault/condition signals feeding arbitration each control tick. Booleans are produced by the
/// sensors and the individual controllers; arbitration decides which one is in charge.
#[derive(Debug, Clone, Copy, Default)]
pub struct SafetyInputs {
    /// Leak sensor wet (§11.4). Highest priority; latches until manual clear.
    pub leak: bool,
    /// Critical over-temp (>35 °C sustained, §9.5) — LED off/min, system fault.
    pub over_temp_critical: bool,
    /// Pump fault (no-rise after N pulses, over-current, or watering-limit, §9.6).
    pub pump_fault: bool,
    /// Moisture sensor invalid/implausible — disables auto-watering (§7.6, §9.6).
    pub moisture_sensor_invalid: bool,
    /// Reservoir low/empty — watering lockout (§9.6).
    pub reservoir_low: bool,
    /// Fan tach missing while commanded on (§9.7). Does not stop watering.
    pub fan_fault: bool,
    /// LED driver/heat-sink fault (§9.5 >80 °C ladder). Does not stop watering.
    pub led_fault: bool,
    /// Dev/overwinter maintenance mode is engaged.
    pub maintenance: bool,
    /// The irrigation controller is mid-pulse this tick.
    pub watering_active: bool,
}

/// Actuator permissions derived from the active state. The individual controllers compute desired
/// outputs; these gates are applied on top, so a safety state can never be out-voted.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gates {
    /// May the pump be energized at all this tick?
    pub pump_allowed: bool,
    /// Multiplier applied to the commanded LED power (0.0 = forced off/min).
    pub led_max_factor: f32,
    /// Force the fan to maximum (heat-dispersion during over-temp).
    pub force_fan_high: bool,
}

/// Report produced by the §9.4 boot sequence. Captures the ordered steps so boot is testable
/// without hardware (the actual peripheral init lives in `controller/`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BootReport {
    pub self_test_passed: bool,
    pub calibration_valid: bool,
    pub sensors_in_range: bool,
    /// Grow-cycle age restored from RTC/NVS (days).
    pub restored_age_days: u32,
    /// Was RTC wall time valid? (Drives safe-schedule fallback, §9.4.)
    pub rtc_valid: bool,
    /// Pump was commanded off before anything else (§9.4 step 7). Always true on a clean boot.
    pub pump_forced_off: bool,
}

/// The top-level controller. Owns the latched leak state and the current lifecycle phase.
#[derive(Debug, Clone, Copy)]
pub struct SafetyController {
    state: SystemState,
    /// Leak latches until a manual clear (§11.4) — a transient wet reading must not silently clear.
    leak_latched: bool,
    booted: bool,
}

impl Default for SafetyController {
    fn default() -> Self {
        Self::new()
    }
}

impl SafetyController {
    pub const fn new() -> Self {
        SafetyController {
            state: SystemState::Boot,
            leak_latched: false,
            booted: false,
        }
    }

    pub fn state(&self) -> SystemState {
        self.state
    }

    pub fn leak_latched(&self) -> bool {
        self.leak_latched
    }

    /// Run the §9.4 boot sequence. Pump is forced off first; on a failed self-test or invalid
    /// calibration we land in SELF_TEST/SafeShutdown rather than NORMAL (fail-safe, §7.6).
    pub fn boot(&mut self, report: BootReport) -> SystemState {
        self.booted = true;
        // Step 7 of §9.4 is non-negotiable: pump off before anything else.
        debug_assert!(report.pump_forced_off, "boot must force pump off (§9.4)");
        self.state = if !report.self_test_passed {
            SystemState::SelfTest // stay in self-test / surface fatal fault
        } else if !report.calibration_valid {
            // Missing/corrupt calibration must not act on bad data — fault, don't run (§7.6).
            SystemState::SensorFault
        } else {
            SystemState::Normal
        };
        self.state
    }

    /// Manually clear a latched leak (service action, §11.4).
    pub fn clear_leak(&mut self) {
        self.leak_latched = false;
    }

    /// Arbitrate the active state from the current inputs. Total ordering, highest first:
    /// LEAK > OVER_TEMP(critical) > PUMP_FAULT > SENSOR_FAULT(watering) > LOW_WATER >
    /// FAN_FAULT > LED_FAULT > MAINTENANCE > WATERING > NORMAL.
    ///
    /// Leak latches: once seen, it stays the active state until [`clear_leak`](Self::clear_leak).
    pub fn arbitrate(&mut self, inp: &SafetyInputs) -> SystemState {
        if inp.leak {
            self.leak_latched = true;
        }
        self.state = if self.leak_latched {
            SystemState::LeakDetected
        } else if inp.over_temp_critical {
            SystemState::OverTemp
        } else if inp.pump_fault {
            SystemState::PumpFault
        } else if inp.moisture_sensor_invalid {
            SystemState::SensorFault
        } else if inp.reservoir_low {
            SystemState::LowWater
        } else if inp.fan_fault {
            SystemState::FanFault
        } else if inp.led_fault {
            SystemState::LedFault
        } else if inp.maintenance {
            SystemState::Maintenance
        } else if inp.watering_active {
            SystemState::Watering
        } else {
            SystemState::Normal
        };
        self.state
    }

    /// Derive actuator gates from the current state. This is where "leak/over-temp override the
    /// controllers" is *enforced* (WI-FW-07 acceptance): pump off, LED off/min.
    pub fn gates(&self) -> Gates {
        match self.state {
            // Leak: everything water-related is dead until cleared.
            SystemState::LeakDetected => Gates {
                pump_allowed: false,
                led_max_factor: 1.0,
                force_fan_high: false,
            },
            // Critical over-temp: LED off/min, fan flat out, no watering on temperature alone.
            SystemState::OverTemp => Gates {
                pump_allowed: false,
                led_max_factor: 0.0,
                force_fan_high: true,
            },
            SystemState::PumpFault | SystemState::SensorFault | SystemState::LowWater => Gates {
                pump_allowed: false,
                led_max_factor: 1.0,
                force_fan_high: false,
            },
            SystemState::LedFault => Gates {
                pump_allowed: true,
                led_max_factor: 0.0,
                force_fan_high: false,
            },
            // Boot/self-test/shutdown: hold actuators safe.
            SystemState::Boot | SystemState::SelfTest | SystemState::SafeShutdown => Gates {
                pump_allowed: false,
                led_max_factor: 0.0,
                force_fan_high: false,
            },
            // Normal operating states: controllers are trusted.
            SystemState::Normal
            | SystemState::Watering
            | SystemState::FanFault
            | SystemState::Maintenance => Gates {
                pump_allowed: true,
                led_max_factor: 1.0,
                force_fan_high: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctrl() -> SafetyController {
        let mut c = SafetyController::new();
        c.boot(BootReport {
            self_test_passed: true,
            calibration_valid: true,
            sensors_in_range: true,
            restored_age_days: 60,
            rtc_valid: true,
            pump_forced_off: true,
        });
        c
    }

    // §10.2 "Fault priority — highest-priority state wins".
    #[test]
    fn leak_beats_everything() {
        let mut c = ctrl();
        let inp = SafetyInputs {
            leak: true,
            over_temp_critical: true,
            pump_fault: true,
            moisture_sensor_invalid: true,
            reservoir_low: true,
            ..Default::default()
        };
        assert_eq!(c.arbitrate(&inp), SystemState::LeakDetected);
        assert!(!c.gates().pump_allowed);
    }

    #[test]
    fn over_temp_beats_pump_and_below() {
        let mut c = ctrl();
        let inp = SafetyInputs {
            over_temp_critical: true,
            pump_fault: true,
            moisture_sensor_invalid: true,
            reservoir_low: true,
            ..Default::default()
        };
        assert_eq!(c.arbitrate(&inp), SystemState::OverTemp);
        let g = c.gates();
        assert!(!g.pump_allowed);
        assert_eq!(g.led_max_factor, 0.0); // LED off/min
        assert!(g.force_fan_high);
    }

    #[test]
    fn priority_ladder_each_rung() {
        let cases = [
            (
                SafetyInputs {
                    pump_fault: true,
                    moisture_sensor_invalid: true,
                    reservoir_low: true,
                    ..Default::default()
                },
                SystemState::PumpFault,
            ),
            (
                SafetyInputs {
                    moisture_sensor_invalid: true,
                    reservoir_low: true,
                    ..Default::default()
                },
                SystemState::SensorFault,
            ),
            (
                SafetyInputs {
                    reservoir_low: true,
                    fan_fault: true,
                    ..Default::default()
                },
                SystemState::LowWater,
            ),
            (
                SafetyInputs {
                    fan_fault: true,
                    led_fault: true,
                    ..Default::default()
                },
                SystemState::FanFault,
            ),
            (
                SafetyInputs {
                    led_fault: true,
                    maintenance: true,
                    ..Default::default()
                },
                SystemState::LedFault,
            ),
            (
                SafetyInputs {
                    watering_active: true,
                    ..Default::default()
                },
                SystemState::Watering,
            ),
            (SafetyInputs::default(), SystemState::Normal),
        ];
        for (inp, want) in cases {
            let mut c = ctrl();
            assert_eq!(c.arbitrate(&inp), want, "inputs {inp:?}");
        }
    }

    #[test]
    fn leak_latches_until_manual_clear() {
        let mut c = ctrl();
        // Transient leak this tick...
        assert_eq!(
            c.arbitrate(&SafetyInputs {
                leak: true,
                ..Default::default()
            }),
            SystemState::LeakDetected
        );
        // ...stays latched even after the wet reading goes away.
        assert_eq!(
            c.arbitrate(&SafetyInputs::default()),
            SystemState::LeakDetected
        );
        assert!(!c.gates().pump_allowed);
        // Manual clear releases it.
        c.clear_leak();
        assert_eq!(c.arbitrate(&SafetyInputs::default()), SystemState::Normal);
        assert!(c.gates().pump_allowed);
    }

    #[test]
    fn sensor_fault_and_low_water_block_pump() {
        let mut c = ctrl();
        c.arbitrate(&SafetyInputs {
            moisture_sensor_invalid: true,
            ..Default::default()
        });
        assert!(!c.gates().pump_allowed);
        c.arbitrate(&SafetyInputs {
            reservoir_low: true,
            ..Default::default()
        });
        assert!(!c.gates().pump_allowed);
    }

    #[test]
    fn fan_and_led_faults_do_not_stop_watering() {
        let mut c = ctrl();
        c.arbitrate(&SafetyInputs {
            fan_fault: true,
            ..Default::default()
        });
        assert!(c.gates().pump_allowed);
        c.arbitrate(&SafetyInputs {
            led_fault: true,
            ..Default::default()
        });
        assert!(c.gates().pump_allowed);
        assert_eq!(c.gates().led_max_factor, 0.0); // but LED is cut
    }

    #[test]
    fn boot_with_bad_calibration_does_not_reach_normal() {
        let mut c = SafetyController::new();
        let s = c.boot(BootReport {
            self_test_passed: true,
            calibration_valid: false,
            sensors_in_range: true,
            restored_age_days: 0,
            rtc_valid: false,
            pump_forced_off: true,
        });
        assert_eq!(s, SystemState::SensorFault);
        assert!(!c.gates().pump_allowed); // fail-safe: no auto-watering on bad calibration
    }
}
