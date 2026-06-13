//! OpenCanopy V1 — platform-agnostic control logic. Spec §9, §10.
//!
//! This crate is `no_std` and **has no `esp-hal` dependency** (workspace dependency rule, §9.2): it
//! talks to the world only through the [`hal`] traits, which `controller/` binds to real ESP32-S3
//! peripherals and `sim/` binds to simulated models. That seam is what lets every control rule be
//! unit-tested on the host (§10.1, §10.2) and the whole loop validated in the simulator (§10.3)
//! with no hardware and no Xtensa toolchain.
//!
//! Test builds link `std` (for the test harness); the shipped library is strictly `no_std`.
//!
//! Module map (mirrors spec §9.2):
//! - [`plant_profile`] — age → stage → setpoints (§5.1, §5.2)
//! - [`light_controller`] — photoperiod, ramp, RTC fallback, thermal derate (§9.5)
//! - [`irrigation_controller`] — pulse-dosing decision loop, caps, windows, lockouts (§9.6)
//! - [`climate_controller`] — VPD + fan/LED nudge (§9.7)
//! - [`safety_controller`] — state machine + fault-priority arbitration (§9.3, §9.4)
//! - [`led_status`] — colorblind-safe status-LED mapping (§9.8)
//! - [`calibration`] — flash-backed calibration store, fail-safe (§9.9, §7.6)
//! - [`logging`] — local rolling logs, connectivity-independent (§9.10, §9.11)
//! - [`app_state`] — the orchestrator that sequences all of the above
//! - [`hal`] — the hardware seam; [`testkit`] — host mocks
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

pub mod math;

pub mod hal;

pub mod app_state;
pub mod board;
pub mod calibration;
pub mod climate_controller;
pub mod i2c_devices;
pub mod irrigation_controller;
pub mod led_status;
pub mod light_controller;
pub mod logging;
pub mod plant_profile;
pub mod safety_controller;
pub mod scheduler;

// Host mocks are always available (pure `core`); `controller/` and `sim/` use them via the `mock`
// feature or directly in tests.
pub mod testkit;
