# Pre-order Photometric Model & DLI Gate — Results (OpenCanopy V1)

**Track:** Plant Science (Electronics input) · **Work item:** [WI-PL-06](../../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md)
· **Spec refs:** §7.2, §5.2, §23 (DR-01) · **Milestone:** M1.5-01 · **Status:** Done
· **Depends on:** [WI-PL-02](../../../docs/dli-targets.md)

This is the **photometric half of the §23 DR-01 pre-order modeling gate.** It predicts the *delivered*
PPFD map, min/avg uniformity, and DLI for candidate fixtures at the locked mounting geometry — **before
any grow light is purchased** — so the §7.2 targets are confirmed physically achievable, not assumed
from a fixture's headline PPF spec sheet. Pairs with the thermal half,
[WI-EE-10](../../../plan/work-items/03-electronics/WI-EE-10-thermal-budget-model.md).

> **Gate rule (spec §23 DR-01):** [WI-EE-01](../../../plan/work-items/03-electronics/WI-EE-01-component-poc.md)
> must **not** finalize the grow-light BOM entry, and CAD must not freeze the light mount, until this
> gate passes. The later **physical** PPFD survey (WI-EE-01, `../`) validates this model against reality
> and re-confirms before mass replication.

Reproduce with:

```sh
python3 validation/ppfd-measurements/model/photometric_model.py            # report
python3 validation/ppfd-measurements/model/photometric_model.py --csv validation/ppfd-measurements/model
python3 validation/ppfd-measurements/model/photometric_model.py --selftest # CI physics checks
```

---

## 1. Model & assumptions

First-order, deliberately conservative (see the script header for the math):

- Each fixture = a set of co-planar **downward Lambertian point emitters** sharing the fixture's total
  PPF (a *bar* = a line of emitters; a *panel* = a 2-D grid).
- Direct PPFD at a canopy point: `E = (φ/π) · h² / (h² + r²)²` summed over emitters
  (inverse-square × Lambertian cosine²), `h` = light-to-canopy clearance, `r` = horizontal offset.
- Frame/wall **reflectance** adds a small uniform fill term (`10 %` of mean direct; open frame → sparse
  walls). Set to 0 for a pure direct-only lower bound.
- DLI via the [WI-PL-02 calculator](../../../docs/dli-targets.md) relationship at the 16 h photoperiod.

**Locked envelope:** mid-compact canopy **350 × 260 mm (0.091 m²)**, 15×11 sample grid, minimum
clearance **150 mm** (spec §7.2). **Targets:** avg **≥350 µmol/m²/s**, **min/avg uniformity ≥0.60**,
DLI capability covering the fruiting band **20–25 mol/m²/day** at 16 h (spec §7.2, WI-PL-02), within the
§7.8 LED power budget (50–100 W).

> Reported PPFD/DLI are at **full drive**. In operation the LED is **dimmed** to the per-stage DLI
> target (the [lifecycle profile](../../../docs/plant-profile-hot-pepper.md) §2), so values well above
> 350 / 23 are *desirable headroom*, not overshoot. The binding test is **uniformity at minimum
> clearance** and avg/DLI **capability**.

---

## 2. Results (sensitivity sweep, clearance 150–250 mm)

Clearance is swept because the canopy **rises toward the light** over the cycle; 150 mm is the
worst-case-uniformity / highest-intensity end. Full machine output is reproducible via the command
above; PPFD maps at 150 mm are in [`ppfd-map-*-150mm.csv`](.).

| Candidate | PPF (µmol/s) | ~Power | @150 mm avg / unif / DLI | Verdict |
|---|---:|---:|---|---|
| **A** compact-min **bar** | 100 | ~45 W @2.2 | 501 / **0.49** / 28.8 | ✗ **fails uniformity** at all clearances; avg drops below 350 by 250 mm |
| **B** preferred **bar** | 150 | ~60 W @2.5 | 751 / **0.49** / 43.3 | ✗ fails uniformity ≤200 mm; only passes ≥225 mm |
| **C** preferred **panel** | 150 | ~60 W @2.5 | 660 / **0.64** / 38.0 | ✓ **PASSES at 150 mm and every clearance** |
| **D** full-yield **bar** | 220 | ~88 W @2.5 | 1081 / **0.50** / 62.3 | ✗ fails uniformity ≤175 mm; near top of §7.8 power budget |

