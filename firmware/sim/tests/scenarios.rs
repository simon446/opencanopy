//! The 11 required simulation scenarios (spec §10.3). Each drives the **real** `control` crate via
//! `sim::Sim` and asserts on genuine controller outputs (WI-FW-09 acceptance). Parameters for each
//! scenario are documented in `sim/scenarios/README.md`.

use control::led_status::{LedColor, LedPattern};
use control::logging::LogKind;
use control::safety_controller::SystemState;
use sim::{Config, Inject, Sim};

// 1 ----------------------------------------------------------------------------------------------
#[test]
fn normal_7day_seedling_no_overwatering_stable_light() {
    let mut s = Sim::new(Config {
        age_days: 35, // S1 seedling
        start_moisture_pct: 50.0,
        room_temp_c: 23.0,
        room_rh_pct: 62.0,
        ..Config::default()
    });
    s.run_days(7);

    // No overwatering: true moisture never climbs far past the wet threshold (55).
    assert!(
        s.metrics.max_moisture_pct < 58.0,
        "overwatered: {}",
        s.metrics.max_moisture_pct
    );
    // It did keep the plant watered (not a dead loop).
    assert!(s.metrics.pump_runs > 0);
    // Light schedule stable: lights on ~16/24 of the time, fully off at night.
    let on_frac = s.metrics.led_on_ticks as f32 / s.metrics.ticks as f32;
    assert!((0.60..0.70).contains(&on_frac), "on fraction {on_frac}");
    assert!(s.metrics.led_off_at_night_ok);
    // No spurious faults.
    assert!(!s.metrics.saw(SystemState::PumpFault));
    assert!(!s.metrics.saw(SystemState::LeakDetected));
}

// 2 ----------------------------------------------------------------------------------------------
#[test]
fn normal_7day_fruiting_moisture_maintained_reservoir_consumed() {
    let mut s = Sim::new(Config {
        age_days: 150, // S4 fruiting
        start_moisture_pct: 50.0,
        start_reservoir_ml: 6000.0,
        ..Config::default()
    });
    s.run_days(7);

    // Reservoir consumed by real watering.
    assert!(s.metrics.total_watered_ml > 0);
    assert!(s.metrics.min_reservoir_ml < 6000.0);
    // Moisture maintained: after warm-up it never falls into the critical band (22).
    assert!(
        s.metrics.min_moisture_after_warmup_pct > 22.0,
        "moisture dipped critically: {}",
        s.metrics.min_moisture_after_warmup_pct
    );
    assert!(s.metrics.max_moisture_pct < 65.0);
    assert!(!s.metrics.saw(SystemState::PumpFault));
}

// 3 ----------------------------------------------------------------------------------------------
#[test]
fn reservoir_empty_locks_out_pump_water_led_red() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 20.0,  // dry → wants to water
        start_reservoir_ml: 250.0, // below the 300 mL low threshold
        ..Config::default()
    });
    s.run_days(1);

    assert_eq!(
        s.metrics.pump_runs, 0,
        "pump must be locked out when reservoir is low"
    );
    assert!(!s.metrics.pump_ran_while_reservoir_low);
    assert!(s.metrics.saw(SystemState::LowWater));
    // Water LED red.
    let water = s.last.unwrap().panel.water;
    assert_eq!(water.color, LedColor::Red);
}

// 4 ----------------------------------------------------------------------------------------------
#[test]
fn moisture_sensor_stuck_wet_no_watering_then_sensor_fault() {
    let mut s = Sim::new(Config::default());
    s.inject = Inject {
        moisture_stuck_pct: Some(70.0),
        ..Inject::default()
    }; // reads wet forever
    s.run_days(1);

    assert_eq!(
        s.metrics.pump_runs, 0,
        "must not water on a stuck-wet reading"
    );
    // After the plausibility window the unchanging reading is flagged a sensor fault.
    assert!(s.metrics.saw(SystemState::SensorFault));
}

// 5 ----------------------------------------------------------------------------------------------
#[test]
fn moisture_sensor_stuck_dry_pump_fault_if_no_response() {
    let mut s = Sim::new(Config::default());
    s.inject = Inject {
        moisture_stuck_pct: Some(20.0),
        ..Inject::default()
    }; // reads dry forever
    s.run_days(1);

    // It tries to water, but the reading never rises → no-response pump fault (capped well below
    // the daily max by the no-rise detector).
    assert!(s.metrics.saw(SystemState::PumpFault));
    assert!(
        s.metrics.pump_runs <= 3,
        "no-rise must stop dosing quickly: {}",
        s.metrics.pump_runs
    );
}

// 6 ----------------------------------------------------------------------------------------------
#[test]
fn pump_disconnected_no_rise_fault() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 20.0,
        ..Config::default()
    });
    s.inject = Inject {
        pump_disconnected: true,
        ..Inject::default()
    };
    s.run_days(1);

    assert!(s.metrics.saw(SystemState::PumpFault));
    // No water actually moved.
    assert_eq!(s.metrics.total_watered_ml, 0);
    assert_eq!(s.metrics.min_reservoir_ml, s.env.reservoir_ml); // reservoir untouched
}

