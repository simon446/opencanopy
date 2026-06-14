# WI-EE-08 — Board bring-up & HIL fixture

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-08, M4-09 |
| Depends on | WI-EE-07, WI-FW-07 |
| Spec refs | §11.2, §10.4, §9.9 |
| Status | In progress — procedures + HIL design + test matrix authored ([bringup](../../../electronics/test/bringup.md), [hil-fixture](../../../electronics/test/hil-fixture.md)). **WI-FW-07 now Done → H1–H10 logic dry-validated against firmware host-tests** ([hil-fixture §2.1](../../../electronics/test/hil-fixture.md)). **Remaining blocker: a fabricated board** (device-physics half only). No fan in V1 ([ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md)). |

## Objective

Bring up the fabricated board safely and build the hardware-in-loop fixture that automates fault
testing.

## Deliverables

- [ ] Bring-up per §11.2: visual, continuity (no rail shorts), current-limited power, rail verify,
      regulator temps, flash, USB/UART, each sensor bus, each output with dummy load.
      *(Procedure written; power-off + rail steps ready. Firmware exists (WI-FW-07 Done) & is host-tested — firmware steps now blocked on **silicon only**.)*
- [ ] Pump MOSFET verified with real pump in water; LED dimming with driver; status LEDs. *(Fan PWM+tach struck — no fan, ECO-001. Procedure written; execution blocked on board.)*
- [ ] 24 h dry burn-in + 24 h wet-bay test (water, no plant). *(Steps 14–15; blocked on board.)*
- [x] HIL fixture (§10.4): programmable moisture, reservoir/leak switch sims, pump dummy load, LED dim
      dummy, current measurement, UART log capture. *(Fixture **designed** — hil-fixture.md §1; physical build pending. Fan tach sim struck — ECO-001.)*
- [ ] Automated HIL tests: pump never enables on leak/low-water; LED commands change; LED patterns;
      watchdog resets safely. **Logic half ✔ dry-validated** against firmware host-tests ([hil-fixture §2.1](../../../electronics/test/hil-fixture.md)); **device run on a board still pending** (the acceptance). *(H1–H10 authored; H6 fan struck; H-INA added for the INA219→pump-fault gap, §2.2.)*
- [ ] Run the §9.9 calibrations (moisture dry/wet, pump ml/s, LED PPFD map, reservoir low). *(Fan-min-PWM struck — no fan, ECO-001. Procedure written (bringup §E → WI-FW-11); values blocked on hardware + PL-06 for PPFD.)*

## Acceptance criteria

- Rails, MCU, sensors, outputs all pass (spec §15.5 M4-08).
- HIL fault tests pass (spec §15.5 M4-09); pump lockout proven in hardware.

## Notes

Calibration values produced here populate [WI-FW-11](../02-firmware/WI-FW-11-calibration-storage.md) NVS schema.

> **Risk [R8](../../../docs/risk-register.md) — firmware half now cleared.**
> [WI-FW-07](../02-firmware/WI-FW-07-safety-state-machine.md) (safety state machine) is **Done**, so
> the leak-lockout (S5) and pump-fail-off (S6) *logic* is implemented and **dry-validated against the
> firmware host-tests** ([hil-fixture §2.1](../../../electronics/test/hil-fixture.md)). The remaining
> R8 exposure is the **in-hardware** proof, gated on a fabricated board. The pump-fail-off hardware
> guarantee (gate pull-down, [WI-EE-03](WI-EE-03-schematic.md)) is already firmware-independent.
> *(R8 currently names WI-FW-07 as "Not started" — that row is **stale** and should be updated by the
> Project & Repo track, which owns the [risk register](../../../docs/risk-register.md).)*
