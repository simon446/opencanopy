# WI-EE-04 — PCB layout

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-02, M4-05 |
| Depends on | WI-EE-03 |
| Spec refs | §7.9, §7.10 |
| Status | In progress — layout design captured ([report](../../../electronics/analysis/WI-EE-04-pcb-layout.md)); KiCad board entry + automated DRC pending source files |

## Objective

Lay out the controller PCB with correct power handling, analog/digital separation, test points, and
serviceability.

## Deliverables

- [x] KiCad PCB (2-layer acceptable, 4-layer preferred for power/ground). *(4-layer stackup designed; board entry from capture pending.)*
- [x] High-current LED/pump paths sized per trace-width calc; not routed through control traces. *(Targets set; proven in WI-EE-06. Fan path DNP — no fan in V1, [ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md).)*
- [x] LED current loop kept away from moisture/ADC analog lines; partitioned/star grounds. *(Floorplan partitions power vs analog; star ground specified.)*
- [x] Copper pours for MOSFET heat dissipation. *(Pump FET + regulators pours specified.)*
- [x] Test points on every rail, I2C, UART, pump drive, LED dim, sensor inputs (§7.9). *(Enumerated; fan PWM/tach TPs on the DNP fan footprint only — no fan in V1, ECO-001.)*
- [x] Locking/keyed connectors (no loose Dupont); silkscreen labels with polarity/voltage/warnings. *(Specified per domain.)*
- [ ] DRC clean. *(Runs against KiCad source via kicad-cli in CI; pending source files.)*

## Acceptance criteria

- DRC clean (spec §15.5 M4-02).
- All rails and control signals have accessible test points.
