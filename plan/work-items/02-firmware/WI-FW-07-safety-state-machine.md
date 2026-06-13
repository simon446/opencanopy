# WI-FW-07 — Safety controller & state machine

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-07, M1-06 |
| Depends on | WI-FW-04, WI-FW-05, WI-FW-06 |
| Spec refs | §9.3, §9.4, §11.4 |
| Status | Not started |

## Objective

Implement the top-level state machine and fault-priority arbitration that overrides the individual
controllers when safety states are active.

## Deliverables

- [ ] States: BOOT, SELF_TEST, NORMAL, WATERING, LOW_WATER, LEAK_DETECTED, SENSOR_FAULT, PUMP_FAULT,
      FAN_FAULT, LED_FAULT, OVER_TEMP, MAINTENANCE, SAFE_SHUTDOWN (§9.3).
- [ ] Priority arbitration: LEAK > OVER_TEMP(critical) > PUMP_FAULT > SENSOR_FAULT(watering) >
      LOW_WATER > NORMAL/WATERING.
- [ ] Boot sequence per §9.4 (self-test, restore grow-cycle age from flash, pump forced off).
- [ ] Watchdog + brownout enable (`esp-hal` RWDT/brownout); leak fault latches until manual clear (§11.4).
- [ ] Unit tests proving highest-priority state always wins.

## Acceptance criteria

- Fault-priority tests pass (spec §10.2 "Fault priority").
- Leak/over-temp states demonstrably override controller outputs (pump off, LED off/min).
