# WI-EE-07 — Fabrication package

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-07 |
| Depends on | WI-EE-04, WI-EE-06 |
| Spec refs | §14.1 (electronics/), §16.1 |
| Status | In progress — BOM + alternates + fab notes done & passing bom_check ([fab-notes](../../../electronics/pcb/fabrication/fab-notes.md)); Gerber/PNP/iBOM export pending KiCad PCB source |

## Objective

Generate a complete, orderable fabrication + assembly package and the BOM.

## Deliverables

- [ ] Gerbers, drill files in `electronics/pcb/gerbers/` + `fabrication/`. *(kicad-cli export; pending KiCad PCB source — commands in fab-notes §3.)*
- [ ] Pick-and-place (PNP) files. *(kicad-cli export pos; pending PCB source.)*
- [ ] Interactive BOM (`ibom/`). *(InteractiveHtmlBom; pending PCB source.)*
- [x] `electronics/bom/bom.csv` + `alternates.csv` satisfying §16.1 core-electronics constraints. *(All §16.1 rows incl. battery-backed RTC; grow light DR-01-gated candidate.)*
- [x] BOM-check script passes (coordinate with `scripts/bom_check.py` / [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md)). *(PASS incl. --strict.)*

## Acceptance criteria

- Fabrication package is complete and self-consistent (spec §15.5 M4-07).
- BOM lists alternates and passes the automated check.
