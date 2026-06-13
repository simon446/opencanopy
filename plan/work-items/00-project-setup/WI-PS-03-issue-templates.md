# WI-PS-03 — Issue templates & contribution workflow

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | M0-03 |
| Depends on | WI-PS-01 |
| Spec refs | §14, §15.1 |
| Status | Done |

## Objective

Make it easy for contributors across all five disciplines to file structured, triageable issues.

## Deliverables

- [x] Issue templates under `.github/ISSUE_TEMPLATE/`: bug, firmware, hardware/PCB, mechanical, docs.
- [x] Each template prompts for: affected track, spec section, hardware revision, repro/steps.
- [x] `CONTRIBUTING.md` covering branch naming, PR review per track, and the test-before-merge rule.
- [x] PR template referencing the relevant work-item ID and its acceptance checklist.

## Acceptance criteria

- A bug, hardware, firmware, mechanical, and docs template each exist and render on GitHub/GitLab.
- PRs are required to cite a work-item ID.
