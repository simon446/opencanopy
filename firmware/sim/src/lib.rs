//! Host plant/environment simulator (§10.3). Drives the **real** `control` crate through its
//! `app_state::App` orchestrator (which itself sits on the `hal.rs` seam), feeding it a simulated
//! environment and applying its actuator commands back into that environment. The 11 required
//! scenarios live in `tests/scenarios.rs`.
//!
//! Nothing here re-implements control policy — assertions are made against the genuine controller
//! outputs (WI-FW-09 acceptance).

pub mod models;

use control::app_state::{App, AppConfig, Commands, SensorFrame};
use control::calibration::Calibration;
use control::hal::{SensorError, TempRh, WallTime};
use control::safety_controller::SystemState;

/// Control-loop period. §9.6 runs checks every 5 minutes.
pub const TICK_MS: u64 = 5 * 60_000;

/// The simulated environment (the "plant + room").
#[derive(Debug, Clone, Copy)]
pub struct Env {
    /// True normalized substrate moisture, 0..100.
    pub moisture_pct: f32,
    /// Water remaining in the reservoir, mL.
    pub reservoir_ml: f32,
    /// Ambient room temperature (the device cannot cool below this), °C.
    pub room_temp_c: f32,
    /// Ambient room relative humidity, %.
    pub room_rh_pct: f32,
    last_led_pct: u8,
    last_fan_pct: u8,
}

/// Fault/condition injections (§10.3 "leak/sensor failure can be injected").
#[derive(Debug, Clone, Copy, Default)]
pub struct Inject {
    pub leak: bool,
    /// Hold the moisture probe at a fixed reported % (stuck sensor), regardless of true moisture.
    pub moisture_stuck_pct: Option<f32>,
    /// Force a moisture-sensor read error.
    pub moisture_error: Option<SensorError>,
    /// Pump motor runs but moves no water (disconnected/clogged) → no moisture rise, no drawdown.
    pub pump_disconnected: bool,
    /// Fan tach reads zero while commanded on → FAN_FAULT.
    pub fan_tach_zero: bool,
}

/// Aggregate metrics collected across a run, for scenario assertions.
#[derive(Debug, Clone)]
pub struct Metrics {
    pub ticks: u32,
    pub pump_runs: u32,
    pub total_watered_ml: u32,
    pub pump_ran_while_leak: bool,
    pub pump_ran_while_reservoir_low: bool,
    pub max_moisture_pct: f32,
    /// Lowest moisture seen after the first simulated day (warm-up excluded).
    pub min_moisture_after_warmup_pct: f32,
    pub min_reservoir_ml: f32,
    pub max_fan_pct: u8,
    pub led_on_ticks: u32,
    /// True iff every lights-off tick had the LED fully off.
    pub led_off_at_night_ok: bool,
    pub saw_led_derate: bool,
    pub saw_climate_red: bool,
    pub saw_rtc_fallback: bool,
    pub states: Vec<SystemState>,
}

impl Metrics {
    pub fn saw(&self, s: SystemState) -> bool {
        self.states.contains(&s)
    }
    pub fn final_state(&self) -> SystemState {
        *self.states.last().expect("no ticks run")
    }
}

/// Pending water deliveries that have not yet reached the probe (soak delay).
#[derive(Debug, Clone, Copy)]
struct Pending {
    apply_at_ms: u64,
    pct: f32,
}

/// The simulator harness.
pub struct Sim {
    pub app: App,
    pub env: Env,
    pub cal: Calibration,
    pub inject: Inject,
    pub rtc_valid: bool,
    led_heat_present: bool,
    start_unix_s: u64,
    now_ms: u64,
    pending: Vec<Pending>,
    pub metrics: Metrics,
    pub last: Option<Commands>,
}

