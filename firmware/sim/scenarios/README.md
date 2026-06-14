# sim/scenarios/ — the 11 required scenarios

The §10.3 simulation scenarios (**ECO-003 passive-watering** revision), implemented as automated
`cargo test` cases in [`../tests/scenarios.rs`](../tests/scenarios.rs). Each drives the **real**
`control` crate via `sim::Sim` and asserts on genuine controller outputs (WI-FW-09 acceptance) — none
re-implements control policy.

V1 is passive (no pump) and fan-less: the firmware **monitors and warns**; the only actuator is the
grow LED. The scenarios exercise the warning/fault paths, not watering actuation.

Run them: `cargo test -p sim` (host, stable Rust, no hardware).

| # | Scenario | Setup (`sim::Config` / `Inject`) | Asserted result (§10.3) |
|---|---|---|---|
| 1 | Normal grow (passive) | S2, 45 % start, 7 days | Wick holds moisture in-band, no dry/wet/sensor warning, reservoir consumed but not low, lights ~16/24 & off at night |
| 2 | Reservoir drains to low | reservoir 600 mL | Crosses the 500 mL mark → `LOW_WATER`, Water LED **amber** (refill prompt), moisture still held |
| 3 | Wick failure | `wick_failure`, full tank | Substrate dries → `MOISTURE_LOW` **with a full reservoir** (not mistaken for `LOW_WATER`) |
| 4 | Over-wet substrate | 70 % start | `MOISTURE_HIGH` warning |
| 5 | Moisture sensor stuck | `moisture_stuck_pct = 45` | `SENSOR_FAULT` after the plausibility window |
| 6 | Moisture sensor bus error | `moisture_error = Bus` | `SENSOR_FAULT`, moisture reads `None` |
| 7 | Leak / overflow | `leak = true` | `LEAK_DETECTED`, Water + System LEDs **red**, latches until manual clear |
| 8 | Hot room | room 31 °C (+LED self-heat) | LED **derate** (only heat lever — no fan/pump), climate **warn**, no runaway |
| 9 | Humid night | 23:00 start, RH 92 % | Climate flags **System amber** (no fan to act), nothing actuated at night |
| 10 | RTC invalid | `rtc_valid = false` | Safe-schedule fallback, **light still cycles**, System LED **amber** |
| 11 | Power loss / reboot | run 2 days, then reboot | Sensor events logged; reboot comes up **NORMAL** (LED off, no resumed warning) |

The simulator runs entirely on stable Rust in CI (no Xtensa toolchain), per WI-FW-09 / WI-PS-06.
Scenario parameters live in the test file; the environment model they exercise is documented in
[`../models/README.md`](../models/README.md).
