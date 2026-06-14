# WI-FW-07 — Safety controller & state machine

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-07, M1-06 |
| Depends on | WI-FW-04, WI-FW-05, WI-FW-06 |
| Spec refs | §9.3, §9.4, §11.4 |
| Status | Done |

## Objective

Implement the top-level state machine and fault-priority arbitration that overrides the individual
controllers when safety states are active.

## Deliverables

- [x] States: BOOT, SELF_TEST, NORMAL, LOW_WATER, LEAK_DETECTED, MOISTURE_LOW, MOISTURE_HIGH,
      SENSOR_FAULT, OVER_TEMP_LED, MAINTENANCE, SAFE_SHUTDOWN (§9.3). *(**ECO-003** 2026-06-14: pump
      removed → dropped `WATERING`/`PUMP_FAULT`, merged the LED-thermal faults into `OVER_TEMP_LED`,
      added `MOISTURE_LOW`/`MOISTURE_HIGH`; `FAN_FAULT` already removed in ECO-001. Spec §9.3 still
      lists the old set and needs the Project track's pass.)*
- [x] Priority arbitration: LEAK > OVER_TEMP_LED > SENSOR_FAULT > LOW_WATER > MOISTURE_HIGH >
      MOISTURE_LOW > MAINTENANCE > NORMAL. (No pump → no pump-gating precedence.)
- [x] Boot sequence per §9.4 (self-test, restore grow-cycle age from flash, **LED** forced off — the
      only actuator now).
- [x] Watchdog + brownout enable (`esp-hal` RWDT/brownout); leak fault latches until manual clear (§11.4).
- [x] Unit tests proving highest-priority state always wins.

## Acceptance criteria

- Fault-priority tests pass (spec §10.2 "Fault priority").
- Leak/over-temp states demonstrably override controller outputs (pump off, LED off/min).

## Implementation

- `control/src/safety_controller.rs`: `SystemState` (11 states — pump/fan states removed, ECO-003/
  ECO-001; `MOISTURE_LOW`/`MOISTURE_HIGH` added), total-ordering `arbitrate()`, and `gates()` which
  now enforces only the **LED power factor** (no pump to gate) — `OVER_TEMP_LED` forces the LED off,
  the sole thermal defense. Latched leak (`clear_leak`, a flood warning now), and the §9.4 `boot()`
  sequence (LED forced off; bad calibration → SENSOR_FAULT, never NORMAL). Host-tested incl.
  "highest-priority state always wins" and warnings never cutting the light.
- Watchdog (RWDT) + brownout enable are esp-hal concerns wired in `controller/src/drivers`
  (`Platform::enable_watchdog`/`feed_watchdog`), verified on hardware at WI-EE-08 bring-up.
