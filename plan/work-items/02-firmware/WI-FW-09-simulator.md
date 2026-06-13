# WI-FW-09 — Plant/environment simulator & scenarios

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-09 |
| Depends on | WI-FW-05, WI-FW-06, WI-FW-07 |
| Spec refs | §10.3, §23 (DR-02) |
| Status | Done |

## Objective

Build a plant/environment simulator sufficient to validate the full control loop without hardware,
and implement the 11 required scenarios.

## Deliverables

- [x] Rust simulator crate (`firmware/sim/`) that drives the **real `control` crate** through host
      implementations of the `hal.rs` traits, with models for: moisture decline (faster under
      light/high VPD), pump→moisture rise after delay, reservoir drawdown, fan→RH effect, LED→heat,
      injectable leak/sensor faults (§10.3). Models may live under `sim/models/`.
- [x] All 11 required scenarios implemented as automated `cargo test` cases (data under `sim/scenarios/`):
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

## Implementation

- `sim/` drives the real `control::app_state::App` (not a reimplementation): `sim/src/models.rs`
  (moisture decline ∝ light/VPD, pump→rise after soak, reservoir drawdown, LED heat, fan
  dispersion; injectable leak/sensor/pump faults), `sim/src/lib.rs` harness, and all **11 scenarios**
  in `sim/tests/scenarios.rs` (all passing via `cargo test -p sim`). Models documented in
  `sim/models/README.md`, scenarios in `sim/scenarios/README.md`, both flagged for
  re-parameterization from bench data (WI-QA-09 / §23 DR-02) before gating a live grow.

### Addendum — on-silicon emulator smoke test

Beyond the host sim, an optional **Wokwi emulator smoke test** runs the *real firmware binary* on an
emulated ESP32-S3 in CI (`controller/` `emulator` feature → `src/emulator.rs`; project under
`controller/wokwi/`; non-blocking `emulator-smoke` CI job). It proves the binary boots, links
esp-hal, and runs the genuine control loop without panicking — the integration layer neither the
unit tests nor the host sim touch. It is a complement to, not a replacement for, HIL (WI-EE-08):
analog signal fidelity stays with hardware. Authored by firmware; verified in CI/at bring-up.
