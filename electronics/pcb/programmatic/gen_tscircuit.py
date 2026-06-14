#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
gen_tscircuit.py — emit a tscircuit board (.tsx) from the controller netlist.

This is the bridge for the **headless / code-only PCB flow** (no KiCad GUI): it reads the schematic
source of truth (`../netlist/controller_netlist.py`) and writes a tscircuit board following the
official tscircuit *skill* idioms (github.com/tscircuit/skill):

  * real elements for passives (`<resistor>`/`<capacitor>`/`<inductor>`/`<diode>`/`<mosfet>`/`<fuse>`)
  * `<chip>` with `pinLabels` for ICs/connectors/module
  * connectivity by **named nets** — every pin → `<trace from="REF.PIN" to="net.NET" />`

Then `tsci build` / `tsci export -f gerbers` (see build.sh) auto-places, autoroutes, and emits
Gerbers + PnP + BOM with no GUI.

HONESTY / SCOPE (read before trusting the output):
  * Only **populated** controller-board parts are emitted (DNP options omitted; status LED board is a
    separate PCB).
  * Passive footprints (0402/0805/1206/1210/2512) are real. **IC / connector / module footprints are
    single-row `pinrowN` placeholders** — correct pin COUNT, not the real land pattern. Swap in real
    footprints (`tsci search`/`import`, or KiCad libs) before fab.
  * Auto-placement + autoroute is **autorouter-grade**: it honours the netlist but NOT the
    power/analog separation, copper pours, or star-ground in `../../analysis/design-rules.md`.
  * Net/pin names are sanitised for tscircuit selectors (`+`->`P`, `-`->`N`, `/`->`_`).

So the output is a real, fab-FORMAT board generated entirely from code — a draft to review/refine,
not a drop-in final layout.

