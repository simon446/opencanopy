# OpenCanopy CAD Brief for Claude

## Purpose

This document translates the OpenCanopy concept sketch into a strict, dimensioned CAD brief.

Claude should **not** model directly from the image. The sketch is concept art: it contains ambiguous curves, decorative details, approximate proportions, and visually inferred components. CAD should be generated from the constraints in this document only.

The goal is to create a clean **v0 parametric CAD block model** first. Visual refinement comes later.

---

## Project

**Name:** OpenCanopy Tabletop Pepper v1  
**Type:** open-source tabletop indoor grow frame  
**Plant:** one hot pepper plant, e.g. Carolina Reaper  
**Form:** open-frame, non-enclosed, table-friendly  
**Interface:** LED status indicators only; no screen, no input controls  

---

## Core CAD Principle

Do not start with complex surfaces.

Start with simple mechanical placeholders:

1. Outer frame
2. Top light module
3. Bottom base / shelf
4. Removable pot placeholder
5. Reservoir placeholder
6. Electronics bay placeholder
7. Fan mount placeholder
8. LED status diffuser
9. Cable channel
10. Assembly

Use **simple solids only** for v0:

- cubes
- cylinders
- rectangular beams
- simple cutouts
- optional rounded boxes only if easy and reliable

Avoid decorative detail until the base geometry is correct.

---

## Coordinate System

Use this coordinate system consistently:

```text
Origin: front-left-bottom corner of total device envelope
X = width, left to right
Y = depth, front to back
Z = height, bottom to top
```

Total envelope:

```text
Width: 480 mm
Depth: 320 mm
Height: 680 mm
```

Coordinate extents:

```text
X: 0 to 480 mm
Y: 0 to 320 mm
Z: 0 to 680 mm
```

---

## Global Design Constraints

```text
Overall envelope: 480 W × 320 D × 680 H mm
Open front: yes
Open sides: mostly yes
Enclosed grow chamber: no
Primary material appearance: matte white frame + light wood accent panels
Use case: tabletop indoor device
Plant count: one
Pot target: 8–10 L removable pot
Reservoir target: 2.5–4 L removable reservoir
Electronics: top/rear dry zone
Water reservoir: bottom wet zone
Electronics must sit above water path
No front screen
No knobs/buttons for v0
Status feedback: 4 small LEDs behind frosted diffuser
```

---

## Required v0 Modules

### 1. `outer_frame()`

Create the main structural frame.

For v0, approximate the rounded concept frame with rectangular beams.

Suggested members:

```text
Left vertical side frame
Right vertical side frame
Top horizontal frame
Bottom horizontal/base frame
Rear support members if needed
```

Approximate beam dimensions:

```text
Side vertical beams: 35–45 mm thick
Top beam height: 45–60 mm
Bottom base height: 80–120 mm
Frame depth: 320 mm
```

Open clear area target:

```text
Clear opening width: ~400 mm
Clear opening height: ~500 mm
```

The frame must leave the front open and avoid forming a sealed cabinet.

---

### 2. `top_light_module()`

Represents the dimmable horticultural LED bar.

Placement:

```text
Position: under top frame
Approximate center: X=240 mm, Y=145–160 mm, Z=615–630 mm
```

Approximate dimensions:

```text
LED bar length: 360–420 mm
LED bar width: 35–60 mm
LED bar height: 10–25 mm
```

For v0, represent as a slim rectangular bar. Do not model individual LED chips unless trivial.

Important clearance:

```text
Grow light should be above plant canopy area.
Allow vertical grow space from pot top to light: target 360–430 mm.
```

---

### 3. `bottom_base()`

Represents the lower structural shelf/base.

Purpose:

- supports pot
- hides reservoir region
- provides front area for LED status strip
- keeps unit stable on table

Suggested dimensions:

```text
Full width: 480 mm
Full depth: 320 mm
Base height: 80–120 mm
Top shelf height: around Z=100 mm
```

The pot should sit on or slightly above the base shelf.

---

### 4. `pot_placeholder()`

Represents a removable 8–10 L pot.

Placement:

```text
Center X: 240 mm
Center Y: 150 mm
Bottom Z: ~100 mm
```

Suggested dimensions:

```text
Top diameter or width: 260–290 mm
Bottom diameter or width: 220–250 mm
Height: 220–260 mm
Volume target: 8–10 L
```

For v0:

- Use a cylinder or tapered cylinder if convenient.
- A simple cylinder is acceptable.
- Do not model soil, roots, plant, or decorative texture.

---

### 5. `reservoir_placeholder()`

Represents the bottom wet-zone reservoir.

Placement:

```text
Rear/bottom region
Y: 170–310 mm
Z: 20–120 mm
```

Suggested dimensions:

```text
Width: 340–400 mm
Depth: 120–160 mm
Height: 80–120 mm
Volume target: 2.5–4 L
```

For v0, model as a rectangular translucent-style placeholder, but actual transparency is optional.

Important:

