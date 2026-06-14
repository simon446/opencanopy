#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
physics_sim.py — free-standing settling test of the v1 assembly (MuJoCo).

ONLY the ground is fixed. The whole unit rests on its four feet on the ground under
gravity; nothing else is pinned to the world. Each part is a rigid body with realistic
mass; the joints are modelled as dowel + screw `connect` constraints, with welds for the
seated parts (pot->shelf->base, LED->bridge). The feet contact the ground (friction).

Two questions after settling:
  * STABILITY — does the free-standing unit sink or tip?  (base displacement + tilt)
  * STRUCTURE — does any part move relative to the base?  (joint integrity)
Acceptance (brief): no part moves > 0.5 mm or rotates > 0.5 deg after settling; we also
remove each screw one at a time (dowels/tabs retained) to confirm the dowels carry shear.

Idealised rigid-body + pin/weld constraints with ground contact (not FEA): validates
free-standing stability and joint redundancy, not material stress.

    .venv-cad/bin/python mechanical/cad/physics_sim.py
"""
import sys
from pathlib import Path
import numpy as np, trimesh, mujoco

PARTS = Path(__file__).resolve().parent / "exports" / "parts"
# part -> mass (kg). base bundles shell + electronics + reservoir+water; pot = wet pot+media+plant
MASS = {"base":8.0, "pot":10.0, "shelf":0.45, "left_arch":0.65, "right_arch":0.65,
        "bridge":0.55, "led_bar":0.70}
FOOT_XY = [(44,44),(436,44),(44,276),(436,276)]   # foot centres (foot_inset=40 -> +-44)
FOOT_R, FOOT_H = 16, 14

def part(name):
    m = trimesh.load_mesh(str(PARTS/f"{name}.stl"))
    return m.centroid, np.maximum((m.bounds[1]-m.bounds[0])/2, 3.0)

FEET = {"left_arch":13, "right_arch":467}; YF=(24,296)
BRIDGE = {"left_arch":26, "right_arch":454}
STIFF = 'solref="0.003 1" solimp="0.995 0.999 0.001"'

def qconj(q): return np.array([q[0],-q[1],-q[2],-q[3]])
def qmul(a,b):
    w0,x0,y0,z0=a; w1,x1,y1,z1=b
    return np.array([w0*w1-x0*x1-y0*y1-z0*z1, w0*x1+x0*w1+y0*z1-z0*y1,
                     w0*y1-x0*z1+y0*w1+z0*x1, w0*z1+x0*y1-y0*x1+z0*w1])
def qrot(q,v): return qmul(qmul(q,np.array([0.0,*v])), qconj(q))[1:]


def build_mjcf():
    P = {n: part(n) for n in MASS}
    bx, by, bz = P["base"][0]
    bodies = ""
    for n,(c,h) in P.items():
        if n=="base":
            feet = "".join(
                f'<geom type="cylinder" pos="{(fx-bx)/1000} {(fy-by)/1000} {(FOOT_H/2-bz)/1000}" '
                f'size="{FOOT_R/1000} {FOOT_H/2000}" mass="0.05" contype="1" conaffinity="1" '
                f'friction="1.2 0.01 0.001" {STIFF} rgba="0.4 0.4 0.42 1"/>' for fx,fy in FOOT_XY)
            bodies += (f'<body name="base" pos="{bx/1000} {by/1000} {bz/1000}"><freejoint/>'
                       f'<geom type="box" size="{h[0]/1000} {h[1]/1000} {h[2]/1000}" mass="{MASS[n]-0.2}" '
                       f'contype="0" conaffinity="0" rgba="0.9 0.9 0.88 1"/>{feet}</body>\n')
        else:
            bodies += (f'<body name="{n}" pos="{c[0]/1000} {c[1]/1000} {c[2]/1000}"><freejoint/>'
                       f'<geom type="box" size="{h[0]/1000} {h[1]/1000} {h[2]/1000}" mass="{MASS[n]}" '
                       f'contype="0" conaffinity="0" rgba="0.7 0.7 0.7 1"/></body>\n')
    eqs, screws = "", []
    def conn(name, b1, b2, anchor):
        a = (np.array(anchor) - P[b1][0]) / 1000.0
        return f'<connect name="{name}" body1="{b1}" body2="{b2}" anchor="{a[0]} {a[1]} {a[2]}" {STIFF}/>\n'
    for nm,a,b in [("w_potshelf","pot","shelf"),("w_shelfbase","shelf","base"),("w_ledbridge","led_bar","bridge")]:
        eqs += f'<weld name="{nm}" body1="{a}" body2="{b}" {STIFF}/>\n'
    for arch,x in FEET.items():
        for j,y in enumerate(YF):
            for k,(kind,anc) in enumerate([("dwl",(x-7,y,114)),("dwl",(x+7,y,114)),("scr",(x,y,108))]):
                nm=f"{kind}_{arch}_{j}{k}"; eqs += conn(nm, arch, "base", anc)
                if kind=="scr": screws.append(nm)
    for arch,x in BRIDGE.items():
        for k,(kind,anc) in enumerate([("dwl",(x,148,628)),("dwl",(x,172,628)),("scr",(x,160,640))]):
            nm=f"{kind}_brg_{arch}_{k}"; eqs += conn(nm, "bridge", arch, anc)
            if kind=="scr": screws.append(nm)
    xml = (f'<mujoco><option gravity="0 0 -9.81" timestep="0.001" iterations="200"/>'
           f'<worldbody><geom name="ground" type="plane" size="3 3 0.1" '
           f'contype="1" conaffinity="1" friction="1.2 0.01 0.001"/>\n{bodies}</worldbody>'
           f'<equality>\n{eqs}</equality></mujoco>')
    return xml, screws


def settle(model, data, disabled):
    mujoco.mj_resetData(model, data)
    for i in range(model.neq):
        nm = mujoco.mj_id2name(model, mujoco.mjtObj.mjOBJ_EQUALITY, i)
        data.eq_active[i] = 0 if nm in disabled else 1
    mujoco.mj_forward(model, data)
    bid = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_BODY, "base")
    # initial pose-relative-to-base for each part
    p0, q0 = data.xpos.copy(), data.xquat.copy()
    rel0 = {i: qrot(qconj(q0[bid]), p0[i]-p0[bid]) for i in range(1, model.nbody)}
    for _ in range(8000):
        mujoco.mj_step(model, data)
    # base stability
    base_settle = np.linalg.norm(data.xpos[bid]-p0[bid])*1000.0
    base_tilt = 2*np.degrees(np.arccos(min(1.0, abs(data.xquat[bid][0]))))
    # per-part motion RELATIVE to the base
    maxd = maxr = 0.0
    qb = data.xquat[bid]
    for i in range(1, model.nbody):
        if i==bid: continue
        rp = qrot(qconj(qb), data.xpos[i]-data.xpos[bid])
        d = np.linalg.norm(rp - rel0[i])*1000.0
        rq = qmul(qconj(qb), data.xquat[i]); a = 2*np.degrees(np.arccos(min(1.0, abs(rq[0]))))
        maxd=max(maxd,d); maxr=max(maxr,a)
    return base_settle, base_tilt, maxd, maxr


def main():
    xml, screws = build_mjcf()
    model = mujoco.MjModel.from_xml_string(xml)
    data = mujoco.MjData(model)

    def run(label, disabled):
        bs, bt, md, mr = settle(model, data, disabled)
        ok = md<=0.5 and mr<=0.5 and bt<=0.5
        print(f"{label:<32} base settle {bs:5.3f}mm tilt {bt:5.3f}deg | part rel {md:5.3f}mm {mr:5.3f}deg  {'PASS' if ok else 'FAIL'}")
        return ok

    print(f"MuJoCo free-standing test — ground fixed only, unit on 4 feet, pot {MASS['pot']:.0f} kg, gravity on")
    print(f"{model.nbody-1} bodies, {model.neq} constraints ({len(screws)} screws); total ~{sum(MASS.values()):.1f} kg\n")
    allok = run("baseline (all screws+dowels)", set())
    for s in screws: allok &= run(f"screw removed: {s}", {s})
    allok &= run("ALL screws removed (dowels only)", set(screws))
    print(f"\nFREE-STANDING TEST: {'PASS' if allok else 'FAIL'} — unit is stable on its feet and "
          f"the dowels/tabs carry shear (holds < 0.5 mm / 0.5 deg without screws)")
    return 0 if allok else 1


if __name__ == "__main__":
    sys.exit(main())
