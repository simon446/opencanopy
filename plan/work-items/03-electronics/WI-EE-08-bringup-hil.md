# WI-EE-08 — Board bring-up & HIL fixture

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-08, M4-09 |
| Depends on | WI-EE-07, WI-FW-07 |
| Spec refs | §11.2, §10.4, §9.9 |
| Status | Blocked — procedures + HIL design + test matrix authored ([bringup](../../../electronics/test/bringup.md), [hil-fixture](../../../electronics/test/hil-fixture.md)); **execution blocked on WI-FW-07 (Not started) + a fabricated board** |

## Objective

Bring up the fabricated board safely and build the hardware-in-loop fixture that automates fault
testing.

## Deliverables

- [ ] Bring-up per §11.2: visual, continuity (no rail shorts), current-limited power, rail verify,
      regulator temps, flash, USB/UART, each sensor bus, each output with dummy load.
      *(Procedure written; power-off + rail steps ready, firmware steps blocked on WI-FW-07.)*
- [ ] Pump MOSFET verified with real pump in water; fan PWM+tach; LED dimming with driver; status LEDs.
      *(Procedure written; execution blocked on board + firmware.)*
- [ ] 24 h dry burn-in + 24 h wet-bay test (water, no plant). *(Steps 14–15; blocked on board.)*
- [x] HIL fixture (§10.4): programmable moisture, reservoir/leak switch sims, pump dummy load, fan tach
      sim, LED dim dummy, current measurement, UART log capture. *(Fixture **designed** — hil-fixture.md §1; physical build pending.)*
- [ ] Automated HIL tests: pump never enables on leak/low-water; LED/fan commands change; LED patterns;
      watchdog resets safely. *(Test matrix H1–H10 **authored**; run blocked on WI-FW-07 + board.)*
- [ ] Run the §9.9 calibrations (moisture dry/wet, pump ml/s, fan min PWM, LED PPFD map, reservoir low).
      *(Procedure written (bringup §E → WI-FW-11); values blocked on hardware + PL-06 for PPFD.)*

## Acceptance criteria

- Rails, MCU, sensors, outputs all pass (spec §15.5 M4-08).
- HIL fault tests pass (spec §15.5 M4-09); pump lockout proven in hardware.

## Notes

Calibration values produced here populate [WI-FW-11](../02-firmware/WI-FW-11-calibration-storage.md) NVS schema.

> **Blocker tracked as [risk R8](../../../docs/risk-register.md):** this item's in-hardware safety
> proofs — pump fail-off (S6) and leak lockout (S5) via the HIL fault tests — are gated on
> [WI-FW-07](../02-firmware/WI-FW-07-safety-state-machine.md) (safety state machine, *Not started*).
> The HIL fixture can be built and dry-validated against the firmware sim / HAL mocks ahead of
> silicon; the pump-fail-off hardware guarantee (gate pull-down, [WI-EE-03](WI-EE-03-schematic.md))
> is already firmware-independent.
