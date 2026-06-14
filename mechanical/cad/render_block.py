#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
render_block.py — export, render and validate the OpenSCAD v1 product model.

Exports each part (OpenSCAD CGAL) and renders with VTK (z-buffered, smooth). Interference
and float verification is done separately and honestly by audit.py (no whitelist,
volume-based). Two render groups:
  * PRODUCT views  — material colours (white shell / wood / dark service / water).
  * VALIDATION views — debug colours: exploded, underside screw-access, base cutaway,
    right-rear-arch conduit cross-section, and a TOP view with a crosshair overlay
    proving the LED optical centerline and the pot are both at X=240, Y=160.

Usage:  .venv-cad/bin/python mechanical/cad/render_block.py
"""
import subprocess, sys
from pathlib import Path
import numpy as np, trimesh, vtk

HERE = Path(__file__).resolve().parent
SCAD = HERE / "opencanopy_tabletop_pepper_v1_block_model.scad"
PARTS = HERE / "exports" / "parts"
RENDERS = HERE.parent.parent / "docs" / "assets" / "renders"
PARTS.mkdir(parents=True, exist_ok=True); RENDERS.mkdir(parents=True, exist_ok=True)
OPENSCAD = next((p for p in ("/Applications/OpenSCAD.app/Contents/MacOS/OpenSCAD","openscad")
                 if Path(p).exists() or p=="openscad"), "openscad")
ENV_W = 480; PIL_X1 = 416
CX, CY = 240, 190                      # grow module + LED optical centerline

WHITE=(0.90,0.90,0.89); WOOD=(0.78,0.60,0.36); DARK=(0.24,0.24,0.27); WATER=(0.30,0.62,0.88)
BASKET=(0.36,0.40,0.37)
# part -> (label, product colour, debug colour, explode vector)
P = {
 "base":         ("base / reservoir body", WHITE, (0.85,0.85,0.88), (0,0,-120)),
 "pillar_left":  ("left wood pillar",       WOOD,  (0.80,0.62,0.38), (-70,0,40)),
 "pillar_right": ("right wood pillar",      WOOD,  (0.80,0.62,0.38), (70,0,40)),
 "light_block":  ("top LED block",          WHITE, (0.70,0.80,0.95), (0,0,170)),
 "led_bar":      ("LED grow bar",           (0.97,0.92,0.5),(0.97,0.85,0.2),(0,0,125)),
 "pcb":          ("controller+driver board",(0.15,0.6,0.3),(0.15,0.7,0.3), (95,0,70)),
 "usb_c":        ("USB-C input",            DARK,  (0.2,0.2,0.22),   (120,-30,50)),
 "grow_insert":  ("removable grow insert",  BASKET, (0.40,0.55,0.45),(0,0,215)),
 "reservoir":    ("passive reservoir (6 L)",WATER, WATER,            (0,-140,0)),
 "status":       ("status pill (4 LEDs)",   (0.30,0.82,0.55),(0.30,0.82,0.55),(0,-70,0)),
 "fill_cap":     ("fill-port cap",          WHITE, (0.7,0.7,0.75),   (0,0,70)),
 "feet":         ("feet (x4)",              (0.55,0.55,0.58),(0.55,0.55,0.58),(0,0,-160)),
 "dowels":       ("anti-rotation dowels",   (0.60,0.60,0.62),(0.1,0.6,0.9),  (0,0,-50)),
 "screws":       ("M4 screws / set screws", (0.20,0.20,0.22),(0.95,0.2,0.2), (0,0,-70)),
 "cable":        ("sensor + USB-C cabling", (0.9,0.6,0.2),(0.9,0.6,0.2),     (0,90,0)),
}
INTERNAL = ("reservoir","dowels","screws","cable")            # hidden in clean product views
CANOPY   = ("light_block","led_bar","pcb","usb_c")            # the top assembly
# name, dir, up, hide, debug, explode, clip_x, focus, scale, overlay
VIEWS = [
 ("p-iso-fl",  (-1,-1,0.5),(0,0,1), INTERNAL, 0,0,None,None,None,None),
 ("p-front",   (0,-1,0.04),(0,0,1), INTERNAL, 0,0,None,None,None,None),
 ("p-side",    (1,0,0.04), (0,0,1), INTERNAL, 0,0,None,None,None,None),
 ("p-top",     (0,0,1),    (0,1,0), INTERNAL, 0,0,None,None,None,None),
 ("p-rear",    (0,1,0.04), (0,0,1), ("reservoir","dowels","screws"), 0,0,None,None,None,None),
 ("p-iso-rr",  (1,1,0.5),  (0,0,1), ("reservoir","dowels","screws"), 0,0,None,None,None,None),
 ("v-exploded",(-1,-1,0.45),(0,0,1),(), 1,1,None,None,None,None),
 ("v-underside",(0.25,0.2,-1),(0,1,0), CANOPY+("grow_insert","reservoir","status","fill_cap"),1,0,None,None,None,None),
 ("v-base-cutaway",(0.7,0.8,0.4),(0,0,1), CANOPY+("dowels","screws","cable","fill_cap"),1,0,300,None,None,None),
 ("v-cable-xsec",(-1,0.3,0.2),(0,0,1), (), 0,0, PIL_X1+16, None,None,None),
 ("v-led-center",(0,0,1),(0,1,0), INTERNAL,1,0,None,None,None,"centerline"),
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
    zc = 690   # above the block so the crosshair/markers sit on top in the top view
    for p0,p1 in [((CX,0,zc),(CX,320,zc)),((0,CY,zc),(480,CY,zc))]:
        ls=vtk.vtkLineSource(); ls.SetPoint1(*p0); ls.SetPoint2(*p1)
        m=vtk.vtkPolyDataMapper(); m.SetInputConnection(ls.GetOutputPort())
        a=vtk.vtkActor(); a.SetMapper(m); a.GetProperty().SetColor(0.9,0.1,0.1); a.GetProperty().SetLineWidth(2.5); acts.append(a)
    for key,col in [("led_bar",(0.95,0.6,0.0)),("grow_insert",(0.1,0.3,0.9))]:
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


def main():
    export_parts()
    meshes={k:trimesh.load_mesh(str(PARTS/f"{k}.stl")) for k in P}
    lc, pc = meshes["led_bar"].centroid, meshes["grow_insert"].centroid
    print(f"LED centroid    X={lc[0]:.1f} Y={lc[1]:.1f}  (target {CX},{CY})")
    print(f"insert centroid X={pc[0]:.1f} Y={pc[1]:.1f}  (target {CX},{CY})")
    print(f"LED<->grow offset  dX={abs(lc[0]-pc[0]):.1f}  dY={abs(lc[1]-pc[1]):.1f}  (accept <=5)")
    print("rendering:"); render(meshes)
    print("\n(interference/float verification: run audit.py — no whitelist, volume-based)")
    return 0


if __name__=="__main__":
    sys.exit(main())
