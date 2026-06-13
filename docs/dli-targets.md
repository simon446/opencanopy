# Light / DLI Targets & Calculator (OpenCanopy V1)

**Track:** Plant Science · **Work item:** [WI-PL-02](../plan/work-items/01-plant-science/WI-PL-02-light-dli-targets.md)
· **Spec refs:** §2.1, §5.2 (DLI formula), §7.2 · **Status:** Done
· **Depends on:** [WI-PL-01](plant-profile-hot-pepper.md)

Defines the per-stage **DLI/PPFD** targets and the conversions that let the light fixture and firmware
be sized correctly. Light is specified in **DLI and PPFD — never lumens** (spec §2.1, source R1).

The conversions are implemented and unit-tested in
[`scripts/dli_calculator.py`](../scripts/dli_calculator.py). This document is the human-readable
target table; the script is the executable source of truth used by CI and the downstream
electronics/firmware work items.

---

## 1. The DLI ⇄ PPFD relationship (spec §5.2)

```text
DLI [mol·m⁻²·day⁻¹] = PPFD [µmol·m⁻²·s⁻¹] × photoperiod_hours × 0.0036
PPFD               = DLI / (photoperiod_hours × 0.0036)

0.0036 = 3600 s/h × 1e-6 mol/µmol
```

DLI is the **total** PAR (400–700 nm) photons delivered per m² per day; PPFD is the **instantaneous**
rate. At V1's 16 h photoperiod (vegetative and later), `PPFD = DLI / 0.0576`. [R1]

### Worked example (reproduces spec §5.2)

```text
Target DLI:  23 mol·m⁻²·day⁻¹
Photoperiod: 16 h
Required average PPFD = 23 / (16 × 0.0036) = 23 / 0.0576 ≈ 399 µmol·m⁻²·s⁻¹
```

```console
$ python3 scripts/dli_calculator.py ppfd --dli 23 --hours 16
Required average PPFD = 399.306 µmol·m⁻²·s⁻¹
```

This is asserted in the calculator's `--selftest` (`worked-example ppfd`, ±0.5 µmol), satisfying the
WI-PL-02 acceptance criterion "reproduces the spec's worked example within rounding."

---

## 2. Per-stage DLI / PPFD targets (spec §5.2)

Transcribed from the [lifecycle profile](plant-profile-hot-pepper.md) §2; all at a **16 h
photoperiod**. PPFD is measured **at the canopy**, averaged across the useful canopy area
(0.07–0.12 m² compact, spec §7.2). [R1, R2, R3, R14]

| Stage | DLI target (mol·m⁻²·d⁻¹) | Photoperiod | PPFD at canopy (µmol·m⁻²·s⁻¹) | PPFD recomputed from DLI |
|---|---:|---:|---:|---:|
| Germination (after emergence) | 0–6 | 0–16 h | 0–100 | n/a (light optional/low) |
| Seedling | 8–12 | 16 h | 140–210 | 139–208 |
| Vegetative | 14–20 | 16 h | 245–350 | 243–347 |
| Flowering | 18–24 | 16 h | 315–420 | 313–417 |
| Fruiting | 20–25 | 16 h | 350–435 | 347–434 |

The last column is the DLI band run back through the formula; it matches the table to within the
±5 µmol rounding the spec uses (PPFD listed in tidy multiples of 5). Verified by
`dli_calculator.py --selftest` and viewable with `dli_calculator.py table`.

**Fruiting target restated (acceptance criterion):** DLI **~20–25 mol·m⁻²·day⁻¹** and a canopy PPFD
of **≥~400 µmol·m⁻²·s⁻¹** (the band's upper half) — peppers are treated as a high-light crop, heat and
fixture capability permitting. [R3, spec §2.1]

---

## 3. Calculator usage (`scripts/dli_calculator.py`)

```console
# DLI from a measured/target PPFD over a photoperiod
$ python3 scripts/dli_calculator.py dli --ppfd 399 --hours 16
DLI = 22.9824 mol·m⁻²·day⁻¹

# Required average PPFD to hit a target DLI
$ python3 scripts/dli_calculator.py ppfd --dli 23 --hours 16
Required average PPFD = 399.306 µmol·m⁻²·s⁻¹

# Estimate delivered canopy PPFD from a fixture's PPF over a canopy area
# (capture = fraction of fixture photons landing on canopy; 1.0 = ideal upper bound)
$ python3 scripts/dli_calculator.py fixture --ppf 45 --area 0.10
Delivered avg PPFD = 450 µmol·m⁻²·s⁻¹ (capture=1.0)
  -> DLI at 16.0 h = 25.92 mol·m⁻²·day⁻¹

# Print the per-stage target table
$ python3 scripts/dli_calculator.py table

# CI / sanity gate
$ python3 scripts/dli_calculator.py --selftest
```

> The `fixture` estimate with `capture=1.0` is an **ideal upper bound** for sizing. Real fixtures lose
> photons to optics, mounting height, side spill, and non-uniformity, so the *delivered* PPFD needs a
> capture fraction < 1 and a proper photometric model — that is exactly the job of the §23 DR-01
> pre-order gate, [WI-PL-06](../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md).

---

## 4. Sizing implication for the fixture (spec §7.2)

From §7.2: hitting **450 µmol·m⁻²·s⁻¹ across 0.10 m²** needs ≈**45 µmol/s delivered** to the canopy.
Because real capture is well below 100 %, the *fixture* PPF must be much higher than the delivered
target. The spec's practical compact range is a **140–220 µmol/s** dimmable fixture (≥2.2 µmol/J PPE),
operated dimmed with headroom; a 220–250 µmol/s fixture belongs in the optional full-yield variant.
WI-PL-06 confirms a specific candidate actually delivers the §7.2 targets in the locked geometry
before any light is purchased.

---

## 5. Downstream consumers (traceability)

| Feeds | Via |
|---|---|
| Grow-light fixture requirements / BOM | [WI-EE-01](../plan/work-items/03-electronics/WI-EE-01-component-poc.md) |
| LED dim-map PPFD calibration (25/50/75/100 %) | [WI-EE-08](../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md) (spec §9.9) |
| Photometric delivery model / pre-order DLI gate | [WI-PL-06](../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md) |
| Firmware light scheduler intensity targets | [WI-FW-04](../plan/work-items/02-firmware/WI-FW-04-light-controller.md) (spec §9.5) |

## 6. Sources

R1 (DLI concept, PPFD/DLI relationship, 400–700 nm PAR), R2 (CEA grow-light planning), R3 (PPFD/DLI
ranges, high-light crop framing), R14 (seedling light-quality response). Full list:
[`references.md`](references.md).
</content>
