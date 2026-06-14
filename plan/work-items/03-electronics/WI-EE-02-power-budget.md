# WI-EE-02 — Power budget & PSU sizing

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M2-09 |
| Depends on | WI-EE-01 |
| Spec refs | §7.8 |
| Status | Done — **no fan in V1** ([ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md)): fan load dropped, headroom now 33 %/36 %. [report](../../../electronics/analysis/power-budget.md) + [power-budget.csv](../../../electronics/analysis/power-budget.csv); reconcile LED/pump figures against WI-EE-01 bench logs when available |

## Objective

Size the external 24 VDC PSU and the internal DC/DC rails with headroom based on measured loads.

## Deliverables

- [x] Power-budget spreadsheet: LED, pump, MCU/sensors typical + peak (§7.8). *(power-budget.csv; fan row zeroed — no fan in V1, [ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md).)*
- [x] PSU recommendation: 24 VDC 120 W (60–80 W LED) / 150 W (100 W LED), certified external brick. *(26 %/30 % continuous headroom.)*
- [x] Rail plan: 24 V (LED/pump), 12 V (optional 12 V pump only — no fan, ECO-001), 5 V (sensors), 3.3 V (MCU) with ≥20% headroom. *(All rails ≥20 %; 12 V buck now optional/DNP.)*
- [x] Connector current ratings selected for worst-case load. *(PSU 10 A / LED 6 A / pump 3 A / buses 1 A; fan header DNP.)*

## Acceptance criteria

- PSU sized with ≥20% headroom against measured peak loads from [WI-EE-01](WI-EE-01-component-poc.md).
- Rail assignment documented for the schematic.
