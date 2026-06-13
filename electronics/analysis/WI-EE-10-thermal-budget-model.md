<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-10 — Pre-order thermal budget model (report)

**Status:** Complete (thermal half of the §23 DR-01 pre-order modeling gate).
**Reproduce:** `python3 electronics/analysis/thermal_budget_model.py`
**Spec refs:** §7.2 (thermal), §7.8, §9.5, §17.2, §12.4, §23 (DR-01, DR-09).

> This is a first-order **lumped steady-state** model. Its job is to catch an unbuildable thermal
> design **before** the LED/driver is ordered and CAD is frozen. The physical test
> ([WI-QA-03](../../plan/work-items/05-validation-qa/WI-QA-03-thermal.md)) validates it post-build
> and re-parameterizes the assumptions from measured data (§23 DR-02).

## 1. Scope and the DR-01 gate

§23 DR-01 requires a pre-order **photometric + thermal** model before the most expensive, least
reversible parts (light + frame) are committed. This work item is the **thermal half**:

- LED junction / heatsink temperature at 50 / 80 / 100 W.
- Required heatsink thermal resistance and minimum fan airflow.
- Canopy-air rise above ambient.
- Upper dry-bay (electronics + RTC) temperature check.

The **photometric half** — PPFD map, ≥0.6 uniformity across 300–400 mm at 150 mm clearance — is
[WI-PL-06](../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md) (Plant Science),
now **Done — PASS**. With this thermal half also passing, **both halves of DR-01 now pass**. Note
the PL-06 result: a **panel** fixture meets ≥0.6 uniformity at the 150 mm clearance target, whereas a
single **bar** only reaches it at ≥200–225 mm — a constraint that flows into the light-candidate
choice ([WI-EE-01](WI-EE-01-component-poc.md)) and the mechanical light mount.

## 2. Model

Per LED electrical power point `P_elec`:

```text
P_heat    = (1 - eta_rad) * P_elec            heat that must leave the LED junction
T_hs      = T_amb + P_heat * Rth(hs-a)        heatsink base temperature (forced air)
T_j       = T_hs  + P_heat * Rth(j-hs)        LED junction temperature
dT_canopy = f_canopy * P_heat / (rho*cp*Vdot) canopy-air rise from the fraction reaching it
T_bay     = T_amb + (P_driver_loss + P_mcu) / (rho*cp*Vdot_bay)
```

### 2.1 Parameters and why

| Symbol | Value | Basis |
|---|---:|---|
| `T_amb` | 25 °C | Worst-case room (spec §17.2 "room 22–25 °C"). |
| `eta_rad` | 0.42 | Radiant (PAR) fraction of a good white horticultural LED at PPE ≈2.5 µmol/J (§7.2). The rest (58 %) is heat at the junction. Conservative — a poorer fixture radiates less and runs hotter, so this is re-checked against the WI-PL-06 fixture's real PPE. |
| `Rth(j-hs)` | 0.5 °C/W | Aggregate junction→heatsink-base incl. thermal interface, for a **distributed** multi-emitter bar (per-emitter Rth is high but they sit in parallel). A single COB would be lower; a cheap MCPCB strip higher. |
| `Rth(hs-a)` | 0.55 °C/W | Candidate extruded finned heatsink under the §7.4 80/92 mm fan at the design operating point. This is the number the mechanical heatsink selection ([WI-ME-05](../../plan/work-items/04-mechanical/WI-ME-05-light-mount.md)) must hit. |
| `T_hs` target | 60 °C | Top of the "normal" band in the spec §9.5 LED-temp table (`<60 °C` normal). |
| `T_j` design / limit | 85 / 105 °C | 85 °C preferred lifetime ceiling (Arrhenius derate headroom); 105 °C typical hard max for mid/high-power white LEDs — confirm against the chosen part's datasheet. |
| `f_canopy` | 0.25 | Fraction of LED heat that warms canopy-zone air in an **open** frame (most radiant escapes the open sides; §6.3, §5.3). |
| `Driver eff` | 0.90 | Constant-current LED driver efficiency → loss dumped into the **upper dry bay** where the remote driver sits (§7.2 "remote driver in upper dry bay"). |

## 3. Results

| `P_elec` | `P_heat` | `T_hs` | `T_j` | Required `Rth(hs-a)` | Driver loss | Min CFM (canopy) | Min CFM (bay) |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 50 W | 29.0 W | 41 °C | 55 °C | ≤1.21 °C/W | 5.0 W | 2.6 | 0.6 |
| 80 W | 46.4 W | 51 °C | 74 °C | ≤0.75 °C/W | 8.0 W | 4.2 | 0.8 |
| 100 W | 58.0 W | 57 °C | **86 °C** | ≤0.60 °C/W | 10.0 W | 5.3 | 0.9 |

