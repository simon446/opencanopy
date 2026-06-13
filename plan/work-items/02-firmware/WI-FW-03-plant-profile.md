# WI-FW-03 — Plant profile module

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-03 |
| Depends on | WI-FW-02, WI-PL-01 |
| Spec refs | §5.1, §5.2 |
| Status | Done |

## Objective

Encode the hot-pepper lifecycle profile as a fixed in-firmware recipe: age → stage → setpoints.

## Deliverables

- [x] `plant_profile` module mapping grow-cycle age (days) to stage S0–S5.
- [x] Per-stage setpoint lookup (photoperiod, PPFD/DLI target, RH/VPD band, watering thresholds).
- [x] `TRANSPLANT_PROFILE` build flag skipping S0/S1.
- [x] Unit tests covering stage boundaries and transplant flag.

## Acceptance criteria

- Age/stage selection tests pass (spec §10.2 "Plant profile" row).
- Setpoints match [WI-PL-01](../01-plant-science/WI-PL-01-lifecycle-profile.md) — single source of truth.

## Implementation

- `control/src/plant_profile.rs`: `Stage` S0–S5, `stage_for_age()` (inclusive-low/exclusive-high
  boundaries), per-stage `Setpoints` const tables transcribed from `docs/plant-profile-hot-pepper.md`
  / `docs/watering-model.md` / `docs/vpd-climate-model.md`, and the compile-time
  `transplant_profile` feature (`reset_age_days()` → day 56). Host-tested: every stage boundary,
  S5-never-auto-selected, transplant flag, and setpoint spot-checks against the docs.
