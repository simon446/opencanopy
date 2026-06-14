<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# ECO-001 — Fan removal (V1 is fan-less)

**Type:** Engineering Change Order (electronics-side record)
**Date:** 2026-06-14
**Status:** Approved (maintainer) · Electronics reconciliation **complete**
**Owning track:** Electronics · **Cross-track:** Mechanical, Firmware, Plant Science, Project & Repo
**Spec touch:** §7.4 (fan), §7.8, §9.5, §9.7, §17.2 — a change to a locked requirement; see §"Project
hand-off" for the risk-register/spec update this needs (Project & Repo track).

## 1. Decision

**V1 has no fan.** The fan is removed from the bill of materials, harness, and power budget, and the
firmware does not drive one. It originated in the mechanical block-model redesign
(commit `50d353e`, "remove the fan entirely — no fan in V1") and was approved by the maintainer on
2026-06-14.

**Clarified intent:** the fan was a **plant-canopy circulation actuator** (air movement / VPD /
boundary-layer for the plant), *not* an electronics-cooling device. The ESP32-S3 + sensors (≈3 W)
never needed cooling. The only real thermal load is the 50–100 W grow LED, which is cooled by its
**heatsink** — see §3.

## 2. Board treatment — "leave the pin unused" (DNP option)

Per the maintainer's steer, the fan **drive** stays on the controller as an **unpopulated (DNP)
optional provision**, so the board *can* drive a fan later without a respin, but V1 ships with none:

- `FAN_PWM` (GPIO12) and `FAN_TACH` (GPIO13) → **reserved / unused in V1** (pin-map updated).
- On-board fan-drive parts (fan flyback `D3`, the `J_FAN`/`CN_FAN` header, tach pull-up) → **DNP /
  optional** (footprints kept, not populated). The fan *module* (`M2`, 92 mm) and its 12 V load are
  **removed** from the populated BOM and power budget.
- The firmware `FAN_PWM` channel is harmless if left configured (no fan connected); the climate
  controller's fan command and the §9.5 "force fan high" branch become **no-ops** — see §4.

## 3. Thermal re-analysis (the one thing that could have bitten)

The earlier thermal model ([WI-EE-10](WI-EE-10-thermal-budget-model.md)) had assumed the fan gave the
LED heatsink **forced air** (`Rth(hs-a)=0.55 °C/W`). With no fan the heatsink runs on **natural
convection** (higher Rth). Re-ran `thermal_budget_model.py` against a passive heatsink:

| LED power | Passive heatsink | T_hs / T_j | Verdict |
|---:|---|---|---|
| **60 W (committed V1, BOM `LED_PANEL`)** | 0.8 °C/W (large passive) | **53 / 70 °C** | **GO, fan-less** |
| 80 W (stretch) | 0.8 °C/W | 62 / 85 °C | passive ceiling (T_j at limit) |
| 100 W (full-yield variant) | 0.8 °C/W | 71 / 100 °C | **NOT viable fan-less** |

**Conclusion: the committed 60 W V1 light is comfortably passively cooled — no fan required.** The
fan-less passive ceiling is ~80 W with a large `≤0.8 °C/W` heatsink. The **100 W full-yield variant**
needs active cooling (or a lower drive) and is **deferred** from the fan-less V1 — a separate later
decision. The dry-bay (driver loss + MCU, 8–11 W) needs <1 CFM-equivalent and is carried by
open-frame natural convection.

This **confirms the maintainer's intuition** that this LED power class does not need active cooling —
provided the mechanical light mount delivers a real `≤0.8 °C/W` passive heatsink with vertical fins
and an unobstructed open-frame airflow path.

## 4. Electronics reconciliation (this ECO's changes)

| Artifact | Change |
|---|---|
| `analysis/thermal_budget_model.py` | Forced-air → **passive** model; verdict on committed 60 W. |
| `analysis/WI-EE-10-thermal-budget-model.md` | Passive GO/no-go; heatsink target `≤0.8 °C/W`; §9.5 fan no-op noted. |
| `analysis/pin-map.csv` | `FAN_PWM`/`FAN_TACH` marked **reserved / DNP (no fan in V1)**. |
| `analysis/power-budget.csv` + `.md` | Fan load removed; rail/connector tables updated; headroom recomputed. |
| `bom/bom.csv` | `M2` fan removed; `D3` fan flyback + `CN_FAN` → **DNP optional**. |
| `wiring/harness-table.csv` + `wiring/README.md` | `J_FAN` marked **not fitted in V1 (DNP option)**. |
| `analysis/WI-EE-03-schematic.md` | Fan drive/flyback/tach noted as DNP option; no fan flyback populated. |
| `analysis/WI-EE-04-pcb-layout.md` | Fan footprints kept as DNP; no fan copper pour needed. |
| `test/pcb-verification.md` (WI-EE-06) | Fan-MOSFET temp row dropped; heatsink is passive. |
| `test/hil-fixture.md` + `test/bringup.md` (WI-EE-08) | Fan PWM/tach test (H6) → **N/A in V1**. |
| `analysis/WI-EE-01-component-poc.md` | Fan bench protocol → N/A; fan deliverable struck. |
| `pcb/fabrication/fab-notes.md` | `CN_FAN`/`D3` flagged DNP in the fab package. |

## 5. Cross-track hand-offs (NOT changed here — flagged to owners)

- **Mechanical:** [WI-ME-06 fan mount](../../plan/work-items/04-mechanical/WI-ME-06-fan-mount.md) is
  **obsolete for V1**. [WI-ME-05 light mount](../../plan/work-items/04-mechanical/WI-ME-05-light-mount.md)
  must deliver a **passive `≤0.8 °C/W` heatsink** + open vent path (intake low / exhaust high). The
  fan-guard child-safety risk (S18) is moot with no fan blades.
- **Firmware:** the climate controller still computes a fan duty and the safety §9.5 path does
  `force_fan_high`; with no fan these are **no-ops**. Functionally safe (LED off/derate is the real
  over-temp mitigation), but firmware should mark the fan channel optional and not raise `FAN_FAULT`
  on a missing tach in a fan-less build. (Tracked for the firmware track — see ECO §6.)
- **Plant Science / firmware-climate:** the fan's *actual* job — **canopy air circulation for VPD /
  transpiration** — is gone. Whether passive open-frame convection gives adequate canopy air movement
  is a **plant-science** question (VPD homogeneity, fungal/still-air risk), not an electronics one.
  Flagged for the plant-science track to assess; may reopen as a non-electronics circulation method.
- **Project & Repo:** §7.4 (fan) and risk-register rows **R5, S14 (fan-failure), S18 (fan guard)**
  reference a fan that no longer exists in V1. This ECO is the electronics record; the spec/risk
  ledger update is the Project track's to make.

## 6. Open firmware item (for the firmware track)

`controller/src/hw.rs` configures `fan_pwm` (GPIO12, LEDC ch1) and the app commands a fan duty. With
no fan this is benign, but the clean follow-up is: gate the fan channel behind a build/config flag and
suppress `FAN_FAULT` when no fan is fitted, so a fan-less unit never shows a phantom fan fault. Logic
is host-tested; no silicon needed to make this change.
