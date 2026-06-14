# Mechanical build — v1 product model

Renders of the **OpenCanopy v1 product model** (480 × 320 × 680 mm) from the parametric
OpenSCAD source (`mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad`),
rendered with VTK (`mechanical/cad/render_block.py`) and validated by an honest,
no-whitelist geometry audit (`mechanical/cad/audit.py`).

**Architecture (major redesign — two-pillar Scandinavian form):**

- One **low integrated base** (the product body, 135 mm) — a **single wet zone**: a passive
  **6 L reservoir** + an integrated **grow well**. There is **no electronics bay in the base**.
- **Two vertical wooden pillars** (Ø28) rising from dry structural bosses, **centred on the
  base depth**.
- One horizontal **top LED block** spanning the pillars; below it, **one centered LED grow
  panel + finned heatsink on a single central mount** (not a strip) — an 8×6 emitter board per
  **WI-PL-06** (fixture C: a panel meets uniformity at the 150 mm target clearance, where a strip
  needed ≥200–225 mm, which would force a taller unit). The LED optical centre is over the plant.
  The **small 1.6 mm controller + driver PCB is encapsulated inside the
  block** (internal bay on standoff bosses, 4 mounting holes), with a **USB-C port through the
  rear face** — no board exposed. The block prints in two parts (body + bottom lid).
- A **removable raised grow insert** (slotted/perforated, semi-hydro) for one pepper plant.
- **Passive self-watering** (reservoir + wicking). **No pump, no fan, no screen/controls;
  4 status LEDs only.**

**Wet/dry separation is now top (electronics) vs bottom (water)** — not an in-base wall. Only
sealed low-voltage sensor leads + status-LED light pipes touch the base (entering through a
grommet at the right pillar); power (USB-C) enters at the top. Pillars and the grow module
share Y (= 160, base centre) so a thin block places the LED directly over the plant with no
cantilever. The base stays low because the grow insert is a **raised planter** above the top.

## Interactive 3D model

**Drag to orbit, scroll to zoom.** The externally-visible assembly (base, pillars, top block,
LED panel + heatsink, grow insert, status pill, feet) in web-standard glTF. Source:
`mechanical/cad/exports/parts/*.stl` → `assets/models/opencanopy-v1.glb`
(`mechanical/cad/render_block.py::export_glb`).

<script type="module" src="https://cdn.jsdelivr.net/npm/@google/model-viewer@4.0.0/dist/model-viewer.min.js"></script>
<model-viewer
  src="assets/models/opencanopy-v1.glb"
  alt="OpenCanopy V1 — drag to rotate the assembly"
  camera-controls auto-rotate touch-action="pan-y"
  interaction-prompt="none" shadow-intensity="1" exposure="1.05"
  camera-orbit="-35deg 72deg auto" min-camera-orbit="auto auto auto" max-camera-orbit="auto auto auto"
  style="width:100%; height:520px; background:#f3f4f6; border-radius:10px;">
  <p slot="poster" style="padding:1rem;">Loading 3D model… (or
  <a href="assets/models/opencanopy-v1.glb">download the GLB</a>)</p>
</model-viewer>

## Product views

![Front-left isometric](assets/renders/p-iso-fl.png)

![Front](assets/renders/p-front.png) ·
![Side](assets/renders/p-side.png) ·
![Rear](assets/renders/p-rear.png)

![Top](assets/renders/p-top.png) ·
![Rear-right isometric](assets/renders/p-iso-rr.png)

## Validation (debug colours)

**LED centering** — the LED optical centreline and the grow module are both at X = 240,
Y = 160; the script confirms numerically `LED<->grow offset dX=0.0 dY=0.0` (acceptance ≤ 5 mm).

![LED centering (top)](assets/renders/v-led-center.png)

**Exploded assembly** · **underside (pillar screw access)** · **base service cutaway**
(reservoir + raised insert + wick path) · **cable cross-section** (sensor leads up the rear of
the right pillar to the top board):

![Exploded](assets/renders/v-exploded.png)

![Underside](assets/renders/v-underside.png) ·
![Base cutaway](assets/renders/v-base-cutaway.png) ·
![Cable cross-section](assets/renders/v-cable-xsec.png)

## Checks

- **Geometry audit (`audit.py`) — CLEAN.** Honest interference check on the real meshes with
  **no whitelist**: it measures the true boolean **overlap volume** of every part pair (so
  abutting/touching faces are *not* mistaken for interpenetration) and each part's
  **nearest-neighbour gap** (so a floating/unsupported part is caught). Result: **no
  interpenetration > 80 mm³ and no floating parts.** Hardware sits in real clearance holes; the
  reservoir and raised insert are seated with a 0.5 mm wick gap; the controller PCB + USB-C are
  encapsulated in the block bay; the panel + heatsink hang on the central mount; the status pill +
  status LEDs sit in the front slot.
- **Grow light: one centered panel + finned heatsink, single central mount** (not a strip) — 8×6
  emitters across one board (WI-PL-06 fixture C), sized to fit between the pillar inner faces.
  Per PL-06 a panel meets uniformity (≥ 0.6) at the 150 mm target clearance, where a single bar
  needed ≥ 200–225 mm (a taller unit) — the panel keeps the form short and centered.
- **Status LEDs** — the 4 indicators use the electrical team's **WS2812B-2020** part (vendor model
  `electronics/pcb/3d-models/WS2812B-2020_C965555.step`), integrated at true datasheet dimensions
  (2.0 × 2.0 × 0.8 mm) behind the diffuser (see `mechanical/cad/vendor/README.md`).
- **LED centred over the grow module** — optical centre offset **0.0 / 0.0 mm** (≤ 5 mm limit).
- **Reservoir** — **6.6 L** gross (≥ 6 L usable target). The grow insert is a **raised planter**
  so the base stays low (≤ 130 mm visible); media capacity is a documented trade-off (see
  [ECO-003](ECO-003-v1-redesign.md) / open questions) pending a grow trial.
- **Joints:** each pillar seats 30 mm into a base socket with an **M4 from the underside** into a
  threaded insert + an **anti-rotation dowel**; the block sockets onto the pillar tops with a
  **rear set screw**. See [fastening & assembly](fastening.md).
- **Physics sim:** deferred for the two-pillar architecture (per maintainer) — geometry is
  validated by `audit.py`; a free-standing load sim can be re-run later (`physics_sim.py`).

Reproduce:

```sh
.venv-cad/bin/python mechanical/cad/render_block.py   # export parts + renders
.venv-cad/bin/python mechanical/cad/audit.py          # interference (volume) + float audit
openscad -D 'part="base"' --render -o base.stl mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad
```
