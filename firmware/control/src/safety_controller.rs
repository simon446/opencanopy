//! Top-level state machine and fault-priority arbitration. Spec §9.3, §9.4, §11.4; ECO-003.
//!
//! V1 is **passive** (no pump) and **fan-less**: the grow LED is the only actuator. This machine no
//! longer gates a pump — its only enforced output is the LED power factor. It is a **monitor + warn**
//! arbiter: it picks the single highest-priority condition to surface on the System LED, and forces
//! the LED off only for over-temp / boot / shutdown.

/// The required firmware states (§9.3, as revised by ECO-003 — pump/fan states removed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    Boot,
    SelfTest,
    Normal,
    /// Reservoir low/empty — refill prompt (passive supply will dry out).
    LowWater,
    /// Catch-tray wet — flood/overflow warning. Latches until manually cleared (§11.4). With no pump
    /// there is nothing to lock out; this is purely a warning.
    LeakDetected,
    /// Substrate above the wet band — water-logging risk.
    MoistureHigh,
    /// Substrate below the dry band — the passive supply is not keeping up (check reservoir/wick).
    MoistureLow,
    /// Moisture (or air) sensor invalid/stuck/uncalibrated — reading not trustworthy (§7.6).
    SensorFault,
    /// Air or LED-heatsink over-temperature — the LED is cut back to shed heat (the only lever).
    OverTempLed,
    Maintenance,
    SafeShutdown,
}

impl SystemState {
    pub const fn name(self) -> &'static str {
        match self {
            SystemState::Boot => "BOOT",
            SystemState::SelfTest => "SELF_TEST",
            SystemState::Normal => "NORMAL",
            SystemState::LowWater => "LOW_WATER",
            SystemState::LeakDetected => "LEAK_DETECTED",
            SystemState::MoistureHigh => "MOISTURE_HIGH",
            SystemState::MoistureLow => "MOISTURE_LOW",
            SystemState::SensorFault => "SENSOR_FAULT",
            SystemState::OverTempLed => "OVER_TEMP_LED",
            SystemState::Maintenance => "MAINTENANCE",
            SystemState::SafeShutdown => "SAFE_SHUTDOWN",
        }
    }
}

/// Raw fault/condition signals feeding arbitration each control tick. Booleans are produced by the
/// sensors and the individual monitors; arbitration decides which one is surfaced.
#[derive(Debug, Clone, Copy, Default)]
pub struct SafetyInputs {
    /// Catch-tray wet (§11.4). Highest priority; latches until manual clear.
    pub leak: bool,
    /// Air or LED-heatsink over-temp (>35 °C air / heatsink fault, §9.5) — cut the LED.
    pub over_temp_led: bool,
    /// Moisture (or air) sensor invalid/implausible (§7.6).
    pub sensor_invalid: bool,
    /// Reservoir low/empty — refill prompt (§9.6).
    pub reservoir_low: bool,
    /// Substrate above the wet band — water-logging warning.
    pub moisture_high: bool,
    /// Substrate below the dry band — drying-out warning.
    pub moisture_low: bool,
    /// Dev/overwinter maintenance mode is engaged.
    pub maintenance: bool,
}

