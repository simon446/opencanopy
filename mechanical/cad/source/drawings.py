#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
drawings.py — Orthographic line drawings of the assembly -> ../../drawings/*.svg.

Generates front / right / top views from the parametric model so the assembly
references stay in sync with the source. Run:

    .venv-cad/bin/python mechanical/cad/source/drawings.py
"""
from pathlib import Path

from build123d import ExportSVG, Color, LineType

from opencanopy import assembly, params as P

OUT = Path(__file__).resolve().parents[2] / "drawings"
OUT.mkdir(parents=True, exist_ok=True)

CX = P.ENV_W / 2
CY = P.ENV_D / 2
CZ = P.ENV_H / 2
FAR = 4000.0

VIEWS = {
    "assembly-front": dict(origin=(CX, -FAR, CZ), up=(0, 0, 1)),    # looking +Y
    "assembly-right": dict(origin=(FAR, CY, CZ), up=(0, 0, 1)),     # looking -X
    "assembly-top":   dict(origin=(CX, CY, FAR), up=(0, 1, 0)),     # looking -Z
}


def main():
    asm = assembly.build_assembly()
    n = 0
    for name, v in VIEWS.items():
        visible, hidden = asm.project_to_viewport(
            v["origin"], viewport_up=v["up"], look_at=(CX, CY, CZ))
        exp = ExportSVG(scale=0.42, margin=12)
        exp.add_layer("hidden", line_color=Color(0.6, 0.6, 0.6),
                      line_weight=0.18, line_type=LineType.DASHED)
        exp.add_layer("visible", line_color=Color(0, 0, 0), line_weight=0.35)
        exp.add_shape(hidden, layer="hidden")
        exp.add_shape(visible, layer="visible")
        exp.write(str(OUT / f"{name}.svg"))
        n += 1
        print(f"wrote {name}.svg  ({len(visible)} visible / {len(hidden)} hidden edges)")
    print(f"exported {n} drawings")


if __name__ == "__main__":
    main()
