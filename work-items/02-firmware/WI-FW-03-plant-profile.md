# WI-FW-03 — Plant profile module

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-03 |
| Depends on | WI-FW-02, WI-PL-01 |
| Spec refs | §5.1, §5.2 |
| Status | Not started |

## Objective

Encode the hot-pepper lifecycle profile as a fixed in-firmware recipe: age → stage → setpoints.

## Deliverables

- [ ] `plant_profile` module mapping grow-cycle age (days) to stage S0–S5.
- [ ] Per-stage setpoint lookup (photoperiod, PPFD/DLI target, RH/VPD band, watering thresholds).
- [ ] `TRANSPLANT_PROFILE` build flag skipping S0/S1.
- [ ] Unit tests covering stage boundaries and transplant flag.

## Acceptance criteria

- Age/stage selection tests pass (spec §10.2 "Plant profile" row).
- Setpoints match [WI-PL-01](../01-plant-science/WI-PL-01-lifecycle-profile.md) — single source of truth.
