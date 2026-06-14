# WI-EE-07 — Fabrication package

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | M4-07 |
| Depends on | WI-EE-04, WI-EE-06 |
| Spec refs | §14.1 (electronics/), §16.1 |
| Status | BOM **complete** — every populated part incl. all passives is in [bom.csv](../../../electronics/bom/bom.csv) (netlist↔BOM coverage enforced in CI); alternates + fab notes done; passes bom_check `--strict`. A **headless programmatic draft** fab package (Gerbers/drill/PnP/BOM) is generated from the netlist via tscircuit → [`programmatic/out/`](../../../electronics/pcb/programmatic/out/) ([ECO-002](../../../electronics/analysis/ECO-002-pcb-toolchain.md); KiCad retired). **Residual:** a *fab-ready* package needs real footprints + a reviewed layout, then re-export. [fab-notes](../../../electronics/pcb/fabrication/fab-notes.md) |

## Objective

Generate a complete, orderable fabrication + assembly package and the BOM.

## Deliverables

- [x] Gerbers + drill files. *(Draft generated headlessly by the tscircuit flow → [`pcb/programmatic/out/controller.gerbers.zip`](../../../electronics/pcb/programmatic/out/controller.gerbers.zip); not yet fab-ready — placeholder footprints, autorouter-grade.)*
- [x] Pick-and-place (PnP) files. *(In the same Gerber zip — `pick_and_place.csv`.)*
- [x] Board BOM (designators). *(In the same Gerber zip — `bom.csv`; the orderable BOM is `bom/bom.csv`.)*
- [x] `electronics/bom/bom.csv` + `alternates.csv` satisfying §16.1 core-electronics constraints. *(All §16.1 rows incl. battery-backed RTC; grow light DR-01-gated candidate.)*
- [x] BOM-check script passes (coordinate with `scripts/bom_check.py` / [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md)). *(PASS incl. --strict.)*

## Acceptance criteria

- Fabrication package is complete and self-consistent (spec §15.5 M4-07).
- BOM lists alternates and passes the automated check.