/// Builder configuration for a run.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub age_days: u32,
    pub start_moisture_pct: f32,
    pub start_reservoir_ml: f32,
    pub room_temp_c: f32,
    pub room_rh_pct: f32,
    pub rtc_valid: bool,
    /// Wall-clock seconds-of-day the run starts at (when `rtc_valid`).
    pub start_unix_s: u64,
    pub led_heat_present: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            age_days: 60, // vegetative
            start_moisture_pct: 50.0,
            start_reservoir_ml: 5000.0,
            room_temp_c: 23.0,
            room_rh_pct: 60.0,
            rtc_valid: true,
            start_unix_s: 6 * 3600, // 06:00 — lights-on
            led_heat_present: false,
        }
    }
}

fn default_cal() -> Calibration {
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
}

impl Sim {
    /// Construct and boot a sim from a config, with valid calibration loaded.
    pub fn new(cfg: Config) -> Sim {
        Self::with_calibration(cfg, Some(default_cal()))
    }

    /// Construct with an explicit calibration (`None` = no calibration in flash → fail-safe path).
    pub fn with_calibration(cfg: Config, cal: Option<Calibration>) -> Sim {
        let rtc = if cfg.rtc_valid {
            WallTime {
                valid: true,
                unix_s: cfg.start_unix_s,
            }
        } else {
            WallTime::INVALID
        };
        let cal_bytes = cal.map(|c| c.encode());
        let app = App::boot(
            AppConfig { utc_offset_s: 0 },
            cal_bytes.as_ref().map(|b| b.as_slice()),
            cfg.age_days,
            0,
            rtc,
            true,
        );
        Sim {
            app,
            env: Env {
                moisture_pct: cfg.start_moisture_pct,
                reservoir_ml: cfg.start_reservoir_ml,
                room_temp_c: cfg.room_temp_c,
                room_rh_pct: cfg.room_rh_pct,
                last_led_pct: 0,
                last_fan_pct: 0,
            },
            cal: cal.unwrap_or_else(default_cal),
            inject: Inject::default(),
            rtc_valid: cfg.rtc_valid,
            led_heat_present: cfg.led_heat_present,
            start_unix_s: cfg.start_unix_s,
            now_ms: 0,
            pending: Vec::new(),
            metrics: Metrics {
                ticks: 0,
                pump_runs: 0,
                total_watered_ml: 0,
                pump_ran_while_leak: false,
                pump_ran_while_reservoir_low: false,
                max_moisture_pct: cfg.start_moisture_pct,
                min_moisture_after_warmup_pct: 100.0,
                min_reservoir_ml: cfg.start_reservoir_ml,
                max_fan_pct: 0,
                led_on_ticks: 0,
                led_off_at_night_ok: true,
                saw_led_derate: false,
                saw_climate_red: false,
                saw_rtc_fallback: false,
                states: Vec::new(),
            },
            last: None,
        }
    }

    fn rtc(&self) -> WallTime {
        if self.rtc_valid {
            WallTime {
                valid: true,
                unix_s: self.start_unix_s + self.now_ms / 1000,
            }
        } else {
            WallTime::INVALID
        }
    }

    fn moisture_raw(&self) -> Result<u16, SensorError> {
        if let Some(e) = self.inject.moisture_error {
            return Err(e);
        }
        let pct = self
            .inject
            .moisture_stuck_pct
            .unwrap_or(self.env.moisture_pct);
        let raw = self.cal.moisture_raw_dry as f32
            + (pct / 100.0) * (self.cal.moisture_raw_wet - self.cal.moisture_raw_dry) as f32;
        Ok(raw as u16)
    }

