# Watering Model — Moisture Thresholds & Windows (OpenCanopy V1)

**Track:** Plant Science · **Work item:** [WI-PL-03](../plan/work-items/01-plant-science/WI-PL-03-watering-model.md)
· **Spec refs:** §5.5, §5.6, §9.6 · **Status:** Done
· **Depends on:** [WI-PL-01](plant-profile-hot-pepper.md)

Defines the substrate, the **moist-but-aerated** watering philosophy, the per-stage dry/wet
thresholds, pulse sizes, watering windows, and daily safety caps the irrigation controller
([WI-FW-05](../plan/work-items/02-firmware/WI-FW-05-irrigation-controller.md)) enforces.

This document owns the **biological targets only.** The raw-capacitive→normalized calibration is a
hardware task ([WI-EE-08](../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md); spec §7.6, §9.9).

> **Watering philosophy (spec §5.6):** V1 does **not** keep the substrate constantly wet. It maintains
> **moist-but-aerated** conditions with a controlled dryback between pulses. Consistent moisture
> protects flower/fruit retention and reduces blossom-end rot, while avoiding the waterlogging that
> harms roots. [R5, R8, R15, R16]

---

## 1. Substrate & pot (spec §5.5)

**Recommended V1 medium:**

- High-quality **peat- or coco-based** potting mix. [R7, R9]
- **Perlite** (or similar) aeration amendment for drainage and root oxygen. [R9]
- A **drainage-capable pot** — never a fully sealed, no-drainage pot.
- Optional internal wicking mat/reservoir is allowed, but **not** a sealed no-drainage planter.
- **Avoid native garden soil indoors** (drainage, compaction, pests). [R9]

**Pot constraints (hard):**

| Constraint | Value | Source |
|---|---|---|
| Pot volume — compact V1 baseline | **8–10 L** | §5.5, §3.3 |
| Pot volume — full-yield variant | 12–19 L | §5.5, §3.3 |
| Minimum for a Reaper-class plant | **≥8 L** (below this only if ornamental/low-yield is explicitly accepted) | §5.5 [R9] |
| Drainage | Must drain into a controlled tray / internal basin | §5.5 |
| Removability | Removable **without disconnecting electronics** | §5.5, §8.4 |
| Standing water | Must **not** sit in stagnant water unless designed as a self-watering planter **with an air gap** | §5.5 |

> Container-grown plants **dry quickly** and need frequent but not excessive watering — the threshold
> model below is built around that behavior, not around keeping a reservoir-fed pot permanently wet.
> [R7, R8]

---

## 2. Moisture states (spec §5.6)

The controller classifies normalized moisture into four states and acts accordingly:

| State | Meaning | Action |
|---|---|---|
| **Too wet** | Above wet threshold for a long period | **Block watering**; amber moisture LED; increase fan if humid |
| **Target** | Within `[dry, wet]` band | No action |
| **Dry** | Below dry threshold | Dose water in **pulses** during the watering window |
| **Critically dry** | Well below dry threshold | Dose with stricter observation; emergency watering allowed any time; red/amber LED if not recovering |

---

## 3. Per-stage thresholds & pulses (spec §5.6)

> **These percentages are normalized wet/dry calibration units, NOT raw capacitive ADC percentages.**
> The firmware must **never** assume a raw capacitive ADC reading maps directly to volumetric water
> content without per-media, per-probe calibration (spec §5.6, §7.6). The mapping
> `normalized = map(raw, raw_dry_media, raw_saturated_media, 0..100)` is established during hardware
> bring-up (WI-EE-08), not here.

| Stage | Dry threshold | Wet threshold | Pulse size target | Recheck delay |
|---|---:|---:|---:|---:|
| Seedling | 35 % | 55 % | 20–50 mL | 15–20 min |
| Vegetative | 30 % | 55 % | 50–150 mL | 20–30 min |
| Flowering | 35 % | 60 % | 75–200 mL | 20–30 min |
| Fruiting | 35 % | 60 % | 100–250 mL | 20–30 min |

(Units = normalized calibration %, per the note above.) Pulse sizes scale with the larger root system
and higher transpiration of later stages. After each pulse the controller **waits `recheck_delay`
then remeasures** before considering another pulse — water must travel through the media, so
back-to-back dosing without rechecking risks overwatering. [R7, R8; spec §5.6/§9.6]

