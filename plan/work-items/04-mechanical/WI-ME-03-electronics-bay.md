# WI-ME-03 — Electronics dry bay

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-03 |
| Depends on | WI-ME-01 |
| Spec refs | §6.2, §8.4, §17.1 |
| Status | Done |

## Objective

Design the upper dry electronics bay so the controller PCB, LED driver, and power distribution stay
isolated from the wet path and are serviceable.

## Deliverables

- [x] CAD for the dry bay housing controller PCB, LED driver, power distribution, status wiring
      (`electronics_bay.build_dry_bay()` + `build_dry_bay_lid()`), topmost module of the stack.
- [x] Heat-set standoffs on a 4-point pattern (no board flex); top-removable lid → serviced **without**
      opening the wet bay (§8.4).
- [x] Splash protection (drop-lip lid); cable entries are raised grommet collars + drip-loop hook posts
      so water cannot track in (§8.5, §17.1).
- [x] Fits a reserved **120 × 90 mm** controller-PCB envelope + 110 × 40 mm driver footprint, published
      to [WI-EE-04](../03-electronics/WI-EE-04-pcb-layout.md) as the mechanical budget (see note).

## Acceptance criteria

- Dry service bay isolated from wet zone (spec §15.6 M5-03). ✅ Bay floor at z 608 mm; zone-separation
  assertion proves no overlap with the wet band (≤180 mm).
- Electronics accessible without touching the reservoir/pump. ✅ Top lid only.

## Notes

WI-EE-04 (PCB outline) is not yet frozen — its layout report reciprocally states the board outline +
mount-hole pattern are *"coordinated with the mechanical electronics-bay before fabrication"*. So the
bay is designed to a generous reserved board envelope (`PCB_W`/`PCB_D` in `params.py`) which is the
agreed coordination point, published back to the Electronics track via `cad-verification-checklist.md`.
When the KiCad board lands, confirm fit and update those change-controlled params if needed, then
re-run `build.py`.
