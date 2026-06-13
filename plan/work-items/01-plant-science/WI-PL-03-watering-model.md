# WI-PL-03 — Moisture thresholds & watering windows

| Field | Value |
|---|---|
| Track | Plant Science |
| Milestone | M1-03, M1-04 |
| Depends on | WI-PL-01 |
| Spec refs | §5.5, §5.6, §9.6 |
| Status | Done |

## Objective

Define the substrate, the moist-but-aerated watering philosophy, the per-stage dry/wet thresholds,
pulse sizes, and the time-of-day watering windows the irrigation controller will enforce.

## Deliverables

- [x] Substrate recommendation (peat/coco + perlite, drainage) and pot constraints (≥8 L) — §5.5.
- [x] Per-stage threshold table: dry %, wet %, pulse size, recheck delay (§5.6).
- [x] Daily-max safety caps per stage (§9.6) documented as caps, **not** consumption targets.
- [x] Watering-window rules: prefer first 60–70% of light period; avoid last 2 h before lights-off;
      emergency watering allowed any time when critically dry.
- [x] Explicit statement that thresholds are **normalized calibration units**, not raw ADC %.

## Acceptance criteria

- Thresholds, pulse sizes, and daily caps match spec §5.6 / §9.6 and are stage-indexed.
- Watering-window logic is unambiguous enough to unit-test in [WI-FW-05](../02-firmware/WI-FW-05-irrigation-controller.md).

## Notes

Calibration of raw→normalized mapping is a hardware task ([WI-EE-08](../03-electronics/WI-EE-08-bringup-hil.md));
this item owns the biological targets only.
