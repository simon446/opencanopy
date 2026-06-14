//! Top-level orchestrator: wires the plant profile, light/moisture/climate monitors, safety state
//! machine, LED panel, calibration and logging into one deterministic per-tick `step`.
//!
//! This is the integration seam the simulator (`sim/`) and the on-target `controller/` both drive:
//! they supply a [`SensorFrame`] from real or simulated peripherals, and apply the returned
//! [`Commands`] back to the actuator. **V1 is passive (no pump) and fan-less (ECO-003/ECO-001): the
//! grow LED is the only actuator.** Watering is monitored and warned, never actuated. All policy
//! lives in the individual modules; this sequences them and enforces the LED gate on top.

use crate::calibration::{self, LoadedCalibration};
use crate::climate_controller::{self, ClimateInputs};
use crate::hal::{SensorError, TempRh, WallTime};
use crate::led_status::{self, LightHealth, MoistureHealth, Panel, PanelInputs, WaterLevel};
use crate::light_controller::{self, LightInputs};
use crate::logging::{LogEntry, LogKind, OnboardLog};
use crate::moisture_monitor::{self, MoistureStatus, MoistureValidator};
use crate::plant_profile::{self, Setpoints, Stage};
use crate::safety_controller::{BootReport, Gates, SafetyController, SafetyInputs, SystemState};

const DAY_MS: u64 = 86_400_000;
/// Firmware version stamped into logs (§9.10).
pub const FIRMWARE_VERSION: u16 = 1;
/// Log a periodic sensor snapshot at most this often (§9.10: every 5–15 min). 10 min here.
const SENSOR_LOG_INTERVAL_MS: u64 = 10 * 60_000;
/// Critical over-temp air threshold (§9.5): above this the LED is cut.
const OVER_TEMP_C: f32 = 35.0;

/// One frame of sensor data + environment for a control tick.
#[derive(Debug, Clone, Copy)]
pub struct SensorFrame {
    pub now_ms: u64,
    pub rtc: WallTime,
    pub temp_rh: Result<TempRh, SensorError>,
    pub moisture_raw: Result<u16, SensorError>,
    /// Reservoir at/below the low-level mark — refill prompt (V1 passive, ECO-003).
    pub reservoir_low: bool,
    /// Catch-tray wet — flood/overflow warning.
    pub leak: bool,
    pub led_heat_c: Option<f32>,
}

/// Commanded actuator output + observability for one tick. The LED is the only actuator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Commands {
    pub led_pct: u8,
    pub state: SystemState,
    pub panel: Panel,
    pub stage: Stage,
    pub light_on: bool,
    pub vpd_kpa: f32,
    /// `None` when the moisture sensor is invalid/untrusted (§7.6).
    pub moisture_pct: Option<f32>,
    /// Observability: moisture warning classification this tick (passive monitor, ECO-003).
    pub moisture_status: MoistureStatus,
    /// Observability: LED power was reduced by thermal/heat-sink derating this tick (§9.5).
    pub led_derated: bool,
    /// Observability: climate (temp/RH/VPD) outside the preferred band — folds into the System LED.
    pub climate_warn: bool,
    /// Observability: running the RTC-invalid safe-schedule fallback (§9.4).
    pub rtc_fallback: bool,
}

/// Configuration fixed at construction.
#[derive(Debug, Clone, Copy, Default)]
pub struct AppConfig {
    /// Fixed UTC offset in seconds for local time-of-day (V1 has no DST). Default 0 = UTC.
    pub utc_offset_s: i32,
}

/// The whole controller. Owns all persistent sub-controller state.
pub struct App {
    cfg: AppConfig,
    cal: LoadedCalibration,
    safety: SafetyController,
    moisture: MoistureValidator,
    log: OnboardLog,
    boot_ms: u64,
    age_days_base: u32,
    maintenance: bool,
    last_sensor_log_ms: Option<u64>,
    last_logged_state: Option<SystemState>,
    last_reservoir_low: bool,
}

