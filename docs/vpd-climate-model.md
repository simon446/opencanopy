# VPD & Climate-Response Model (OpenCanopy V1)

**Track:** Plant Science · **Work item:** [WI-PL-04](../plan/work-items/01-plant-science/WI-PL-04-vpd-climate-model.md)
· **Spec refs:** §5.3, §5.4, §9.7 · **Status:** Done
· **Depends on:** [WI-PL-01](plant-profile-hot-pepper.md)

Specifies the **VPD calculation** and the open-frame **climate-response rules** (fan boosts, LED
derating, RH guardrails, temperature response) so the climate controller
([WI-FW-06](../plan/work-items/02-firmware/WI-FW-06-climate-controller.md)) behaves like a
**monitor-and-nudge** system, not an HVAC.

> **Non-goal (spec §3.6, §5.2, §5.3):** the firmware **does not hold an air-temperature setpoint** and
> **cannot cool below ambient.** A 22–25 °C room is the accepted operating assumption. The only heat
> source the device controls is the LED; the fan provides circulation and heat dispersion, not active
> cooling. VPD is the **internal decision variable**; only a coarse climate **status** is exposed to
> the user (§5.4). [R13]

---

## 1. VPD definition & formula (spec §5.4)

VPD (vapor-pressure deficit) is the difference between how much water vapor the air **could** hold at
its temperature and how much it **does** hold. Higher VPD → stronger transpiration pull; very low VPD
→ stagnant, disease-prone, low transpiration. [R13]

V1 computes **air VPD** from the air temperature and relative humidity read by the SHT-class sensor
(spec §7.5). The exact formula firmware must implement:

```text
# Saturation vapor pressure (Tetens equation), kPa, T in °C:
SVP(T) = 0.6108 × exp( 17.27 × T / (T + 237.3) )

# Air vapor-pressure deficit, kPa:
VPD(T, RH) = SVP(T) × (1 − RH/100)        # RH in percent, 0..100
```

Implementation notes:

- This is **air VPD** (a.k.a. VPDair), computed from a single air temp + RH. V1 has no leaf-temperature
  sensor, so no leaf-offset term is applied — consistent with the spec's "temp + RH → kPa" definition.
  If a leaf-temperature input is ever added, substitute leaf temperature in `SVP` for a true
  leaf-to-air VPD; until then, document readings as air VPD.
- Use `exp` from the platform math library; the constants `0.6108`, `17.27`, `237.3` are the standard
  Tetens coefficients over the 0–50 °C range relevant here.
- Clamp `RH` to `[0, 100]` before use; a sensor returning out-of-range RH is a sensor-fault condition
  (spec §7.6), not a valid VPD input.

### 1.1 Reference test vectors (for the WI-FW-06 deterministic unit test)

Computed from the formula above; round your implementation's output to **4 decimal places (kPa)** and
compare. These span the decision bands in §2.

| T (°C) | RH (%) | SVP(T) (kPa) | **VPD (kPa)** | Falls in band |
|---:|---:|---:|---:|---|
| 24 | 90 | 2.9839 | **0.2984** | <0.4 — too humid |
| 24 | 85 | 2.9839 | **0.4476** | 0.5–1.2 (low edge) |
| 18 | 75 | 2.0640 | **0.5160** | 0.5–1.2 |
| 20 | 60 | 2.3383 | **0.9353** | 0.5–1.2 |
| 24 | 65 | 2.9839 | **1.0444** | 0.5–1.2 |
| 22 | 60 | 2.6439 | **1.0576** | 0.5–1.2 |
| 22 | 55 | 2.6439 | **1.1898** | 0.5–1.2 |
| 25 | 60 | 3.1678 | **1.2671** | 1.2–1.6 — dry air |
| 26 | 55 | 3.3614 | **1.5126** | 1.2–1.6 |
| 25 | 50 | 3.1678 | **1.5839** | 1.2–1.6 |
| 30 | 40 | 4.2431 | **2.5458** | >1.6 — stress |

SVP anchors for an isolated `SVP(T)` unit test: SVP(20)=2.3383, SVP(25)=3.1678, SVP(30)=4.2431 kPa.

---

## 2. VPD decision table (spec §5.4)

VPD is interpreted in four bands, each driving fan / watering / LED actions. (Per-stage VPD *targets*
live in the [lifecycle profile](plant-profile-hot-pepper.md) §2; this table is the *response* logic.)

