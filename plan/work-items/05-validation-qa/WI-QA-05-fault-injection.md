# WI-QA-05 — Status LED & fault injection

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M6-06, M6-07 |
| Depends on | WI-QA-01 |
| Spec refs | §10.4, §9.8, §11.4 |
| Status | Not started |

## Objective

Verify every status-LED pattern and inject the safety faults to confirm correct lockouts and recovery.

## Deliverables

- [ ] Status-LED verification (video/table) covering all patterns and the 5 indicators (§9.8).
- [ ] Fault injection: leak, reservoir-low, moisture-sensor stuck/invalid, pump no-rise, fan tach loss,
      over-temp, RTC invalid.
- [ ] Confirm leak fault latches until manual clear (§11.4); pump never enables during leak/low-water.
- [ ] Report in `validation/test-plans/`.

## Acceptance criteria

- All LED patterns correct (spec §15.7 M6-06).
- Leak/low-water/sensor faults pass with correct lockouts (spec §15.7 M6-07).
