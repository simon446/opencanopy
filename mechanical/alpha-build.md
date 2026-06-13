# Alpha build notes (spec §15.6 M5-09; WI-ME-08)

The first full unit assembled from the released source. This document is the
**assembly procedure + build log**; photos go in `validation/photos/` (Validation
track) and are linked here once the physical build is made.

> **Status:** Design complete and buildable on paper — all parts modelled,
> exported, manifold-clean (CI `stl_check`), and the assembly fits the locked
> envelope with verified clearances (`cad-verification-checklist.md`). The
> **physical alpha print/build is PENDING** and is performed jointly with the
> Validation track (it feeds WI-QA-01/02/03/04). Tick the photo/log boxes during
> that build.

## Bill of parts

- Printed parts: `stl/printable/*.stl` (see `print-settings.md` for material/zone).
- Bought parts: `bom-mechanical.csv` (pot, reservoir, extrusion, fasteners, fan,
  grommets, tubing, feet) + the LED head from the Light track + the PCB/harness
  from Electronics (WI-EE-04/05).

## Assembly order (bottom-up — water lives low, electronics high)

1. **Frame.** Cut and join the 20×20 uprights with top/mid/low rails. Fit the
   four levelling feet. Verify square and that it is stable with a full reservoir
   and a loaded pot (CG ≈ 305 mm, 44 % of height — bottom-heavy).
2. **Wet bay.** Drop in the `leak-tray`; seat the leak sensor in the sump boss.
   Slide the reservoir onto the cradle rails (drawer pulls out the front).
3. **Pump.** Clip the pump into the `pump-clip` on its silicone pad inside the
   reservoir; fit the intake filter; route 8 mm tube up the channel's tube pocket.
4. **Pot deck.** Fit the `pot-tray` on the mid rail; confirm the downspout drops
   into the leak tray. Seat the 10 L pot on the locating ring.
5. **Routing.** Mount the `cable-channel` to the rear-right upright. Run wiring in
   the wire pocket (separate from the tube). Fit grommets + form a **drip loop**
   below each dry-bay entry. Clip the moisture probe with `sensor-clip`.
6. **Fan.** Bolt the `fan-mount` to the rear with rubber grommets (no metal-to-frame
   contact); fit the guard side toward the canopy.
7. **Light.** Fix the `light-mount` carrier to the upright hole-row at the height
   giving 150–300 mm canopy clearance; attach the LED head; **fit the secondary
   retention tether** through both eyelets.
8. **Electronics.** Mount the controller PCB and remote LED driver on the dry-bay
   heat-set bosses; land the labelled harness through the grommets; close the lid.
9. **Power-on dry check** before any water (hand off to WI-QA-01).

## Build log

- [ ] All printed parts printed in the materials from `print-settings.md`.
- [ ] Coupons printed and `fit-tests.md` filled; `params.py` clearances calibrated.
- [ ] Frame assembled, square, stable with full load.
- [ ] Reservoir + pot insert/remove tested tool-free (no wires disturbed).
- [ ] Drip loops formed at every electronics entry; harness labels match
      `electronics/wiring/harness-table.csv`.
- [ ] Secondary retention on the light verified (fixture cannot fall).
- [ ] Photos captured to `validation/photos/alpha-build/` and linked here.
- [ ] Assembly feasibility confirmed → M5-09 acceptance.

## Notes / deviations

_Record any fit issues, part substitutions, or `params.py` changes here. Any change
to a locked envelope value requires a risk-register entry (scope-lock contract)._
