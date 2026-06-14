# WI-ME-01 — Full assembly CAD

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-01 |
| Depends on | WI-PS-04 |
| Spec refs | §8.1, §8.2, §3.3, §3.4, §12.1 |
| Status | Done — **re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md)** |

> **🔄 Re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md):** the assembly is now a
> **two-pillar** form — base + two wooden pillars + top LED block + removable raised grow insert —
> with **electronics in the top block** and **passive watering**; wet/dry separation is **top vs
> bottom**. The authoritative model is now the OpenSCAD
> `mechanical/cad/opencanopy_tabletop_pepper_v1_block_model.scad` (geometry audit CLEAN; LED centred
> over the plant, offset 0.0/0.0). The build123d source + STEP below are the superseded arched-frame
> model, retained for history.

## Objective

Model the complete **two-pillar** assembly (base + two pillars + top LED block + raised grow insert)
within the locked compact envelope, with the LED centred over the plant and **top/bottom** wet/dry
separation.

## Deliverables

- [x] Source CAD + STEP for the full assembly (`mechanical/cad/source/` parametric build123d
      model; `mechanical/cad/step/opencanopy-assembly.step`).
- [x] Open-frame vertical stack: upper dry bay, LED zone, grow zone, lower pot, bottom wet bay (§8.1).
- [x] Within locked envelope 480 × 320 × 700 mm (assembly bbox 468 × 314 × 700, asserted on every
      build); open front, hidden wiring, translucent status diffuser per §3.4.
- [x] CAD verification checklist (§12.1): see `mechanical/cad-verification-checklist.md`, with numbers
      computed by `verify.py` (insert/remove paths, clearances, CG 305 mm/44 % H with full reservoir +
      plant, drip/leak path) — all PASS.

## Acceptance criteria

- All modules placed (spec §15.6 M5-01); envelope within §3.3 compact target. ✅
- "Water fails downward; electronics live upward" rule (§6.2) visibly satisfied. ✅
  (`assembly.assert_zone_separation()` proves no dry part dips below the pot deck.)

## Notes

Authoritative source is the parametric build123d model (plain-text, regenerates STEP/STL/SVG
deterministically). Drawings in `mechanical/drawings/`. Toolchain + regen in
`mechanical/cad/source/README.md`.
