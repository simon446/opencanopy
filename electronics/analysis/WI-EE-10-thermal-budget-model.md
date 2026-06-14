<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-10 — Pre-order thermal budget model (report)

**Status:** Complete (thermal half of the §23 DR-01 pre-order modeling gate).
**Revision:** PASSIVE / no-fan — see [ECO-001](ECO-001-fan-removal.md) (2026-06-14).
**Reproduce:** `python3 electronics/analysis/thermal_budget_model.py`
**Spec refs:** §7.2 (thermal), §7.8, §9.5, §17.2, §12.4, §23 (DR-01, DR-09).

> This is a first-order **lumped steady-state** model. Its job is to catch an unbuildable thermal
> design **before** the LED/driver is ordered and CAD is frozen. The physical test
> ([WI-QA-03](../../plan/work-items/05-validation-qa/WI-QA-03-thermal.md)) validates it post-build
> and re-parameterizes the assumptions from measured data (§23 DR-02).

> **No-fan revision (ECO-001).** V1 ships with **no fan** — the fan was a plant-canopy circulation
> actuator (VPD/airflow for the plant), removed in the mechanical redesign and not added back. The
> *previous* version of this model had quietly leaned on that fan for **forced-air** heatsink cooling
> (`Rth(hs-a)=0.55 °C/W`). With no fan the LED heatsink runs on **natural convection**, so its thermal
> resistance is several times higher and becomes the binding constraint. This revision sizes the LED
> against a **passive** heatsink. **Result: the committed 60 W V1 light is comfortably passively
> cooled (T_hs 53 °C, T_j 70 °C) — no fan required.** Only the 100 W full-yield variant cannot go
> fan-less.

## 1. Scope and the DR-01 gate

§23 DR-01 requires a pre-order **photometric + thermal** model before the most expensive, least
reversible parts (light + frame) are committed. This work item is the **thermal half**:

- LED junction / heatsink temperature at 50 / 60 / 80 / 100 W on a **passive** heatsink.
- Required passive heatsink thermal resistance to stay in spec.
- Upper dry-bay (electronics + RTC) temperature check **without a fan**.

The **photometric half** — PPFD map, ≥0.6 uniformity across 300–400 mm at 150 mm clearance — is
[WI-PL-06](../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md) (Plant Science),
**Done — PASS**. With this thermal half also passing, **both halves of DR-01 pass**. The PL-06
result still constrains the fixture choice: a **panel** meets ≥0.6 uniformity at the 150 mm clearance
target, whereas a single **bar** only reaches it at ≥200–225 mm — flows into the light-candidate
choice ([WI-EE-01](WI-EE-01-component-poc.md)) and the mechanical light mount.

## 2. Model

Per LED electrical power point `P_elec`:

```text
P_heat = (1 - eta_rad) * P_elec            heat that must leave the LED junction
T_hs   = T_amb + P_heat * Rth(hs-a)        heatsink base temperature (NATURAL convection)
T_j    = T_hs  + P_heat * Rth(j-hs)        LED junction temperature
T_bay  = T_amb + (P_driver_loss + P_mcu) / (rho*cp*Vdot_bay)   upper dry-bay air
```

### 2.1 Parameters and why

| Symbol | Value | Basis |
|---|---:|---|
| `T_amb` | 25 °C | Worst-case room (spec §17.2 "room 22–25 °C"). |
| `eta_rad` | 0.42 | Radiant (PAR) fraction of a good white horticultural LED at PPE ≈2.5 µmol/J (§7.2). The rest (58 %) is heat at the junction. Conservative — re-checked against the WI-PL-06 fixture's real PPE. |
| `Rth(j-hs)` | 0.5 °C/W | Aggregate junction→heatsink-base incl. thermal interface, for a **distributed** multi-emitter panel/bar (per-emitter Rth is high but they sit in parallel). |
| `Rth(hs-a)` (passive) | **1.2 / 0.8 °C/W** | **Natural-convection** finned heatsink, no fan. `1.2` = moderate large finned extrusion; `0.8` = a dedicated large passive LED cooler (vertical fins, open frame). `0.8` is the **WI-ME-05 light-mount design target** for V1. (The old `0.55` was forced-air and no longer applies — ECO-001.) |
| `T_hs` target | 60 °C | Top of the "normal" band in the spec §9.5 LED-temp table (`<60 °C` normal). |
| `T_j` design / limit | 85 / 105 °C | 85 °C preferred lifetime ceiling (Arrhenius derate headroom); 105 °C typical hard max — confirm against the chosen part's datasheet. |
| `Driver eff` | 0.90 | Constant-current LED driver efficiency → loss dumped into the **upper dry bay** where the remote driver sits (§7.2). |

