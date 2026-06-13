# WI-PL-06 — Photometric delivery model & DLI gate

| Field | Value |
|---|---|
| Track | Plant Science (Electronics input) |
| Milestone | M1.5-01 (new pre-order gate — §23 DR-01) |
| Depends on | WI-PL-02 |
| Spec refs | §7.2, §5.2, §23 (DR-01) |
| Status | Done |

## Objective

Before any grow light is **purchased**, model the *delivered* PPFD map, uniformity, and DLI for the
specific candidate fixture(s) at the actual mounting geometry (clearance, canopy area, frame
reflectances) — to confirm the §7.2 targets are physically achievable in the locked envelope, rather
than assumed from a fixture's headline PPF spec sheet. This is the photometric half of the §23 DR-01
pre-order modeling gate.

## Deliverables

- [x] Photometric model (multi-emitter inverse-square + frame/wall reflectance) producing a predicted
      PPFD map across the 300–400 × 220–300 mm canopy at the planned light-to-canopy clearance
      (≥150 mm, §7.2), per candidate fixture and dim level. (`validation/ppfd-measurements/model/photometric_model.py`)
- [x] Predicted **average PPFD**, **min/avg uniformity**, and **DLI** at the planned photoperiod
      (using the [WI-PL-02](WI-PL-02-light-dli-targets.md) calculator) for each candidate.
- [x] Sensitivity sweep vs clearance (150–250 mm) and canopy growth (plant rises toward the light over
      the cycle).
- [x] Go/no-go recommendation per candidate fixture, recorded in `validation/ppfd-measurements/model/`. (`README.md` + PPFD-map CSVs)

## Acceptance criteria

- At least one candidate fixture is modeled to meet **≥350 µmol/m²/s average at ≥0.6 uniformity**
  across the canopy at ≥150 mm clearance, hitting the WI-PL-02 DLI target within the §7.8 LED
  power/thermal budget.
- If no candidate passes, the finding (need larger fixture / full-yield variant / smaller canopy) is
  documented **before any light is ordered**.

## Notes

Pairs with the thermal half ([WI-EE-10](../03-electronics/WI-EE-10-thermal-budget-model.md)).
[WI-EE-01](../03-electronics/WI-EE-01-component-poc.md) must not finalize the grow-light BOM entry
until this gate passes. The later physical PPFD measurement (WI-EE-01 / `ppfd-measurements/`)
validates this model against reality.
