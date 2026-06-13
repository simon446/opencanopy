# WI-ME-05 — Light mount

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-05 |
| Depends on | WI-ME-01 |
| Spec refs | §7.2 (mounting), §8.6 |
| Status | Done |

## Objective

Design the overhead LED mount with adjustable height, safe clearance, secondary retention, and a
thermal path that does not dump heat into electronics or plastics.

## Deliverables

- [x] CAD/STL for the light mount (`light_mount.build_light_mount`); **adjustable** height via a 6-step
      hole row giving 150–300 mm above canopy (159 mm at the nominal hole, §8.6).
- [x] Mechanical secondary retention: two Ø4 mm tether eyelets so the fixture cannot fall into the
      plant/wet area (§7.2).
- [x] Remote LED driver kept in the dry bay (mount carries the head only) → driver heat off the canopy;
      part specified in **ASA/ABS** (heat zone) per `print-settings.md`.
- [x] Adjustable design, so the fixed-height ≥600 mm rule does not apply; grow zone is 428 mm open with
      150–300 mm light travel above the rim.

## Acceptance criteria

- Adjustable mount with verified clearance (spec §15.6 M5-05). ✅ `verify.py`: 159 mm, in 150–300 band.
- Secondary retention present ✅; thermal path validation is handed to
  [WI-QA-03](../05-validation-qa/WI-QA-03-thermal.md) (QA track) — the design isolates driver heat and
  uses heat-rated material to make that test passable.
