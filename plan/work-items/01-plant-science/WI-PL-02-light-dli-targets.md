# WI-PL-02 — Light / DLI targets & calculator

| Field | Value |
|---|---|
| Track | Plant Science |
| Milestone | M1-02 |
| Depends on | WI-PL-01 |
| Spec refs | §2.1, §5.2 (DLI formula), §7.2 |
| Status | Done |

## Objective

Define the DLI/PPFD targets per stage and provide a calculator that converts between DLI, PPFD,
and photoperiod so the light fixture and firmware can be sized correctly.

## Deliverables

- [x] `scripts/dli_calculator.py` implementing `DLI = PPFD × hours × 0.0036` and its inverse.
  - [x] Given a target DLI + photoperiod, output required average PPFD.
  - [x] Given fixture PPF + canopy area, estimate delivered PPFD.
- [x] Per-stage DLI/PPFD target table (seedling → fruiting) documented and cross-checked vs §5.2. (`docs/dli-targets.md`)
- [x] Worked example reproducing spec §5.2 (23 mol·m⁻²·day⁻¹, 16 h → ≈399 µmol·m⁻²·s⁻¹). (asserted in `--selftest`)

## Acceptance criteria

- Calculator unit-tested; reproduces the spec's worked example within rounding.
- Fruiting target of ~20–25 mol·m⁻²·day⁻¹ and ≥~400 µmol·m⁻²·s⁻¹ canopy PPFD are represented.

## Notes

Feeds the light fixture requirements in [WI-EE-01](../03-electronics/WI-EE-01-component-poc.md) and the
LED dim map calibration in [WI-EE-08](../03-electronics/WI-EE-08-bringup-hil.md).
