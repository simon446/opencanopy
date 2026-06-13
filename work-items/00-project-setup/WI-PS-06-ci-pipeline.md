# WI-PS-06 — CI/CD pipeline

| Field | Value |
|---|---|
| Track | Project & Repo |
| Milestone | (supports M3/M4) |
| Depends on | WI-PS-01 |
| Spec refs | §10.5 |
| Status | Done |

## Objective

Automate the repo's quality gates so firmware, docs, and hardware artifacts are checked on every PR.

## Deliverables

- [x] `.github/workflows/ci.yml` running, per spec §10.5:
  - [x] Markdown lint + docs link/reference check.
  - [x] Firmware formatting + static analysis.
  - [x] Host unit tests + simulation tests.
  - [x] BOM generation check; CAD file presence check.
  - [x] STL manifold check (if feasible in CI).
- [x] `.github/workflows/docs.yml` for docs build/publish.
- [x] Schematic ERC / PCB DRC steps wired in if the EDA tool exposes a CLI (coordinate with EE track).

## Acceptance criteria

- A PR that breaks unit tests, sim scenarios, or markdown lint fails CI.
- BOM check fails when a grow light lacks required fields (see [WI-EE-01](../03-electronics/WI-EE-01-component-poc.md) / §16.3).
