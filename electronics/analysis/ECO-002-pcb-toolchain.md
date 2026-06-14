<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# ECO-002 — PCB toolchain: programmatic (tscircuit); KiCad retired

**Type:** Engineering Change Order (electronics-side record)
**Date:** 2026-06-14
**Status:** Approved (maintainer) · executed
**Owning track:** Electronics · **Spec touch:** §7.9, §10.5, §16.1 (KiCad/`kicad-cli` references)

## 1. Decision

The controller PCB is designed with a **headless, code-only flow** built on
[tscircuit](https://tscircuit.com): the netlist
([`controller_netlist.py`](../pcb/netlist/controller_netlist.py)) generates a tscircuit board, which
auto-routes and exports Gerbers/PnP/BOM with **no GUI** ([`pcb/programmatic/`](../pcb/programmatic/)).

**KiCad is retired as a project tool.** The KiCad route required a GUI this project does not use, so
the `pcb/kicad/` scaffolding, the empty `pcb/gerbers/` and `pcb/ibom/` output placeholders, and the
`eda` CI job that installed `kicad-cli` (and only ever skipped) are removed. They were never used.

**The bridge is kept, KiCad is not required.** The flow still *exports* a `controller.kicad_pcb`
([`pcb/programmatic/out/`](../pcb/programmatic/out/)) and the netlist still emits a standard
`controller.net`, so anyone who *wants* KiCad can open the design — but it is **optional interchange**,
not part of the workflow.

## 2. What changed

| Removed | Reframed |
|---|---|
| `pcb/kicad/` (KiCad-entry README + placeholder) | `pcb/netlist/` README → tscircuit is the primary flow; `.net` is optional interchange |
| `pcb/gerbers/`, `pcb/ibom/` (empty KiCad-output dirs) | `pcb/fabrication/fab-notes.md` → fab package = `pcb/programmatic/out/`; dropped `kicad-cli` export recipe |
| `eda` CI job (`kicad-cli sch erc` / `pcb drc`) | WI-EE-03/04/07/09 statuses → "residual is real footprints + reviewed placement", tool = tscircuit |

The **committed electrical gate is unchanged**: the plain-Python netlist ERC
(`controller_netlist.py --selftest`) runs in the `BOM` CI job and enforces no-floating-nets,
no-double-driven-pins, fail-OFF pump gate, BOM coverage, and the firmware pin contract.

## 3. Rationale

- **Executable here.** tscircuit runs headless (npm/bun); KiCad's ERC/DRC/layout need the GUI.
- **One source of truth.** The netlist (`.py`) already drives both the ERC and the tscircuit board;
  KiCad would have been a second, manual capture to keep in sync.
- **Reversible.** The `.kicad_pcb` + `.net` exports mean KiCad can be re-adopted at any time without
  re-capture — so retiring it loses nothing.

## 4. Honest residual (unchanged by toolchain)

Whether the layout is finished in tscircuit or KiCad, a **fabricable** board still needs real
footprints (the module/connector footprints in the draft are `pinrowN` placeholders) and a **reviewed
power/analog/thermal placement** per [`design-rules.md`](design-rules.md). The autorouter removes the
GUI, not that engineering review. See [`pcb/programmatic/README.md`](../pcb/programmatic/README.md).

## 5. Project hand-off

Spec §7.9/§10.5/§16.1 and any docs that name **KiCad / `kicad-cli`** as the EDA tool now describe a
retired path; updating the spec/CI wording there is the Project & Repo track's to make. This ECO is
the electronics-side record.
