<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-01 — Component selection & breadboard PoC

**Status:** Selection + bench protocol complete; **bench measurements pending hardware**.
**Spec refs:** §7.1–§7.7, §16, §23 (DR-01, DR-04).
**Feeds:** schematic [WI-EE-03](WI-EE-03-schematic.md), power budget [WI-EE-02](power-budget.md),
BOM [WI-EE-07] ([bom.csv](../bom/bom.csv)).

> **Two-part work item.** *Component selection* (the design decision — concrete candidate parts that
> satisfy §7 and §16) is delivered here and is what the schematic/BOM/power-budget consume. The
> *bench logs* (M2-02…M2-08 measured data) require physical hardware on a breadboard; this document
> provides the **bench protocol, acceptance thresholds, and logging templates** so the PoC can be
> run and recorded the moment parts arrive. Measurement-dependent boxes are flagged below.

## 1. Selected components (candidate parts)

Each row is a concrete, orderable candidate that meets the §7 requirement and carries the data the
BOM (§16) needs. Alternates live in [alternates.csv](../bom/alternates.csv).

| # | Function | Candidate part | Key spec | Why / spec ref |
|---|---|---|---|---|
| 1 | **MCU** | ESP32-S3-DevKitC-1 (N8R8) | Xtensa LX7 dual, 8 MB flash / 8 MB PSRAM, Wi-Fi/BLE, ≥30 GPIO | §7.1 preferred; Rust `esp-hal`; camera-expansion headroom. **M2-01 ✔** |
| 2 | **Temp/RH** | Sensirion SHT40 (or SHT31-DIS-B) | I²C, ±0.2 °C / ±1.8 %RH, 0x44 | §7.5/§16.1 SHT3x/4x-class; mid-height shaded mount |
| 3 | **Soil moisture** | Capacitive v2 probe, conformal-coated, JST | Analog 0–3 V, corrosion-resistant | §7.6 capacitive only; replaceable connector, strain-relieved |
| 4 | **Reservoir low-level** | Vertical float switch (PP, reed) + optional eTape analog | Dry/closed contact; low-level reliable | §7.7 low-level required; full/percent optional |
| 5 | **Leak sensor** | Conductive trace pad / rope sensor on bottom tray | Open/closed via comparator | §7.5; drives pump lockout (latched, §11.4) |
| 6 | **Pump** | 24 V brushless DC submersible centrifugal, ≥1.0 m head, 80–240 L/h | <5 W typ, removable filter, 6/8 mm tubing | §7.3 preferred; quiet, continuous-rated |
| 7 | **Pump current sense** | INA219 (I²C) or shunt + amp on pump rail | mA resolution on 24 V/12 V rail | §7.5 **required** in V1 (DR-04 — clog/dry-run detection) |
| 8 | **Fan** | 92×92×25 mm 12 V PWM, fluid-dynamic bearing, 4-pin (PWM+tach) | 5–20 CFM, ≤20 dBA nominal | §7.4; dual-duty canopy + bay (per WI-EE-10) |
| 9 | **Grow light** | Full-spectrum white hort LED **panel** (preferred at 150 mm clearance per PL-06), 50–80 W, PWM/0–10 V dim, remote driver | PPF 140–220 µmol/s, PPE ≥2.5 | §7.2/§16.3 — DR-01 now PASS; finalize per §4 |
| 10 | **RTC** | DS3231SN (battery-backed, I²C) + CR2032 | ±2 ppm TCXO | §16.1 / DR-05; placed away from driver heat (WI-EE-10 §6) |
| 11 | **Status LEDs** | 5 × WS2812B-2020 **or** 5 × discrete RGB + drivers | Dimmable, PWM night mode | §7.11; detailed in [WI-EE-09](WI-EE-09-status-led-board.md) |

## 2. PoC bench protocol

Run on an ESP32-S3 dev board + breadboard. Log to `electronics/test/poc-logs/` (template in §3).
Each test maps to a §15.3 M2 acceptance row.

