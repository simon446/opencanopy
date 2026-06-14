#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
render_block.py — Export, render and collision-check the OpenSCAD v0 block model.

1. Exports each part of opencanopy_tabletop_pepper_v1_block_model.scad to a clean
   manifold STL (OpenSCAD CGAL --render).
2. Renders the assembly from many angles with VTK (a real z-buffered renderer: smooth,
   correct occlusion, clean feature edges) + a colour legend, into docs/assets/renders/.
3. Runs FCL pairwise collision detection on the real parts and reports/flags any
   unintended part-vs-part collision (e.g. electronics hitting a casing wall).

Usage:  .venv-cad/bin/python mechanical/cad/render_block.py
"""
import subprocess
import sys
from itertools import combinations
from pathlib import Path

import numpy as np
import trimesh
import fcl
import vtk

HERE = Path(__file__).resolve().parent
SCAD = HERE / "opencanopy_tabletop_pepper_v1_block_model.scad"
PARTS = HERE / "exports" / "parts"
RENDERS = HERE.parent.parent / "docs" / "assets" / "renders"
PARTS.mkdir(parents=True, exist_ok=True)
RENDERS.mkdir(parents=True, exist_ok=True)

OPENSCAD = next((p for p in (
    "/Applications/OpenSCAD.app/Contents/MacOS/OpenSCAD", "openscad") if Path(p).exists() or p == "openscad"), "openscad")

# part key -> (legend label, RGB 0-1). Material grouping: white shell / wood / dark service.
WHITE = (0.92, 0.92, 0.90); WOOD = (0.80, 0.62, 0.38); DARK = (0.26, 0.26, 0.29)
PARTS_DEF = {
    "side_frames": ("side-frame arches (shell)", WHITE),
    "base":        ("base shell",                WHITE),
    "bridge":      ("top light bridge (shell)",  WHITE),
    "shelf":       ("wood shelf + pot well",     WOOD),
    "led_bar":     ("LED grow bar",              (0.96, 0.90, 0.45)),
    "pot":         ("pot (~9.5 L, hollow)",      (0.82, 0.55, 0.42)),
    "reservoir":   ("reservoir (water)",         (0.30, 0.62, 0.88)),
    "pcb":         ("controller PCB",            DARK),
    "driver":      ("LED driver",                DARK),
    "power":       ("power input (24 V)",        DARK),
    "iso_wall":    ("sealed wet|dry wall",       (0.86, 0.88, 0.92)),
    "status":      ("status pill (4 LEDs)",      (0.30, 0.82, 0.55)),
    "feet":        ("feet (x4)",                 (0.45, 0.45, 0.48)),
}

# Pairs that are SUPPOSED to touch (seated on / mounted to each other).
EXPECTED_CONTACT = {
    frozenset(("side_frames", "base")), frozenset(("bridge", "side_frames")),
    frozenset(("shelf", "base")), frozenset(("pot", "shelf")),
    frozenset(("led_bar", "bridge")), frozenset(("feet", "base")),
    frozenset(("iso_wall", "base")), frozenset(("status", "base")),
}

# Rear/service cutaway hides the shell + canopy parts to expose the base internals.
_CUT = ("side_frames", "bridge", "led_bar", "pot", "shelf")
VIEWS = [  # name, camera-direction (from focal pt), view-up, parts to hide
    ("block-front",   (0, -1, 0.04),  (0, 0, 1), ()),
    ("block-side",    (1, 0, 0.04),   (0, 0, 1), ()),
    ("block-iso",     (1, -1, 0.5),   (0, 0, 1), ()),
    ("block-iso-left",(-1, -1, 0.5),  (0, 0, 1), ()),
    ("block-top",     (0, 0, 1),      (0, 1, 0), ()),
    ("block-service", (0.7, 1, 0.45), (0, 0, 1), _CUT),
]
EDGE_ANGLE = 40.0


def export_parts():
    for key in PARTS_DEF:
        out = PARTS / f"{key}.stl"
        subprocess.run([OPENSCAD, "-q", "--render", "-o", str(out),
                        "-D", f'part="{key}"', str(SCAD)],
                       check=True, capture_output=True)
    print(f"exported {len(PARTS_DEF)} part STLs -> {PARTS}")


def _polydata(mesh):
    pts = vtk.vtkPoints()
    pts.SetNumberOfPoints(len(mesh.vertices))
    for i, v in enumerate(mesh.vertices):
        pts.SetPoint(i, *map(float, v))
    cells = vtk.vtkCellArray()
    for f in mesh.faces:
        cells.InsertNextCell(3)
        for idx in f:
            cells.InsertCellPoint(int(idx))
    pd = vtk.vtkPolyData(); pd.SetPoints(pts); pd.SetPolys(cells)
    return pd


def _actors(meshes):
    actors = {}
    for key, mesh in meshes.items():
        color = PARTS_DEF[key][1]
        pd = _polydata(mesh)
        nrm = vtk.vtkPolyDataNormals(); nrm.SetInputData(pd)
        nrm.SetFeatureAngle(30); nrm.SplittingOn(); nrm.Update()
        mp = vtk.vtkPolyDataMapper(); mp.SetInputData(nrm.GetOutput())
        mp.SetResolveCoincidentTopologyToPolygonOffset()
        ac = vtk.vtkActor(); ac.SetMapper(mp)
        p = ac.GetProperty(); p.SetColor(*color); p.SetAmbient(0.34); p.SetDiffuse(0.72); p.SetSpecular(0.06)
        fe = vtk.vtkFeatureEdges(); fe.SetInputData(pd)
        fe.BoundaryEdgesOn(); fe.FeatureEdgesOn(); fe.SetFeatureAngle(EDGE_ANGLE)
        fe.ManifoldEdgesOff(); fe.NonManifoldEdgesOff(); fe.Update()
        emp = vtk.vtkPolyDataMapper(); emp.SetInputConnection(fe.GetOutputPort())
        emp.SetResolveCoincidentTopologyToPolygonOffset()
        ea = vtk.vtkActor(); ea.SetMapper(emp)
        ea.GetProperty().SetColor(0.12, 0.12, 0.12); ea.GetProperty().SetLineWidth(1.0)
        actors[key] = (ac, ea)
    return actors


def _legend(keys):
    lg = vtk.vtkLegendBoxActor(); lg.SetNumberOfEntries(len(keys))
    sq = vtk.vtkPlaneSource(); sq.Update()
    for i, k in enumerate(keys):
        lg.SetEntry(i, sq.GetOutput(), PARTS_DEF[k][0], list(PARTS_DEF[k][1]))
    lg.GetPositionCoordinate().SetCoordinateSystemToNormalizedViewport()
    lg.GetPositionCoordinate().SetValue(0.02, 0.10)
    lg.GetPosition2Coordinate().SetCoordinateSystemToNormalizedViewport()
    lg.GetPosition2Coordinate().SetValue(0.96, 0.80)
    lg.GetEntryTextProperty().SetFontSize(12)
    return lg


def render(meshes):
    actors = _actors(meshes)
    ren = vtk.vtkRenderer(); ren.SetViewport(0, 0, 0.80, 1); ren.SetBackground(1, 1, 1)
    for ac, ea in actors.values():
        ren.AddActor(ac); ren.AddActor(ea)
    vtk.vtkLightKit().AddLightsToRenderer(ren)
    rleg = vtk.vtkRenderer(); rleg.SetViewport(0.80, 0, 1, 1); rleg.SetBackground(1, 1, 1)
    rleg.AddActor(_legend(list(PARTS_DEF)))
    rw = vtk.vtkRenderWindow(); rw.SetOffScreenRendering(1)
    rw.AddRenderer(ren); rw.AddRenderer(rleg); rw.SetSize(1300, 1040); rw.SetMultiSamples(8)
    cam = ren.GetActiveCamera(); cam.ParallelProjectionOn()
    for name, d, up, hide in VIEWS:
        for k, (ac, ea) in actors.items():
            vis = 0 if k in hide else 1
            ac.SetVisibility(vis); ea.SetVisibility(vis)
        cam.SetFocalPoint(0, 0, 0); cam.SetPosition(*d); cam.SetViewUp(*up)
        ren.ResetCamera(); cam.Zoom(1.15); rw.Render()
        w2i = vtk.vtkWindowToImageFilter(); w2i.SetInput(rw); w2i.ReadFrontBufferOff(); w2i.Update()
        wr = vtk.vtkPNGWriter(); wr.SetFileName(str(RENDERS / f"{name}.png"))
        wr.SetInputConnection(w2i.GetOutputPort()); wr.Write()
        print(f"  wrote {name}.png")


def _fcl_obj(mesh):
    m = fcl.BVHModel(); m.beginModel(len(mesh.vertices), len(mesh.faces))
    m.addSubModel(np.asarray(mesh.vertices, float), np.asarray(mesh.faces, np.int64)); m.endModel()
    return fcl.CollisionObject(m, fcl.Transform())


def collision_check(meshes):
    objs = {k: _fcl_obj(m) for k, m in meshes.items()}
    print("\nFCL collision check (part vs part):")
    fails = 0
    for a, b in combinations(sorted(objs), 2):
        res = fcl.DistanceResult()
        gap = fcl.distance(objs[a], objs[b], fcl.DistanceRequest(), res)
        contact = frozenset((a, b)) in EXPECTED_CONTACT
        if gap <= 1e-6:
            cres = fcl.CollisionResult()
            fcl.collide(objs[a], objs[b], fcl.CollisionRequest(enable_contact=True, num_max_contacts=10), cres)
            pen = max((c.penetration_depth for c in cres.contacts), default=0.0)
            if contact:
                print(f"  contact  {a:<10} ~ {b:<10} (expected)")
            else:
                fails += 1
                print(f"  COLLIDE  {a:<10} x {b:<10}  penetration {pen:.1f} mm   <-- UNINTENDED")
        elif gap < 8 and not contact:
            print(f"  near     {a:<10} . {b:<10}  gap {gap:.1f} mm")
    print("COLLISION CHECK:", "PASS" if fails == 0 else f"FAIL ({fails} unintended)")
    return fails


def main():
    export_parts()
    meshes = {k: trimesh.load_mesh(str(PARTS / f"{k}.stl")) for k in PARTS_DEF}
    print("rendering views:")
    render(meshes)
    fails = collision_check(meshes)
    return 0 if fails == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
