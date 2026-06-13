# WI-PS-06 — CI/CD pipeline

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | (supports M3/M4) |
| Depends on | WI-PS-01 |
| Spec refs | §10.5 |
| Status | Not started |

## Objective

Automate the repo's quality gates so firmware, docs, and hardware artifacts are checked on every PR.

## Deliverables

- [ ] `.github/workflows/ci.yml` running, per spec §10.5:
  - [ ] Markdown lint + docs link/reference check.
  - [ ] Firmware formatting + static analysis.
  - [ ] Host unit tests + simulation tests.
  - [ ] BOM generation check; CAD file presence check.
  - [ ] STL manifold check (if feasible in CI).
- [ ] `.github/workflows/docs.yml` for docs build/publish.
- [ ] Schematic ERC / PCB DRC steps wired in if the EDA tool exposes a CLI (coordinate with EE track).

## Acceptance criteria

- A PR that breaks unit tests, sim scenarios, or markdown lint fails CI.
- BOM check fails when a grow light lacks required fields (see [WI-EE-01](../03-electronics/WI-EE-01-component-poc.md) / §16.3).
