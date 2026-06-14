# firmware/sim/

Host-side simulation of the OpenCanopy grow loop. Runs entirely on a developer machine and in CI —
no ESP32-S3 and no wet hardware required. Owned by the **Firmware** track (spec §10.3).

This is a host Rust crate that drives the **real `control` crate** (not a reimplementation) through
host implementations of its `hal.rs` traits, against software models of the plant, soil moisture,
reservoir, and thermal behavior, then asserts the control logic reacts correctly. Scenarios run as
`cargo test` on stable Rust — no Xtensa toolchain needed.

## Layout

- `src/` — the simulation runner and host trait implementations that feed the `control` crate.
- `scenarios/` — the scenarios exercised in CI (V1 is passive — monitor + warn, no pump), e.g.:
  - passive wick holds moisture → normal grow,
  - reservoir drain → LOW_WATER refill warning,
  - wick failure → MOISTURE_LOW with a full reservoir,
  - leak/overflow → LEAK_DETECTED warning (Water + System red),
  - canopy over-temperature → LED derate,
  - simultaneous faults → correct fault priority ordering.
- `models/` — the plant/soil/thermal/reservoir models the scenarios drive.

## Acceptance

Simulation scenarios are a CI gate (spec §10.5, §21 *Firmware*): a PR that breaks a scenario fails CI.