| ID | Test | Method | Acceptance threshold |
|---|---|---|---|
| M2-01 | MCU | Flash blink + I²C/ADC bring-up sketch | Boots, all buses enumerate. **✔ (selection)** |
| M2-02 | Temp/RH | Log SHT40 vs a calibrated reference hygrometer, 2 h, 3 RH points | within ±1 °C and ±3 %RH vs reference; stable (no drift/spikes) |
| M2-03 | Moisture | Record raw ADC in **air**, **dry media**, **field-capacity media**, **saturated media** (the §7.6 model) | Four distinct, monotonic, repeatable raw values; normalized map spans 0–100 |
| M2-04 | Reservoir low | Fill/drain cycle; log switch transition vs water level | Reliable low-level edge; no chatter at threshold |
| M2-05 | Leak | Drip water across the trace pad; log signal + verify firmware pump-lockout hook | Detects within seconds; pump-enable line forced low; latches until manual clear |
| M2-06 | Pump | Run into a graduated cylinder for 30 s ×3; SPL meter @1 m | ml/s logged (±25 % repeatable, §7.3); ≤30 dBA @1 m |
| M2-07 | Fan | Sweep PWM 0–100 %; log tach RPM; SPL @1 m | Lowest reliable spinning duty found; ≤20 dBA nominal / ≤30 dBA max |
| M2-08 | LED dim | Drive dim input 25/50/75/100 %; measure PPFD grid with PAR meter at canopy plane | Monotonic PPFD map; values usable by [WI-PL-02 DLI calc](../../plan/work-items/01-plant-science/WI-PL-02-light-dli-targets.md) |
| — | Pump current | Log INA219 mA at idle / normal / blocked-intake / dry-run | Distinguishable current signatures for clog & dry-run (DR-04) |

## 3. Bench-log template

A ready-to-fill template is committed at
[`electronics/test/poc-logs/poc-log-template.csv`](../test/poc-logs/poc-log-template.csv):

```csv
test_id,component,part_no,date,operator,condition,measured_value,unit,reference_value,pass_fail,notes
M2-02,SHT40,SEN-SHT40,,,,,degC/%RH,,,
M2-06,pump,PMP-24-1m,,,30s_run_1,,ml,,,
```

## 4. Grow-light gate (§23 DR-01) — both halves now PASS

- **Thermal half — DONE:** [WI-EE-10](WI-EE-10-thermal-budget-model.md) confirms 50–80 W is a clean
  GO and bounds the heatsink/fan the fixture must use; 100 W is full-yield-variant only.
- **Photometric half — DONE (PASS):** [WI-PL-06](../../plan/work-items/01-plant-science/WI-PL-06-photometric-model.md)
  shows ≥0.6 uniformity and the §7.2 PPFD across the canopy. **Form-factor constraint:** a **panel**
  meets uniformity at the 150 mm clearance target, while a single **bar** only reaches ≥0.6 at
  ≥200–225 mm. The compact frame is height-constrained (DR-07), so the **panel form factor is
  preferred**; a bar is acceptable only if the mechanical mount allows ≥200 mm clearance.
- **§16.3 gate:** any candidate must publish real power draw, PPF/PPFD map, dimming method,
  horticultural spectrum, and thermal-mounting data — `scripts/bom_check.py` enforces this. No
  lumen-/"equivalent-watt"-only fixture passes.

DR-01 is clear, so the light may now be **finalized** by selecting a real fixture that meets the §16.3
data requirements **and** the PL-06 form-factor/clearance result. Pending that procurement pick, the
BOM carries a compliant **panel candidate** (`LIGHT-CANDIDATE-60W`) referencing the
[PL-06 PPFD maps](../../validation/ppfd-measurements/model/); the 100 W bar is the full-yield
[alternate](../bom/alternates.csv) (needs ≥200 mm clearance).

## 5. Deliverable status

| Deliverable | State |
|---|---|
| MCU selected + BOM entry | ✔ done |
| Temp/RH selection + protocol | ✔ selection / ⏳ bench log |
| Moisture selection + protocol | ✔ selection / ⏳ bench log |
| Reservoir-level selection + protocol | ✔ selection / ⏳ bench log |
| Leak selection + protocol | ✔ selection / ⏳ bench log |
| Pump selection + protocol | ✔ selection / ⏳ flow+noise log |
| Fan selection + protocol | ✔ selection / ⏳ duty+noise log |
| LED dim selection + protocol | ✔ selection / ⏳ PPFD map |
| Grow-light §16.3 gating | ✔ DR-01 PASS (thermal ✔ EE-10 / photometric ✔ PL-06); panel form preferred |
| Grow-light BOM finalization | gate clear — panel candidate in BOM; final procurement pick outstanding |

⏳ = requires breadboard hardware (and, for PPFD, a PAR meter + the WI-PL-06 photometric model).