impl App {
    /// Construct and run the §9.4 boot sequence. `stored_cal` is the raw calibration record from
    /// flash (`None` = none present). `restored_age_days` comes from NVS/RTC (§9.4 step 5).
    pub fn boot(
        cfg: AppConfig,
        stored_cal: Option<&[u8]>,
        restored_age_days: u32,
        boot_ms: u64,
        rtc: WallTime,
        self_test_passed: bool,
    ) -> App {
        let cal = calibration::load(stored_cal);
        let mut safety = SafetyController::new();
        safety.boot(BootReport {
            self_test_passed,
            calibration_valid: cal.source == calibration::CalSource::Valid,
            sensors_in_range: true,
            restored_age_days,
            rtc_valid: rtc.valid,
            // §9.4 step 7: the LED (only actuator) is forced off before anything else.
            led_forced_off: true,
        });
        let mut app = App {
            cfg,
            cal,
            safety,
            moisture: MoistureValidator::default(),
            log: OnboardLog::new(),
            boot_ms,
            age_days_base: restored_age_days,
            maintenance: false,
            last_sensor_log_ms: None,
            last_logged_state: None,
            last_reservoir_low: false,
        };
        // Stamp versions at boot (§9.10).
        app.log.push(LogEntry {
            ts_unix_s: rtc.unix_s,
            ts_valid: rtc.valid,
            kind: LogKind::Versions {
                firmware: FIRMWARE_VERSION,
                calibration: app.cal.cal.version,
            },
        });
        app
    }

    pub fn state(&self) -> SystemState {
        self.safety.state()
    }
    pub fn log(&self) -> &OnboardLog {
        &self.log
    }
    pub fn calibration(&self) -> &LoadedCalibration {
        &self.cal
    }
    pub fn set_maintenance(&mut self, on: bool) {
        self.maintenance = on;
    }
    /// Service action: clear a latched leak (§11.4).
    pub fn clear_leak(&mut self) {
        self.safety.clear_leak();
    }

    /// Current grow-cycle age in days (restored base + elapsed since boot).
    pub fn age_days(&self, now_ms: u64) -> u32 {
        self.age_days_base + (now_ms.saturating_sub(self.boot_ms) / DAY_MS) as u32
    }

