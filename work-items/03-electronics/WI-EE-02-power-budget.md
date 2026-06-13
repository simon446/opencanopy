# WI-EE-02 — Power budget & PSU sizing

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M2-09 |
| Depends on | WI-EE-01 |
| Spec refs | §7.8 |
| Status | Not started |

## Objective

Size the external 24 VDC PSU and the internal DC/DC rails with headroom based on measured loads.

## Deliverables

- [ ] Power-budget spreadsheet: LED, pump, fan, MCU/sensors typical + peak (§7.8).
- [ ] PSU recommendation: 24 VDC 120 W (60–80 W LED) / 150 W (100 W LED), certified external brick.
- [ ] Rail plan: 24 V (LED/pump), 12 V (fan/pump), 5 V (sensors), 3.3 V (MCU) with ≥20% headroom.
- [ ] Connector current ratings selected for worst-case load.

## Acceptance criteria

- PSU sized with ≥20% headroom against measured peak loads from [WI-EE-01](WI-EE-01-component-poc.md).
- Rail assignment documented for the schematic.
