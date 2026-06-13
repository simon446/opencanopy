# WI-QA-10 — Surrogate full-loop shakedown & multi-unit trial

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M7-00 (new pre-trial gate — §23 DR-03) |
| Depends on | WI-QA-02, WI-QA-05, WI-QA-06, WI-QA-09 |
| Spec refs | §13.4, §23 (DR-03) |
| Status | Not started |

## Objective

De-risk the slow, single n=1 pepper grow trial ([WI-QA-07](WI-QA-07-grow-trial.md), 120+ days) by
first running a short **full-loop shakedown** on a fast-growing surrogate crop, and by running the
pepper trial on **more than one unit** where feasible — so a control or hardware failure surfaces in
days, not after a lost four-month cycle (§23 DR-03).

## Deliverables

- [ ] Full-loop shakedown on a fast surrogate (e.g. basil / lettuce / bean, ~2–4 week visible
      response) on a complete unit: exercises light schedule, closed-loop watering, reservoir refill,
      fan, logging, and fault handling end to end.
- [ ] Shakedown report in `validation/grow-trials/shakedown-001/`: control/hardware issues found and
      fixed before the pepper trial starts.
- [ ] ≥2 pepper units run in parallel for WI-QA-07 where hardware allows (n>1), **or** a documented
      justification (and risk-register entry) if only one unit is available.

## Acceptance criteria

- A surrogate shakedown completes with **no unresolved control/hardware/safety issue** before WI-QA-07
  begins — this is the gate on the long trial.
- WI-QA-07 runs with n>1, or the single-unit risk is explicitly accepted in `docs/risk-register.md`.

## Notes

This sits in front of [WI-QA-07](WI-QA-07-grow-trial.md): the long pepper trial should not start until
the full loop is proven on a fast crop. Together with [WI-QA-09](WI-QA-09-bench-characterization.md) it
converts the spec's single, slow, post-build validation gate into a staged one.