/// Actuator permission derived from the active state. The only actuator in V1 is the LED.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gates {
    /// Multiplier applied to the commanded LED power (0.0 = forced off/min).
    pub led_max_factor: f32,
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
    /// LED was commanded off before anything else (§9.4 step 7). Always true on a clean boot.
    pub led_forced_off: bool,
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

    /// Run the §9.4 boot sequence. LED is forced off first; on a failed self-test or invalid
    /// calibration we land in SELF_TEST/SENSOR_FAULT rather than NORMAL (fail-safe, §7.6).
    pub fn boot(&mut self, report: BootReport) -> SystemState {
        self.booted = true;
        // Step 7 of §9.4: actuators off before anything else (the LED is the only one now).
        debug_assert!(report.led_forced_off, "boot must force the LED off (§9.4)");
        self.state = if !report.self_test_passed {
            SystemState::SelfTest // stay in self-test / surface fatal fault
        } else if !report.calibration_valid {
            // Missing/corrupt calibration must not act on bad data — fault, don't trust (§7.6).
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
    /// LEAK > OVER_TEMP_LED > SENSOR_FAULT > LOW_WATER > MOISTURE_HIGH > MOISTURE_LOW >
    /// MAINTENANCE > NORMAL.
    ///
    /// Leak latches: once seen, it stays the active state until [`clear_leak`](Self::clear_leak).
    pub fn arbitrate(&mut self, inp: &SafetyInputs) -> SystemState {
        if inp.leak {
            self.leak_latched = true;
        }
        self.state = if self.leak_latched {
            SystemState::LeakDetected
        } else if inp.over_temp_led {
            SystemState::OverTempLed
        } else if inp.sensor_invalid {
            SystemState::SensorFault
        } else if inp.reservoir_low {
            SystemState::LowWater
        } else if inp.moisture_high {
            SystemState::MoistureHigh
        } else if inp.moisture_low {
            SystemState::MoistureLow
        } else if inp.maintenance {
            SystemState::Maintenance
        } else {
            SystemState::Normal
        };
        self.state
    }

    /// Derive the LED gate from the current state. Over-temp / boot / shutdown force the LED off;
    /// every other state trusts the light controller's commanded power.
    pub fn gates(&self) -> Gates {
        match self.state {
            SystemState::OverTempLed
            | SystemState::Boot
            | SystemState::SelfTest
            | SystemState::SafeShutdown => Gates {
                led_max_factor: 0.0,
            },
            _ => Gates {
                led_max_factor: 1.0,
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
            led_forced_off: true,
        });
        c
    }

    // §10.2 "Fault priority — highest-priority state wins".
    #[test]
    fn leak_beats_everything() {
        let mut c = ctrl();
        let inp = SafetyInputs {
            leak: true,
            over_temp_led: true,
            sensor_invalid: true,
            reservoir_low: true,
            moisture_low: true,
            ..Default::default()
        };
        assert_eq!(c.arbitrate(&inp), SystemState::LeakDetected);
    }

    #[test]
    fn over_temp_cuts_the_led() {
        let mut c = ctrl();
        let inp = SafetyInputs {
            over_temp_led: true,
            sensor_invalid: true,
            reservoir_low: true,
            ..Default::default()
        };
        assert_eq!(c.arbitrate(&inp), SystemState::OverTempLed);
        assert_eq!(c.gates().led_max_factor, 0.0); // LED off/min — only thermal lever
    }

    #[test]
    fn priority_ladder_each_rung() {
        let cases = [
            (
                SafetyInputs {
                    sensor_invalid: true,
                    reservoir_low: true,
                    moisture_low: true,
                    ..Default::default()
                },
                SystemState::SensorFault,
            ),
            (
                SafetyInputs {
                    reservoir_low: true,
                    moisture_high: true,
                    moisture_low: true,
                    ..Default::default()
                },
                SystemState::LowWater,
            ),
            (
                SafetyInputs {
                    moisture_high: true,
                    moisture_low: true,
                    ..Default::default()
                },
                SystemState::MoistureHigh,
            ),
            (
                SafetyInputs {
                    moisture_low: true,
                    ..Default::default()
                },
                SystemState::MoistureLow,
            ),
            (
                SafetyInputs {
                    maintenance: true,
                    ..Default::default()
                },
                SystemState::Maintenance,
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
        // Manual clear releases it.
        c.clear_leak();
        assert_eq!(c.arbitrate(&SafetyInputs::default()), SystemState::Normal);
    }

    #[test]
    fn warnings_do_not_cut_the_led() {
        // A moisture/low-water warning must not dim the grow light — only over-temp does.
        let mut c = ctrl();
        c.arbitrate(&SafetyInputs {
            reservoir_low: true,
            moisture_low: true,
            ..Default::default()
        });
        assert_eq!(c.gates().led_max_factor, 1.0);
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
            led_forced_off: true,
        });
        assert_eq!(s, SystemState::SensorFault);
    }
}
