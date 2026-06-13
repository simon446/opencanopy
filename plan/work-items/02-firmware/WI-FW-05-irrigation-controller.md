# WI-FW-05 — Irrigation controller

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-05 |
| Depends on | WI-FW-03, WI-PL-03 |
| Spec refs | §9.6, §5.6 |
| Status | Done |

## Objective

Implement the safety-first watering decision loop: pulse dosing with remeasure, daily caps, watering
windows, and hard lockouts.

## Deliverables

- [x] Decision loop per §9.6 pseudo-code, evaluated every 5 min.
- [x] Lockout precedence: leak → reservoir-low → moisture-sensor-invalid → pump-fault → normal.
- [x] Pulse dosing using calibrated `ml_per_sec`; recheck delay; "moisture did not rise after N pulses
      → PUMP_FAULT" detection.
- [x] Daily-max caps per stage; max 3 pulses/hour; single-run ≤30 s; emergency watering when critically dry.
- [x] Watering-window enforcement (first 60–70% of light period; not last 2 h).
- [x] Unit tests: timeout, daily max, low water, leak lockout, no-rise fault, window logic.

## Acceptance criteria

- Pulse/lockout tests pass (spec §10.2 "Pump safety" + "Irrigation thresholds").
- Pump can never enable while leak or reservoir-low is asserted.

## Notes

Pump-off-on-reset is also a hardware guarantee (pull-down) — see [WI-EE-03](../03-electronics/WI-EE-03-schematic.md).
This item owns the firmware side.

## Implementation

- `control/src/irrigation_controller.rs`: the §9.6 decision loop with the exact interlock
  precedence (leak → reservoir-low → moisture-invalid → pump-fault), pulse dosing with
  calibrated `ml_per_sec` + recheck delay, no-rise → `PUMP_FAULT` after N pulses, ≤3 pulses/hour,
  single-run ≤30 s, per-stage daily caps, emergency (critically-dry) bypass, and the
  `within_watering_window` gate. Includes `MoistureValidator` (stuck/plausibility → sensor fault).
  Host-tested for every rule. Fixed a latent rate-limit bug (a pulse at `now_ms==0` was dropped by
  a `0` sentinel — now `Option<u64>`).
