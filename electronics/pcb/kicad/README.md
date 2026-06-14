<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# KiCad source

The controller's electrical design is **already complete as a netlist** — entering KiCad here is an
import + layout step, not a re-capture.

1. **Schematic** = import [`../netlist/controller.net`](../netlist/controller.net) (KiCad
   *File → Import → Netlist*). That instantiates all 90 components / 61 nets from the source of truth
   ([`controller_netlist.py`](../netlist/controller_netlist.py)); KiCad's `kicad-cli sch erc` then
   supersedes the netlist's [ERC stand-in](../netlist/README.md). Design intent (sheets, protection,
   fail-off pump drive) is in [WI-EE-03-schematic.md](../../analysis/WI-EE-03-schematic.md); pin
   contract in the [pin map](../../analysis/pin-map.csv).
2. **Footprints** = each component's `footprint` field names the class (e.g. `C_0402`, `SOIC-8`).
3. **Layout** = follow [WI-EE-04-pcb-layout.md](../../analysis/WI-EE-04-pcb-layout.md) and the
   deterministic [design-rules.md](../../analysis/design-rules.md) (net classes, widths, clearances,
   DRC values). Route, pour GND/power planes, stitch vias.
4. **Fab** = generate the package with `kicad-cli` into `../gerbers/`, `../fabrication/`, `../ibom/`
   per [WI-EE-07 fab-notes](../fabrication/fab-notes.md).

When the `.kicad_sch` / `.kicad_pcb` land here, the `eda` CI job auto-detects them and runs
`kicad-cli sch erc` / `pcb drc` (spec §10.5). Until then the netlist ERC runs in the `BOM` CI job.