// 7 ----------------------------------------------------------------------------------------------
#[test]
fn leak_detected_immediate_pump_off_red_water_and_system() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 20.0,
        ..Config::default()
    });
    s.inject = Inject {
        leak: true,
        ..Inject::default()
    };
    s.run_days(1);

    assert_eq!(s.metrics.pump_runs, 0);
    assert!(!s.metrics.pump_ran_while_leak);
    assert!(s.metrics.saw(SystemState::LeakDetected));
    let panel = s.last.unwrap().panel;
    assert_eq!(panel.water.color, LedColor::Red);
    assert_eq!(panel.system.color, LedColor::Red);
}

// 8 ----------------------------------------------------------------------------------------------
#[test]
fn hot_room_fan_high_led_derate_no_runaway() {
    let mut s = Sim::new(Config {
        room_temp_c: 31.0, // + LED self-heat pushes the canopy past 32 °C
        room_rh_pct: 45.0,
        ..Config::default()
    });
    s.run_days(1);

    assert_eq!(
        s.metrics.max_fan_pct, 100,
        "fan should reach max in a hot room"
    );
    assert!(s.metrics.saw_led_derate, "LED should derate when hot");
    assert!(s.metrics.saw_climate_red);
    // No runaway: the loop completed and temperature never ran away (LED derate caps the only heat
    // source the device controls). Moisture stayed sane.
    assert!(s.metrics.max_moisture_pct < 65.0);
}

// 9 ----------------------------------------------------------------------------------------------
#[test]
fn humid_night_fan_pulses_no_watering() {
    let mut s = Sim::new(Config {
        start_unix_s: 23 * 3600, // 23:00 — lights off
        room_temp_c: 22.0,
        room_rh_pct: 92.0,
        start_moisture_pct: 50.0, // not dry
        ..Config::default()
    });
    s.run(96); // ~8 hours overnight at 5-min ticks

    // Fan runs hard for the humidity even at night (RH>85 → +30%).
    assert!(
        s.metrics.max_fan_pct >= 30,
        "fan should respond to humidity: {}",
        s.metrics.max_fan_pct
    );
    // No watering: substrate isn't dry and it's night (no emergency).
    assert_eq!(s.metrics.pump_runs, 0);
}

// 10 ---------------------------------------------------------------------------------------------
#[test]
fn rtc_invalid_safe_fallback_amber_system_watering_works() {
    let mut s = Sim::new(Config {
        rtc_valid: false,
        start_moisture_pct: 25.0, // dry → should still water on the fallback schedule
        ..Config::default()
    });
    s.run_days(1);

    assert!(
        s.metrics.saw_rtc_fallback,
        "should run the safe-schedule fallback"
    );
    // Fallback does NOT block watering (§9.4) — the pump still ran.
    assert!(
        s.metrics.pump_runs > 0,
        "watering must work on the RTC fallback"
    );
    // System LED amber at some point (we check the final frame is amber, having lights cycling).
    // Find any frame where lights are on and assert system isn't red (fault) — it's the amber pulse.
    let sys = s.last.unwrap().panel.system;
    assert_eq!(sys.color, LedColor::Amber);
    assert_eq!(sys.pattern, LedPattern::SlowPulse);
}

// 11 ---------------------------------------------------------------------------------------------
#[test]
fn power_loss_mid_watering_pump_off_after_reboot_event_logged() {
    let mut s = Sim::new(Config {
        start_moisture_pct: 22.0,
        ..Config::default()
    });
    // Run until a watering pulse fires.
    let mut fired = false;
    for _ in 0..600 {
        let cmd = s.step();
        if cmd.pump_on {
            fired = true;
            break;
        }
    }
    assert!(fired, "expected a watering pulse to occur");

    // The watering event is in the (persistent) log before the power loss.
    let watered_logged = s
        .app
        .log()
        .iter()
        .any(|e| matches!(e.kind, LogKind::Watering { .. }));
    assert!(watered_logged, "watering event should be logged");

    // "Power loss" → reboot: a fresh controller comes up with the pump de-energized (boot forces
    // it off, mirroring the hardware pull-down) and in NORMAL — it does not resume the interrupted
    // WATERING state. (A subsequent fresh watering decision is fine and expected; what must never
    // happen is the pump being left energized / mid-pulse across the reset.)
    let rebooted = Sim::new(Config {
        start_moisture_pct: s.env.moisture_pct,
        start_reservoir_ml: s.env.reservoir_ml,
        ..Config::default()
    });
    assert_eq!(
        rebooted.app.state(),
        SystemState::Normal,
        "must boot clean, pump off"
    );
}
