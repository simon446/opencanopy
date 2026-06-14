//! The required simulation scenarios (spec §10.3; ECO-003 passive-watering revision). Each drives the
//! **real** `control` crate via `sim::Sim` and asserts on genuine controller outputs (WI-FW-09
//! acceptance). V1 is passive (no pump) and fan-less: the firmware monitors and **warns**, the only
//! actuator is the grow LED. Parameters are documented in `sim/scenarios/README.md`.

use control::led_status::LedColor;
use control::safety_controller::SystemState;
use sim::{Config, Inject, Sim};

// 1 ----------------------------------------------------------------------------------------------
#[test]
fn normal_grow_passive_holds_moisture_stable_light() {
    let mut s = Sim::new(Config {
        age_days: 60, // S2 vegetative
        start_moisture_pct: 45.0,
        room_temp_c: 23.0,
        room_rh_pct: 62.0,
        ..Config::default()
    });
    s.run_days(7);

    // The passive wick holds the substrate in-band the whole week (no dry/wet warning after warm-up).
    assert!(
        s.metrics.min_moisture_after_warmup_pct > 30.0,
        "substrate dried out: {}",
        s.metrics.min_moisture_after_warmup_pct
    );
    assert!(s.metrics.max_moisture_pct < 55.0);
    assert!(!s.metrics.saw(SystemState::MoistureLow));
    assert!(!s.metrics.saw(SystemState::MoistureHigh));
    // Reservoir is consumed by transpiration but does not run low in a week.
    assert!(s.metrics.min_reservoir_ml < 6000.0);
    assert!(!s.metrics.saw(SystemState::LowWater));
    // Light schedule stable: lights on ~16/24, fully off at night.
    let on_frac = s.metrics.led_on_ticks as f32 / s.metrics.ticks as f32;
    assert!((0.60..0.70).contains(&on_frac), "on fraction {on_frac}");
    assert!(s.metrics.led_off_at_night_ok);
    // No spurious faults.
    assert!(!s.metrics.saw(SystemState::SensorFault));
    assert!(!s.metrics.saw(SystemState::LeakDetected));
}

// 2 ----------------------------------------------------------------------------------------------
#[test]
fn reservoir_drains_to_low_water_warning() {
    let mut s = Sim::new(Config {
        start_reservoir_ml: 600.0, // just above the 500 mL low mark — drains below within a day
        start_moisture_pct: 45.0,
        ..Config::default()
    });
    s.run_days(1);

    // Reservoir crosses the low mark → LOW_WATER refill warning (no actuation — passive).
    assert!(s.metrics.saw(SystemState::LowWater));
    assert!(s.metrics.min_reservoir_ml < 500.0);
    // The Water LED is amber (refill prompt), not red.
    let water = s.last.unwrap().panel.water;
    assert_eq!(water.color, LedColor::Amber);
    // The wick still holds moisture while a little water remains — the warning is an early prompt.
    assert!(!s.metrics.saw(SystemState::MoistureLow));
}

// 3 ----------------------------------------------------------------------------------------------
#[test]
fn wick_failure_dries_substrate_moisture_low_with_full_reservoir() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 45.0,
        start_reservoir_ml: 6000.0,
        ..Config::default()
    });
    s.inject = Inject {
        wick_failure: true, // clogged wick: substrate dries even with a full tank
        ..Inject::default()
    };
    s.run_days(2);

    // The passive-watering failure mode is caught: substrate dries → MOISTURE_LOW...
    assert!(s.metrics.saw(SystemState::MoistureLow));
    // ...and it is NOT mistaken for a low reservoir (the tank is still full).
    assert!(!s.metrics.saw(SystemState::LowWater));
    assert!(s.metrics.min_reservoir_ml > 500.0);
}

// 4 ----------------------------------------------------------------------------------------------
#[test]
fn overwet_substrate_warns_moisture_high() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 70.0, // above the wet band (55)
        ..Config::default()
    });
    s.run(12); // a few ticks before the wick pulls it back toward equilibrium

    assert!(s.metrics.saw(SystemState::MoistureHigh));
}

// 5 ----------------------------------------------------------------------------------------------
#[test]
fn moisture_sensor_stuck_then_sensor_fault() {
    let mut s = Sim::new(Config::default());
    s.inject = Inject {
        moisture_stuck_pct: Some(45.0),
        ..Inject::default()
    }; // reads a fixed value forever
    s.run_days(1);

    // After the plausibility window the unchanging reading is flagged a sensor fault.
    assert!(s.metrics.saw(SystemState::SensorFault));
}

