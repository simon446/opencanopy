#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
render.py — Sketch renders of the FULL assembly from many angles (VTK).

Reads the committed placed assembly meshes (mechanical/stl/assembly/*.stl — every
part in its assembly position) and renders shaded views with clean silhouette/feature
edges into docs/assets/renders/. A real z-buffered renderer (VTK) is used so there is
correct occlusion (no painter's-algorithm "clipping") and flat, uniform face colour;
feature-edge extraction draws only real edges/creases, not triangulation diagonals.

Each part gets a distinct colour + an on-image legend so reviewers can identify and
comment on specific modules.

Headless: VTK renders fully offscreen (no display). On Linux CI install the
`vtk-osmesa` wheel instead of `vtk` (same `import vtk`, software OSMesa, no X server).

    .venv-cad/bin/python mechanical/cad/source/render.py
"""
from pathlib import Path

import trimesh
import vtk

HERE = Path(__file__).resolve()
MESH_DIR = HERE.parents[2] / "stl" / "assembly"
OUT = HERE.parents[2].parent / "docs" / "assets" / "renders"
OUT.mkdir(parents=True, exist_ok=True)

WIDTH, HEIGHT = 1180, 1000
SMOOTH_ANGLE = 30.0      # below this, normals are smoothed (rounds the pot/cone)
EDGE_ANGLE = 42.0        # only creases sharper than this draw an edge (hides facets)

# Distinct per-part colours (RGB 0-1). Keys are the STL stems.
PALETTE = {
    "frame":                   (0.50, 0.52, 0.55),
    "leak-tray":               (0.20, 0.55, 0.85),
    "reservoir":               (0.15, 0.70, 0.75),
    "pump-clip":               (0.10, 0.35, 0.65),
    "pot-tray":                (0.55, 0.40, 0.80),
    "pot":                     (0.55, 0.75, 0.35),
    "fan-mount":               (0.95, 0.55, 0.15),
    "cable-channel":           (0.85, 0.75, 0.20),
    "status-diffuser":         (0.90, 0.40, 0.75),
    "led-fixture":             (0.85, 0.25, 0.25),
    "light-mount":             (0.95, 0.45, 0.45),
    "electronics-dry-bay":     (0.30, 0.65, 0.45),
    "electronics-dry-bay-lid": (0.65, 0.80, 0.70),
}
FALLBACK = (0.7, 0.7, 0.7)

# Camera directions (unit-ish vector from focal point toward camera) + view-up.
# Assembly frame: +X right, +Y back, +Z up; the open face is -Y (front).
VIEWS = [
    ("front",           (0, -1, 0),       (0, 0, 1)),
    ("right",           (1, 0, 0),        (0, 0, 1)),
    ("back",            (0, 1, 0),        (0, 0, 1)),
    ("top",             (0, 0, 1),        (0, 1, 0)),
    ("iso-front-right", (1, -1, 0.65),    (0, 0, 1)),
    ("iso-front-left",  (-1, -1, 0.65),   (0, 0, 1)),
]


def _polydata(mesh):
    pts = vtk.vtkPoints()
    pts.SetNumberOfPoints(len(mesh.vertices))
    for i, v in enumerate(mesh.vertices):
        pts.SetPoint(i, float(v[0]), float(v[1]), float(v[2]))
    cells = vtk.vtkCellArray()
    for f in mesh.faces:
        cells.InsertNextCell(3)
        cells.InsertCellPoint(int(f[0]))
        cells.InsertCellPoint(int(f[1]))
        cells.InsertCellPoint(int(f[2]))
    pd = vtk.vtkPolyData()
    pd.SetPoints(pts)
    pd.SetPolys(cells)
    return pd


def _square_symbol():
    src = vtk.vtkPlaneSource()
    src.Update()
    return src.GetOutput()


def _build_actors():
    surf_actors, edge_actors, legend_items = [], [], []
    for f in sorted(MESH_DIR.glob("*.stl")):
        name = f.stem
        color = PALETTE.get(name, FALLBACK)
        pd = _polydata(trimesh.load_mesh(str(f)))

        normals = vtk.vtkPolyDataNormals()
        normals.SetInputData(pd)
        normals.SetFeatureAngle(SMOOTH_ANGLE)
        normals.SplittingOn()
        normals.ConsistencyOn()
        normals.Update()
        smoothed = normals.GetOutput()

        mp = vtk.vtkPolyDataMapper()
        mp.SetInputData(smoothed)
        mp.SetResolveCoincidentTopologyToPolygonOffset()
        ac = vtk.vtkActor()
        ac.SetMapper(mp)
        p = ac.GetProperty()
        p.SetColor(*color)
        p.SetAmbient(0.32)
        p.SetDiffuse(0.75)
        p.SetSpecular(0.08)
        surf_actors.append(ac)

        fe = vtk.vtkFeatureEdges()
        fe.SetInputData(pd)
        fe.BoundaryEdgesOn()
        fe.FeatureEdgesOn()
        fe.SetFeatureAngle(EDGE_ANGLE)
        fe.ManifoldEdgesOff()
        fe.NonManifoldEdgesOff()
        fe.Update()
        emp = vtk.vtkPolyDataMapper()
        emp.SetInputConnection(fe.GetOutputPort())
        emp.SetResolveCoincidentTopologyToPolygonOffset()
        ea = vtk.vtkActor()
        ea.SetMapper(emp)
        ea.GetProperty().SetColor(0.12, 0.12, 0.12)
        ea.GetProperty().SetLineWidth(1.1)
        edge_actors.append(ea)

        legend_items.append((name, color))
    return surf_actors, edge_actors, legend_items


def _legend(items):
    legend = vtk.vtkLegendBoxActor()
    legend.SetNumberOfEntries(len(items))
    sym = _square_symbol()
    for i, (name, color) in enumerate(items):
        legend.SetEntry(i, sym, name, list(color))
    # fills its own (right-hand) viewport
    legend.GetPositionCoordinate().SetCoordinateSystemToNormalizedViewport()
    legend.GetPositionCoordinate().SetValue(0.04, 0.18)
    legend.GetPosition2Coordinate().SetCoordinateSystemToNormalizedViewport()
    legend.GetPosition2Coordinate().SetValue(0.92, 0.64)
    legend.SetPadding(2)
    legend.GetEntryTextProperty().SetFontSize(12)
    return legend


def main():
    surf_actors, edge_actors, items = _build_actors()
    if not items:
        raise SystemExit(f"no meshes in {MESH_DIR} — run build.py first")

    # 3D scene on the left ~82 % of the frame; legend in its own right-hand strip
    # so it never overlaps the model.
    ren = vtk.vtkRenderer()
    ren.SetViewport(0.0, 0.0, 0.82, 1.0)
    ren.SetBackground(1.0, 1.0, 1.0)
    for a in surf_actors + edge_actors:
        ren.AddActor(a)

    lk = vtk.vtkLightKit()
    lk.SetKeyLightIntensity(1.0)
    lk.AddLightsToRenderer(ren)

    ren_leg = vtk.vtkRenderer()
    ren_leg.SetViewport(0.82, 0.0, 1.0, 1.0)
    ren_leg.SetBackground(1.0, 1.0, 1.0)
    ren_leg.AddActor(_legend(items))

    rw = vtk.vtkRenderWindow()
    rw.SetOffScreenRendering(1)
    rw.AddRenderer(ren)
    rw.AddRenderer(ren_leg)
    rw.SetSize(WIDTH, HEIGHT)
    rw.SetMultiSamples(8)          # anti-aliasing

    cam = ren.GetActiveCamera()
    cam.ParallelProjectionOn()     # clean technical / orthographic look

    for name, direction, up in VIEWS:
        cam.SetFocalPoint(0, 0, 0)
        cam.SetPosition(*direction)
        cam.SetViewUp(*up)
        ren.ResetCamera()
        cam.Zoom(1.05)
        rw.Render()

        w2i = vtk.vtkWindowToImageFilter()
        w2i.SetInput(rw)
        w2i.SetScale(1)
        w2i.ReadFrontBufferOff()
        w2i.Update()
        wr = vtk.vtkPNGWriter()
        wr.SetFileName(str(OUT / f"assembly-{name}.png"))
        wr.SetInputConnection(w2i.GetOutputPort())
        wr.Write()
        print(f"wrote assembly-{name}.png")

    print(f"rendered {len(VIEWS)} views -> {OUT.relative_to(HERE.parents[3])}")


if __name__ == "__main__":
    main()
