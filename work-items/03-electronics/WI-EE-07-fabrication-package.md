# WI-EE-07 — Fabrication package

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-07 |
| Depends on | WI-EE-04, WI-EE-06 |
| Spec refs | §14.1 (electronics/), §16.1 |
| Status | Not started |

## Objective

Generate a complete, orderable fabrication + assembly package and the BOM.

## Deliverables

- [ ] Gerbers, drill files in `electronics/pcb/gerbers/` + `fabrication/`.
- [ ] Pick-and-place (PNP) files.
- [ ] Interactive BOM (`ibom/`).
- [ ] `electronics/bom/bom.csv` + `alternates.csv` satisfying §16.1 core-electronics constraints.
- [ ] BOM-check script passes (coordinate with `scripts/bom_check.py` / [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md)).

## Acceptance criteria

- Fabrication package is complete and self-consistent (spec §15.5 M4-07).
- BOM lists alternates and passes the automated check.
