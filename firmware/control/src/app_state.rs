//! Top-level orchestrator: wires the plant profile, light/irrigation/climate controllers, safety
//! state machine, LED panel, calibration and logging into one deterministic per-tick `step`.
//!
//! This is the integration seam the simulator (`sim/`) and the on-target `controller/` both drive:
//! they supply a [`SensorFrame`] from real or simulated peripherals, and apply the returned
//! [`Commands`] back to the actuators. All the policy lives in the individual controller modules;
//! this just sequences them and enforces the safety gates on top.

use crate::calibration::{self, LoadedCalibration};
use crate::climate_controller::{self, ClimateInputs};
use crate::hal::{SensorError, TempRh, WallTime};
use crate::irrigation_controller::{self as irr, IrrigationController, MoistureValidator};
use crate::led_status::{
    self, ClimateHealth, LightHealth, MoistureHealth, Panel, PanelInputs, WaterLevel,
};
use crate::light_controller::{self, LightInputs};
use crate::logging::{LogEntry, LogKind, OnboardLog};
use crate::plant_profile::{self, Setpoints, Stage};
use crate::safety_controller::{BootReport, Gates, SafetyController, SafetyInputs, SystemState};
use crate::scheduler;

const DAY_MS: u64 = 86_400_000;
/// Firmware version stamped into logs (§9.10).
pub const FIRMWARE_VERSION: u16 = 1;
/// Log a periodic sensor snapshot at most this often (§9.10: every 5–15 min). 10 min here.
const SENSOR_LOG_INTERVAL_MS: u64 = 10 * 60_000;

/// One frame of sensor data + environment for a control tick.
#[derive(Debug, Clone, Copy)]
pub struct SensorFrame {
    pub now_ms: u64,
    pub rtc: WallTime,
    pub temp_rh: Result<TempRh, SensorError>,
    pub moisture_raw: Result<u16, SensorError>,
    pub reservoir_low: bool,
    pub leak: bool,
    pub led_heat_c: Option<f32>,
    pub fan_tach_rpm: Option<u16>,
}

