# WI-FW-02 — Hardware abstraction layer & mocks

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-02 |
| Depends on | WI-FW-01 |
| Spec refs | §10.1, §9.2 |
| Status | Not started |

## Objective

Define hardware-abstraction interfaces (sensors, actuators, time/clock) with mock implementations so
all control logic is testable on the host without physical hardware.

## Deliverables

- [ ] Interfaces for: temp/RH sensor, capacitive moisture, reservoir level, leak sensor, fan
      (PWM + tach), pump (drive + optional current), LED dimming, status LEDs, RTC/time.
- [ ] Mock implementations with injectable readings and fault injection hooks.
- [ ] **Simulated time** source so schedules and timeouts run deterministically in tests.

## Acceptance criteria

- Every control module depends only on interfaces, never on concrete drivers.
- Mocks support injecting leak, sensor failure, and stuck-reading conditions (needed by §10.3 scenarios).

## Notes

This is the seam that makes the entire §10.2 unit-test matrix and §10.3 simulator possible. Get the
interfaces right before writing controllers.
