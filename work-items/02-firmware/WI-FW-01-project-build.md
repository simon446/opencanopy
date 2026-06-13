# WI-FW-01 — Firmware project & build

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-01 |
| Depends on | WI-PS-01 |
| Spec refs | §9.1, §9.2, §14.1 |
| Status | Not started |

## Objective

Create the buildable firmware project with the layout from spec §9.2, targeting ESP32-S3, and a
host-native build path so control logic compiles and tests off-target.

## Deliverables

- [ ] `firmware/controller/` tree per §9.2 (`src/`, `include/`, `tests/`, `sim/`, `tools/`).
- [ ] `platformio.ini` or `CMakeLists.txt` building for both ESP32-S3 and host (native) targets.
- [ ] Host build compiles core control library with no hardware dependencies.
- [ ] CI hook so `firmware/` builds on every PR (coordinate with [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md)).

## Acceptance criteria

- `firmware` builds clean for both targets in CI.
- Directory layout matches spec §9.2.
