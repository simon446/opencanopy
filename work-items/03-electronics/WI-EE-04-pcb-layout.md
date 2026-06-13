# WI-EE-04 — PCB layout

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-02, M4-05 |
| Depends on | WI-EE-03 |
| Spec refs | §7.9, §7.10 |
| Status | Not started |

## Objective

Lay out the controller PCB with correct power handling, analog/digital separation, test points, and
serviceability.

## Deliverables

- [ ] KiCad PCB (2-layer acceptable, 4-layer preferred for power/ground).
- [ ] High-current LED/pump/fan paths sized per trace-width calc; not routed through control traces.
- [ ] LED current loop kept away from moisture/ADC analog lines; partitioned/star grounds.
- [ ] Copper pours for MOSFET heat dissipation.
- [ ] Test points on every rail, I2C, UART, pump drive, fan PWM/tach, LED dim, sensor inputs (§7.9).
- [ ] Locking/keyed connectors (no loose Dupont); silkscreen labels with polarity/voltage/warnings.
- [ ] DRC clean.

## Acceptance criteria

- DRC clean (spec §15.5 M4-02).
- All rails and control signals have accessible test points.