    /// Advance one control tick.
    pub fn step(&mut self) -> Commands {
        let now = self.now_ms;

        // 1) Apply matured water (soaked in) before the controller reads the probe.
        let matured: f32 = self
            .pending
            .iter()
            .filter(|p| p.apply_at_ms <= now)
            .map(|p| p.pct)
            .sum();
        if matured != 0.0 {
            self.env.moisture_pct = (self.env.moisture_pct + matured).clamp(0.0, 100.0);
        }
        self.pending.retain(|p| p.apply_at_ms > now);

        // 2) Build the sensor frame from the environment.
        let temp_c = models::air_temp(
            self.env.room_temp_c,
            self.env.last_led_pct,
            self.env.last_fan_pct,
        );
        let rh = models::air_rh(self.env.room_rh_pct, self.env.last_fan_pct);
        let led_heat = if self.led_heat_present {
            Some(self.env.room_temp_c + self.env.last_led_pct as f32 * 0.6)
        } else {
            None
        };
        let reservoir_low = self.env.reservoir_ml <= models::RESERVOIR_LOW_ML;
        let frame = SensorFrame {
            now_ms: now,
            rtc: self.rtc(),
            temp_rh: Ok(TempRh { temp_c, rh_pct: rh }),
            moisture_raw: self.moisture_raw(),
            reservoir_low,
            leak: self.inject.leak,
            led_heat_c: led_heat,
            fan_tach_rpm: if self.inject.fan_tach_zero {
                Some(0)
            } else {
                Some(1500)
            },
        };

        // 3) Run the real controller.
        let cmd = self.app.step(&frame);

        // 4) Apply actuator effects back into the environment.
        if cmd.pump_on {
            self.metrics.pump_runs += 1;
            self.metrics.pump_ran_while_leak |= self.inject.leak;
            self.metrics.pump_ran_while_reservoir_low |= reservoir_low;
            if !self.inject.pump_disconnected {
                let delivered_ml = cmd.pump_run_seconds * self.cal.pump_ml_per_sec;
                self.env.reservoir_ml = (self.env.reservoir_ml - delivered_ml).max(0.0);
                self.metrics.total_watered_ml += delivered_ml as u32;
                self.pending.push(Pending {
                    apply_at_ms: now + models::SOAK_MS,
                    pct: delivered_ml / models::POT_ML_PER_PCT,
                });
            }
        }

        // 5) Moisture decline over the tick (faster under light + high VPD).
        let decline =
            models::moisture_decline(TICK_MS as f32 / 60_000.0, cmd.light_on, cmd.vpd_kpa);
        self.env.moisture_pct = (self.env.moisture_pct - decline).clamp(0.0, 100.0);

        // 6) Record metrics.
        self.record(&cmd);

        // 7) Persist actuator state for next tick's environment model + advance time.
        self.env.last_led_pct = cmd.led_pct;
        self.env.last_fan_pct = cmd.fan_pct;
        self.now_ms += TICK_MS;
        self.last = Some(cmd);
        cmd
    }

    fn record(&mut self, cmd: &Commands) {
        let warmed_up = self.now_ms >= 86_400_000; // after day 1
        let m = &mut self.metrics;
        m.ticks += 1;
        m.states.push(cmd.state);
        m.max_moisture_pct = m.max_moisture_pct.max(self.env.moisture_pct);
        if warmed_up {
            m.min_moisture_after_warmup_pct =
                m.min_moisture_after_warmup_pct.min(self.env.moisture_pct);
        }
        m.min_reservoir_ml = m.min_reservoir_ml.min(self.env.reservoir_ml);
        m.max_fan_pct = m.max_fan_pct.max(cmd.fan_pct);
        m.saw_led_derate |= cmd.led_derated;
        m.saw_climate_red |= cmd.climate_red;
        m.saw_rtc_fallback |= cmd.rtc_fallback;
        if cmd.light_on {
            m.led_on_ticks += 1;
        } else if cmd.led_pct != 0 {
            m.led_off_at_night_ok = false;
        }
    }

    /// Run `n` ticks.
    pub fn run(&mut self, n: u32) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Run a number of simulated days.
    pub fn run_days(&mut self, days: u32) {
        let ticks = days as u64 * 86_400_000 / TICK_MS;
        self.run(ticks as u32);
    }

    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }
}
