# CAD source (parametric, build123d)

The OpenCanopy mechanical model is **code CAD**: a parametric Python model built on
[build123d](https://github.com/gumyr/build123d) (OpenCascade B-rep kernel). Source is
plain text, diffable, and regenerates every released STEP/STL deterministically — so
review happens on the model, not on opaque binaries.

## Layout

```text
opencanopy/
  params.py          single source of truth: locked envelope + all interfaces
  util.py            assembly-frame landmarks / align helpers
  frame.py           open-frame vertical stack            (WI-ME-01)
  pot_reservoir.py   10 L pot + 4 L reservoir             (WI-ME-02)
  electronics_bay.py upper dry bay + lid                  (WI-ME-03)
  wet_bay.py         leak tray, pump clip, pot tray       (WI-ME-04)
  light_mount.py     adjustable LED carrier + retention   (WI-ME-05)
  fan_mount.py       guarded, isolated fan mount          (WI-ME-06)
  routing.py         cable/tube channel, clips, diffuser  (WI-ME-07)
  coupons.py         7 tolerance coupons                  (WI-ME-08)
  assembly.py        full assembly + §12.1 checks         (WI-ME-01)
build.py             export STEP (../step) + STL (../../stl)
verify.py            compute §12.1 verification numbers
drawings.py          orthographic SVGs -> ../../drawings
```

## Toolchain setup (headless, no GUI)

build123d needs an OpenCascade wheel (OCP), which currently ships for CPython ≤ 3.12.
Use [`uv`](https://github.com/astral-sh/uv) to get a self-contained interpreter:

```bash
brew install uv                       # or: pipx install uv
uv venv --python 3.12 .venv-cad
uv pip install --python .venv-cad/bin/python build123d
```

`.venv-cad/` is git-ignored.

## Regenerate everything

```bash
.venv-cad/bin/python mechanical/cad/source/build.py      # STEP + STL
.venv-cad/bin/python mechanical/cad/source/verify.py     # §12.1 numbers
.venv-cad/bin/python mechanical/cad/source/drawings.py   # SVG views
.venv-cad/bin/python scripts/stl_check.py --dir mechanical/stl   # CI manifold gate
```

Each module runs standalone too (`python -m opencanopy.<module>`) and prints its
bounding box — handy when iterating on one part.

## Conventions

- **Units:** millimetres. **Frame:** origin at front-left-bottom; +X right, +Y back
  (open face is −Y / front), +Z up. See the docstring in `params.py`.
- **Change control:** locked values (envelope, pot, reservoir) live in `params.py`;
  changing one requires a risk-register entry per the scope-lock contract (WI-PS-04).
- **Cross-track interfaces** (PCB envelope, harness labels, LED head size) are also in
  `params.py` and are the numbers published to Electronics/Light — see
  `../../cad-verification-checklist.md`.
