# Drawings

Orthographic assembly references, generated from the parametric model by
`../cad/source/drawings.py` (regenerate after any geometry change). Solid lines are
visible edges; dashed grey are hidden edges.

| File | View | Looking |
|---|---|---|
| `assembly-front.svg` | Front elevation | along +Y (the open user-facing front) |
| `assembly-right.svg` | Right elevation | along −X |
| `assembly-top.svg` | Plan / top | along −Z |

These show the locked **480 × 320 × 700 mm** envelope, the vertical stack (dry bay
on top, grow zone with light + fan, pot on its tray, wet bay at the bottom) and the
clearances tabulated in `../cad-verification-checklist.md`.

Dimensioned manufacturing drawings of individual printed parts are not maintained as
separate files — the parametric source (`../cad/source/`) and the STEP exports
(`../cad/step/`) are the dimensional authority.
