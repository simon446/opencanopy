# WI-ME-08 — Print tolerances & alpha build

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-08, M5-09 |
| Depends on | WI-ME-02, WI-ME-03, WI-ME-04, WI-ME-05, WI-ME-06, WI-ME-07 |
| Spec refs | §8.3, §12.2, §14.1 (mechanical/) |
| Status | Not started |

## Objective

Validate print tolerances with coupons, then build and document the first full alpha unit.

## Deliverables

- [ ] Tolerance coupons (§12.2): snap-fit, screw boss, heat-set insert, tube clip, diffuser slot,
      cable-channel clip, reservoir rail/slide.
- [ ] `mechanical/print-settings.md` + `fit-tests.md` with results.
- [ ] Released STLs in `mechanical/stl/printable/`; material choices per §8.3 (PETG/ASA/ABS, no PLA near
      heat/humidity).
- [ ] Alpha build: photos + assembly notes proving the unit is buildable.

## Acceptance criteria

- Coupons pass §12.2 acceptance (no cracking, no excessive force, parts survive 40 °C without warping).
- Assembly feasible (spec §15.6 M5-09).
