# WI-QA-04 — Acoustic verification

| Field | Value |
|---|---|
| Track | Validation & QA |
| Milestone | M6-05 |
| Depends on | WI-QA-01 |
| Spec refs | §12.5, §8.7 |
| Status | Not started |

## Objective

Measure noise at 1 m across operating modes and confirm the living-room acoustic targets.

## Deliverables

- [ ] dBA @ 1 m front for: night idle, day normal, fan max, pump active, fault mode (§12.5).
- [ ] Report in `validation/acoustic/`.

## Acceptance criteria

- Normal day ≤30 dBA; night ≤25 dBA; pump active ≤35 dBA (if possible); no high-frequency PWM whine.
