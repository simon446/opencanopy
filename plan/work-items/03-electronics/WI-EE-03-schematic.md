# WI-EE-03 — Schematic & protection

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-01, M4-04 |
| Depends on | WI-EE-01, WI-EE-02 |
| Spec refs | §7.9, §7.10, §11.1, §17.1 |
| Status | In progress — design captured ([report](../../../electronics/analysis/WI-EE-03-schematic.md) + [pin-map.csv](../../../electronics/analysis/pin-map.csv)); KiCad entry + automated ERC pending source files |

## Objective

Capture the full controller schematic in KiCad, including all protection circuits and the
fail-safe pump drive.

## Deliverables

- [x] KiCad schematic covering MCU, all sensor buses, fan PWM/tach, pump driver, LED dim interface,
      status LED board connector, and expansion headers (camera/PAR/load-cell/pH/EC, unpopulated).
      *(Captured as 5-sheet design + full pin map; KiCad entry from capture pending.)*
- [x] Protection: input fuse, reverse-polarity protection, TVS, flyback on inductive loads (§17.1).
      *(F1 6.3/8 A, P-FET reverse-polarity, SMBJ28A TVS, flyback on pump & fan.)*
- [x] Pump MOSFET with **gate pull-down** so pump fails OFF on MCU reset/crash (§9.6, §11.4).
      *(External 10 kΩ gate→GND pull-down; hardware-guaranteed, not firmware-dependent.)*
- [x] Logic-level MOSFETs at 3.3 V gate, or gate driver (§7.10). *(Logic-level FET enhanced at V_GS=2.5 V; series gate R + optional driver.)*
- [ ] Schematic passes ERC; design review per §11.1 completed. *(§11.1 checklist worked against the capture; automated KiCad-CLI ERC pending source files.)*

## Acceptance criteria

- ERC clean; review checklist (§11.1) signed off.
- Fuse, reverse-polarity, TVS, and pump-fail-off present and labeled.