Run:  python3 gen_tscircuit.py            # writes controller.circuit.tsx
"""
from __future__ import annotations

import importlib.util
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
NETLIST_PY = HERE.parent / "netlist" / "controller_netlist.py"
OUT_TSX = HERE / "controller.circuit.tsx"


def _load_netlist():
    spec = importlib.util.spec_from_file_location("controller_netlist", NETLIST_PY)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def net_name(n: str) -> str:
    s = n.replace("+", "P").replace("-", "N").replace("/", "_").replace(".", "_")
    s = re.sub(r"[^0-9A-Za-z_]", "_", s)
    return s if re.match(r"[A-Za-z_]", s) else "N" + s


def pin_id(p: str) -> str:
    s = p.replace("+", "P").replace("-", "N").replace("/", "_").replace(".", "_")
    return re.sub(r"[^0-9A-Za-z_]", "_", s)


PASSIVE_FP = {
    "C_0402": "0402", "C_0805": "0805", "C_1206": "1206", "C_1210": "1210",
    "CP_Elec_8x10": "1210", "R_0402": "0402", "R_2512": "2512", "L_12x12": "1210",
}

# Functional subsystems (for per-block schematic renders / docs). Every populated controller part
# belongs to exactly one. A part connects to nets outside its block via labelled net stubs.
SUBSYSTEMS = {
    "power-input": ["J_PWR", "F1", "Q2", "R_RP", "DZ_RP", "D1", "C_BULK1", "C_BULK2", "C_BULK3"],
    "power-rails": ["U7", "L5", "D_CAT5", "C_IN5", "C_BT5", "C_OUT5", "R_FB5A", "R_FB5B",
                    "U8", "C_IN33", "C_OUT33"],
    "mcu": ["U1", "C1", "C2", "C3", "R_EN", "C_EN", "SW1", "SW2", "J_USB", "J_DBG"],
    "i2c-rtc-current": ["R_SDA", "R_SCL", "U3", "C_RTC", "BT1", "U4", "C_INA", "R_SHUNT", "J_SENS"],
    "moisture-reservoir": ["R_MOIST", "C_MOIST", "J_MOIST", "J_RES"],
    "leak-detect": ["U5", "C_U5", "R_LK_PU", "R_LK_RA", "R_LK_RB", "R_LK_OUT", "C_LK", "J_LEAK"],
    "pump-drive": ["Q1", "R1", "R2", "D2", "J_PUMP"],
    "led-status": ["R_DIM", "J_LED", "R_DATA", "J_STATUS"],
}


def elem_type(ref: str) -> str:
    if ref.startswith("LED"):
        return "chip"
    return {"C": "capacitor", "R": "resistor", "L": "inductor", "D": "diode", "Q": "mosfet"}.get(
        ref[0], "fuse" if ref == "F1" else "chip"
    )


def val_num(v: str) -> str:
    # "100R"->"100", "0.1R"->"0.1", "4.7k"->"4.7k", "100nF"->"100nF"
    return v[:-1] if v.endswith("R") else v


def emit_component(ref, c, pins) -> tuple[str, dict]:
    """Return (jsx, pin_selector_map). pin_selector_map: original-pin -> 'REF.sel'."""
    t = elem_type(ref)
    npins = len(pins)
    if t == "resistor":
        return f'  <resistor name="{ref}" resistance="{val_num(c.value)}" footprint="{PASSIVE_FP.get(c.footprint, "0402")}" />', \
            {p: f"{ref}.{ {'1':'pin1','2':'pin2'}.get(p, p) }".replace(" ", "") for p in pins}
    if t == "capacitor":
        pol = " polarized" if c.footprint.startswith("CP_") else ""
        return f'  <capacitor name="{ref}" capacitance="{c.value}" footprint="{PASSIVE_FP.get(c.footprint, "0402")}"{pol} />', \
            {p: f"{ref}." + {"1": "pin1", "2": "pin2", "+": "pin1", "-": "pin2"}.get(p, p) for p in pins}
    if t == "inductor":
        return f'  <inductor name="{ref}" inductance="{c.value}" footprint="{PASSIVE_FP.get(c.footprint, "1210")}" />', \
            {p: f"{ref}." + {"1": "pin1", "2": "pin2"}.get(p, p) for p in pins}
    if t == "diode":
        return f'  <diode name="{ref}" footprint="sod123" />', \
            {p: f"{ref}." + {"A": "anode", "K": "cathode"}.get(p, p) for p in pins}
    if t == "mosfet":
        ch = "p" if ref == "Q2" else "n"
        return f'  <mosfet name="{ref}" channelType="{ch}" mosfetMode="enhancement" footprint="sot23" />', \
            {p: f"{ref}." + {"G": "gate", "S": "source", "D": "drain"}.get(p, p) for p in pins}
    if t == "fuse":
        return f'  <fuse name="{ref}" currentRating="6.3A" voltageRating="32V" footprint="1206" />', \
            {p: f"{ref}.{ {'1':'pin1','2':'pin2'}.get(p, p) }".replace(" ", "") for p in pins}
    # chip: pinLabels keyed pin1..N -> sanitised label; selectors use the label
    ordered = sorted(pins)
    labels = {f"pin{i+1}": pin_id(p) for i, p in enumerate(ordered)}
    sel = {p: f"{ref}.{pin_id(p)}" for p in ordered}
    lbl = ", ".join(f'{k}: "{v}"' for k, v in labels.items())
    return f'  <chip name="{ref}" footprint="pinrow{npins}" pinLabels={{{{{lbl}}}}} />', sel


def emit_board(ref_filter=None, width="140mm", height="100mm") -> str:
    """Emit a tscircuit <board>. ref_filter=None -> whole controller; a set -> just that subset
    (a subsystem block), with every pin wired to a labelled `net.<NET>` so cross-block I/O shows as
    stubs. Connectivity is taken verbatim from the netlist, so blocks can't drift from the schematic."""
    nl = _load_netlist()
    nets = nl._norm_pins()
    comps = nl.COMPONENTS

    def populated_ctrl(ref):
        c = comps.get(ref)
        return c and c.board == "ctrl" and c.populated

    def included(ref):
        return populated_ctrl(ref) and (ref_filter is None or ref in ref_filter)

    # nets that are real connections on the FULL board (>=2 populated pins) — drives which stubs exist
    full_active = {net for net, pins in nets.items()
                   if sum(populated_ctrl(p.split(".", 1)[0]) for p in pins) >= 2}

    pins_by_ref: dict[str, set] = {}
    for net, pins in nets.items():
        for p in pins:
            ref, _, pin = p.partition(".")
            if included(ref):
                pins_by_ref.setdefault(ref, set()).add(pin)

    comp_jsx, sel_map = [], {}
    for ref in sorted(pins_by_ref):
        jsx, sels = emit_component(ref, comps[ref], pins_by_ref[ref])
        comp_jsx.append(jsx)
        sel_map.update({(ref, p): s for p, s in sels.items()})

    trace_jsx = []
    for net, pins in nets.items():
        if net not in full_active:
            continue
        nn = net_name(net)
        for p in pins:
            ref, _, pin = p.partition(".")
            if included(ref):
                trace_jsx.append(f'  <trace from="{sel_map[(ref, pin)]}" to="net.{nn}" />')

    body = "\n".join(comp_jsx) + "\n\n" + "\n".join(trace_jsx)
    return f"""// AUTO-GENERATED from electronics/pcb/netlist/controller_netlist.py by gen_tscircuit.py.
// Headless code->PCB flow (tscircuit). Draft: IC/connector/module footprints are pinrowN
// placeholders; autoroute does not encode design-rules.md. See README.md. Do not hand-edit.
export default () => (
  <board width="{width}" height="{height}" layers={{2}}>
{body}
  </board>
)
"""


if __name__ == "__main__":
    import sys
    nl = _load_netlist()
    if "--subsystems" in sys.argv:
        outdir = HERE / "blocks"
        outdir.mkdir(exist_ok=True)
        for name, refs in SUBSYSTEMS.items():
            (outdir / f"{name}.circuit.tsx").write_text(
                emit_board(set(refs), width="60mm", height="50mm"), encoding="utf-8")
        # coverage check: every populated ctrl part is in exactly one subsystem
        mapped = {r for refs in SUBSYSTEMS.values() for r in refs}
        allpop = {r for r, c in nl.COMPONENTS.items() if c.board == "ctrl" and c.populated}
        miss = allpop - mapped
        if miss:
            print(f"WARNING: unmapped populated parts: {sorted(miss)}")
        print(f"wrote {len(SUBSYSTEMS)} subsystem blocks to {outdir.name}/ "
              f"({len(allpop)} parts, {len(allpop - miss)} mapped)")
    else:
        OUT_TSX.write_text(emit_board(width="110mm", height="75mm"), encoding="utf-8")
        n_comp = sum(1 for c in nl.COMPONENTS.values() if c.board == "ctrl" and c.populated)
        print(f"wrote {OUT_TSX.name} ({n_comp} populated controller parts)")
