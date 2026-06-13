#!/usr/bin/env python3
# SPDX-License-Identifier: CERN-OHL-S-2.0
"""
render.py — Sketch/wireframe renders of the FULL assembly from many angles.

Reads the committed placed assembly meshes (mechanical/stl/assembly/*.stl — every
part in its assembly position) and renders shaded sketch views with edge lines, one
per camera angle, into docs/assets/renders/. Each part gets a distinct colour + a
legend so validators can identify and comment on it.

Deliberately dependency-light (numpy + matplotlib + trimesh, headless Agg) so it runs
in the Pages CI in seconds WITHOUT an OpenCascade kernel — the heavy build123d step
already ran locally to produce the meshes.

    .venv-cad/bin/python mechanical/cad/source/render.py
"""
from pathlib import Path

import numpy as np
import trimesh
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch
from mpl_toolkits.mplot3d.art3d import Poly3DCollection

HERE = Path(__file__).resolve()
MESH_DIR = HERE.parents[2] / "stl" / "assembly"
OUT = HERE.parents[2].parent / "docs" / "assets" / "renders"
OUT.mkdir(parents=True, exist_ok=True)

# Camera angles (name, elevation, azimuth). A spread of orthographic-ish + isometric.
VIEWS = [
    ("front",       12, -90),
    ("right",       12,   0),
    ("back",        12,  90),
    ("iso-front-right", 24, -55),
    ("iso-front-left",  24, -125),
    ("top",         78, -90),
]

LIGHT = np.array([0.4, -0.7, 0.8])
LIGHT = LIGHT / np.linalg.norm(LIGHT)
AMBIENT, DIFFUSE = 0.45, 0.55


def _load():
    parts = {}
    for f in sorted(MESH_DIR.glob("*.stl")):
        parts[f.stem] = trimesh.load_mesh(str(f))
    if not parts:
        raise SystemExit(f"no meshes in {MESH_DIR} — run build.py first")
    return parts


def _shaded_faces(tris, base_rgb):
    """Per-triangle Lambert shading -> (N,4) RGBA."""
    e1 = tris[:, 1] - tris[:, 0]
    e2 = tris[:, 2] - tris[:, 0]
    n = np.cross(e1, e2)
    ln = np.linalg.norm(n, axis=1, keepdims=True)
    ln[ln == 0] = 1.0
    n = n / ln
    inten = AMBIENT + DIFFUSE * np.clip(n @ LIGHT, 0, 1)
    fc = np.clip(np.array(base_rgb)[None, :] * inten[:, None], 0, 1)
    return np.concatenate([fc, np.ones((len(fc), 1))], axis=1)


def main():
    parts = _load()
    names = list(parts)
    cmap = plt.get_cmap("tab20")
    colors = {nm: cmap(i % 20)[:3] for i, nm in enumerate(names)}

    # global bounds for a consistent equal-aspect box across all views
    allv = np.vstack([p.vertices for p in parts.values()])
    lo, hi = allv.min(0), allv.max(0)
    ctr = (lo + hi) / 2
    span = (hi - lo).max() / 2 * 1.05

    # one combined triangle soup with per-face shaded colours (correct depth sort)
    tri_list, col_list = [], []
    for nm, mesh in parts.items():
        t = mesh.triangles  # (F,3,3)
        tri_list.append(t)
        col_list.append(_shaded_faces(t, colors[nm]))
    tris = np.concatenate(tri_list)
    cols = np.concatenate(col_list)

    legend = [Patch(facecolor=colors[nm], edgecolor="0.3", label=nm) for nm in names]

    for name, elev, azim in VIEWS:
        fig = plt.figure(figsize=(7.5, 8.0))
        ax = fig.add_subplot(111, projection="3d")
        pc = Poly3DCollection(tris, facecolors=cols,
                              edgecolors=(0, 0, 0, 0.18), linewidths=0.15)
        ax.add_collection3d(pc)
        ax.set_xlim(ctr[0] - span, ctr[0] + span)
        ax.set_ylim(ctr[1] - span, ctr[1] + span)
        ax.set_zlim(ctr[2] - span, ctr[2] + span)
        ax.set_box_aspect((1, 1, 1))
        ax.view_init(elev=elev, azim=azim)
        ax.set_axis_off()
        ax.set_title(f"OpenCanopy V1 — {name}  (480 x 320 x 700 mm)", fontsize=10)
        ax.legend(handles=legend, loc="center left", bbox_to_anchor=(0.98, 0.5),
                  fontsize=6, frameon=False)
        fig.savefig(OUT / f"assembly-{name}.png", dpi=110,
                    bbox_inches="tight", pad_inches=0.1)
        plt.close(fig)
        print(f"wrote assembly-{name}.png")
    print(f"rendered {len(VIEWS)} views -> {OUT.relative_to(HERE.parents[3])}")


if __name__ == "__main__":
    main()
