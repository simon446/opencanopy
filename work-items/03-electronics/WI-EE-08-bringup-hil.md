# WI-EE-08 — Board bring-up & HIL fixture

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-08, M4-09 |
| Depends on | WI-EE-07, WI-FW-07 |
| Spec refs | §11.2, §10.4, §9.9 |
| Status | Not started |

## Objective

Bring up the fabricated board safely and build the hardware-in-loop fixture that automates fault
testing.

## Deliverables

- [ ] Bring-up per §11.2: visual, continuity (no rail shorts), current-limited power, rail verify,
      regulator temps, flash, USB/UART, each sensor bus, each output with dummy load.
- [ ] Pump MOSFET verified with real pump in water; fan PWM+tach; LED dimming with driver; status LEDs.
- [ ] 24 h dry burn-in + 24 h wet-bay test (water, no plant).
- [ ] HIL fixture (§10.4): programmable moisture, reservoir/leak switch sims, pump dummy load, fan tach
      sim, LED dim dummy, current measurement, UART log capture.
- [ ] Automated HIL tests: pump never enables on leak/low-water; LED/fan commands change; LED patterns;
      watchdog resets safely.
- [ ] Run the §9.9 calibrations (moisture dry/wet, pump ml/s, fan min PWM, LED PPFD map, reservoir low).

## Acceptance criteria

- Rails, MCU, sensors, outputs all pass (spec §15.5 M4-08).
- HIL fault tests pass (spec §15.5 M4-09); pump lockout proven in hardware.

## Notes

Calibration values produced here populate [WI-FW-11](../02-firmware/WI-FW-11-calibration-storage.md) NVS schema.
