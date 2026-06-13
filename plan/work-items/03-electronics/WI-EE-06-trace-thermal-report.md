# WI-EE-06 — Trace current & thermal report

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-06 |
| Depends on | WI-EE-04 |
| Spec refs | §7.10, §11.3 |
| Status | Not started |

## Objective

Prove every high-current path and heat-dissipating component is within rating, with measured evidence.

## Deliverables

- [ ] Trace-width calculation for each power path; maximum-current table (§11.3).
- [ ] Voltage-drop measurement under max load.
- [ ] MOSFET temperature at 100% pump and fan; regulator temp at worst-case ambient.
- [ ] Thermal camera image at max pump/fan/LED-control load.
- [ ] Connector current-rating table.

## Acceptance criteria

- All high-current paths checked and documented (spec §15.5 M4-06).
- No component exceeds its rated temperature under worst-case load.
