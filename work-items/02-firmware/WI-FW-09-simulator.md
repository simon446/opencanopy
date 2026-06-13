# WI-FW-09 — Plant/environment simulator & scenarios

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-09 |
| Depends on | WI-FW-05, WI-FW-06, WI-FW-07 |
| Spec refs | §10.3, §23 (DR-02) |
| Status | Not started |

## Objective

Build a plant/environment simulator sufficient to validate the full control loop without hardware,
and implement the 11 required scenarios.

## Deliverables

- [ ] Rust simulator crate (`firmware/sim/`) that drives the **real `control` crate** through host
      implementations of the `hal.rs` traits, with models for: moisture decline (faster under
      light/high VPD), pump→moisture rise after delay, reservoir drawdown, fan→RH effect, LED→heat,
      injectable leak/sensor faults (§10.3). Models may live under `sim/models/`.
- [ ] All 11 required scenarios implemented as automated `cargo test` cases (data under `sim/scenarios/`):
      normal seedling, normal fruiting, reservoir empty, sensor stuck wet, sensor stuck dry, pump
      disconnected, leak, hot room, humid night, RTC invalid, power loss mid-watering.

## Acceptance criteria

- All 11 §10.3 scenarios pass via `cargo test` against the actual `control` logic (not a reimplementation).
- Simulator runs in CI on stable Rust (no hardware, no Xtensa toolchain) — coordinate with [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md).

## Notes

The models here start from engineering estimates but **must be re-parameterized from measured bench
data** ([WI-QA-09](../05-validation-qa/WI-QA-09-bench-characterization.md), §23 DR-02) before the sim
is trusted to gate a live-plant grow loop. Passing scenarios proves the control logic, not that
reality matches the model.
