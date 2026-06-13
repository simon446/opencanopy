# Hot-Pepper Lifecycle Profile (OpenCanopy V1)

**Track:** Plant Science · **Work item:** [WI-PL-01](../plan/work-items/01-plant-science/WI-PL-01-lifecycle-profile.md)
· **Spec refs:** §5.1, §5.2, §2 · **Status:** Done

> **This file is the single source of truth for "what the plant wants."** Firmware (the plant-profile
> module, [WI-FW-03](../plan/work-items/02-firmware/WI-FW-03-plant-profile.md)) must encode these
> values verbatim and must **not** hardcode setpoints that diverge from this document. If a target
> here is wrong, fix it here first, then re-derive the firmware table — never the other way around.

The target crop is a Carolina Reaper-class superhot, treated as a **hot-pepper profile for
*Capsicum chinense***, with environmental targets informed by broader pepper (*Capsicum* spp.) and
controlled-environment-agriculture (CEA) research (spec §2). V1 ships this as a **fixed, no-config
recipe**: the user never selects a species or growth program.

Every numeric target below carries a source ID (`R1`–`R17`) traceable to spec §2.2 and mirrored in
[`references.md`](references.md). Where a value is an engineering choice derived from those sources
(rather than quoted from one), it is marked **[design]** with the sources it was derived from.

---

## 1. Growth stages (spec §5.1)

The firmware implements a built-in lifecycle as an ordered set of stages. Because V1 has **no
camera**, **stage detection is purely time-based**: the controller tracks *plant age in days* from the
grow-cycle start and selects the stage whose age window contains it. The hidden service button resets
the grow-cycle age to zero — this is equivalent to starting a timer when a new plant is inserted and
is **not** considered user "configuration" (spec §5.1).

| Stage | Code | Age window | Trigger | Main objective |
|---|---|---:|---|---|
| Germination | S0 | day 0–21 | Grow-cycle reset (service button) | Warm, moist media until emergence [R6, R17] |
| Seedling | S1 | day 21–56 | Time-based | Compact growth, avoid stretch [R6, R14] |
| Vegetative | S2 | day 56–100 | Time-based (plant size optional, manual) | Strong canopy and root system [R4, R5] |
| Flowering | S3 | day 100–140 | Time-based (first flower may be noted manually, optional) | Stable moisture, pollination support [R5, R15] |
| Fruiting / ripening | S4 | day 140+ | Time-based | Stable water, high light, avoid heat stress [R4, R5, R16] |
| Maintenance / overwinter | S5 | optional | **Manual dev setting only** | Lower light/water, survival mode [design ← R4] |

Notes:

- **Boundaries are inclusive-low, exclusive-high** in days: S0 = `[0, 21)`, S1 = `[21, 56)`,
  S2 = `[56, 100)`, S3 = `[100, 140)`, S4 = `[140, ∞)`. This makes stage selection a single
  deterministic lookup on `age_days`, unit-testable in
  [WI-FW-03](../plan/work-items/02-firmware/WI-FW-03-plant-profile.md) (and §10.2 "Plant profile —
  stage selection by age").
- Durations are **approximate biological guidance**, not hard agronomic guarantees — superhot
  *C. chinense* is famously slow to germinate and slow to set fruit [R17]. The time windows are chosen
  to keep the *control regime* (light/water/climate) appropriate for the plant's likely state, not to
  predict the plant exactly.
- **S5 is never entered automatically.** It exists only for developers/overwintering and must be set
  via a hidden dev path, never by the age timer.

### 1.1 `TRANSPLANT_PROFILE` build flag (spec §5.1)

The **public default build includes all stages** (S0–S4) and starts at day 0 / S0.

A builder who only ever starts from a purchased transplant can flash `TRANSPLANT_PROFILE=true`. Its
defined behavior:

- **Skip S0 (Germination) and S1 (Seedling).** The grow cycle starts at the **S2 Vegetative** regime.
- Concretely, on grow-cycle reset the age clock is **initialized to day 56** (the S1→S2 boundary)
  rather than day 0, so all downstream age-based logic (stage selection, light ramp, watering
  thresholds) continues to work unchanged.
- All S2–S4 targets and transitions are identical to the default build.
- The flag is a **compile-time** option, not a runtime user setting — consistent with the no-config
  appliance philosophy (§3.5).

---

## 2. Per-stage environmental targets (spec §5.2)

These are **biological targets**. Because V1 is **non-enclosed** (§3.6), the firmware treats **air
temperature and RH as *monitored* conditions, not actively controlled setpoints** — see §3 below and
the [VPD & climate model](vpd-climate-model.md). **Light and watering are the only actively controlled
variables.** A 22–25 °C room is an accepted operating assumption (spec §2.3, §3.6).

| Stage | Preferred air temp | Open-room assumption | RH target | VPD target (kPa) | DLI target (mol·m⁻²·d⁻¹) | Photoperiod | PPFD at canopy (µmol·m⁻²·s⁻¹) |
|---|---|---|---:|---:|---:|---:|---:|
| **S0** Germination | 26–30 °C *media* | Room 22–25 °C OK but slower; optional external heat mat recommended | 65–80 % | n/a | 0–6 (after emergence) | 0–16 h | 0–100 |
| **S1** Seedling | 23–27 °C day, 18–22 °C night | 22–25 °C acceptable | 60–75 % | 0.5–0.9 | 8–12 | 16 h | 140–210 |
| **S2** Vegetative | 22–28 °C | 22–25 °C ideal/acceptable | 55–75 % | 0.7–1.2 | 14–20 | 16 h | 245–350 |
| **S3** Flowering | 22–27 °C day, 18–22 °C night preferred | 22–25 °C OK; avoid LED raising canopy >28–30 °C | 55–70 % | 0.8–1.2 | 18–24 | 16 h | 315–420 |
| **S4** Fruiting | 22–27 °C day, 18–22 °C night preferred | 22–25 °C OK; prioritize stable moisture + high light | 55–70 % | 0.8–1.3 | 20–25 | 16 h | 350–435 |

Source mapping for the table:

- **Temperature bands** — pepper warm-season ranges and fruit-set sensitivity: [R4, R5]. Night
  preference 18–22 °C is an optimization, not a V1 requirement (§5.3). Carolina-Reaper-specific warm
  germination context: [R17].
- **RH bands** — humidity/disease and transpiration framing: [R13]; flowering/fruiting 55–70 %
  preferred range is a **[design]** narrowing from [R13].
- **VPD targets** — derived from temp+RH via the [VPD model](vpd-climate-model.md); productive
  band framing: [R13]. **[design ← R13]**.
- **DLI / photoperiod / PPFD** — DLI/PPFD as the engineering quantities and high-light framing for
  fruiting peppers: [R1, R2, R3]. Seedling light-quality response (blue/UV-A/far-red): [R14]. The
  per-stage DLI→PPFD numbers are internally consistent at a 16 h photoperiod (see §2.1) and are
  developed in detail in the [light/DLI targets](dli-targets.md) deliverable
  ([WI-PL-02](../plan/work-items/01-plant-science/WI-PL-02-light-dli-targets.md)).

### 2.1 DLI ⇄ PPFD consistency (spec §5.2 formula)

```text
DLI = PPFD × photoperiod_hours × 0.0036
PPFD = DLI / (photoperiod_hours × 0.0036)
```

At the V1 vegetative-and-later photoperiod of **16 h**, `PPFD = DLI / 0.0576`. Each stage's PPFD band
is the DLI band run through this formula, confirming the table is self-consistent:

| Stage | DLI band | DLI band ÷ 0.0576 | PPFD band in table |
|---|---:|---:|---:|
| Seedling | 8–12 | 139–208 | 140–210 ✓ |
| Vegetative | 14–20 | 243–347 | 245–350 ✓ |
| Flowering | 18–24 | 313–417 | 315–420 ✓ |
| Fruiting | 20–25 | 347–434 | 350–435 ✓ |

The worked fruiting example from §5.2 (DLI 23, 16 h → ≈399 µmol·m⁻²·s⁻¹) is reproduced by the
calculator in [WI-PL-02](../plan/work-items/01-plant-science/WI-PL-02-light-dli-targets.md). [R1]

> **Germination photoperiod (S0):** lighting is *optional and low* until emergence; after emergence the
> seedling regime (16 h, low intensity, see firmware §9.5) takes over. The 0–6 DLI / 0–100 PPFD band
> reflects "off or gentle" rather than a controlled target. [R6, R17]

---

## 3. Open-frame control caveat (spec §3.6, §5.2, §5.3)

This profile is consumed by a **monitor-and-nudge** appliance, not an HVAC:

- The firmware **does not hold an air-temperature setpoint** and **cannot cool below ambient.** The
  temperature/RH/VPD columns above are *targets to monitor against and report*, plus inputs to
  fan-boost and LED-derate decisions — not closed-loop setpoints. Detailed rules live in the
  [VPD & climate model](vpd-climate-model.md). [R13, spec §5.3/§5.4]
- **Actively controlled:** light schedule + intensity (toward the DLI/PPFD targets) and watering
  (toward the moisture targets in the [watering model](watering-model.md)).
- **Monitored / nudged only:** temperature (fan boost, LED derate), humidity/VPD (fan boost,
  watering caution), with status surfaced on the Climate LED.
- **Fruit-set protection:** avoid sustained canopy temperature **>32 °C**; published pepper guidance
  repeatedly flags poor fruit set / blossom drop at and above this range. [R4, R5] — enforced via LED
  derating in the climate model, not by cooling.

---

## 4. Downstream consumers (traceability)

| This profile feeds | Via |
|---|---|
| Firmware plant-profile module — stage table, age windows, `TRANSPLANT_PROFILE` | [WI-FW-03](../plan/work-items/02-firmware/WI-FW-03-plant-profile.md) |
| Light/DLI targets + calculator | [WI-PL-02](../plan/work-items/01-plant-science/WI-PL-02-light-dli-targets.md) → [`dli-targets.md`](dli-targets.md), [`dli_calculator.py`](../scripts/dli_calculator.py) |
| Watering thresholds & windows | [WI-PL-03](../plan/work-items/01-plant-science/WI-PL-03-watering-model.md) → [`watering-model.md`](watering-model.md) |
| VPD / climate response | [WI-PL-04](../plan/work-items/01-plant-science/WI-PL-04-vpd-climate-model.md) → [`vpd-climate-model.md`](vpd-climate-model.md) |
| Nutrient / pH / EC grow-guide content | [WI-PL-05](../plan/work-items/01-plant-science/WI-PL-05-nutrient-guidance.md) → [`nutrient-ph-ec-guidance.md`](nutrient-ph-ec-guidance.md) |

## 5. Source list

Full bibliography in [`references.md`](references.md) (spec §2.2). IDs used in this document:
**R1–R3** (DLI/PPFD, high-light framing), **R4, R5** (pepper temperature, fruit set, pH),
**R6** (seed starting, germination warmth), **R13** (VPD/humidity), **R14** (seedling light quality),
**R15, R16** (blossom-end rot / consistent moisture), **R17** (Carolina Reaper specifics).
</content>
</invoke>
