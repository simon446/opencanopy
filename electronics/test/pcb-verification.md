<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-06 — Trace current & thermal verification report

**Status:** Trace-width calc, max-current table, connector ratings, and predicted voltage drop /
component temperatures **complete (computed)**. The **measured** voltage drop, MOSFET/regulator
temperatures, and thermal-camera image are recorded at board bring-up
([WI-EE-08](bringup.md) → this file), and require a fabricated board.
**Spec refs:** §7.10, §11.3.
**Reproduce calc:** `python3 electronics/analysis/trace_width.py`.

## 1. Trace-width calculation & max-current table (§11.3) — computed

IPC-2221, 10 °C allowed rise, 1 oz outer copper. `used_w` carries ≥1.5× margin over the IPC minimum;
the 24 V/LED paths are realised as **filled copper pours** ≥ `used_w` (see [WI-EE-04](../analysis/WI-EE-04-pcb-layout.md)).

| Power path | Worst-case I | IPC min width | Used width | Predicted V-drop (1-way) |
|---|---:|---:|---:|---:|
| 24 V input → regulators/LED | 4.20 A | 2.17 mm | 3.3 mm / pour | 37.8 mV |
| LED driver feed (24 V) | 4.20 A | 2.17 mm | 3.3 mm / pour | 25.2 mV |
| Pump MOSFET drain/source | 0.63 A | 0.16 mm | 0.3 mm + pour | 31.2 mV |
| 12 V rail (optional 12 V pump only; no fan — ECO-001) | 1.70 A | 0.62 mm | 0.9 mm | 46.8 mV |
| 5 V rail | 0.60 A | 0.15 mm | 0.3 mm | 49.6 mV |
| 3V3 rail (MCU) | 0.60 A | 0.15 mm | 0.3 mm | 39.6 mV |

All predicted drops are <50 mV — negligible against rail tolerances. The LED feed is the only path
near 4 A; the pour keeps its rise and drop low.

## 2. Predicted component temperatures (to be confirmed by measurement)

| Component | Loss (computed) | Prediction | Basis |
|---|---:|---|---|
| Pump MOSFET @ peak | ~12–30 mW (I²R, logic-level FET R_DS(on)≈30 mΩ, ≤1 A) | <5 °C rise — effectively ambient | low current + copper pour |
| 24→12 V buck @ 1.7 A *(optional/DNP)* | ~2.2 W (η≈0.90) | ~30–40 °C rise with pour → confirm <125 °C | **only if the 12 V pump option is fitted**; default 24 V-pump build omits this buck entirely (no fan — ECO-001) |
| 24→5 V buck @ 0.6 A | ~0.4 W | small rise | — |
| 5→3.3 V reg @ 0.6 A | ~0.3 W | small rise | — |

The big LED-driver dissipation is in the **remote driver module**, not on the controller PCB; the
controller only passes 24 V through to the LED connector (pour). Its thermal envelope is the
[WI-EE-10 model](../analysis/WI-EE-10-thermal-budget-model.md).

## 3. Connector current-rating table (§11.3) — computed

Carried from [WI-EE-02 §4](../analysis/power-budget.md):

| Connector | Worst-case I | Selection | Rated |
|---|---:|---|---:|
| J_PWR (24 V in) | 6.25 A | XT30 / 2-pos screw | ≥10 A |
| J_LED (24 V) | 4.2 A | JST VH | ≥6 A |
| J_PUMP (24 V) | 0.63 A | JST VH | ≥3 A |
| ~~J_FAN (12 V)~~ | — | **DNP — no fan in V1 (ECO-001)** | — |
| Sensor buses | <0.1 A | JST PH/XH | ≥1 A |

## 4. Measurement templates — fill at bring-up (WI-EE-08)

> These require the fabricated board powered from a current-limited bench supply.

| ID | Measurement | Method | Pass criterion | Measured |
|---|---|---|---|---|
| T1 | 24 V feed V-drop @ 4.2 A | DMM across input→LED connector at max load | within prediction; <100 mV | `pending` |
| T2 | 12 V V-drop @ full load | DMM across buck out→12 V pump | <100 mV | `pending` *(only if 12 V pump option fitted; no fan)* |
| T3 | Pump MOSFET temp @ 100 % | thermocouple / IR on FET, real pump in water | < FET rating, < 60 °C case | `pending` |
| ~~T4~~ | ~~Fan-drive temp @ 100 %~~ | **N/A — no fan in V1 (ECO-001)** | — | `n/a` |
| T5 | Regulator temps @ worst-case ambient | IR on each DC/DC at 25–40 °C bay | junction < rating | `pending` |
| T6 | Thermal-camera image @ max pump/LED-control load | thermal cam, full-load steady state | no hotspot over rating | `pending` (save to `validation/thermal/`) |

## 5. Deliverable status

| Deliverable | State |
|---|---|
| Trace-width calc + max-current table | ✔ computed |
| Voltage-drop measurement under max load | predicted ✔ / **measured ⏳** (T1–T2) |
| MOSFET temp @100 % pump; regulator temp worst-case (no fan — ECO-001) | predicted ✔ / **measured ⏳** (T3, T5) |
| Thermal-camera image at max load | template ✔ / **image ⏳** (T6) |
| Connector current-rating table | ✔ computed |