Germination (S0) has no automated watering threshold row: the media is kept warm and moist by hand /
seed-starting method until emergence (spec §5.2), after which the seedling row applies.

---

## 4. Daily safety caps (spec §9.6) — caps, not targets

> **These are safety ceilings, not consumption targets.** They bound how much the device may dose in a
> day so a stuck-dry sensor, a calibration error, or a clog cannot drain the reservoir into the pot.
> They are **not** "how much the plant should drink." Actual consumption is driven by the threshold
> logic in §3; these only cap the worst case. They are revisited after grow trials (spec §9.6).

| Stage | Daily max (cap) |
|---|---:|
| Seedling | 250 mL/day |
| Vegetative | 800 mL/day |
| Flowering | 1200 mL/day |
| Fruiting | 1800 mL/day |

When `daily_watered_ml + next_dose > daily_max(stage)`, the controller **withholds the dose** and
flags a watering-limit / pump-fault condition (spec §9.6 decision loop) rather than exceeding the cap.

Related pump-safety bounds (spec §9.6, owned by firmware but stated here for completeness): absolute
single-run ≤ **30 s** (or lower per calibrated flow), **≤3 pulses/hour**, pulses/day stage-dependent,
and pump **off on crash/reset** via hardware pull-down.

---

## 5. Watering windows (spec §5.6)

Time-of-day rules the controller enforces, expressed relative to the light period:

- **Prefer watering during the light period.**
- **Prefer the first 60–70 % of the light period** — water early so the substrate works through a
  dryback before lights-off. [R8, R10]
- **Avoid routine watering in the last 2 hours before lights-off** (reduces overnight wet-feet /
  disease risk). [R10, R13]
- **Emergency watering is allowed at any time** — including at night — when the plant is
  **critically dry** (below the critical-dry threshold).
- **Hard interlocks — never water when:** reservoir is low, leak sensor is active, or a pump-fault
  state is active (spec §5.6, §9.6).
- **Never water continuously** — always use pulses and remeasure (§3).

### Window logic (unambiguous, for unit testing in WI-FW-05)

Let `f` = fraction of the light period elapsed (0.0 at lights-on, 1.0 at lights-off), and let
`hours_to_off` = hours remaining until lights-off.

```text
within_watering_window(state, f, hours_to_off, light_on):
    if state == CRITICALLY_DRY:
        return true                      # emergency: any time, day or night
    if not light_on:
        return false                     # routine watering only during light period
    if f > 0.70:
        return false                     # past first 70% of light period
    if hours_to_off < 2.0:
        return false                     # last 2 h before lights-off
    return true                          # routine watering allowed
```

Notes for the implementer:

- The `f > 0.70` and `hours_to_off < 2.0` rules are **both** applied; for a 16 h photoperiod the
  70 % rule (≈11.2 h in, ≈4.8 h before off) is the binding constraint, but on shorter fallback
  photoperiods the 2 h rule can bind first. Keep both.
- `CRITICALLY_DRY` bypasses **all** window rules but **not** the hard interlocks in §5 (low reservoir
  / leak / pump fault still block the pump — see the §9.6 decision-loop ordering).
- This function gates *routine* dosing only; the dry/critical classification and dose sizing come from
  §2–§3.

---

## 6. Traceability

| Consumes this model | Via |
|---|---|
| Irrigation controller (decision loop, thresholds, caps, windows) | [WI-FW-05](../plan/work-items/02-firmware/WI-FW-05-irrigation-controller.md) (spec §9.6) |
| Raw→normalized moisture calibration | [WI-EE-08](../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md) (spec §7.6, §9.9) |
| Pump dose calibration (`ml_per_second`) | [WI-FW-05] / `scripts/pump_calibration.py` (spec §7.3) |
| Grow-guide watering section | [WI-DOC-06](../plan/work-items/06-documentation/WI-DOC-06-maintenance-grow-guide.md) |

## 7. Sources

R5 (moisture & blossom drop), R7 (container moisture behavior), R8 (container watering, avoid
waterlogging & full dry-out), R9 (container size, drainage), R10 (drip/soaker watering, wet-leaf
disease avoidance), R13 (humidity/disease timing), R15 & R16 (blossom-end rot from dry/wet swings).
Full list: [`references.md`](references.md).
</content>