## 3. Results (passive heatsink)

Junction/heatsink temperature at each LED power, for the two passive heatsink classes:

| `P_elec` | `P_heat` | Moderate (1.2 °C/W) `T_hs` / `T_j` | Large (0.8 °C/W) `T_hs` / `T_j` | Req. `Rth(hs-a)` for T_hs≤60 |
|---:|---:|---|---|---:|
| 50 W | 29.0 W | 60 °C / 74 °C — GO | 48 °C / 63 °C — GO | ≤1.21 °C/W |
| **60 W** (V1) | 34.8 W | 67 °C / 84 °C — marginal | **53 °C / 70 °C — GO** | ≤1.01 °C/W |
| 80 W | 46.4 W | 81 °C / 104 °C — marginal | 62 °C / 85 °C — marginal | ≤0.75 °C/W |
| 100 W | 58.0 W | 95 °C / 124 °C — **NO-GO** | 71 °C / 100 °C — marginal | ≤0.60 °C/W |

The committed V1 light (BOM `LED_PANEL`, **60 W**) on the large passive heatsink lands at **T_hs 53 °C
/ T_j 70 °C** — comfortably inside both the §9.5 "normal" heatsink band and the 85 °C junction
lifetime ceiling, **fan-less**.

## 4. Findings

- **Committed 60 W V1 light, passive: comfortable PASS.** On a `≤0.8 °C/W` passive heatsink the
  junction sits ~15 °C under its lifetime ceiling and the heatsink stays in the "normal" band — no
  active cooling needed. Even a moderate `1.2 °C/W` heatsink keeps the junction at/under 85 °C (T_hs
  is then slightly over the 60 °C "normal" band — acceptable but the large heatsink is preferred for
  margin).
- **80 W: passive ceiling.** Viable only with a genuinely large `≤0.8 °C/W` passive heatsink, and the
  junction lands right at the 85 °C ceiling — acceptable only if the chosen LED datasheet supports it.
  Treat 80 W as the **fan-less upper bound**; ≤60–65 W is the comfortable compact band.
- **100 W (full-yield variant): NOT viable fan-less.** Even on a `0.8 °C/W` passive heatsink the
  junction reaches ~100 °C (no lifetime margin), and on a moderate heatsink it exceeds the hard max.
  The 100 W full-yield variant therefore needs **active cooling or a substantially lower drive** and
  is **out of scope for the fan-less V1** — its cooling is a separate later decision.
- **The electronics never needed active cooling.** The ESP32-S3 + sensors dissipate ≈3 W; the bay
  check (§6) shows the dry bay holds well below limits passively. The "chip" was never the issue —
  only the 50–100 W LED, and that is handled by the heatsink, not a fan.

## 5. Go / No-Go (thermal half of DR-01) — fan-less

| Build target | Thermal verdict | Condition |
|---|---|---|
| **Compact V1 (committed 60 W)** | **GO, fan-less** | Passive heatsink `Rth(hs-a) ≤ 0.8 °C/W` (T_hs 53 °C, T_j 70 °C). A moderate `≤1.0 °C/W` still keeps T_j ≤85 °C with less heatsink margin. |
| **Compact V1, stretched to 80 W** | **GO with a large heatsink** | Needs `≤0.8 °C/W` passive and lands T_j at the 85 °C ceiling; only with datasheet headroom. |
| **Full-yield variant (100 W)** | **NO-GO fan-less** | Re-introduce active cooling, or cap drive to ~65–70 W, before this variant is offered. Decision deferred (ECO-001). |

