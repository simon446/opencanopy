<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# Programmatic PCB — headless code → Gerbers (no KiCad GUI)

A **fully scripted, GUI-free** path from our schematic netlist to a routed board and a fabrication
package, using [**tscircuit**](https://tscircuit.com) (React/TSX EDA with a built-in autorouter and
Gerber export). This answers "can the PCB be built programmatically?" — **yes**, and this directory is
the working proof, generated from [`../netlist/controller_netlist.py`](../netlist/controller_netlist.py).

Built following the official [tscircuit skill](https://github.com/tscircuit/skill) idioms (named-net
connectivity, real passive elements, `pinLabels`).

## Files

| File | What |
|---|---|
| [`gen_tscircuit.py`](gen_tscircuit.py) | Generator: reads the netlist → emits the tscircuit board. |
| `controller.circuit.tsx` | Generated board (60 populated parts, 169 net connections). Do not hand-edit. |
| [`build.sh`](build.sh) | One-shot pipeline: install toolchain → generate → build → export. |
| `out/controller.gerbers.zip` | **Gerbers + drill + BOM + pick-and-place** — a complete fab package. |
| `out/controller.kicad_pcb` | The same board as a KiCad PCB (bridge back to KiCad for refinement). |
| `out/controller.pcb.png` | Rendered preview of the auto-placed/-routed board. |

## Reproduce

```sh
./build.sh          # needs node+npm, bun, network (installs @tscircuit/cli on first run)
```

## The honest status — this is a DRAFT, not a fab-ready board

The pipeline is real and the outputs are genuine fab-format files, but **don't send them to a fab as
is**:

1. **Footprints:** passive footprints (0402/0805/1206/1210/2512) are real; **IC, connector, and the
   ESP32-S3 module footprints are single-row `pinrowN` placeholders** — correct pin *count*, wrong
   land pattern. Swap in real footprints (`tsci search`/`tsci import`, or KiCad libraries) before fab.
2. **Layout quality:** auto-placement + autoroute is **autorouter-grade**. The autorouter logged
   `ran out of iterations` — routing is incomplete/rough. It honours the *netlist* but **not** the
   power/analog separation, copper-pour heatsinks, or star ground in
   [`../../analysis/design-rules.md`](../../analysis/design-rules.md). For a 24 V high-current +
   analog-moisture board, that separation is a correctness/safety concern, not cosmetics.
3. **Scope:** only **populated** controller parts are emitted (DNP options omitted; the status-LED
   board is a separate PCB). Diode anode/cathode and module pinouts on placeholder footprints need a
   review pass.

## Where this fits

This is **the** PCB route ([ECO-002](../../analysis/ECO-002-pcb-toolchain.md); KiCad retired): the
(complete, CI-checked) netlist → tscircuit → autoroute → Gerbers, no GUI. Best for fast, reproducible
drafts and CI; refine footprints/placement, then re-export.

To reach a *fabricable* board the remaining work is real footprints + a reviewed power/analog/thermal
placement per [`design-rules.md`](../../analysis/design-rules.md) — the autorouter doesn't remove that
review. If you'd rather do that refinement in KiCad, `out/controller.kicad_pcb` (and the netlist's
`controller.net`) open the design there — **optional interchange**, not required.

> Toolchain note: tscircuit (npm/bun) is **not** a repo dependency or a CI gate — it's an optional
> build tool installed on demand by `build.sh`. The plain-Python netlist ERC remains the committed gate.
