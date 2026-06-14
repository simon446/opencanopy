#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
physics_sim.py — rigid-body settling test of the v1 assembly (MuJoCo).

Models each part as a rigid body with realistic mass; the BASE is fixed to the world;
the joints are modelled as constraints: dowel pins + screws as `connect` (point) equality
constraints at the real foot/bridge anchors, and seated parts (pot->shelf->base, LED->bridge)
as `weld` equalities. Gravity acts; the assembly settles; we measure each part's
displacement (mm) and rotation (deg) from its built pose.

Acceptance (per the brief): no part moves > 0.5 mm or rotates > 0.5 deg after settling.
We then remove each SCREW constraint one at a time (dowels/tabs retained) to verify the
dowels carry the shear — i.e. the joint is not screw-dependent for holding.

This is an idealised rigid-body + pin/weld-constraint model (not FEA): it validates
kinematic stability and joint redundancy, not material stress.

    .venv-cad/bin/python mechanical/cad/physics_sim.py
"""
import sys
from pathlib import Path
import numpy as np, trimesh, mujoco

PARTS = Path(__file__).resolve().parent / "exports" / "parts"
# part -> mass (kg). Pot = wet pot + media + plant (mid of the 8-12 kg band).
MASS = {"base":5.0, "pot":10.0, "shelf":0.45, "left_arch":0.65, "right_arch":0.65,
        "bridge":0.55, "led_bar":0.70}

def part(name):
    m = trimesh.load_mesh(str(PARTS / f"{name}.stl"))
    c = m.centroid
    sz = (m.bounds[1] - m.bounds[0])
    return c, np.maximum(sz/2, 3.0)   # half-extents (mm), clamped

# --- joint anchors (world mm) -------------------------------------------------
# arch feet: x, and Y of front/back foot; dowels at x+-7, screw at x
FEET = {"left_arch": 13, "right_arch": 467}
YF = (24, 296)
def foot_anchors(x):
    a = []
    for y in YF:
        a += [("dwl",(x-7,y,114)), ("dwl",(x+7,y,114)), ("scr",(x,y,108))]
    return a
BRIDGE = {"left_arch": 26, "right_arch": 454}
def bridge_anchors(x):
    return [("dwl",(x,148,628)), ("dwl",(x,172,628)), ("scr",(x,160,640))]

STIFF = 'solref="0.002 1" solimp="0.995 0.999 0.001"'


def build_mjcf():
    P = {n: part(n) for n in MASS}
    bodies = ""
    for n,(c,h) in P.items():
        free = "" if n=="base" else "<freejoint/>"
        bodies += (f'<body name="{n}" pos="{c[0]/1000} {c[1]/1000} {c[2]/1000}">{free}'
                   f'<geom type="box" size="{h[0]/1000} {h[1]/1000} {h[2]/1000}" '
                   f'mass="{MASS[n]}" contype="0" conaffinity="0" rgba="0.7 0.7 0.7 1"/></body>\n')
    eqs, screws = "", []
    def connect(name, b1, anchor):
        c1 = P[b1][0]
        a = (np.array(anchor) - c1)/1000.0
        return f'<connect name="{name}" body1="{b1}" body2="base" anchor="{a[0]} {a[1]} {a[2]}" {STIFF}/>\n'
    # welds for seated parts
    for nm,b1,b2 in [("w_potshelf","pot","shelf"),("w_shelfbase","shelf","base"),("w_ledbridge","led_bar","bridge")]:
        eqs += f'<weld name="{nm}" body1="{b1}" body2="{b2}" {STIFF}/>\n'
    # arch feet -> base
    for arch,x in FEET.items():
        for i,(kind,anc) in enumerate(foot_anchors(x)):
            nm=f"{kind}_{arch}_{i}"; eqs += connect(nm, arch, anc)
            if kind=="scr": screws.append(nm)
    # bridge -> arches
    for arch,x in BRIDGE.items():
        for i,(kind,anc) in enumerate(bridge_anchors(x)):
            nm=f"{kind}_brg_{arch}_{i}"
            c1=P["bridge"][0]; a=(np.array(anc)-c1)/1000.0
            # bridge connects to the arch (body2=arch)
            eqs += f'<connect name="{nm}" body1="bridge" body2="{arch}" anchor="{a[0]} {a[1]} {a[2]}" {STIFF}/>\n'
            if kind=="scr": screws.append(nm)
    xml = (f'<mujoco><option gravity="0 0 -9.81" timestep="0.002" iterations="200"/>'
           f'<worldbody>\n{bodies}</worldbody><equality>\n{eqs}</equality></mujoco>')
    return xml, list(MASS), screws


def settle(model, data, disabled):
    mujoco.mj_resetData(model, data)
    for nm in [mujoco.mj_id2name(model, mujoco.mjtObj.mjOBJ_EQUALITY, i) for i in range(model.neq)]:
        idx = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_EQUALITY, nm)
        data.eq_active[idx] = 0 if nm in disabled else 1
    mujoco.mj_forward(model, data)
    pos0 = data.xpos.copy()
    for _ in range(5000):
        mujoco.mj_step(model, data)
    # per-body displacement (mm) and rotation (deg)
    res = {}
    for i in range(1, model.nbody):
        nm = mujoco.mj_id2name(model, mujoco.mjtObj.mjOBJ_BODY, i)
        d_mm = np.linalg.norm(data.xpos[i] - pos0[i]) * 1000.0
        q = data.xquat[i]; ang = 2*np.degrees(np.arccos(min(1.0, abs(q[0]))))
        res[nm] = (d_mm, ang)
    return res


def main():
    xml, parts, screws = build_mjcf()
    model = mujoco.MjModel.from_xml_string(xml)
    data = mujoco.MjData(model)

    def run(label, disabled):
        r = settle(model, data, disabled)
        md = max(v[0] for v in r.values()); ma = max(v[1] for v in r.values())
        ok = md <= 0.5 and ma <= 0.5
        print(f"{label:<34} max disp {md:5.3f} mm  max rot {ma:5.3f} deg  {'PASS' if ok else 'FAIL'}")
        return ok, md, ma

    print(f"MuJoCo settling test — base fixed, pot {MASS['pot']:.0f} kg, gravity on")
    print(f"{model.nbody-1} bodies, {model.neq} joint constraints ({len(screws)} screws)\n")
    allok = True
    ok,_,_ = run("baseline (all screws + dowels)", set()); allok &= ok
    for s in screws:                      # remove each screw one at a time
        ok,_,_ = run(f"screw removed: {s}", {s}); allok &= ok
    ok,_,_ = run("ALL screws removed (dowels/tabs only)", set(screws)); allok &= ok
    print(f"\nSETTLING TEST: {'PASS' if allok else 'FAIL'} "
          f"(dowels/tabs carry shear: assembly holds < 0.5 mm / 0.5 deg without screws)")
    return 0 if allok else 1


if __name__ == "__main__":
    sys.exit(main())
