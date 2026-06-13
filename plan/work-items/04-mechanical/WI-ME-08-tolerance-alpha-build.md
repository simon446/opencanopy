# WI-ME-08 — Print tolerances & alpha build

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-08, M5-09 |
| Depends on | WI-ME-02, WI-ME-03, WI-ME-04, WI-ME-05, WI-ME-06, WI-ME-07 |
| Spec refs | §8.3, §12.2, §14.1 (mechanical/) |
| Status | In progress |

## Objective

Validate print tolerances with coupons, then build and document the first full alpha unit.

## Deliverables

- [x] Tolerance coupons (§12.2): all seven — snap-fit, screw boss, heat-set insert, tube clip, diffuser
      slot, cable-channel clip, reservoir rail/slide — in `mechanical/stl/prototypes/` (graded fits).
- [x] `mechanical/print-settings.md` (material-by-zone + slicer baseline) + `mechanical/fit-tests.md`
      (coupon matrix; results table awaiting the physical print run).
- [x] Released STLs in `mechanical/stl/printable/`; material choices per §8.3 documented in
      `print-settings.md` (PETG/ASA/ABS, **no PLA** near heat/humidity).
- [x] Alpha build assembly procedure + log in `mechanical/alpha-build.md`.
- [ ] **Physical alpha build: photos** (`validation/photos/alpha-build/`) — PENDING a print/build.

## Acceptance criteria

- **Fit risk retired by simulation.** `cad/source/collision_check.py` runs the real models through
  **FCL** (python-fcl) with a configurable worst-case error margin: zero interferences, every clearance
  ≥2 mm, surviving a 1.0 mm typical-FDM closure budget (and a 1.6 mm harsh band). It also caught + fixed
  8 real interferences in the first model. See `mechanical/tolerance-analysis.md`. The coupons are now a
  one-print **process-band confirmation**, not a fit discovery.
- Coupons pass §12.2 acceptance (no cracking, no excessive force, survive 40 °C). ⏳ Dimensional fits
  validated by sim; snap-fit strain within PETG allowable (1.4 %); 40 °C warp covered by material
  selection (PETG/ASA Tg ≫ 40 °C). Remaining physical confirmation runs during the alpha build (with
  the Validation track).
- Assembly feasible (spec §15.6 M5-09). ⏳ Design demonstrates feasibility (envelope + clearances +
  tool-free service all verified in `cad-verification-checklist.md`; assembly order in `alpha-build.md`);
  physical confirmation pending the build.

## Notes

This item spans M5-08 (coupons — design **done**) and M5-09 (alpha print/build — **pending hardware**).
Everything that can be produced without a printer is complete; the remaining checkboxes require an
actual print run and are executed with the Validation track (feeds WI-QA-01/02/03/04). Once printed,
record clearances in `fit-tests.md`, calibrate `params.py`, and flip this to Done.