| VPD (kPa) | Interpretation | Action |
|---:|---|---|
| **< 0.4** | Air too humid / low transpiration | Fan **increase**; **avoid watering** unless substrate is dry |
| **0.5–1.2** | Normal productive range | Normal control |
| **1.2–1.6** | Dry air / high transpiration | **Watch moisture**; shorter dryback allowed |
| **> 1.6** | Stress risk | Fan may not help; **alert if persistent**; **avoid LED heat increase** |

> The 0.4–0.5 gap between the "too humid" and "normal" rows is the spec's wording (§5.4); treat
> `[0.4, 0.5)` as the bottom edge of the normal/productive band (no humid-specific action, no alert).

---

## 3. RH guardrails (spec §5.4)

RH is a secondary guardrail layered on top of VPD; V1 has **no humidification** and only warns +
adjusts fan/light/watering conservatively (§5.4).

| RH (%) | Action |
|---:|---|
| **> 85 sustained** | Climate **amber/red**; fan **high**; disease-risk warning [R13] |
| **70–85** | Accept if short-term; **increase airflow** |
| **55–70** | **Preferred** flowering/fruiting range |
| **< 40** | **Dry-air warning** if VPD also high |

---

## 4. Temperature response (spec §5.3)

The firmware monitors temperature and uses **fan + LED derating** to avoid adding avoidable heat
stress. It does **not** pretend it can cool the plant below room temperature (§5.3).

| Condition | Action |
|---|---|
| **< 16 °C air** | Climate LED amber; fan minimum only; **do not** increase watering (slow uptake) |
| **16–20 °C** | Accept; possibly slower growth; **no active heating** in V1 |
| **20–25 °C** | Normal open-room operating band |
| **25–28 °C** | Normal if transient; **increase fan slightly** during lights-on |
| **28–30 °C** | Climate amber if sustained; **fan high**; prevent additional LED heat if rising |
| **30–32 °C** | Fan high; **reduce LED 20–40 %** if LED heat is contributing |
| **> 32 °C** | Climate **red**; **reduce LED 40–70 %**; pump only if substrate is dry; **log heat fault** |
| **> 35 °C** | **Critical over-temp**; LED **off or minimum**; fan high; **system fault** if sustained |

### 4.1 Fruit-set protection (spec §5.3)

- **Avoid sustained canopy temperatures above 32 °C.** Published pepper guidance repeatedly flags poor
  fruit set / blossom drop at and above this range. [R4, R5] This is enforced via LED derating (rows
  above), **not** by cooling.
- A constant 22–25 °C room is acceptable for V1 and does **not** justify an enclosure. [spec §2.3]
- Flowering may improve if nights naturally fall to **18–22 °C**, but this is an optimization, **not** a
  V1 requirement. [R5]
- **Never make a watering decision on temperature alone** — always verify substrate moisture first
  (§5.3; cross-check the [watering model](watering-model.md)). [R7, R8]

---

## 5. Fan-control mapping (spec §9.7, for firmware)

The plant-science response above maps onto the firmware fan duties (spec §9.7), restated here so the
climate-model and firmware tables can be cross-checked. Per-stage **fan minimums** (lights-on /
lights-off) are in spec §9.7 and firmware [WI-FW-06]. **Fan boosts** are additive on top of the
stage minimum:

| Condition | Fan action |
|---|---|
| RH > 75 % (lights on) | +15 % |
| RH > 85 % | +30 %, amber climate |
| VPD < 0.5 kPa | +20 % |
| Temp > 28 °C | +20 % |
| Temp > 30 °C | +40 % |
| Temp > 32 °C | **max fan**, LED derate |
| Fan tach missing | `FAN_FAULT` |

> Avoid blasting seedlings directly — the fan must produce gentle **circulation**, not wind stress
> (§5.9, §9.7).

---

## 6. Traceability

| Consumes this model | Via |
|---|---|
| Climate/fan controller + VPD calc (deterministic unit test on §1.1 vectors) | [WI-FW-06](../plan/work-items/02-firmware/WI-FW-06-climate-controller.md) (spec §9.7) |
| VPD calculator unit test | §10.2 "VPD calculator — temp/RH to kPa" |
| LED thermal derating | [WI-FW-04](../plan/work-items/02-firmware/WI-FW-04-light-controller.md) (spec §9.5) |
| Grow-guide climate section | [WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md) |

## 7. Sources

R4 & R5 (pepper temperature, fruit-set/blossom-drop thresholds), R13 (VPD logic, humidity disease &
transpiration framing), R7 & R8 (don't water on temperature alone). Full list:
[`references.md`](references.md).
</content>
