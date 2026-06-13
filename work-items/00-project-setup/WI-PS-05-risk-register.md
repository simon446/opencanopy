# WI-PS-05 — Risk register

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | M0-06 |
| Depends on | WI-PS-04 |
| Spec refs | §22 (key risks), §17 (safety) |
| Status | Not started |

## Objective

Track the engineering and safety risks the spec calls out, with owners and mitigations.

## Deliverables

- [ ] `docs/risk-register.md` seeded with the seven §22 risks:
  1. Underpowered / poorly specified light.
  2. Too-small pot (root volume).
  3. Overwatering from bad moisture calibration.
  4. Water/electronics isolation failure.
  5. Heat/noise from forcing a high-light crop into a compact frame.
  6. Excessive yield expectations from the 8–10 L baseline.
  7. Scope creep (AI/app/display/enclosure).
- [ ] Each risk has: likelihood, impact, owning track, mitigation, and the work item that closes it.

## Acceptance criteria

- All §22 risks and all §17 safety risks appear with an owner and a linked mitigating work item.
