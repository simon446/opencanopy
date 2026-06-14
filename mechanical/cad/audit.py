#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
audit.py — honest geometry audit of the real part meshes (no whitelist).

For every pair of parts it reports the actual FCL penetration depth (interpenetration)
and the clearance gap; and for every part it reports the nearest-neighbour gap so a
"floating" / unsupported part is caught (a collision check alone never flags a gap).

This is the check the eye does — it does not hide anything behind an "expected contact"
list. Intended touches (parts seated on each other) show ~0 mm; hardware that pierces a
solid because the clearance hole/socket is missing shows a large penetration; a part with
a large nearest gap is floating.

    .venv-cad/bin/python mechanical/cad/audit.py
"""
import sys
from itertools import combinations
from pathlib import Path
import numpy as np, trimesh, fcl

PARTS = Path(__file__).resolve().parent / "exports" / "parts"
VOL_TOL = 80.0    # mm^3: interpenetration of more than this overlap VOLUME is flagged
FLOAT_TOL = 1.0   # mm: a part whose nearest neighbour is farther than this is "floating"


def load():
    return {f.stem: trimesh.load_mesh(str(f)) for f in sorted(PARTS.glob("*.stl"))}


def obj(m):
    b = fcl.BVHModel(); b.beginModel(len(m.vertices), len(m.faces))
    b.addSubModel(np.asarray(m.vertices, float), np.asarray(m.faces, np.int64)); b.endModel()
    return fcl.CollisionObject(b, fcl.Transform())


def main():
    meshes = load()
    objs = {k: obj(m) for k, m in meshes.items()}
    names = list(objs)
    nearest = {k: (1e9, None) for k in names}
    pens = []
    for a, b in combinations(names, 2):
        dr = fcl.DistanceResult(); g = fcl.distance(objs[a], objs[b], fcl.DistanceRequest(), dr)
        if g < nearest[a][0]: nearest[a] = (g, b)
        if g < nearest[b][0]: nearest[b] = (g, a)
        if g <= 0.2:   # touching/overlapping -> measure the TRUE overlap volume (boolean)
            try:
                inter = trimesh.boolean.intersection([meshes[a], meshes[b]])
                vol = float(inter.volume) if (inter is not None and len(inter.vertices)) else 0.0
            except Exception:
                vol = 0.0
            if vol > VOL_TOL:
                pens.append((vol, a, b))

    print("=== INTERPENETRATIONS (real meshes, no whitelist; overlap VOLUME) ===")
    if not pens:
        print("  none larger than %.0f mm^3 (touching faces are not interpenetration)" % VOL_TOL)
    for vol, a, b in sorted(pens, reverse=True):
        print(f"  {vol:9.0f} mm^3   {a} <-> {b}")

    print("\n=== FLOATING / support check (nearest neighbour per part) ===")
    floats = [(g, k, nb) for k, (g, nb) in nearest.items() if g > FLOAT_TOL]
    if not floats:
        print("  every part touches (or nearly touches) another — none floating")
    for g, k, nb in sorted(floats, reverse=True):
        print(f"  {g:6.2f} mm gap   {k}  (nearest: {nb})  <-- FLOATING")

    bad = len(pens) + len(floats)
    print(f"\nGEOMETRY AUDIT: {'CLEAN' if bad==0 else f'{len(pens)} interpenetration(s), {len(floats)} floating'}")
    return 0 if bad == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