// 6 ----------------------------------------------------------------------------------------------
#[test]
fn moisture_sensor_bus_error_sensor_fault() {
    let mut s = Sim::new(Config::default());
    s.inject = Inject {
        moisture_error: Some(control::hal::SensorError::Bus),
        ..Inject::default()
    };
    s.run(3);

    assert!(s.metrics.saw(SystemState::SensorFault));
    assert!(s.last.unwrap().moisture_pct.is_none());
}

// 7 ----------------------------------------------------------------------------------------------
#[test]
fn leak_detected_warns_red_water_and_system_latched() {
    let mut s = Sim::new(Config::default());
    s.inject = Inject {
        leak: true,
        ..Inject::default()
    };
    s.run_days(1);

    assert!(s.metrics.saw(SystemState::LeakDetected));
    let panel = s.last.unwrap().panel;
    assert_eq!(panel.water.color, LedColor::Red);
    assert_eq!(panel.system.color, LedColor::Red);
    // Latches: clear the inject and it stays latched until a manual clear.
    s.inject.leak = false;
    let cmd = s.step();
    assert_eq!(cmd.state, SystemState::LeakDetected);
    s.app.clear_leak();
    let cmd = s.step();
    assert_ne!(cmd.state, SystemState::LeakDetected);
}

// 8 ----------------------------------------------------------------------------------------------
#[test]
fn hot_room_led_derate_no_runaway() {
    let mut s = Sim::new(Config {
        room_temp_c: 31.0, // + LED self-heat pushes the canopy past 32 °C
        room_rh_pct: 45.0,
        ..Config::default()
    });
    s.run_days(1);

    // With no fan and no pump, cutting the grow LED is the only thermal defense.
    assert!(s.metrics.saw_led_derate, "LED should derate when hot");
    assert!(s.metrics.saw_climate_warn);
    // No runaway: the loop completed and the substrate stayed sane.
    assert!(s.metrics.max_moisture_pct < 55.0);
}

// 9 ----------------------------------------------------------------------------------------------
#[test]
fn humid_night_climate_flags_no_actuation() {
    let mut s = Sim::new(Config {
        start_unix_s: 23 * 3600, // 23:00 — lights off
        room_temp_c: 22.0,
        room_rh_pct: 92.0,
        start_moisture_pct: 45.0, // in band
        ..Config::default()
    });
    s.run(96); // ~8 hours overnight at 5-min ticks

    // V1 has no fan/pump, so high humidity has no actuator — the climate monitor surfaces it on the
    // System LED (amber), and nothing is actuated at night.
    assert!(s.metrics.saw_climate_warn);
    assert!(s.metrics.led_off_at_night_ok);
    assert!(!s.metrics.saw(SystemState::MoistureLow));
    let sys = s.last.unwrap().panel.system;
    assert_eq!(sys.color, LedColor::Amber);
}

// 10 ---------------------------------------------------------------------------------------------
#[test]
fn rtc_invalid_safe_fallback_amber_system_light_still_cycles() {
    let mut s = Sim::new(Config {
        rtc_valid: false,
        start_moisture_pct: 45.0,
        ..Config::default()
    });
    s.run_days(1);

    assert!(
        s.metrics.saw_rtc_fallback,
        "should run the safe-schedule fallback"
    );
    // The light still runs on the boot-relative fallback schedule.
    assert!(
        s.metrics.led_on_ticks > 0,
        "light must cycle on the fallback"
    );
    // System LED amber pulse (fallback) — and it is never red (no fault).
    let sys = s.last.unwrap().panel.system;
    assert_eq!(sys.color, LedColor::Amber);
}

// 11 ---------------------------------------------------------------------------------------------
#[test]
fn power_loss_reboot_comes_up_clean_normal() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 45.0,
        ..Config::default()
    });
    s.run_days(2);

    // A sensor snapshot is in the (persistent) log before the power loss.
    let logged = s
        .app
        .log()
        .iter()
        .any(|e| matches!(e.kind, control::logging::LogKind::Sensors { .. }));
    assert!(logged, "sensor events should be logged");

    // "Power loss" → reboot: a fresh controller comes up with the LED de-energized (boot forces it
    // off) and in NORMAL — it does not resume any prior warning state.
    let rebooted = Sim::new(Config {
        start_moisture_pct: s.env.moisture_pct,
        start_reservoir_ml: s.env.reservoir_ml,
        ..Config::default()
    });
    assert_eq!(rebooted.app.state(), SystemState::Normal, "must boot clean");
}
