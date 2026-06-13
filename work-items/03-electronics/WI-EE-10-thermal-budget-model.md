# WI-EE-10 — Pre-order thermal budget model

| Field | Value |
|---|---|
| Track | Electronics (Mechanical input) |
| Milestone | M1.5-02 (new pre-order gate — §23 DR-01) |
| Depends on | WI-PS-04, WI-PL-06 |
| Spec refs | §7.2 (thermal), §7.8, §17.2, §12.4, §23 (DR-01) |
| Status | Not started |

## Objective

Before ordering the LED/driver and freezing CAD, compute a thermal budget for the worst-case light
load (up to 100 W full-yield) in the compact open frame: predict LED junction/heatsink temperature,
the temperature rise at the canopy and at the **upper electronics dry bay**, and the required
heatsink/airflow — to confirm the design stays within LED, printed-material, and electronics limits at
room 22–25 °C. Thermal half of the §23 DR-01 pre-order modeling gate.

## Deliverables

- [ ] Steady-state thermal budget for 50 / 80 / 100 W LED loads: estimated LED junction & heatsink
      temperatures for the candidate heatsink + fan airflow; canopy-air rise above ambient.
- [ ] Required heatsink thermal resistance and **minimum fan airflow** to keep the LED + driver in
      spec and printed parts (PETG/ASA/ABS) below their limits (§17.2).
- [ ] Electronics-bay check: confirm the upper dry-bay electronics **and the RTC** (§16.1) stay within
      rating given LED/driver heat rises into the same upper region.
- [ ] Go/no-go recommendation; if marginal, document mitigations (larger heatsink, lower max W, remote
      driver placement, vent geometry) **before** ordering or CAD freeze.

## Acceptance criteria

- Modeled LED junction/heatsink and printed-material temperatures are within spec at the chosen max
  LED power and 25 °C ambient, with a defined fan operating point.
- Predicted electronics-bay temperature is within component ratings; otherwise a documented design
  change precedes part orders (M2) and CAD freeze (M5).

## Notes

The physical thermal test ([WI-QA-03](../05-validation-qa/WI-QA-03-thermal.md)) validates this model
post-build; the point of this WI is to catch an unbuildable thermal design **before** money is spent.
Coordinate heatsink/fan with [WI-ME-05](../04-mechanical/WI-ME-05-light-mount.md) /
[WI-ME-06](../04-mechanical/WI-ME-06-fan-mount.md) and the power budget
[WI-EE-02](WI-EE-02-power-budget.md).
