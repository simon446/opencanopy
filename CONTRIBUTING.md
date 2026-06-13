# Contributing to OpenCanopy

OpenCanopy is an open-hardware tabletop pepper grower built **work item by work item**, organized by
engineering discipline. This guide covers how to file issues, name branches, open PRs, get them
reviewed by the right track, and the test-before-merge rule. It implements spec §14 and §15.1.

If you only have a question, open a [Discussion](https://github.com/opencanopy/opencanopy/discussions)
rather than an issue.

## Tracks and ownership

Every change belongs to one of seven tracks. Each track owns its subtree and reviews PRs that touch it.

| Track | Folder | Owns |
|---|---|---|
| Project & Repo | `plan/work-items/00-project-setup/` | Repo skeleton, licensing, requirements/scope, risk register, CI |
| Plant Science | `01-plant-science/` | Plant profile, DLI, watering, VPD, nutrients |
| Firmware | `02-firmware/`, `firmware/` | Control logic, simulator, state machine, logging |
| Electronics | `03-electronics/`, `electronics/` | PoC, schematic, PCB, harness, bring-up, HIL |
| Mechanical | `04-mechanical/`, `mechanical/` | Frame, bays, mounts, routing, printed parts |
| Validation & QA | `05-validation-qa/`, `validation/` | Dry/wet runs, thermal, acoustic, fault injection, grow trial |
| Documentation | `06-documentation/`, `docs/` | README, safety, assembly, calibration, grow guide |

The work-item breakdown lives in `plan/work-items/` (start at its `README.md`). Treat each work
item's checkbox list as its definition of done.

## Filing issues

Use the structured [issue forms](.github/ISSUE_TEMPLATE/): **bug**, **firmware**, **hardware/PCB**,
**mechanical**, or **docs**. Every form asks for:

- **Affected track** — who should triage it.
- **Spec section** — the V1 spec subsection it concerns.
- **Hardware revision** — PCB/mechanical rev, firmware build/commit, or `sim`/`N/A`.
- **Steps to reproduce** — minimal, ordered.

Safety-related reports (water/electrical, thermal) should reference `docs/safety.md` and the risk
register (`docs/risk-register.md`).

## Branch naming

Branch off `main`. Name branches `<track>/<work-item-id>-<short-slug>`, lowercase, e.g.:

```
firmware/WI-FW-04-irrigation-rules
electronics/WI-EE-02-schematic-erc
docs/WI-DOC-03-calibration-guide
project/WI-PS-06-ci-pipeline
```

Use the track prefixes: `project`, `plant`, `firmware`, `electronics`, `mechanical`, `validation`,
`docs`. One work item per branch where practical; keep PRs focused.

## Commits

- Write imperative, present-tense commit subjects (e.g. "Add leak-lockout scenario").
- Reference the work item in the body where useful (e.g. `Part of WI-FW-04`).
- Add an SPDX header to new source files (`firmware/`, `scripts/`): `SPDX-License-Identifier: Apache-2.0`.
  Hardware/mechanical/doc files are governed by the subtree map in `LICENSES/README.md`.

## Pull requests

Open PRs against `main` using the [PR template](.github/PULL_REQUEST_TEMPLATE.md). Every PR **must**:

1. **Cite a work-item ID** (e.g. `WI-ME-03`) and paste that work item's acceptance checklist into the
   PR description. PRs without a work-item reference will not be reviewed.
2. **Check off the boxes** it satisfies in the work-item file itself, as part of the PR.
3. **State how it was tested** (see below) with evidence.
4. **Address safety (§17)** for firmware/electronics/mechanical changes, or add/update a
   risk-register entry if a risk changes.

### Review per track

A PR is reviewed and approved by a maintainer of the **owning track** (the one whose subtree it
changes). Cross-track changes (e.g. a firmware change forced by a pin reassignment) need approval
from **each** affected track. Tag the owning-track reviewer in the PR. A PR merges only when:

- the owning track (and any other affected track) has approved, **and**
- CI is green.

### Test-before-merge rule

**No PR merges with failing or skipped required checks.** CI (`.github/workflows/ci.yml`, spec §10.5)
runs on every PR. Depending on what the PR touches, the relevant gates must pass:

| If the PR touches… | These must pass before merge |
|---|---|
| `firmware/` | host unit tests, simulation scenarios, firmware formatting + static analysis; HIL if hardware-affecting |
| `docs/`, any `*.md` | markdown lint, docs link/reference check |
| `electronics/bom/` | `scripts/bom_check.py` (a grow light missing required §16.3 fields fails CI) |
| `electronics/pcb/` | ERC/DRC where the EDA CLI supports it; trace/current sanity |
| `mechanical/` | CAD presence check, STL manifold check (where feasible) |

If a check genuinely does not apply, say so explicitly in the PR and explain why — do not disable a
required check to merge. Safety-critical firmware paths (pump fail-off on reset/brownout, leak-triggered
pump lockout) must remain covered by tests.

## Code of conduct

Be respectful and constructive. This is a DIY hardware project; assume good faith and help newcomers
find the right track.

## License of contributions

By contributing you agree your contribution is licensed under the license that governs the subtree you
changed, per `LICENSES/README.md` (Apache-2.0 firmware/scripts, CERN-OHL-S v2 hardware/mechanical,
CC BY 4.0 docs).
