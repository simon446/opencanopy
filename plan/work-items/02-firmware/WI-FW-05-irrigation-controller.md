# WI-FW-05 — Moisture monitor (was: irrigation controller)

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-05 |
| Depends on | WI-FW-03, WI-PL-03 |
| Spec refs | §9.6, §5.6 |
| Status | Done (redesigned: pump removed) |

> **SCOPE CHANGE (2026-06-14): the pump was removed from V1** ([ECO-003](../../../docs/ECO-003-v1-redesign.md)).
> Watering is now **passive** (reservoir + wick). There is no watering actuator, so the pulse-dosing
> decision loop is gone — this item becomes a **moisture monitor**: validate the probe reading and
> classify it into a warning (`MOISTURE_LOW` / `MOISTURE_HIGH` / `SENSOR_FAULT`). The pump interlocks,
> daily caps, watering window, pump-fault/no-rise detection, and `ml_per_sec` calibration are all
> removed. Spec §9.6 still describes the pumped loop and needs the Project track's passive-watering pass.

## Objective

Validate the substrate-moisture reading and classify it for the **Moisture status warning** (passive
watering — monitor only, no actuation). *(Originally: a safety-first pulse-dosing loop for a pump.)*

## Deliverables

- [x] Moisture validation: bus/range error, uncalibrated normalize, and a **stuck** reading across the
      plausibility window → `None` (= `SENSOR_FAULT`). (Kept from the original `MoistureValidator`.)
- [x] Band classification → `MOISTURE_LOW` (below dry), `MOISTURE_HIGH` (above wet), `CriticalLow`
      (below critical), `Ok`. Drives the Moisture LED / warning state; **never actuates**.
- [x] Unit tests for the validator and the band classification.
- [x] ~~Pulse dosing, daily caps, watering window, leak→pump lockout, no-rise → PUMP_FAULT~~ — removed
      with the pump (ECO-003).

## Acceptance criteria

- Moisture validation + classification tests pass (spec §10.2 "Moisture monitor").
- The monitor commands no actuator (V1 is passive — there is no pump).

## Notes

With no pump there is no flood/overwater failure mode and no pump-fail-off requirement. The leak
sensor is retained as a **flood/overflow warning** only (handled in `safety_controller`, WI-FW-07).

## Implementation

- `control/src/moisture_monitor.rs` (renamed from `irrigation_controller.rs`): `MoistureValidator`
  (stuck/plausibility → sensor fault) plus `classify()` → `MoistureStatus`. No dosing, no pump, no
  daily caps, no watering window. Host-tested for every rule.
