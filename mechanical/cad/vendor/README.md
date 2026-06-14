<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# mechanical/cad/vendor — vendor part models (for CAD integration)

How the electrical team's component models are integrated into the mechanical CAD.

## WS2812B-2020 (status indicator LEDs)

The electrical team provided the vendor model at
[`electronics/pcb/3d-models/WS2812B-2020_C965555.step`](../../../electronics/pcb/3d-models/README.md)
(LCSC `C965555`, 2.0 × 2.0 × 0.84 mm). The mechanical model integrates it as the
**`ws2812b_2020()`** module in `opencanopy_tabletop_pepper_v1_block_model.scad` — a **true
datasheet-dimension outline** (2.0 × 2.0 × 0.8 mm body + emitter lens, centred at origin,
emitter on +Z), placed ×4 behind the status diffuser facing forward.

**Why the outline and not an `import()` of the mesh:** OpenSCAD **2021.01** renders via CGAL,
whose Nef kernel rejects the vendor STEP's tessellation (it is two disjoint solids — body + lens —
and CGAL asserts). The part's own README explicitly blesses the datasheet outline ("the
2.0 × 2.0 × 0.8 mm 4-pad outline is trivial to regenerate from the datasheet") as the
license-clean equivalent. The outline matches the vendor model's footprint and height, so the
diffuser and front status board are designed around the real part.

Regenerate the vendor mesh (if a newer OpenSCAD with the Manifold backend, or another tool, is
used) — build123d is in `.venv-cad`:

```sh
.venv-cad/bin/python -c "from build123d import import_step, export_stl; \
  export_stl(import_step('electronics/pcb/3d-models/WS2812B-2020_C965555.step'), \
             'mechanical/cad/vendor/ws2812b-2020.stl')"
```

## Grow light (`LED_PANEL`)

No discrete vendor model — per `electronics/pcb/3d-models/README.md` the grow light is an **LED
board + heatsink**, modelled in the CAD as the centered **`led_panel()` + `heatsink()`** (WI-PL-06
fixture C: ~300 × 210 mm panel, 8 × 6 emitters, single central mount). Finalise the exact board
SKU/outline against the published PPFD map (PL-06) when the part is ordered.
