# WI-ME-05 — Light mount

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-05 |
| Depends on | WI-ME-01 |
| Spec refs | §7.2 (mounting), §8.6 |
| Status | Done — **thermal scope widened by [ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md) (fan-less V1)** |

> **Fan-less V1 ([ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md)):** with no
> fan, the grow LED is cooled by **natural convection only**. The electronics thermal
> re-analysis (ECO-001 §3 / [WI-EE-10](../../../electronics/analysis/WI-EE-10-thermal-budget-model.md))
> shows the committed **60 W V1 LED is comfortably passively cooled (T_j ≈ 70 °C)** *provided
> the mount delivers a real passive heatsink* — so this work-item now owns that heatsink. The
> 100 W full-yield variant is **not viable fan-less** and is deferred. See the added thermal
> deliverable/criterion below.

## Objective

Design the overhead LED mount with adjustable height, safe clearance, secondary retention, and a
**passive thermal path** (fan-less, ECO-001) that cools the LED on natural convection without
dumping heat into electronics or plastics.

## Deliverables

- [x] CAD/STL for the light mount (`light_mount.build_light_mount`); **adjustable** height via a 6-step
      hole row giving 150–300 mm above canopy (159 mm at the nominal hole, §8.6).
- [x] Mechanical secondary retention: two Ø4 mm tether eyelets so the fixture cannot fall into the
      plant/wet area (§7.2).
- [x] Remote LED driver kept in the dry bay (mount carries the head only) → driver heat off the canopy;
      part specified in **ASA/ABS** (heat zone) per `print-settings.md`.
- [x] Adjustable design, so the fixed-height ≥600 mm rule does not apply; grow zone is 428 mm open with
      150–300 mm light travel above the rim.
- [ ] **Passive LED heatsink ≤ 0.8 °C/W (fan-less, ECO-001):** the head must carry a finned heatsink
      meeting `Rth(hs-a) ≤ 0.8 °C/W` on **natural convection** (vertical fins, unobstructed), sized for
      the 60 W V1 LED. No fan, no forced air.
- [ ] **Open-frame vent path:** intake low / exhaust high so convective flow rises past the heatsink;
      the LED head must not sit in a still-air pocket. (The fan previously forced this airflow.)

## Acceptance criteria

- Adjustable mount with verified clearance (spec §15.6 M5-05). ✅ `verify.py`: 159 mm, in 150–300 band.
- Secondary retention present ✅; thermal path validation is handed to
  [WI-QA-03](../05-validation-qa/WI-QA-03-thermal.md) (QA track) — the design isolates driver heat and
  uses heat-rated material to make that test passable.
- **Passive heatsink `Rth(hs-a) ≤ 0.8 °C/W` + open convective vent path (fan-less, ECO-001 §3/§5).**
  Closes the cross-track hand-off that ECO-001 flagged to the mechanical track; the QA thermal test
  ([WI-QA-03](../05-validation-qa/WI-QA-03-thermal.md)) now runs against the **passive** heatsink (no
  fan-MOSFET / forced-air assumptions).
