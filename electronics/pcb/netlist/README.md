<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# Controller netlist — the schematic, as data (WI-EE-03)

This directory holds the OpenCanopy controller's **complete electrical design** as a reviewable,
diff-able, CI-checked netlist. It exists because this project is developed without a KiCad GUI, so the
schematic is captured here as data instead of a binary `.kicad_sch`. It is the **single source of
truth** for connectivity and the populated parts list.

## Files

| File | What it is |
|---|---|
| [`controller_netlist.py`](controller_netlist.py) | **Source of truth.** Every component (incl. all passives) and every pin-level net. Also the ERC checker. |
| `controller-netlist.csv` | Generated flat netlist (`net,ref,pin,populated,part`) — human/diff-friendly. Regenerate: `--emit-csv`. |
| `controller.net` | Generated **KiCad-importable** netlist (S-expression). This is the KiCad-entry handoff (see below). Regenerate: `--emit-kicad`. |

The two generated files are committed so the layout step and reviewers have them without running
Python; they are reproduced exactly by the source.

## Commands

```sh
python3 controller_netlist.py --selftest     # ERC + design assertions (the CI gate)
python3 controller_netlist.py --check        # ERC against the live netlist (verbose)
python3 controller_netlist.py --stats        # component / net counts
python3 controller_netlist.py --emit-csv     # (re)write controller-netlist.csv
python3 controller_netlist.py --emit-kicad   # (re)write controller.net
```

## What the ERC check enforces (stand-in for `kicad-cli sch erc`)

- No **floating** nets (every net has ≥2 populated pins, unless explicitly reserved/DNP/no-connect).
- No pin assigned to **two nets** (no shorts / double-drives).
- Every **populated** component sits on ≥1 net (no orphans).
- The core power rails (`+24V`, `+5V`, `+3V3`, `GND`) exist.
- The **firmware pin contract** holds: each MCU net matches `electronics/analysis/pin-map.csv`
  (e.g. `SDA`→GPIO8) — so the schematic can't drift from the firmware.
- **BOM coverage**: every populated part is in `electronics/bom/bom.csv` (connectors via their `CN_*`
  alias) — so the schematic can't drift from the buildable parts list.
- Design assertions: the **pump gate fail-OFF** (R1 pull-down to GND) is present, and every **fan**
  part is DNP (no fan in V1, [ECO-001](../../analysis/ECO-001-fan-removal.md)).

CI runs `--selftest` as a **blocking** check in the `BOM` job.

## How this becomes a KiCad schematic / PCB

This netlist is the bridge, not a replacement for layout:

1. In KiCad, create the project under [`../kicad/`](../kicad/) and **Import Netlist** →
   `controller.net`. That instantiates every component and net; you then place symbols and the tool's
   own ERC supersedes the check here.
2. Assign footprints (the `footprint` field names the class, e.g. `C_0402`, `SOIC-8`) and lay out the
   board per [`WI-EE-04`](../../analysis/WI-EE-04-pcb-layout.md) (stackup, floorplan, net classes,
   design rules in [`design-rules.md`](../../analysis/design-rules.md)).
3. Route, then generate the fab package with `kicad-cli` into `../gerbers/`, `../fabrication/`,
   `../ibom/` per [`fab-notes.md`](../fabrication/fab-notes.md). The `eda` CI job then runs ERC/DRC on
   the real source.

Steps 2–3 (placement, routing, Gerber export) can be done **in the KiCad GUI** — or **headlessly**
via the [`../programmatic/`](../programmatic/) tscircuit flow, which auto-routes this netlist and
exports a Gerber/PnP/BOM **draft** with no GUI. Either way the residual is the same: real footprints
and a reviewed power/analog/thermal placement (the autorouter doesn't remove that review). This
netlist makes the schematic-entry step mechanical rather than a re-capture from prose.
