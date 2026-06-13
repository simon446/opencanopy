#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
build.py — Regenerate every released artifact from the parametric source.

Exports:
  * STEP (neutral interop) for the full assembly and each module -> ../step/
  * manifold STL for each printed part            -> ../stl/printable/
  * manifold STL for each tolerance coupon         -> ../stl/prototypes/

Bought/represented parts (pot, reservoir, LED fixture, aluminium frame) are exported
to STEP only; they are not printed. Run from anywhere:

    .venv-cad/bin/python mechanical/cad/source/build.py

Deterministic: same source -> byte-stable STEP/STL, so the repo diff is meaningful.
"""

import re
from pathlib import Path

from build123d import export_step, export_stl

# OpenCascade stamps the current wall-clock time into the STEP FILE_NAME header,
# which would make every re-export churn the file. Pin it so the build is
# byte-reproducible (same source -> identical STEP).
_EPOCH = "1970-01-01T00:00:00"
_TS_RE = re.compile(r"(FILE_NAME\('[^']*',')[^']*(')")


def export_step_reproducible(shape, path):
    export_step(shape, path)
    p = Path(path)
    p.write_text(_TS_RE.sub(rf"\g<1>{_EPOCH}\g<2>", p.read_text(), count=1))

from opencanopy import (assembly, frame, pot_reservoir as pr, electronics_bay as eb,
                        wet_bay as wb, light_mount as lm, fan_mount as fan, routing as rt)
from opencanopy.coupons import build_coupons

HERE = Path(__file__).resolve().parent
STEP = HERE.parent / "step"
PRINTABLE = HERE.parent.parent / "stl" / "printable"
PROTO = HERE.parent.parent / "stl" / "prototypes"
# Assembly-coordinate meshes (every part in its placed position). These are the
# input for render.py and collision_check.py — they let CI render/inspect the whole
# build WITHOUT an OpenCascade kernel. Coarser tessellation: visualization, not print.
ASSEMBLY = HERE.parent.parent / "stl" / "assembly"
for d in (STEP, PRINTABLE, PROTO, ASSEMBLY):
    d.mkdir(parents=True, exist_ok=True)

STL_TOL = 0.12          # linear deflection (mm) — fine enough for fit, modest file size
STL_ANG = 0.45          # angular deflection (rad)
ASM_TOL = 0.4           # coarser deflection for the placed visualization meshes
ASM_ANG = 0.6

# Printed release parts -> printable STL + STEP.
PRINTED = [
    eb.build_dry_bay(), eb.build_dry_bay_lid(),
    wb.build_leak_tray(), wb.build_pump_clip(), wb.build_pot_tray(),
    lm.build_light_mount(),
    fan.build_fan_mount(),
    rt.build_cable_channel(), rt.build_tube_clip(), rt.build_cable_clip(),
    rt.build_sensor_clip(), rt.build_status_diffuser(),
]

# Bought / represented parts -> STEP only.
REPRESENTED = [
    frame.build_frame(), pr.build_pot(), pr.build_reservoir(), lm.build_led_fixture(),
]


def main():
    n_step = n_stl = 0

    asm = assembly.build_assembly()
    export_step_reproducible(asm, str(STEP / "opencanopy-assembly.step"))
    n_step += 1

    for p in PRINTED:
        export_step_reproducible(p, str(STEP / f"{p.label}.step"))
        export_stl(p, str(PRINTABLE / f"{p.label}.stl"),
                   tolerance=STL_TOL, angular_tolerance=STL_ANG)
        n_step += 1
        n_stl += 1

    for p in REPRESENTED:
        export_step_reproducible(p, str(STEP / f"{p.label}.step"))
        n_step += 1

    for p in build_coupons():
        export_step_reproducible(p, str(STEP / f"{p.label}.step"))
        export_stl(p, str(PROTO / f"{p.label}.stl"),
                   tolerance=STL_TOL, angular_tolerance=STL_ANG)
        n_step += 1
        n_stl += 1

    # placed assembly meshes (coarse) for rendering / collision, one per body
    n_asm = 0
    for label, solid, _zone in assembly._placed():
        export_stl(solid, str(ASSEMBLY / f"{label}.stl"),
                   tolerance=ASM_TOL, angular_tolerance=ASM_ANG)
        n_asm += 1

    print(f"exported {n_step} STEP, {n_stl} STL, {n_asm} placed assembly meshes")
    # report the §12.1 checks alongside the build
    sz = assembly.envelope_report()
    assembly.assert_zone_separation()
    cg = assembly.cg_report()
    print(f"envelope OK: {sz.X:.0f}x{sz.Y:.0f}x{sz.Z:.0f} mm | "
          f"CG {cg['cg_z']:.0f} mm ({cg['frac_h']*100:.0f}% H), ~{cg['total_kg']:.1f} kg full")


if __name__ == "__main__":
    main()