```text
Reservoir must remain physically lower than electronics bay.
Reservoir must be removable or accessible from rear/bottom in later versions.
```

---

### 6. `electronics_bay()`

Represents the top/rear dry electronics compartment.

Placement:

```text
Y: 250–320 mm
Z: 590–670 mm
X: approximately 40–440 mm
```

Suggested dimensions:

```text
Width: 360–420 mm
Depth: 50–70 mm
Height: 45–70 mm
```

Contents represented only as placeholders:

```text
Controller PCB
LED driver
Power input
Optional camera mount position
Cable exit area
```

For v0, use one rectangular bay plus optional small placeholder boxes inside.

Important:

```text
Electronics bay must be above the water path.
Do not place electronics under pot or reservoir.
```

---

### 7. `fan_mount()`

Represents quiet low-noise fan position.

Placement:

```text
Upper rear-right area
Approximate center: X=390 mm, Y=300 mm, Z=500 mm
```

Suggested fan size:

```text
40 mm or 60 mm square fan placeholder
Thickness: 10–25 mm
```

For v0:

- Use a square plate with circular fan cutout, or just a fan block.
- No blade geometry needed.

Airflow should be shown structurally as compatible with open frame; do not enclose the plant chamber.

---

### 8. `status_led_diffuser()`

Represents the front LED status strip.

Placement:

```text
Centered on lower front face
X center: 240 mm
Y: 0–10 mm
Z center: ~55 mm
```

Suggested dimensions:

```text
Diffuser width: 100–140 mm
Diffuser height: 12–20 mm
Diffuser depth: 3–8 mm
```

It should represent four tiny status LEDs behind a frosted diffuser:

```text
WATER
MOISTURE
LIGHT
SYSTEM
```

For v0:

- Use one small horizontal rounded or rectangular diffuser.
- Optionally add four small circles behind/within it.
- No labels required on CAD unless easy.

Do not add a screen, rotary knob, buttons, or control panel.

---

### 9. `cable_channel()`

Represents controlled routing from dry electronics bay to lower wet zone.

Placement:

```text
Rear vertical channel, preferably right side or rear-right
```

Purpose:

- route low-voltage pump wires downward
- route sensor wires
- avoid cables passing through open grow space
- allow drip loops before entering electronics bay

For v0, model as a simple rectangular channel or cover.

Important note:

```text
Only low-voltage pump/sensor wires descend into the wet zone.
Mains input and high-current electronics remain in dry zone.
```

---

### 10. `assembly()`

Combines all modules into the full v0 CAD model.

The assembly should be valid and visually readable from front/isometric view.

Do not add:

- plant model
- labels
- screws
- screw holes
- wiring detail
- decorative curves
- wood grain
- material textures
- transparent effects unless trivial

---

## Explicit Part Placement Summary

```text
Overall device:
  width = 480 mm
  depth = 320 mm
  height = 680 mm

Pot:
  center X = 240 mm
  center Y = 150 mm
  bottom Z = 100 mm
  top diameter/width = 260–290 mm
  height = 220–260 mm

Grow light:
  under top frame
  center X = 240 mm
  center Y = 145–160 mm
  Z = 615–630 mm

Electronics bay:
  rear top
  X = 40–440 mm
  Y = 250–320 mm
  Z = 590–670 mm

Reservoir:
  rear bottom/base
  X = 40–440 mm
  Y = 170–310 mm
  Z = 20–120 mm

Fan:
  upper rear-right
  approximate center X = 390 mm
  Y = 300 mm
  Z = 500 mm

LED status diffuser:
  lower front
  center X = 240 mm
  Y = 0–10 mm
  center Z = 55 mm

Cable channel:
  rear or rear-right vertical path
  connects top dry zone to bottom wet zone
```

---

## What Claude Should Ignore for v0

Do not attempt these yet:

```text
rounded organic corners
Scandinavian styling beyond simple white/wood blocks
wood grain
plant leaves or peppers
wiring detail
transparent reservoir material
labels or engraved text
hidden fasteners
screw holes
cable clips
print tolerances
snap fits
rubber feet
removable latches
filleted decorative faces
exact manufacturable part splits
```

---

## CAD Format Recommendation

Prefer **OpenSCAD** for the first model.

Reasons:

- simple parametric geometry
- easy to inspect and modify
- robust for LLM-generated code
- fast to iterate
- suitable for later export to STL

CadQuery or FreeCAD Python can be used later after the geometry is stable.

---

## OpenSCAD Prompt for Claude

Use this prompt directly:

```text
Create a parametric OpenSCAD CAD model for OpenCanopy Tabletop Pepper v1.

Do not infer from the concept sketch. Use only the dimensions and constraints below.

Coordinate system:
- Origin at front-left-bottom corner of the total envelope.
- X = width left to right.
- Y = depth front to back.
- Z = height bottom to top.

Overall envelope:
- Width: 480 mm
- Depth: 320 mm
- Height: 680 mm

Design:
- Open-frame tabletop grow system.
- Non-enclosed front and sides.
- Rounded white outer frame can be approximated with rectangular beams for v0.
- Light wood accent panels at top front and bottom shelf.
- One removable pot centered in the grow zone.
- Reservoir in bottom wet zone.
- Electronics bay in top rear dry zone.
- LED grow light mounted under top frame.
- Fan mounted upper rear-right.
- Four LED status indicators behind a small front diffuser.
- No screen and no input controls.

Generate the model in separate modules:
1. outer_frame()
2. top_light_module()
3. bottom_base()
4. pot_placeholder()
5. reservoir_placeholder()
6. electronics_bay()
7. fan_mount()
8. status_led_diffuser()
9. cable_channel()
10. assembly()

Use simple solids only:
- cubes
- cylinders
- rectangular beams
- boolean cutouts where necessary
- rounded boxes only if easy and robust

Do not add screws, wiring, fillets, decorative curves, labels, or plant geometry in v0.

Required placement:
- Pot centered at X=240 mm, Y=150 mm.
- Pot sits on bottom shelf around Z=100 mm.
- Grow light under top frame around Z=620 mm.
- Electronics bay at rear top: Y=250–320 mm, Z=590–670 mm.
- Reservoir in bottom rear/base: Y=170–310 mm, Z=20–120 mm.
- Fan on upper rear-right side around X=390 mm, Y=300 mm, Z=500 mm.
- LED status diffuser centered on front bottom at X=240 mm, Y=0–10 mm, Z=55 mm.

Output complete OpenSCAD code.
Prioritize valid, clean, parametric code over visual detail.
```

---

## Recommended Iteration Plan

### v0 — Block Model

Goal:

```text
All major volumes correctly placed.
No visual refinement.
No manufacturability details.
```

Acceptance criteria:

```text
- Opens in OpenSCAD without syntax errors.
- Overall envelope is 480 × 320 × 680 mm.
- Front is visibly open.
- Pot, reservoir, electronics bay, light, fan, and status diffuser are all present.
- Electronics are above the water path.
- No screen or controls are present.
```

---

### v1 — Printable Frame Pieces

Add:

```text
- split frame into printable parts
- simple screw bosses or joinery placeholders
- rear service access concept
- removable reservoir clearance
- removable pot clearance
```

Still avoid decorative styling.

---

### v2 — Mechanical Integration

Add:

```text
- screw holes
- cable channels
- drip-loop routing geometry
- pump tube routing
- fan mounting holes
- LED bar mounting holes
- PCB mounting holes
- reservoir retaining features
```

---

### v3 — Aesthetic Refinement

Add:

```text
- rounded frame corners
- smoother transitions
- wood accent panels
- diffuser recess
- hidden fasteners where practical
- front proportions adjusted for appearance
```

---

### v4 — Manufacturing and Repo Release

Add:

```text
- STL exports
- STEP exports if possible
- print orientation notes
- tolerance test pieces
- fastener BOM
- assembly guide
- CAD screenshots
- release checklist
```

---

## Common Failure Modes to Avoid

### 1. Overfitting to the concept sketch

The sketch is visual direction only. It is not dimensionally reliable.

### 2. Adding detail too early

Do not model screws, wires, clips, decorative radii, or labels before the block model is correct.

### 3. Accidentally making an enclosure

The design is open-frame. Do not close the front, sides, or grow zone.

### 4. Putting electronics below water

Electronics must sit in the top/rear dry zone. The reservoir and pump belong in the bottom wet zone.

### 5. Reintroducing UI controls

The current approved direction is:

```text
No screen.
No knob.
No buttons.
LED status only.
```

### 6. Making the pot too small

The pot is intentionally large relative to the frame. For one pepper plant, the CAD should reserve space for an 8–10 L pot.

### 7. Making the light decorative

The light is a real horticultural LED bar placeholder, not ambient lighting.

### 8. Ignoring service access

Even in v0, leave conceptual space for removing:

```text
pot
reservoir
pump
electronics bay cover
LED bar
fan
```

---

## Minimal Output Expected from Claude

Claude should output one complete OpenSCAD file containing:

```text
parameter definitions
helper functions/modules if needed
outer_frame()
top_light_module()
bottom_base()
pot_placeholder()
reservoir_placeholder()
electronics_bay()
fan_mount()
status_led_diffuser()
cable_channel()
assembly()
assembly(); call at the end
```

The first output does not need to be beautiful. It needs to be valid, modular, and dimensionally coherent.

---

## Suggested Repo Location

```text
opencanopy/
  mechanical/
    cad/
      opencanopy_tabletop_pepper_v1_block_model.scad
    exports/
    screenshots/
  docs/
    cad_brief_for_claude.md
```

---

## Summary

The correct approach is:

```text
Concept sketch → dimensioned mechanical brief → parametric block CAD → printable engineering CAD → aesthetic refinement
```

Not:

```text
Concept sketch → detailed CAD directly
```

For now, Claude should generate a **simple, correct OpenSCAD block model** and stop there.
