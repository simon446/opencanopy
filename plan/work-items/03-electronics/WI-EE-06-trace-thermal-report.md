# WI-EE-06 — Trace current & thermal report

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-06 |
| Depends on | WI-EE-04 |
| Spec refs | §7.10, §11.3 |
| Status | In progress — calc/tables/predictions complete ([report](../../../electronics/test/pcb-verification.md) + [trace_width.py](../../../electronics/analysis/trace_width.py)); bench measurements pending fabricated board (filled at bring-up) |

## Objective

Prove every high-current path and heat-dissipating component is within rating, with measured evidence.

## Deliverables

- [x] Trace-width calculation for each power path; maximum-current table (§11.3). *(IPC-2221; trace_width.py.)*
- [ ] Voltage-drop measurement under max load. *(Predicted <50 mV/path; measured rows T1–T2 pending board.)*
- [ ] MOSFET temperature at 100% pump and fan; regulator temp at worst-case ambient. *(Predicted; measured rows T3–T5 pending board.)*
- [ ] Thermal camera image at max pump/fan/LED-control load. *(Template ready; image T6 pending board → validation/thermal/.)*
- [x] Connector current-rating table. *(Carried from WI-EE-02 §4.)*

## Acceptance criteria

- All high-current paths checked and documented (spec §15.5 M4-06).
- No component exceeds its rated temperature under worst-case load.