Minimum-airflow columns are well inside the §7.4 fan's **5–20 CFM** usable range, so a single 80/92 mm
PWM fan covers both canopy mixing and bay ventilation at every power point. Canopy-air rise is held to
≤5 °C → canopy ≤30 °C at 25 °C ambient, i.e. at/under the §9.5 fan-ramp threshold (28–30 °C). This is
**self-consistent** with the firmware climate model: at full light the fan is expected to be running.

## 4. Findings

- **50 W and 80 W (the compact V1 band, §7.2/§20): comfortable PASS.** Junction 55–74 °C, heatsink
  41–51 °C, required heatsink resistance (0.75–1.21 °C/W) easily met by a moderate finned heatsink +
  the chosen fan.
- **100 W (full-yield variant only): marginal.** Junction lands at ~86 °C — just over the preferred
  85 °C lifetime ceiling, and it requires a genuinely good ≤0.60 °C/W forced-air heatsink. This
  matches the §23 DR-01 warning and the spec's standing decision that **100 W belongs only to the
  larger/full-yield variant** (§7.2, §20, §22).

## 5. Go / No-Go (thermal half of DR-01)

| Build target | Thermal verdict | Condition |
|---|---|---|
| **Compact V1 (50–80 W)** | **GO** | Heatsink ≤0.75 °C/W at the design fan point; fan ≥5 CFM at full light. |
| **Full-yield variant (100 W)** | **GO with mitigation** | Requires ≤0.60 °C/W heatsink **and** one of: (a) accept ~86 °C junction if the chosen LED's datasheet max ≥105 °C with margin, or (b) lower `T_hs` target to ~50 °C (bigger heatsink), or (c) cap max drive at ~90 W. Re-run the model with the real part's `Rth(j-hs)` and PPE before ordering. |

**Mitigations carried forward if a point is marginal** (per WI-EE-10 deliverable 4):

1. Larger / lower-Rth heatsink (drives `T_hs` and `T_j` down linearly).
2. Lower max LED drive current (the cleanest lever; 80 W is fully in spec).
3. Remote driver placement with its own vent path (keeps the 8–10 W driver loss out of the LED/RTC region — see §6 below).
4. Vent geometry that gives the heatsink a dedicated exhaust separate from the canopy and bay intakes.

## 6. Electronics-bay & RTC check (deliverable 3; DR-05, DR-09)

The upper dry bay carries the MCU/sensors (≈3 W typ) **and** the remote LED driver loss (5–10 W).
Total bay heat 8–13 W. Holding the bay-air rise to ≤25 °C (bay ≤50 °C) needs <1 CFM of through-flow
(table above) — trivial for the system fan, **provided** the bay has an actual airflow path and the
driver is not in a dead pocket. Margins to component ratings at a 50 °C bay:

- **Battery-backed RTC** (DS3231, §16.1 / DR-05): commercial 0–70 °C, industrial −40–85 °C → margin
  ample, but DS3231 oscillator drift is temperature-dependent, so **place the RTC away from the
  driver** and the heatsink exhaust to keep photoperiod stable over the months-long cycle.
- **ESP32-S3**: ambient max 85 °C → fine.
- **Electrolytic caps / driver**: derate hardest with heat — the 50 °C budget keeps them in life.

**DR-09 layering note (carried into §9.5/§17.2):** the firmware's only temperature input is the
shaded *air* sensor — a weak proxy for LED junction. This model confirms the design should treat the
**LED driver's own thermal foldback as the primary LED protection**, with the firmware air-temp
derate (§9.5) as the secondary layer. An optional LED-heatsink NTC (§7.2) would let firmware act on
the real `T_hs` bands in §9.5 and is **recommended** for the 100 W variant.

## 7. Hand-offs

- **Heatsink resistance target** `Rth(hs-a) ≤ 0.75 °C/W` (compact) / `≤0.60 °C/W` (100 W) →
  [WI-ME-05 light mount](../../plan/work-items/04-mechanical/WI-ME-05-light-mount.md).
- **Fan operating point** ≥5 CFM at full light, dual-duty (canopy + bay) →
  [WI-ME-06 fan mount](../../plan/work-items/04-mechanical/WI-ME-06-fan-mount.md) and the
  firmware climate controller.
- **Driver loss budget** 5–10 W into the bay → [power budget WI-EE-02](power-budget.md).
- **Combined DR-01 sign-off:** photometric half [WI-PL-06](../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md)
  is **Done — PASS**; with this thermal half passing, **both halves of DR-01 now pass**. The
  light/CAD-freeze gate is clear, subject to choosing a fixture form factor that satisfies both: a
  **panel** at 150 mm, or a bar at ≥200–225 mm (PL-06).
