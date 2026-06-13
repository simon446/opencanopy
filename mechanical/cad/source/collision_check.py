#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
collision_check.py — Worst-case interference / clearance simulation (spec §12.1, §12.2).

Uses the **actual 3D models** (the same parametric parts that produce the released
STEP/STL), tessellated in their assembly-placed positions, and runs them through
**FCL** (the Flexible Collision Library, via python-fcl) — the same battle-tested
narrow-phase engine used in robotics (MoveIt). No hand-rolled geometry math.

For every pair of physical bodies it computes the exact minimum gap (and penetration
depth if they touch), then applies a **configurable manufacturing error margin** to
ask the worst-case question: *if every printed surface grew by `growth` and every
part shifted by `position`, would a clearance fit close up?*

    closure budget = (growthA + growthB) + (positionA + positionB)
    a clearance pair survives  <=>  measured_gap >= closure_budget

Fastened/resting joints (lid-on-rim, reservoir-on-rails, bolted mounts, …) are
expected to touch and are reported but never failed.

Run:
    .venv-cad/bin/python mechanical/cad/source/collision_check.py
    .venv-cad/bin/python mechanical/cad/source/collision_check.py --growth 0.4 --position 0.3
"""
import argparse
import tempfile
from itertools import combinations
from pathlib import Path

import numpy as np
import trimesh
import fcl
from build123d import export_stl

from opencanopy import assembly

# Pairs that are SUPPOSED to touch (rest on / bolt to / fasten into each other).
# Reported for information; never counted as a clearance failure.
EXPECTED_CONTACT = {
    frozenset(("reservoir", "leak-tray")),               # drawer rests on cradle rails
    frozenset(("electronics-dry-bay-lid", "electronics-dry-bay")),  # lid in rebate
    frozenset(("led-fixture", "light-mount")),           # fixture bolted to carrier
    frozenset(("light-mount", "frame")),                 # arms pin to uprights
    frozenset(("cable-channel", "frame")),               # channel mounts to upright
    frozenset(("status-diffuser", "frame")),             # diffuser on front rail
    frozenset(("fan-mount", "frame")),                   # bolted to rear rail
    frozenset(("pot", "pot-tray")),                      # pot seats on locating ring
    frozenset(("pot-tray", "frame")),                    # tray on mid rail
    frozenset(("leak-tray", "frame")),                   # tray in wet bay
    frozenset(("pump-clip", "reservoir")),               # pump clip inside reservoir
    frozenset(("pump-clip", "leak-tray")),
    frozenset(("cable-channel", "electronics-dry-bay")),  # channel feeds the bay
}

TESS_TOL = 0.1            # mesh deflection for collision (mm)
REPORT_NEAR = 12.0       # only print pairs closer than this (mm), plus all failures


def _fcl_obj(mesh: trimesh.Trimesh):
    m = fcl.BVHModel()
    m.beginModel(len(mesh.vertices), len(mesh.faces))
    m.addSubModel(np.asarray(mesh.vertices, dtype=np.float64),
                  np.asarray(mesh.faces, dtype=np.int64))
    m.endModel()
    return fcl.CollisionObject(m, fcl.Transform())


def _load_placed():
    """Export each placed part (real geometry, assembly coords) and load as a mesh."""
    objs = {}
    with tempfile.TemporaryDirectory() as td:
        for label, solid, _zone in assembly._placed():
            p = Path(td) / f"{label}.stl"
            export_stl(solid, str(p), tolerance=TESS_TOL, angular_tolerance=0.3)
            mesh = trimesh.load_mesh(str(p))
            objs[label] = (_fcl_obj(mesh), mesh)
    return objs


def _gap(o1, o2):
    res = fcl.DistanceResult()
    d = fcl.distance(o1, o2, fcl.DistanceRequest(), res)
    return d


def _penetration(o1, o2):
    res = fcl.CollisionResult()
    hit = fcl.collide(o1, o2, fcl.CollisionRequest(enable_contact=True, num_max_contacts=20), res)
    if not hit:
        return 0.0
    return max((c.penetration_depth for c in res.contacts), default=0.0)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--growth", type=float, default=0.30,
                    help="worst-case outward growth per printed surface (mm)")
    ap.add_argument("--position", type=float, default=0.20,
                    help="worst-case positional/warp shift per part (mm)")
    args = ap.parse_args(argv)

    budget = 2 * args.growth + 2 * args.position   # both bodies, both directions
    print(f"FCL worst-case interference sim | growth={args.growth} mm, "
          f"position={args.position} mm -> closure budget {budget:.2f} mm\n")

    objs = _load_placed()
    rows = []
    fails = 0
    for a, b in combinations(sorted(objs), 2):
        gap = _gap(objs[a][0], objs[b][0])
        contact = frozenset((a, b)) in EXPECTED_CONTACT
        if gap <= 1e-6:
            pen = _penetration(objs[a][0], objs[b][0])
            kind = "CONTACT" if contact else "COLLIDE"
            status = "ok" if contact else "FAIL"
            if not contact:
                fails += 1
            if gap <= 1e-6:
                rows.append((a, b, 0.0, pen, kind, status))
        else:
            if contact:
                rows.append((a, b, gap, 0.0, "near-contact", "ok"))
                continue
            margin = gap - budget
            status = "ok" if margin >= 0 else "FAIL"
            if margin < 0:
                fails += 1
            if gap < REPORT_NEAR or status == "FAIL":
                rows.append((a, b, gap, 0.0, "clearance", status))

    rows.sort(key=lambda r: (r[5] != "FAIL", r[2]))
    print(f"{'PART A':<24}{'PART B':<24}{'GAP':>7}{'PEN':>7}  {'KIND':<13}{'W/C':>5}")
    for a, b, gap, pen, kind, status in rows:
        wc = "" if kind in ("CONTACT", "near-contact") else f"{gap - budget:+.2f}"
        print(f"{a:<24}{b:<24}{gap:7.2f}{pen:7.2f}  {kind:<13}{status:>5} {wc}")

    print(f"\nbodies: {len(objs)}  pairs: {len(objs)*(len(objs)-1)//2}  "
          f"shown: {len(rows)}  clearance failures: {fails}")
    print("COLLISION SIM:", "PASS" if fails == 0 else "FAIL")
    return 0 if fails == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
