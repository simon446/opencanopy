# WI-EE-10 — Pre-order thermal budget model

| Field | Value |
|---|---|
| Track | Electronics (Mechanical input) |
| Milestone | M1.5-02 (new pre-order gate — §23 DR-01) |
| Depends on | WI-PS-04, WI-PL-06 |
| Spec refs | §7.2 (thermal), §7.8, §17.2, §12.4, §23 (DR-01) |
| Status | Done — **revised PASSIVE / no-fan** ([ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md), 2026-06-14): committed 60 W V1 light is passively cooled, no fan needed. [report](../../../electronics/analysis/WI-EE-10-thermal-budget-model.md) + [`thermal_budget_model.py`](../../../electronics/analysis/thermal_budget_model.py) |

## Objective

Before ordering the LED/driver and freezing CAD, compute a thermal budget for the worst-case light
load (up to 100 W full-yield) in the compact open frame: predict LED junction/heatsink temperature,
the temperature rise at the canopy and at the **upper electronics dry bay**, and the required
heatsink/airflow — to confirm the design stays within LED, printed-material, and electronics limits at
room 22–25 °C. Thermal half of the §23 DR-01 pre-order modeling gate.

## Deliverables

- [x] Steady-state thermal budget for 50 / 60 / 80 / 100 W LED loads: estimated LED junction &
      heatsink temperatures for a **passive (natural-convection) heatsink** (no fan, ECO-001).
      *(Committed 60 W V1 on a 0.8 °C/W passive heatsink: Tj 70 °C / Ths 53 °C. 100 W not viable fan-less.)*
- [x] Required **passive** heatsink thermal resistance to keep the LED + driver in spec and printed
      parts (PETG/ASA/ABS) below their limits (§17.2).
      *(Rth(hs-a) ≤0.8 °C/W natural convection is the WI-ME-05 target; comfortable for 60 W, the passive ceiling is ~80 W.)*
- [x] Electronics-bay check: confirm the upper dry-bay electronics **and the RTC** (§16.1) stay within
      rating given LED/driver heat rises into the same upper region, **with no fan**.
      *(Bay ≤50 °C at <1 CFM-equiv through-flow — carried by open-frame convection; RTC away from driver/plume — DR-05/DR-09.)*
- [x] Go/no-go recommendation; if marginal, document mitigations (larger passive heatsink, lower max
      W, remote driver placement, open vent geometry) **before** ordering or CAD freeze.
      *(GO fan-less for committed 60 W; passive ceiling ~80 W; 100 W full-yield needs active cooling — deferred, ECO-001.)*

## Acceptance criteria

- Modeled LED junction/heatsink and printed-material temperatures are within spec at the chosen max
  LED power and 25 °C ambient, on a **passive heatsink** (no fan — [ECO-001](../../../electronics/analysis/ECO-001-fan-removal.md)).
- Predicted electronics-bay temperature is within component ratings; otherwise a documented design
  change precedes part orders (M2) and CAD freeze (M5).

## Notes

The physical thermal test ([WI-QA-03](../05-validation-qa/WI-QA-03-thermal.md)) validates this model
post-build; the point of this WI is to catch an unbuildable thermal design **before** money is spent.
Coordinate the **passive** heatsink (`Rth(hs-a) ≤0.8 °C/W`, natural convection) with
[WI-ME-05](../04-mechanical/WI-ME-05-light-mount.md) and the open-frame vent geometry; the fan mount
[WI-ME-06](../04-mechanical/WI-ME-06-fan-mount.md) is **obsolete for V1** (no fan — ECO-001). Power
budget: [WI-EE-02](WI-EE-02-power-budget.md).

> **DR-01 gate status:** this WI closes the **thermal** half. The **photometric** half
> ([WI-PL-06](../01-plant-science/WI-PL-06-photometric-model.md)) is now **Done — PASS**, so **both
> halves of DR-01 now pass**. The light-purchase / CAD-freeze gate is clear, subject to choosing a
> fixture that satisfies PL-06's uniformity at the intended clearance (panel @150 mm, or bar
> @≥200–225 mm) — see [WI-EE-01](WI-EE-01-component-poc.md).
