# WI-EE-03 — Schematic & protection

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-01, M4-04 |
| Depends on | WI-EE-01, WI-EE-02 |
| Spec refs | §7.9, §7.10, §11.1, §17.1 |
| Status | Electrical design **complete** — schematic captured as a CI-checked netlist ([controller_netlist.py](../../../electronics/pcb/netlist/controller_netlist.py), 90 parts / 61 nets); ERC + BOM-coverage + firmware-pin-contract checks pass in CI. Board built by the headless tscircuit flow ([ECO-002](../../../electronics/analysis/ECO-002-pcb-toolchain.md); KiCad retired). [report](../../../electronics/analysis/WI-EE-03-schematic.md) |

## Objective

Capture the full controller schematic (as the machine-readable netlist), including all protection
circuits and the fail-safe pump drive.

## Deliverables

- [x] Schematic covering MCU, all sensor buses, pump driver, LED dim interface, status LED board
      connector, and expansion headers (camera/PAR/load-cell/pH/EC, unpopulated); fan PWM/tach kept as
      a **DNP** option (no fan in V1, [ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md)).
      *(Formalised as the netlist — 90 parts / 61 nets — + full pin map.)*
- [x] Protection: input fuse, reverse-polarity protection, TVS, flyback on inductive loads (§17.1).
      *(F1 6.3/8 A, P-FET reverse-polarity, SMBJ28A TVS, flyback on pump; fan flyback DNP — no fan, ECO-001.)*
- [x] Pump MOSFET with **gate pull-down** so pump fails OFF on MCU reset/crash (§9.6, §11.4).
      *(External 10 kΩ gate→GND pull-down; hardware-guaranteed, not firmware-dependent.)*
- [x] Logic-level MOSFETs at 3.3 V gate, or gate driver (§7.10). *(Logic-level FET enhanced at V_GS=2.5 V; series gate R + optional driver.)*
- [x] Schematic passes ERC; design review per §11.1 completed. *(§11.1 checklist worked; **netlist ERC passes in CI** — no floating nets, no double-driven pins, fail-OFF pump gate, full BOM coverage, firmware-pin-contract match.)*

## Acceptance criteria

- ERC clean; review checklist (§11.1) signed off.
- Fuse, reverse-polarity, TVS, and pump-fail-off present and labeled.