/// Commanded actuator outputs + observability for one tick.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Commands {
    pub pump_on: bool,
    pub pump_run_seconds: f32,
    pub pump_dose_ml: u16,
    pub led_pct: u8,
    pub fan_pct: u8,
    pub state: SystemState,
    pub panel: Panel,
    pub stage: Stage,
    pub light_on: bool,
    pub vpd_kpa: f32,
    /// `None` when the moisture sensor is invalid (auto-watering disabled).
    pub moisture_pct: Option<f32>,
    /// Observability: LED power was reduced by thermal/heat-sink derating this tick (§9.5).
    pub led_derated: bool,
    /// Observability: climate flagged a critical temp/humidity condition this tick (§9.7).
    pub climate_red: bool,
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
    irrigation: IrrigationController,
    moisture: MoistureValidator,
    log: OnboardLog,
    boot_ms: u64,
    age_days_base: u32,
    maintenance: bool,
    last_sensor_log_ms: Option<u64>,
    last_logged_state: Option<SystemState>,
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
            // §9.4 step 7: pump forced off before anything — the HAL pump defaults off; this asserts it.
            pump_forced_off: true,
        });
        let mut app = App {
            cfg,
            cal,
            safety,
            irrigation: IrrigationController::new(),
            moisture: MoistureValidator::default(),
            log: OnboardLog::new(),
            boot_ms,
            age_days_base: restored_age_days,
            maintenance: false,
            last_sensor_log_ms: None,
            last_logged_state: None,
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

        // --- Light controller. ---
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

        // --- Climate / fan controller. ---
        let minute = scheduler::minute_of_hour(frame.rtc, self.cfg.utc_offset_s, self.boot_ms, now);
        let climate = climate_controller::evaluate(&ClimateInputs {
            air,
            sp: &sp,
            lights_on: light.on,
            minute_of_hour: minute,
            tach_rpm: frame.fan_tach_rpm,
        });

        // --- Moisture validation (drives auto-watering enable + sensor fault). ---
        let moisture = self.moisture.validate(
            now,
            frame.moisture_raw,
            &self.cal.cal,
            self.cal.auto_watering_enabled,
        );

        // --- Irrigation decision. ---
        let day_index = scheduler::day_index(frame.rtc, self.cfg.utc_offset_s, self.boot_ms, now);
        let decision = self.irrigation.tick(&irr::Inputs {
            sp: &sp,
            now_ms: now,
            moisture,
            ml_per_sec: self.cal.cal.pump_ml_per_sec,
            reservoir_low: frame.reservoir_low,
            leak: frame.leak,
            light_on: light.on,
            light_fraction: light.phase.fraction(),
            hours_to_off: light.phase.hours_to_off(),
            day_index,
        });

        // --- Safety arbitration. ---
        let over_temp_critical = air_ok && air.temp_c > 35.0;
        let pump_fault = self.irrigation.pump_fault_latched()
            || decision.fault == Some(irr::IrrigationFault::PumpFault);
        let sensor_invalid = moisture.is_none();
        let safety_inputs = SafetyInputs {
            leak: frame.leak,
            over_temp_critical,
            pump_fault,
            moisture_sensor_invalid: sensor_invalid,
            reservoir_low: frame.reservoir_low,
            fan_fault: climate.fan_fault,
            led_fault: light.derate.led_fault,
            maintenance: self.maintenance,
            watering_active: decision.watering_active,
        };
        let state = self.safety.arbitrate(&safety_inputs);
        let gates: Gates = self.safety.gates();

        // --- Apply safety gates on top of controller outputs (safety always wins). ---
        let pump_on = decision.pump_on && gates.pump_allowed;
        let led_pct = (light.commanded_pct as f32 * gates.led_max_factor) as u8;
        let fan_pct = if gates.force_fan_high {
            100
        } else {
            climate.fan_pct
        };

        // --- LED status panel. ---
        let panel = self.render_panel(
            state,
            frame.reservoir_low,
            moisture,
            &sp,
            &light,
            &climate,
            air_ok,
            light.on,
        );

        // --- Logging (§9.10). ---
        self.log_events(
            frame,
            &decision,
            state,
            &light,
            air,
            climate.vpd_kpa,
            led_pct,
            fan_pct,
            moisture,
        );

        Commands {
            pump_on,
            pump_run_seconds: if pump_on { decision.run_seconds } else { 0.0 },
            pump_dose_ml: if pump_on { decision.dose_ml } else { 0 },
            led_pct,
            fan_pct,
            state,
            panel,
            stage,
            light_on: light.on,
            vpd_kpa: climate.vpd_kpa,
            moisture_pct: moisture,
            led_derated: light.derate.derated,
            climate_red: climate.climate_red,
            rtc_fallback: light.rtc_fallback,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_panel(
        &self,
        state: SystemState,
        reservoir_low: bool,
        moisture: Option<f32>,
        sp: &Setpoints,
        light: &light_controller::LightOutput,
        climate: &climate_controller::ClimateOutput,
        air_ok: bool,
        light_on: bool,
    ) -> Panel {
        // Leak is a water-subsystem fault that locks out watering, so it reds the Water LED too
        // (spec §10.3 "leak → red water/system"), alongside the reservoir-low lockout.
        let water = if reservoir_low || state == SystemState::LeakDetected {
            WaterLevel::EmptyLockout
        } else {
            WaterLevel::Ok
        };
        let moisture_h = match moisture {
            None => MoistureHealth::FaultOrCriticalOrWaterlogged,
            Some(m) if m < sp.moisture_critical_pct => MoistureHealth::FaultOrCriticalOrWaterlogged,
            Some(m) if m < sp.moisture_dry_pct || m > sp.moisture_wet_pct => {
                MoistureHealth::DrySoonOrWetHigh
            }
            Some(_) => MoistureHealth::InTarget,
        };
        let light_h = if light.derate.led_fault || light.derate.critical {
            LightHealth::FaultOrOverTemp
        } else if light.derate.derated || light.rtc_fallback {
            LightHealth::ThermalDimOrUncertain
        } else {
            LightHealth::Normal
        };
        let climate_h = if climate.climate_red || !air_ok {
            ClimateHealth::CriticalTempOrHumidity
        } else if climate.climate_amber {
            ClimateHealth::OutsidePreferred
        } else {
            ClimateHealth::Ok
        };
        led_status::render(&PanelInputs {
            state,
            water,
            moisture: moisture_h,
            light: light_h,
            climate: climate_h,
            night_mode: !light_on,
            maintenance_due: self.maintenance,
            rtc_fallback: light.rtc_fallback,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn log_events(
        &mut self,
        frame: &SensorFrame,
        decision: &irr::Decision,
        state: SystemState,
        light: &light_controller::LightOutput,
        air: TempRh,
        vpd: f32,
        led_pct: u8,
        fan_pct: u8,
        moisture: Option<f32>,
    ) {
        let ts = frame.rtc.unix_s;
        let tv = frame.rtc.valid;

        // Watering events.
        if decision.watering_active {
            self.log.push(LogEntry {
                ts_unix_s: ts,
                ts_valid: tv,
                kind: LogKind::Watering {
                    dose_ml: decision.dose_ml,
                    run_seconds_x10: (decision.run_seconds * 10.0) as u16,
                    daily_total_ml: self.irrigation.daily_watered_ml() as u16,
                },
            });
        }

        // Fault/state transitions.
        if self.last_logged_state != Some(state)
            && !matches!(state, SystemState::Normal | SystemState::Watering)
        {
            self.log.push(LogEntry {
                ts_unix_s: ts,
                ts_valid: tv,
                kind: LogKind::Fault { state },
            });
        }
        self.last_logged_state = Some(state);

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
                    fan_pct,
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
            version: 3,
            moisture_raw_dry: 1000,
            moisture_raw_wet: 3000,
            pump_ml_per_sec: 3.8,
            fan_min_pwm: 28,
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
            fan_tach_rpm: Some(1200),
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
        assert!(app.calibration().auto_watering_enabled);
    }

    #[test]
    fn full_tick_waters_when_dry_midday() {
        let mut app = booted_app();
        // 12:00, lights on, dry → should dose.
        let cmd = app.step(&frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            20.0,
        ));
        assert_eq!(cmd.stage, Stage::Vegetative);
        assert!(cmd.light_on);
        assert!(cmd.pump_on);
        assert_eq!(cmd.state, SystemState::Watering);
    }

    #[test]
    fn leak_gate_overrides_everything() {
        let mut app = booted_app();
        let mut f = frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            10.0,
        );
        f.leak = true;
        let cmd = app.step(&f);
        assert!(!cmd.pump_on);
        assert_eq!(cmd.state, SystemState::LeakDetected);
        // latched: even after leak clears the state holds until manual clear.
        let cmd2 = app.step(&frame(
            60_000,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            10.0,
        ));
        assert_eq!(cmd2.state, SystemState::LeakDetected);
        app.clear_leak();
        let cmd3 = app.step(&frame(
            120_000,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            50.0,
        ));
        assert_ne!(cmd3.state, SystemState::LeakDetected);
    }

    #[test]
    fn missing_calibration_disables_watering_and_faults() {
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
        assert!(!app.calibration().auto_watering_enabled);
        let cmd = app.step(&frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            10.0,
        ));
        assert!(!cmd.pump_on);
        assert_eq!(cmd.state, SystemState::SensorFault);
        assert!(cmd.moisture_pct.is_none());
    }

    #[test]
    fn over_temp_cuts_led_and_pump() {
        let mut app = booted_app();
        let mut f = frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            10.0,
        );
        f.temp_rh = Ok(TempRh {
            temp_c: 36.0,
            rh_pct: 40.0,
        });
        let cmd = app.step(&f);
        assert_eq!(cmd.state, SystemState::OverTemp);
        assert_eq!(cmd.led_pct, 0); // LED off/min
        assert!(!cmd.pump_on); // no watering on temperature alone
        assert_eq!(cmd.fan_pct, 100); // fan forced high
    }

    #[test]
    fn versions_logged_at_boot() {
        let app = booted_app();
        let has_versions = app.log().iter().any(|e| {
            matches!(
                e.kind,
                LogKind::Versions {
                    firmware: 1,
                    calibration: 3
                }
            )
        });
        assert!(has_versions);
    }
}
