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
| `controller.net` | Generated standard netlist (S-expression) — optional KiCad/other-tool interchange. Regenerate: `--emit-kicad`. |

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

## What the ERC check enforces (the electrical-rule gate)

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

## How this becomes a board

The PCB is built with the **headless tscircuit flow** in
[`../programmatic/`](../programmatic/) — [`gen_tscircuit.py`](../programmatic/gen_tscircuit.py) reads
this netlist directly and emits a tscircuit board that auto-routes and exports Gerbers/PnP/BOM with no
GUI (see [ECO-002](../../analysis/ECO-002-pcb-toolchain.md); **KiCad is retired**). Layout follows
[`WI-EE-04`](../../analysis/WI-EE-04-pcb-layout.md) + [`design-rules.md`](../../analysis/design-rules.md).

The remaining work — real footprints (the draft uses `pinrowN` placeholders for ICs/connectors/module)
and a reviewed power/analog/thermal placement — is the same regardless of tool; the autorouter doesn't
remove that review.

**Optional KiCad interchange:** `controller.net` (this netlist, KiCad/standard format) and the
`controller.kicad_pcb` that the programmatic flow exports let anyone open the design in KiCad if they
want — interchange only, not part of the workflow.
