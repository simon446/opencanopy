# WI-FW-02 — Hardware abstraction layer & mocks

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-02 |
| Depends on | WI-FW-01 |
| Spec refs | §10.1, §9.2 |
| Status | Done |

## Objective

Define the hardware-abstraction **traits** (sensors, actuators, time/clock) in the `control` crate's
`hal.rs`, with mock implementations so all control logic is testable on the host without physical
hardware. These `no_std` traits are the seam between `control/` and the `controller/` (`esp-hal`) and
`sim/` crates.

## Deliverables

- [x] `no_std` traits for: temp/RH sensor, capacitive moisture, reservoir level, leak sensor,
      LED dimming, status LEDs, `Clock`/time.
      Use `embedded-hal` traits where a standard one fits; define project traits otherwise.
      *(The `Fan` trait was removed with the fan (ECO-001) and the `Pump` trait + INA219 current-sense
      with the pump (ECO-003) — V1 is passive, the grow LED is the only actuator.)*
- [x] Mock (host) trait implementations with injectable readings and fault-injection hooks.
- [x] **Injected `Clock` trait** (simulated time) so schedules and timeouts run deterministically in tests.

## Acceptance criteria

- Every control module is generic over the traits and depends only on them, never on `esp-hal` or
  concrete drivers (enforced by `control/` having no `esp-hal` dependency).
- Mocks support injecting leak, sensor failure, and stuck-reading conditions (needed by §10.3 scenarios).

## Notes

This is the seam that makes the entire §10.2 unit-test matrix and §10.3 simulator possible. Get the
traits right before writing controllers. Prefer generics/monomorphization over `dyn` to stay
allocation-free in `no_std`.

## Implementation

- `control/src/hal.rs`: project traits for temp/RH, capacitive moisture, reservoir, leak,
  grow-LED, status LEDs, plus injected `Clock` (monotonic) and `Rtc` (wall-clock) traits.
  Controllers are generic over these by `&mut T` (monomorphized, no `dyn`/alloc).
  *(The `Fan` and `Pump` traits were removed with the fan/pump — ECO-001/ECO-003.)*
- `control/src/testkit.rs`: host mocks for every trait with injectable readings + fault hooks
  (leak, sensor error, stuck reading via constant raw). Exercised end-to-end in
  `control/tests/hal_integration.rs` (leak warns red Water+System; injected sensor fault →
  SENSOR_FAULT; a moisture warning never cuts the grow light) and as the `sim/` foundation.
