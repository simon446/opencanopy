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

- [x] States: BOOT, SELF_TEST, NORMAL, WATERING, LOW_WATER, LEAK_DETECTED, SENSOR_FAULT, PUMP_FAULT,
      LED_FAULT, OVER_TEMP, MAINTENANCE, SAFE_SHUTDOWN (§9.3). *(`FAN_FAULT` removed 2026-06-14 — the
      circulation fan was dropped from V1; spec §9.3 still lists it and needs a coordinated revision.)*
- [x] Priority arbitration: LEAK > OVER_TEMP(critical) > PUMP_FAULT > SENSOR_FAULT(watering) >
      LOW_WATER > NORMAL/WATERING.
- [x] Boot sequence per §9.4 (self-test, restore grow-cycle age from flash, pump forced off).
- [x] Watchdog + brownout enable (`esp-hal` RWDT/brownout); leak fault latches until manual clear (§11.4).
- [x] Unit tests proving highest-priority state always wins.

## Acceptance criteria

- Fault-priority tests pass (spec §10.2 "Fault priority").
- Leak/over-temp states demonstrably override controller outputs (pump off, LED off/min).

## Implementation

- `control/src/safety_controller.rs`: `SystemState` (12 states — `FAN_FAULT` removed with the fan),
  total-ordering `arbitrate()`, `gates()` (pump-off / LED-off-min enforcement; the over-temp gate's
  LED cut is now the sole thermal defense), latched leak (`clear_leak`), and the §9.4
  `boot()` sequence (pump forced off; bad calibration → fault, never NORMAL). Host-tested incl.
  "highest-priority state always wins" and leak/over-temp overriding controller outputs.
- Watchdog (RWDT) + brownout enable are esp-hal concerns wired in `controller/src/drivers`
  (`Platform::enable_watchdog`/`feed_watchdog`), verified on hardware at WI-EE-08 bring-up.
