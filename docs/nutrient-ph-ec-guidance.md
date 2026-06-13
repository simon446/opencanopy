# Nutrient, pH & EC Guidance (OpenCanopy V1)

**Track:** Plant Science · **Work item:** [WI-PL-05](../plan/work-items/01-plant-science/WI-PL-05-nutrient-guidance.md)
· **Spec refs:** §5.7, §5.8 · **Status:** Done
· **Depends on:** [WI-PL-01](plant-profile-hot-pepper.md)

> **V1 does NO automatic dosing.** This is a **written grow-guide deliverable**, not a control loop.
> There are no pH or EC sensors in the baseline build and no nutrient/pH pumps (spec §4.2, §5.7, §5.8).
> Everything below is a **manual** regimen the grower follows. Content lands in the grow guide
> ([WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md)).

The default V1 is a **potting-mix + drip** system, **not** hydroponic/DWC — hydroponics adds
significant pH/EC calibration and maintenance burden and is explicitly **not** the V1 baseline (§5.7,
§5.8). A hydroponic variant is documented in the appendix for builders who accept that burden.

---

## 1. Manual feeding schedule — potting-mix default (spec §5.7)

Use a **complete fertilizer formulated for container vegetables / peppers / tomatoes.** The guiding
principle is **avoid over-fertilization** — container media has little buffering, and excess salts harm
roots and can worsen blossom-end rot via disrupted water/calcium uptake. [R7, R15, R16]

| Plant phase (→ lifecycle stage) | Feeding guidance |
|---|---|
| **Seedling establishment** (S1) | **Start low-strength** feeding only **after seedlings are established** — not at emergence. Dilute below the label's vegetable rate. [R6, R7] |
| **Vegetative** (S2) | Regular **complete** fertilizer at a moderate container-vegetable rate; favor balanced N for canopy/root build. [R4] |
| **After flowering / fruit set** (S3→S4) | Switch to / increase a **potassium-supported fruiting feed** — only **after** flowers/fruit set, not before. [R4] |
| **Throughout, if soluble + drainage** | **Monthly flush / refresh:** water heavily with plain water to leach accumulated salts, since the pot drains (§5.5). [R7, R10] |

Key rules (spec §5.7):

- **Do not** front-load potassium/"bloom" feed before fruit set — it does not help and adds salt load.
- **Do not** skip the flush if you use soluble fertilizer; salt accumulation is the main failure mode
  of container feeding over a 3–4 month cycle. [R7]
- Match feed strength to the plant's stage and visible vigor, not a fixed calendar dose.

### 1.1 Simplest-operation option (spec §5.7)

For the lowest-maintenance path: use a **slow-release (controlled-release) container fertilizer mixed
into the media**, and keep **plain water in the reservoir.** This avoids dosing soluble feed through
the irrigation system entirely and is the recommended default for a hands-off appliance. Top-dress
slow-release per its labeled interval; still flush occasionally if salts visibly accumulate.

> **Drift caveat (spec §23 DR-06):** even with a correct manual regimen, substrate EC/pH **drift**
> over the long cycle with nothing observing it, and such drift can be **misread as a control failure**
> during the grow trial. The trial unit *may* carry a single optional **EC probe** (header exists,
> §4.3) purely to *observe* drift — there is still **no dosing.** Decision on promoting that probe is
> tracked in DR-06 / the [risk register](risk-register.md); it is **not** part of the baseline BOM.

---

## 2. pH & EC — potting-mix default (spec §5.8)

- **No continuous pH/EC sensors** in the baseline.
- **Target potting-media pH: 6.0–6.8.** [R4, R10]
- Optional **manual** test guidance only: an occasional cheap soil pH test or a runoff pH/EC check is
  *allowed* but **not required**; there is nothing to automate against.
- **Avoid repeated heavy fertilization without drainage/flush** — this is the main driver of pH/EC
  drift in container media. [R7, R10]

---

## 3. Appendix — optional hydroponic variant (spec §5.8) — NOT the V1 baseline

This appendix exists for builders who choose a hydroponic/DWC variant. **It is out of V1 baseline
scope:** pH/EC **automation is explicitly excluded** from V1 (§4.2), and a hydroponic build carries a
**manual** pH/EC testing and calibration burden the default avoids.

| Parameter | Target | Source |
|---|---|---|
| Nutrient **solution** pH | **5.5–6.0** | R11 |
| **Root-zone** pH | **≈6.0–6.5** | R11 |
| Pepper **EC** reference | **0.8–1.8 mS/cm** (OSU hydroponic table) | R11 |
| Meter calibration | **Weekly** pH/EC meter calibration **if** pH/EC automation is ever added | R12 |

Caveats (spec §5.8):

- The 0.8–1.8 mS/cm EC band is a **reference**; commercial greenhouse pepper guidance can run higher
  depending on stage and system. [R11]
- pH/EC meters **drift and require regular calibration** — this maintenance burden is precisely why
  hydroponics is **not** the V1 baseline. [R12]
- A hydroponic variant requires **separate documentation** and manual pH/EC testing (§5.7).

---

## 4. Traceability

| Consumes this guidance | Via |
|---|---|
| Maintenance & grow guide (feeding schedule, pH/EC, flush) | [WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md) |
| EC-drift-observation decision (trial unit) | [risk-register.md](risk-register.md) DR-06; spec §23 |

## 5. Sources

R4 (pepper nutrition & pH), R6 (seedling establishment), R7 (container fertilizer caution & flush),
R10 (pH, drainage), R11 (hydroponic pH/EC reference values), R12 (nutrient-solution management, meter
calibration limits), R15 & R16 (over-fertilization / inconsistent uptake → blossom-end rot). Full
list: [`references.md`](references.md).
</content>
