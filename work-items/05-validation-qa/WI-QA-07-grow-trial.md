# WI-QA-07 — Grow trial

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M7-01 … M7-08 |
| Depends on | WI-QA-02, WI-QA-03, WI-QA-05, WI-QA-06, WI-QA-10 |
| Spec refs | §13.4, §5.9, §23 (DR-03) |
| Status | Not started |

## Objective

Run a real Carolina Reaper (or similar hot pepper) trial to validate that the integrated system keeps
a plant healthy with minimal intervention.

## Deliverables

- [ ] `validation/grow-trials/trial-001/` with a completed log template (§13.4).
- [ ] Weekly dated photos (consistent angle); daily/weekly water-use CSV; min/max temp/RH/VPD.
- [ ] Milestone dates: germination, first flower, first fruit set, first ripe fruit.
- [ ] Root/media inspection notes (no chronic waterlogging); pruning + fertilizer events.
- [ ] Threshold-update PR based on trial data (M7-07).
- [ ] Minimum trial: 60 days seedling/vegetative; 120+ days for fruiting validation.
- [ ] Gated by the surrogate shakedown ([WI-QA-10](WI-QA-10-surrogate-shakedown.md)); run with **n>1**
      units where feasible, or accept the single-unit risk in `docs/risk-register.md` (§23 DR-03).

## Acceptance criteria

- Plant stays healthy; no chronic over/under-watering; ≥1 flowering/fruiting event at maturity.
- User intervention limited to refill/feed/prune/support; no water/electrical safety events (§13.4).