    /// Run one control tick. Pure function of `frame` + internal state.
    pub fn step(&mut self, frame: &SensorFrame) -> Commands {
        let now = frame.now_ms;
        let stage = if self.maintenance {
            Stage::Maintenance
        } else {
            plant_profile::stage_for_age(self.age_days(now))
        };
        let sp: Setpoints = plant_profile::setpoints(stage);

        // --- Air sensor (climate). On error, use neutral values but flag the climate sensor. ---
        let (air, air_ok) = match frame.temp_rh {
            Ok(v) => (v, true),
            Err(_) => (
                TempRh {
                    temp_c: 24.0,
                    rh_pct: 60.0,
                },
                false,
            ),
        };

        // --- Light controller (only actuator). ---
        let light = light_controller::evaluate(&LightInputs {
            sp: &sp,
            cal: &self.cal.cal,
            rtc: frame.rtc,
            utc_offset_s: self.cfg.utc_offset_s,
            boot_ms: self.boot_ms,
            now_ms: now,
            air_temp_c: air.temp_c,
            led_heat_c: frame.led_heat_c,
        });

        // --- Climate monitor (no actuator — VPD/health flags + LED-derate request). ---
        let climate = climate_controller::evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: light.on,
        });

        // --- Moisture monitor: validate, then classify into a warning (passive, no dosing). ---
        let moisture = self.moisture.validate(
            now,
            frame.moisture_raw,
            &self.cal.cal,
            self.cal.moisture_trusted,
        );
        let moisture_status = moisture_monitor::classify(moisture, &sp);

        // --- Safety arbitration (monitor + warn). ---
        // Over-temp on the LED: critical air temp, or the LED heat-sink ladder tripped its fault.
        let over_temp_led = (air_ok && air.temp_c > OVER_TEMP_C) || light.derate.led_fault;
        let safety_inputs = SafetyInputs {
            leak: frame.leak,
            over_temp_led,
            sensor_invalid: moisture_status == MoistureStatus::Fault,
            reservoir_low: frame.reservoir_low,
            moisture_high: moisture_status == MoistureStatus::High,
            moisture_low: matches!(
                moisture_status,
                MoistureStatus::Low | MoistureStatus::CriticalLow
            ),
            maintenance: self.maintenance,
        };
        let state = self.safety.arbitrate(&safety_inputs);
        let gates: Gates = self.safety.gates();

        // --- Apply the LED gate on top of the commanded power (safety always wins). ---
        let led_pct = (light.commanded_pct as f32 * gates.led_max_factor) as u8;
        let climate_warn = climate.climate_amber || climate.climate_red;

        // --- LED status panel (4 LEDs). ---
        let panel = self.render_panel(
            state,
            frame.reservoir_low,
            moisture_status,
            &light,
            climate_warn,
            light.on,
        );

        // --- Logging (§9.10). ---
        self.log_events(
            frame,
            state,
            &light,
            air,
            climate.vpd_kpa,
            led_pct,
            moisture,
        );

        Commands {
            led_pct,
            state,
            panel,
            stage,
            light_on: light.on,
            vpd_kpa: climate.vpd_kpa,
            moisture_pct: moisture,
            moisture_status,
            led_derated: light.derate.derated,
            climate_warn,
            rtc_fallback: light.rtc_fallback,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_panel(
        &self,
        state: SystemState,
        reservoir_low: bool,
        moisture_status: MoistureStatus,
        light: &light_controller::LightOutput,
        climate_warn: bool,
        light_on: bool,
    ) -> Panel {
        // Leak reds the Water LED (flood = water-subsystem fault); reservoir-low is an amber refill
        // prompt below it.
        let water = if state == SystemState::LeakDetected {
            WaterLevel::Empty
        } else if reservoir_low {
            WaterLevel::LowSoon
        } else {
            WaterLevel::Ok
        };
        let moisture_h = match moisture_status {
            MoistureStatus::Fault | MoistureStatus::CriticalLow => {
                MoistureHealth::FaultOrCriticalOrWaterlogged
            }
            MoistureStatus::Low | MoistureStatus::High => MoistureHealth::DrySoonOrWetHigh,
            MoistureStatus::Ok => MoistureHealth::InTarget,
        };
        let light_h =
            if state == SystemState::OverTempLed || light.derate.led_fault || light.derate.critical
            {
                LightHealth::FaultOrOverTemp
            } else if light.derate.derated || light.rtc_fallback {
                LightHealth::ThermalDimOrUncertain
            } else {
                LightHealth::Normal
            };
        led_status::render(&PanelInputs {
            state,
            water,
            moisture: moisture_h,
            light: light_h,
            night_mode: !light_on,
            maintenance_due: self.maintenance,
            rtc_fallback: light.rtc_fallback,
            climate_warn,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn log_events(
        &mut self,
        frame: &SensorFrame,
        state: SystemState,
        light: &light_controller::LightOutput,
        air: TempRh,
        vpd: f32,
        led_pct: u8,
        moisture: Option<f32>,
    ) {
        let ts = frame.rtc.unix_s;
        let tv = frame.rtc.valid;

        // Fault/warning state transitions.
        if self.last_logged_state != Some(state) && state != SystemState::Normal {
            self.log.push(LogEntry {
                ts_unix_s: ts,
                ts_valid: tv,
                kind: LogKind::Fault { state },
            });
        }
        self.last_logged_state = Some(state);

        // Reservoir-low rising edge (refill event).
        if frame.reservoir_low && !self.last_reservoir_low {
            self.log.push(LogEntry {
                ts_unix_s: ts,
                ts_valid: tv,
                kind: LogKind::ReservoirLow,
            });
        }
        self.last_reservoir_low = frame.reservoir_low;

        // LED derating events.
        if light.derate.derated {
            self.log.push(LogEntry {
                ts_unix_s: ts,
                ts_valid: tv,
                kind: LogKind::LedDerate {
                    factor_pct: (light.derate.factor * 100.0) as u8,
                    air_temp_c: air.temp_c,
                },
            });
        }

        // Periodic sensor snapshot.
        let due = match self.last_sensor_log_ms {
            None => true,
            Some(t) => frame.now_ms.saturating_sub(t) >= SENSOR_LOG_INTERVAL_MS,
        };
        if due {
            self.last_sensor_log_ms = Some(frame.now_ms);
            self.log.push(LogEntry {
                ts_unix_s: ts,
                ts_valid: tv,
                kind: LogKind::Sensors {
                    temp_c: air.temp_c,
                    rh_pct: air.rh_pct,
                    vpd_kpa: vpd,
                    moisture_pct: moisture.map(|m| m as i16).unwrap_or(-1),
                    reservoir_low: frame.reservoir_low,
                    light_pct: led_pct,
                },
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calibration::Calibration;

    fn valid_cal_bytes() -> [u8; calibration::RECORD_LEN] {
        Calibration {
            version: 4,
            moisture_raw_dry: 1000,
            moisture_raw_wet: 3000,
            led_ppfd_25: 120,
            led_ppfd_50: 240,
            led_ppfd_75: 360,
            led_ppfd_100: 480,
            reservoir_low_adc: 600,
        }
        .encode()
    }

    // raw count for a given normalized moisture % given the cal above.
    fn raw_for(pct: f32) -> u16 {
        (1000.0 + pct / 100.0 * 2000.0) as u16
    }

    fn frame(now: u64, rtc: WallTime, moisture_pct: f32) -> SensorFrame {
        SensorFrame {
            now_ms: now,
            rtc,
            temp_rh: Ok(TempRh {
                temp_c: 24.0,
                rh_pct: 60.0,
            }),
            moisture_raw: Ok(raw_for(moisture_pct)),
            reservoir_low: false,
            leak: false,
            led_heat_c: None,
        }
    }

    fn booted_app() -> App {
        let bytes = valid_cal_bytes();
        App::boot(
            AppConfig::default(),
            Some(&bytes),
            60, // vegetative
            0,
            WallTime {
                valid: true,
                unix_s: 12 * 3600,
            },
            true,
        )
    }

    #[test]
    fn boots_to_normal_with_valid_calibration() {
        let app = booted_app();
        assert_eq!(app.state(), SystemState::Normal);
        assert!(app.calibration().moisture_trusted);
    }

    #[test]
    fn full_tick_in_band_is_normal_no_warning() {
        let mut app = booted_app();
        // 08:00, lights on, moisture in band (45 %) → NORMAL, light on, no warning.
        let cmd = app.step(&frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        ));
        assert_eq!(cmd.stage, Stage::Vegetative);
        assert!(cmd.light_on);
        assert_eq!(cmd.state, SystemState::Normal);
        assert_eq!(cmd.moisture_status, MoistureStatus::Ok);
    }

    #[test]
    fn dry_substrate_warns_moisture_low_but_keeps_the_light() {
        let mut app = booted_app();
        // 20 % is below the vegetative dry threshold (30) but above critical (17).
        let cmd = app.step(&frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            20.0,
        ));
        assert_eq!(cmd.state, SystemState::MoistureLow);
        assert_eq!(cmd.moisture_status, MoistureStatus::Low);
        // A moisture warning must NOT cut the grow light (only over-temp does).
        assert!(cmd.led_pct > 0);
    }

    #[test]
    fn too_wet_warns_moisture_high() {
        let mut app = booted_app();
        // 70 % is above the wet threshold (55).
        let cmd = app.step(&frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            70.0,
        ));
        assert_eq!(cmd.state, SystemState::MoistureHigh);
    }

    #[test]
    fn reservoir_low_warns_low_water() {
        let mut app = booted_app();
        let mut f = frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        );
        f.reservoir_low = true;
        let cmd = app.step(&f);
        assert_eq!(cmd.state, SystemState::LowWater);
        assert_eq!(cmd.panel.water.color, led_status::LedColor::Amber);
    }

    #[test]
    fn leak_warns_and_latches_red_water_and_system() {
        let mut app = booted_app();
        let mut f = frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        );
        f.leak = true;
        let cmd = app.step(&f);
        assert_eq!(cmd.state, SystemState::LeakDetected);
        assert_eq!(cmd.panel.water.color, led_status::LedColor::Red);
        assert_eq!(cmd.panel.system.color, led_status::LedColor::Red);
        // Latches: even after the leak clears the state holds until manual clear.
        let cmd2 = app.step(&frame(
            60_000,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        ));
        assert_eq!(cmd2.state, SystemState::LeakDetected);
        app.clear_leak();
        let cmd3 = app.step(&frame(
            120_000,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        ));
        assert_ne!(cmd3.state, SystemState::LeakDetected);
    }

    #[test]
    fn missing_calibration_distrusts_moisture_and_faults() {
        let mut app = App::boot(
            AppConfig::default(),
            None, // no calibration
            60,
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            true,
        );
        assert!(!app.calibration().moisture_trusted);
        let cmd = app.step(&frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        ));
        assert_eq!(cmd.state, SystemState::SensorFault);
        assert!(cmd.moisture_pct.is_none());
    }

    #[test]
    fn over_temp_cuts_the_led() {
        let mut app = booted_app();
        let mut f = frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            45.0,
        );
        f.temp_rh = Ok(TempRh {
            temp_c: 36.0,
            rh_pct: 40.0,
        });
        let cmd = app.step(&f);
        assert_eq!(cmd.state, SystemState::OverTempLed);
        assert_eq!(cmd.led_pct, 0); // LED off/min — the only thermal lever (no fan/pump)
    }

    #[test]
    fn versions_logged_at_boot() {
        let app = booted_app();
        let has_versions = app.log().iter().any(|e| {
            matches!(
                e.kind,
                LogKind::Versions {
                    firmware: 1,
                    calibration: 4
                }
            )
        });
        assert!(has_versions);
    }
}