**Key finding (confirms DR-01):** a **single narrow bar cannot meet ≥0.6 uniformity across a
300–400 mm canopy at 150 mm clearance** — every bar candidate lands at ~0.49–0.50 there, regardless of
power. Raising the bar to ≥225 mm recovers uniformity but needs a taller grow zone (spec §8.6 allows
150–300 mm) and sacrifices intensity. A **2-D panel / multi-bar array of the same total PPF** spreads
emitters over the footprint and clears the uniformity bar at the minimum clearance.

---

## 3. Go / No-Go recommendation

**GATE RESULT: GO — Candidate C (≈150 µmol/s broad-spectrum white panel / 2-D bar array) is approved
for purchase.**

It meets every WI-PL-06 acceptance criterion in the locked envelope:

- ✅ **Average ≥350 µmol/m²/s** — 660 at 150 mm, ≥414 even at 250 mm (ample dim-down headroom).
- ✅ **Uniformity ≥0.60** — 0.64 at 150 mm, improving to 0.72 at 250 mm.
- ✅ **DLI target reachable** — covers 20–25 mol/m²/day at 16 h by dimming (full-drive 38 → ~60 % for
  DLI 23); also reaches flowering/seedling targets at lower drive.
- ✅ **Within §7.8 power/thermal budget** — ~60 W at 2.5 µmol/J PPE, comfortably inside the 50–100 W
  LED budget (hand-off to the thermal half, [WI-EE-10](../../../plan/work-items/03-electronics/WI-EE-10-thermal-budget-model.md)).

**Procurement constraints for WI-EE-01 (carry into the BOM):**

1. **Buy a panel or multi-bar 2-D array, not a single narrow bar.** A single bar of any wattage fails
   uniformity at 150 mm in this envelope.
2. Target **~140–160 µmol/s** total PPF, **≥2.5 µmol/J** PPE, **broad-spectrum white** with ≥10 % blue
   and 660 nm red (spec §7.2) — **not** a blurple-only fixture.
3. Emitter footprint should span **≈360 × 240 mm** (slightly inside the canopy) to keep edge PPFD up.
4. Must be **dimmable** (PWM or 0–10 V) — the fixture runs dimmed to the per-stage DLI.

**Rejected:** Candidates A, B, D as *single bars*. A is also under-powered. D (full-yield bar) only
suits a taller variant at ≥200 mm clearance and pushes the power budget.

---

## 4. Limitations & validation hook

- This is a **first-order** model: point-emitter Lambertian + uniform reflectance fill. It does **not**
  model real LED optics/lenses, exact emitter layout, spectral PPF distribution, or measured frame
  reflectances. It is intended to **catch infeasible geometries before purchase**, not to predict
  absolute PPFD to the photon.
- The model is **conservative on uniformity** (reflections idealized as uniform fill) yet the bars
  *still* fail — strengthening the panel recommendation.
- **Validation hook (spec §23 DR-01):** once Candidate C is acquired, the physical PPFD survey
  (WI-EE-01, [`../`](..)) measures the real PPFD grid at 25/50/75/100 % drive (the §9.9 LED dim map) and
  is compared back to these predictions. A >20 % shortfall vs. model re-opens the gate before
  CAD freeze / replication.

## 5. Sources

R1 (DLI/PPFD, PAR framing), R2 (CEA grow-light planning), R3 (PPFD/DLI ranges, high-light framing),
R14 (broad-spectrum/seedling light quality). Full list:
[`../../../docs/references.md`](../../../docs/references.md).
</content>
