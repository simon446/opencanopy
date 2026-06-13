<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# KiCad source

The KiCad schematic and PCB for the OpenCanopy controller are entered from the design-capture
documents in [`electronics/analysis/`](../../analysis/):

- **Schematic** — [WI-EE-03-schematic.md](../../analysis/WI-EE-03-schematic.md): sheet structure,
  full net list, protection, fail-off pump drive, and the [pin map](../../analysis/pin-map.csv).
- **Layout** — [WI-EE-04-pcb-layout.md](../../analysis/WI-EE-04-pcb-layout.md): stackup, floorplan,
  grounding, copper pours, test points, connector placement.

When the `.kicad_sch` / `.kicad_pcb` sources are added here, wire `kicad-cli sch erc` and
`kicad-cli pcb drc` into CI (spec §10.5) and generate the fabrication package
([WI-EE-07](../fabrication/fab-notes.md)) into `../gerbers/`, `../fabrication/`,
and `../ibom/`.
