#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
render_block.py — export, render and validate the OpenSCAD v1 product model.

Exports each part (OpenSCAD CGAL), renders with VTK (z-buffered, smooth), and runs an
FCL part-vs-part collision check. Two render groups:
  * PRODUCT views  — material colours (white shell / wood / dark service / water).
  * VALIDATION views — debug colours: exploded, underside screw-access, base cutaway,
    right-rear-arch conduit cross-section, and a TOP view with a crosshair overlay
    proving the LED optical centerline and the pot are both at X=240, Y=160.

Usage:  .venv-cad/bin/python mechanical/cad/render_block.py
"""
import subprocess, sys
from itertools import combinations
from pathlib import Path
import numpy as np, trimesh, fcl, vtk

HERE = Path(__file__).resolve().parent
SCAD = HERE / "opencanopy_tabletop_pepper_v1_block_model.scad"
PARTS = HERE / "exports" / "parts"
RENDERS = HERE.parent.parent / "docs" / "assets" / "renders"
PARTS.mkdir(parents=True, exist_ok=True); RENDERS.mkdir(parents=True, exist_ok=True)
OPENSCAD = next((p for p in ("/Applications/OpenSCAD.app/Contents/MacOS/OpenSCAD","openscad")
                 if Path(p).exists() or p=="openscad"), "openscad")
ENV_W, SIDE_T = 480, 26
CX, CY = 240, 160                      # pot + LED optical centerline

WHITE=(0.90,0.90,0.89); WOOD=(0.78,0.60,0.36); DARK=(0.24,0.24,0.27); WATER=(0.30,0.62,0.88)
# part -> (label, product colour, debug colour, explode vector)
P = {
 "left_arch":  ("left side-frame arch", WHITE, (0.55,0.70,0.95), (-130,0,0)),
 "right_arch": ("right side-frame arch",WHITE, (0.55,0.70,0.95), (130,0,0)),
 "base":       ("base shell",           WHITE, (0.85,0.85,0.88), (0,0,-90)),
 "bridge":     ("top light bridge",     WHITE, (0.70,0.80,0.95), (0,0,150)),
 "shelf":      ("wood shelf + pot well",WOOD,  WOOD,              (0,0,70)),
 "led_bar":    ("LED grow bar",         (0.97,0.92,0.5),(0.97,0.85,0.2),(0,0,110)),
 "pot":        ("pot (~9.5 L)",         (0.80,0.55,0.42),(0.80,0.55,0.42),(0,0,150)),
 "reservoir":  ("reservoir (water)",    WATER, WATER,             (0,-120,0)),
 "pcb":        ("controller PCB",       DARK,  (0.15,0.7,0.3),    (130,40,0)),
 "driver":     ("LED driver",           DARK,  (0.9,0.5,0.2),     (150,0,0)),
 "power":      ("power input (24 V)",   DARK,  (0.7,0.7,0.2),     (150,-40,0)),
 "iso_wall":   ("sealed wet|dry wall",  (0.86,0.88,0.92),(0.95,0.4,0.4),(0,80,0)),
 "status":     ("status pill (4 LEDs)", (0.30,0.82,0.55),(0.30,0.82,0.55),(0,-60,0)),
 "feet":       ("feet (x4)",            (0.55,0.55,0.58),(0.55,0.55,0.58),(0,0,-130)),
 "dowels":     ("dowel pins",           (0.60,0.60,0.62),(0.1,0.6,0.9),  (0,0,-40)),
 "screws":     ("M4/M3 screws",         (0.20,0.20,0.22),(0.95,0.2,0.2), (0,0,-70)),
 "cable":      ("cable path",           (0.9,0.6,0.2),(0.9,0.6,0.2),     (0,0,0)),
}
EXPECTED_CONTACT = {frozenset(p) for p in [
 ("left_arch","base"),("right_arch","base"),("bridge","left_arch"),("bridge","right_arch"),
 ("shelf","base"),("pot","shelf"),("led_bar","bridge"),("feet","base"),("iso_wall","base"),
 ("status","base"),("dowels","base"),("dowels","left_arch"),("dowels","right_arch"),
 ("screws","base"),("screws","left_arch"),("screws","right_arch"),("cable","right_arch"),
 ("cable","base"),("led_bar","cable"),("bridge","cable")]}

SHELL_CANOPY = ("left_arch","right_arch","bridge","led_bar","pot","shelf","status")
# name, dir, up, hide, debug, explode, clip_x, focus, scale, overlay
VIEWS = [
 ("p-iso-fl",  (-1,-1,0.5),(0,0,1), ("cable",), 0,0,None,None,None,None),
 ("p-front",   (0,-1,0.04),(0,0,1), ("cable",), 0,0,None,None,None,None),
 ("p-side",    (1,0,0.04), (0,0,1), ("cable",), 0,0,None,None,None,None),
 ("p-top",     (0,0,1),    (0,1,0), ("cable",), 0,0,None,None,None,None),
 ("p-rear",    (0,1,0.04), (0,0,1), ("cable",), 0,0,None,None,None,None),
 ("p-iso-rr",  (1,1,0.5),  (0,0,1), ("cable",), 0,0,None,None,None,None),
 ("v-exploded",(-1,-1,0.45),(0,0,1),("cable",), 1,1,None,None,None,None),
 ("v-underside",(0.25,0.2,-1),(0,1,0),("cable","pot","shelf","led_bar","bridge"),1,0,None,None,None,None),
 ("v-base-cutaway",(0.7,1,0.45),(0,0,1), SHELL_CANOPY+("cable",),1,0,None,None,None,None),
 ("v-conduit-xsec",(1,0.4,0.25),(0,0,1), (), 1,0, ENV_W-SIDE_T/2, None,None,None),
 ("v-led-center",(0,0,1),(0,1,0), ("cable",),1,0,None,None,None,"centerline"),
]
EDGE_ANGLE = 40.0


def export_parts():
    for k in P:
        subprocess.run([OPENSCAD,"-q","--render","-o",str(PARTS/f"{k}.stl"),"-D",f'part="{k}"',str(SCAD)],
                       check=True, capture_output=True)
    print(f"exported {len(P)} part STLs")


def _pd(m):
    pts=vtk.vtkPoints(); pts.SetNumberOfPoints(len(m.vertices))
    for i,v in enumerate(m.vertices): pts.SetPoint(i,*map(float,v))
    c=vtk.vtkCellArray()
    for f in m.faces:
        c.InsertNextCell(3); [c.InsertCellPoint(int(i)) for i in f]
    pd=vtk.vtkPolyData(); pd.SetPoints(pts); pd.SetPolys(c); return pd


def build(meshes):
    A={}
    for k,m in meshes.items():
        pd=_pd(m)
        n=vtk.vtkPolyDataNormals(); n.SetInputData(pd); n.SetFeatureAngle(30); n.SplittingOn(); n.Update()
        mp=vtk.vtkPolyDataMapper(); mp.SetInputData(n.GetOutput()); mp.SetResolveCoincidentTopologyToPolygonOffset()
        ac=vtk.vtkActor(); ac.SetMapper(mp); pr=ac.GetProperty(); pr.SetAmbient(0.34); pr.SetDiffuse(0.72); pr.SetSpecular(0.05)
        fe=vtk.vtkFeatureEdges(); fe.SetInputData(pd); fe.BoundaryEdgesOn(); fe.FeatureEdgesOn()
        fe.SetFeatureAngle(EDGE_ANGLE); fe.ManifoldEdgesOff(); fe.NonManifoldEdgesOff(); fe.Update()
        emp=vtk.vtkPolyDataMapper(); emp.SetInputConnection(fe.GetOutputPort()); emp.SetResolveCoincidentTopologyToPolygonOffset()
        ea=vtk.vtkActor(); ea.SetMapper(emp); ea.GetProperty().SetColor(0.12,0.12,0.12); ea.GetProperty().SetLineWidth(1.0)
        A[k]=(ac,ea,mp,emp)
    return A


def overlay_actors(meshes):
    """crosshair + centroid markers proving LED & pot are centered at (CX,CY)."""
    acts=[]
    zc = 692   # above the bridge so the crosshair/markers sit on top in the top view
    for p0,p1 in [((CX,0,zc),(CX,320,zc)),((0,CY,zc),(480,CY,zc))]:
        ls=vtk.vtkLineSource(); ls.SetPoint1(*p0); ls.SetPoint2(*p1)
        m=vtk.vtkPolyDataMapper(); m.SetInputConnection(ls.GetOutputPort())
        a=vtk.vtkActor(); a.SetMapper(m); a.GetProperty().SetColor(0.9,0.1,0.1); a.GetProperty().SetLineWidth(2.5); acts.append(a)
    for key,col in [("led_bar",(0.95,0.6,0.0)),("pot",(0.1,0.3,0.9))]:
        c=meshes[key].centroid
        s=vtk.vtkSphereSource(); s.SetCenter(c[0],c[1],zc); s.SetRadius(7)
        m=vtk.vtkPolyDataMapper(); m.SetInputConnection(s.GetOutputPort())
        a=vtk.vtkActor(); a.SetMapper(m); a.GetProperty().SetColor(*col); acts.append(a)
    return acts


def render(meshes):
    A=build(meshes)
    ren=vtk.vtkRenderer(); ren.SetViewport(0,0,0.80,1); ren.SetBackground(1,1,1)
    for ac,ea,_,_ in A.values(): ren.AddActor(ac); ren.AddActor(ea)
    vtk.vtkLightKit().AddLightsToRenderer(ren)
    rleg=vtk.vtkRenderer(); rleg.SetViewport(0.80,0,1,1); rleg.SetBackground(1,1,1)
    lg=vtk.vtkLegendBoxActor(); lg.SetNumberOfEntries(len(P)); sq=vtk.vtkPlaneSource(); sq.Update()
    for i,k in enumerate(P): lg.SetEntry(i, sq.GetOutput(), P[k][0], list(P[k][1]))
    lg.GetPositionCoordinate().SetCoordinateSystemToNormalizedViewport(); lg.GetPositionCoordinate().SetValue(0.02,0.06)
    lg.GetPosition2Coordinate().SetCoordinateSystemToNormalizedViewport(); lg.GetPosition2Coordinate().SetValue(0.96,0.9)
    lg.GetEntryTextProperty().SetFontSize(11); rleg.AddActor(lg)
    rw=vtk.vtkRenderWindow(); rw.SetOffScreenRendering(1); rw.AddRenderer(ren); rw.AddRenderer(rleg)
    rw.SetSize(1340,1040); rw.SetMultiSamples(8)
    cam=ren.GetActiveCamera(); cam.ParallelProjectionOn()
    ovl=overlay_actors(meshes)

    for name,d,up,hide,debug,explode,clipx,focus,scale,overlay in VIEWS:
        for k,(ac,ea,mp,emp) in A.items():
            ac.SetVisibility(0 if k in hide else 1); ea.SetVisibility(0 if k in hide else 1)
            ac.GetProperty().SetColor(*(P[k][2] if debug else P[k][1]))
            ac.SetPosition(*(np.array(P[k][3]) if explode else (0,0,0)))
            ea.SetPosition(ac.GetPosition())
            mp.RemoveAllClippingPlanes(); emp.RemoveAllClippingPlanes()
            if clipx is not None:
                pl=vtk.vtkPlane(); pl.SetOrigin(clipx,0,0); pl.SetNormal(-1,0,0)
                mp.AddClippingPlane(pl); emp.AddClippingPlane(pl)
        for a in ovl: ren.RemoveActor(a)
        if overlay=="centerline":
            for a in ovl: ren.AddActor(a)
        cam.SetFocalPoint(0,0,0); cam.SetPosition(*d); cam.SetViewUp(*up); ren.ResetCamera(); cam.Zoom(1.1)
        rw.Render()
        w2i=vtk.vtkWindowToImageFilter(); w2i.SetInput(rw); w2i.ReadFrontBufferOff(); w2i.Update()
        wr=vtk.vtkPNGWriter(); wr.SetFileName(str(RENDERS/f"{name}.png")); wr.SetInputConnection(w2i.GetOutputPort()); wr.Write()
        print(f"  wrote {name}.png")
    for a in ovl: ren.RemoveActor(a)


def _obj(m):
    b=fcl.BVHModel(); b.beginModel(len(m.vertices),len(m.faces))
    b.addSubModel(np.asarray(m.vertices,float),np.asarray(m.faces,np.int64)); b.endModel()
    return fcl.CollisionObject(b,fcl.Transform())


def collision(meshes):
    keys=[k for k in meshes if k!="cable"]; objs={k:_obj(meshes[k]) for k in keys}
    print("\nFCL collision check:"); fails=0
    for a,b in combinations(sorted(objs),2):
        r=fcl.DistanceResult(); g=fcl.distance(objs[a],objs[b],fcl.DistanceRequest(),r)
        contact=frozenset((a,b)) in EXPECTED_CONTACT
        if g<=1e-6 and not contact:
            fails+=1; print(f"  COLLIDE {a} x {b}  <-- UNINTENDED")
    print("COLLISION CHECK:", "PASS" if fails==0 else f"FAIL ({fails})"); return fails


def main():
    export_parts()
    meshes={k:trimesh.load_mesh(str(PARTS/f"{k}.stl")) for k in P}
    lc, pc = meshes["led_bar"].centroid, meshes["pot"].centroid
    print(f"LED centroid  X={lc[0]:.1f} Y={lc[1]:.1f}  (target {CX},{CY})")
    print(f"pot centroid  X={pc[0]:.1f} Y={pc[1]:.1f}  (target {CX},{CY})")
    print("rendering:"); render(meshes)
    f=collision(meshes); return 0 if f==0 else 1


if __name__=="__main__":
    sys.exit(main())
