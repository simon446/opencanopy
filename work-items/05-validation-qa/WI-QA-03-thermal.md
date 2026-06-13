# WI-QA-03 — Thermal verification

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M6-04 |
| Depends on | WI-QA-01 |
| Spec refs | §12.4, §9.5 (derating), §17.2 |
| Status | Not started |

## Objective

Measure thermal behavior across the documented cases and prove LED derating triggers before unsafe
temperatures.

## Deliverables

- [ ] Temperature measurements: canopy air, LED heat sink, electronics bay, driver case, printed parts
      near LED, reservoir water (§12.4).
- [ ] Test cases: LED 50%+fan 4 h, LED 100%+fan 4 h, LED 100%+fan failed (to trip), hot room 30 °C 4 h,
      night fan off/pulse 8 h.
- [ ] Report in `validation/thermal/`.

## Acceptance criteria

- No plastic deformation; electronics bay within component ratings.
- LED derating triggers before unsafe temps; canopy air does not exceed critical threshold without fault.
