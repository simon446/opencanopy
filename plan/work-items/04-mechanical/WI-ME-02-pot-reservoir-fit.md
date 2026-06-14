# WI-ME-02 — Pot & reservoir selection and fit

| Field | Value |
|---|---|
| Track | Mechanical |
| Milestone | M5-02 |
| Depends on | WI-ME-01 |
| Spec refs | §5.5, §7.7, §16.2 |
| Status | Done — **re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md)** |

> **🔄 Re-scoped by [ECO-003](../../../docs/ECO-003-v1-redesign.md):** the separate **10 L pot** is
> replaced by an **integrated removable grow insert** (slotted/perforated, semi-hydro), and the
> reservoir is raised **4 L → 6 L** (6.6 L gross modelled). Watering is **passive wicking** — the
> insert lifts straight out of the well; the reservoir is the base wet zone, filled from the top. The
> 10 L pot / 4 L drawer below are the superseded arched-model values, kept for history. Effective
> media (~5 L) vs the old 10 L is a documented trade-off (risk-register R2/R9/R14), grow-trial-gated.

## Objective

Select and model the **removable grow insert** and the **6 L passive reservoir**, proving tool-free
lift-out, capillary wick contact, and correct overflow (no water path to the top electronics).

## Deliverables

- [x] Pot: **10 L** compact (~9.7 L usable), drain-capable, removable without disconnecting wires.
      `pot_reservoir.build_pot()`. Tapered 250→210 mm to clear the 268 mm interior depth (18 mm margin).
- [x] Reservoir: **4 L** compact (~4.1 L usable), food-safe, tool-free pull-out drawer, hand-cleanable
      opening. `pot_reservoir.build_reservoir()`.
- [x] CAD models + BOM entries (`mechanical/bom-mechanical.csv`); overflow path routes pot→pot-tray
      gutter→downspout→leak tray, and reservoir→front weir, away from electronics (§7.7).
- [x] Pot sits on the pot-tray locating ring, kept above any standing tray water (no stagnant soak).

## Acceptance criteria

- Fit verified in assembly (spec §15.6 M5-02). ✅ `verify.py`: pot OD 250 < interior 268; reservoir
  4.1 L; reservoir top 146 mm < pot deck 180 mm.
- Reservoir/pot removable per serviceability rules (§8.4). ✅ Reservoir slides out below the pot deck
  without touching the plant; pot lifts off the ring into 159 mm of headroom.
