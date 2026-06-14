# sim/scenarios/ — the 11 required scenarios

The §10.3 simulation scenarios, implemented as automated `cargo test` cases in
[`../tests/scenarios.rs`](../tests/scenarios.rs). Each drives the **real** `control` crate via
`sim::Sim` and asserts on genuine controller outputs (WI-FW-09 acceptance) — none re-implements
control policy.

Run them: `cargo test -p sim` (host, stable Rust, no hardware).

| # | Scenario | Setup (`sim::Config` / `Inject`) | Asserted result (§10.3) |
|---|---|---|---|
| 1 | Normal 7-day seedling | S1, 50% start, 7 days | No overwatering (`max_moisture < 58`), lights ~16/24 & off at night, no faults |
| 2 | Normal 7-day fruiting | S4, 6 L reservoir, 7 days | Reservoir consumed, moisture stays above critical, no pump fault |
| 3 | Reservoir empty | reservoir 250 mL (<300), dry | Pump locked out (0 runs), `LOW_WATER`, Water LED **red** |
| 4 | Moisture sensor stuck wet | `moisture_stuck_pct = 70` | No watering, `SENSOR_FAULT` after the plausibility window |
| 5 | Moisture sensor stuck dry | `moisture_stuck_pct = 20` | Tries then `PUMP_FAULT` (no-rise), capped well below daily max |
| 6 | Pump disconnected | `pump_disconnected`, dry | `PUMP_FAULT` (no rise), zero water moved, reservoir untouched |
| 7 | Leak detected | `leak = true`, dry | Pump never on, `LEAK_DETECTED`, Water + System LEDs **red** |
| 8 | Hot room | room 31 °C (+LED self-heat) | LED **derate** (the only heat lever — no fan), climate **red**, no runaway |
| 9 | Humid night | 23:00 start, RH 92%, not dry | Climate flags **red** (RH>90; no fan to act), **no watering** (not dry, night) |
| 10 | RTC invalid | `rtc_valid = false`, dry | Safe-schedule fallback, **watering still works**, System LED **amber pulse** |
| 11 | Power loss mid-watering | run to a pulse, then reboot | Watering event logged, reboot comes up **NORMAL** (pump off, not resumed) |

The simulator runs entirely on stable Rust in CI (no Xtensa toolchain), per WI-FW-09 / WI-PS-06.
Scenario parameters live in the test file; the environment model they exercise is documented in
[`../models/README.md`](../models/README.md).
