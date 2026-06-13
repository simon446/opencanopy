# WI-FW-05 — Irrigation controller

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-05 |
| Depends on | WI-FW-03, WI-PL-03 |
| Spec refs | §9.6, §5.6 |
| Status | Not started |

## Objective

Implement the safety-first watering decision loop: pulse dosing with remeasure, daily caps, watering
windows, and hard lockouts.

## Deliverables

- [ ] Decision loop per §9.6 pseudo-code, evaluated every 5 min.
- [ ] Lockout precedence: leak → reservoir-low → moisture-sensor-invalid → pump-fault → normal.
- [ ] Pulse dosing using calibrated `ml_per_sec`; recheck delay; "moisture did not rise after N pulses
      → PUMP_FAULT" detection.
- [ ] Daily-max caps per stage; max 3 pulses/hour; single-run ≤30 s; emergency watering when critically dry.
- [ ] Watering-window enforcement (first 60–70% of light period; not last 2 h).
- [ ] Unit tests: timeout, daily max, low water, leak lockout, no-rise fault, window logic.

## Acceptance criteria

- Pulse/lockout tests pass (spec §10.2 "Pump safety" + "Irrigation thresholds").
- Pump can never enable while leak or reservoir-low is asserted.

## Notes

Pump-off-on-reset is also a hardware guarantee (pull-down) — see [WI-EE-03](../03-electronics/WI-EE-03-schematic.md).
This item owns the firmware side.
