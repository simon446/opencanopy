# WI-QA-01 — Dry electrical run

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M6-01 |
| Depends on | WI-EE-08, WI-FW-09 |
| Spec refs | §13.2, §11.2 |
| Status | Not started |

## Objective

Run the integrated unit for 7+ days with no plant and no/dummy water loads to prove firmware and
electronics stability.

## Deliverables

- [ ] 7-day continuous run log (`validation/logs/`).
- [ ] Records: no firmware crash, stable light schedule, persisted logs, correct status LEDs, watchdog
      not repeatedly firing, stable fan control, no overheating.

## Acceptance criteria

- All §13.2 dry-run pass conditions met over ≥7 days.