**Mitigations if a point is marginal:**

1. Larger / lower-Rth passive heatsink (drives `T_hs` and `T_j` down linearly) — the primary lever now.
2. Lower max LED drive current — the cleanest lever; 60 W is comfortably in spec.
3. Remote driver placement with its own open vent path (keeps the 6–9 W driver loss out of the
   LED/RTC region — see §6).
4. Vent geometry that gives the heatsink unobstructed vertical airflow (natural convection is
   orientation-sensitive: vertical fins, open top and bottom).

## 6. Electronics-bay & RTC check (deliverable 3; DR-05, DR-09) — no fan

The upper dry bay carries the MCU/sensors (≈3 W typ) **and** the remote LED driver loss (5–9 W at
50–80 W LED). Total bay heat 8–11 W. Holding the bay-air rise to ≤25 °C (bay ≤50 °C) needs only
**<1 CFM-equivalent** of through-flow (model §"Upper dry-bay") — comfortably supplied by **open-frame
natural convection** on the dry side, **provided** the bay is genuinely open (intake low, exhaust
high) and the driver is not in a dead pocket. Margins to component ratings at a 50 °C bay:

- **Battery-backed RTC** (DS3231, §16.1 / DR-05): commercial 0–70 °C → ample margin, but DS3231
  oscillator drift is temperature-dependent, so **place the RTC away from the driver** and the
  heatsink plume to keep photoperiod stable over the months-long cycle.
- **ESP32-S3**: ambient max 85 °C → fine.
- **Electrolytic caps / driver**: derate hardest with heat — the 50 °C budget keeps them in life.

**DR-09 layering note (now more important with no fan):** the firmware's only temperature input is
the shaded *air* sensor — a weak proxy for LED junction. With no fan there is also **no fan-assist
cooling layer**, so the protection stack is: **(1) the LED driver's own thermal foldback = primary
LED protection**, **(2) the firmware air-temp derate (§9.5) = secondary**. The §9.5 over-temp action
"force fan high" is now a **no-op** (no fan) — flagged to firmware ([ECO-001 §firmware](ECO-001-fan-removal.md));
the real over-temp mitigation (LED off/derate) is unaffected. An optional LED-heatsink NTC (§7.2,
pin-map `LED_HS_NTC`) lets firmware act on the real `T_hs` bands in §9.5 and is **recommended** if the
80 W stretch build is ever offered.

## 7. Hand-offs

- **Passive heatsink target** `Rth(hs-a) ≤ 0.8 °C/W` (natural convection, vertical fins) →
  [WI-ME-05 light mount](../../plan/work-items/04-mechanical/WI-ME-05-light-mount.md). This replaces
  the old forced-air `≤0.75/0.60 °C/W` target and is the single most important mechanical hand-off.
- **No fan / no fan mount.** [WI-ME-06 fan mount](../../plan/work-items/04-mechanical/WI-ME-06-fan-mount.md)
  is obsolete for V1; the open-frame vent geometry (open dry-bay top/bottom + unobstructed heatsink
  airflow) now carries both bay and heatsink cooling — a mechanical requirement, not a fan.
- **Driver loss budget** 5–9 W into the bay → [power budget WI-EE-02](power-budget.md).
- **Firmware:** the §9.5 "force fan high" branch is a no-op; LED off/derate is the live mitigation.
- **Combined DR-01 sign-off:** photometric half [WI-PL-06](../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md)
  **Done — PASS**; with this passive thermal half passing, **both halves of DR-01 pass** for the
  committed 60 W fan-less light. The light/CAD-freeze gate is clear, subject to a panel @150 mm (or a
  bar @≥200–225 mm, PL-06) **and** a `≤0.8 °C/W` passive heatsink.
