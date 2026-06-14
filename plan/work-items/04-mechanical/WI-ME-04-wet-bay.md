# WI-ME-04 — Wet bay design

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-04 |
| Depends on | WI-ME-02 |
| Spec refs | §6.2, §7.7, §8.4, §12.3 |
| Status | Done — **re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md)** (no pump) |

> **🔄 Re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md):** V1 is **passive self-watering —
> no pump**. The base **is** the wet zone: a **6 L reservoir** + the **grow-insert well** with a
> capillary wick path, a top **fill port**, **overflow**, and **insert lift-out** for cleaning. The
> **pump mount / pump clip / intake filter / pump-impeller guard are removed** (and the pump-specific
> leak→lockout is gone — leak/overflow is monitor-and-warn). Electronics are at the **top**, so there
> is no wet-bay-to-dry-bay overflow concern in the base. Pump-related deliverables below are superseded.

## Objective

Design the base **wet zone**: the 6 L passive reservoir, the grow-insert well + capillary wick path,
fill port, overflow, and tool-free insert lift-out for cleaning. **No pump** (ECO-003).

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
