# WI-QA-06 — Calibration guide validation

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M6-08 |
| Depends on | WI-EE-08 |
| Spec refs | §9.9, §18.2 |
| Status | Not started |

## Objective

Prove a new builder can calibrate the unit using only the written guide and the helper scripts.

## Deliverables

- [ ] Dry-run a fresh calibration following [WI-DOC-04](../06-documentation/WI-DOC-04-calibration-operation.md):
      moisture dry/wet, pump ml/s, fan min PWM, LED PPFD map, reservoir low, leak test.
- [ ] Verify `scripts/pump_calibration.py` and `scripts/moisture_calibration.py` work end-to-end.
- [ ] File doc-update PRs for any gaps found.

## Acceptance criteria

- A builder unfamiliar with the project completes calibration from the doc alone (spec §15.7 M6-08).
