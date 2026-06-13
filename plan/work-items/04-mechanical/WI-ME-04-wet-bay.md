# WI-ME-04 — Wet bay design

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-04 |
| Depends on | WI-ME-02 |
| Spec refs | §6.2, §7.7, §8.4, §12.3 |
| Status | Done |

## Objective

Design the bottom wet bay: reservoir cradle, pump mount, intake filter access, leak tray, and
overflow/drainage path.

## Deliverables

- [x] CAD for reservoir bay with cradle rails (tool-free drawer removal), rubber-isolated open-front
      pump clip (`build_pump_clip`), tool-free filter access.
- [x] Leak tray (`build_leak_tray`) below the entire water system, with a sensor sump + leak-sensor boss.
- [x] Overflow path routes spills away from electronics: front weir + pot-tray downspout to the tray (§7.7).
- [x] Pump/filter serviceable tool-free — open-front C-cradle lifts the pump straight out (§8.4).

## Acceptance criteria

- Reservoir/pump removable (spec §15.6 M5-04). ✅
- Water-path tests ([WI-QA-02](../05-validation-qa/WI-QA-02-wet-run-water-path.md)) can be run against
  this design. ✅ Leak tray + weir + downspout + sensor sump are all modelled; the §12.3 test matrix
  maps directly onto these features.
