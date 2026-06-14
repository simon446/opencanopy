<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-02 — Power budget & PSU sizing

**Status:** Complete (design figures from spec §7.8 + WI-EE-01 component selection; to be reconciled
against WI-EE-01 *measured* loads when the breadboard PoC runs).
**Spec refs:** §7.8.
**Spreadsheet:** [power-budget.csv](power-budget.csv).

## 1. Load budget

Per-load typical/peak draw, by rail (see [power-budget.csv](power-budget.csv) for the machine-readable
version):

| Load | Rail | Typ | Peak | Notes |
|---|---:|---:|---:|---|
| LED driver | 24 V | 60 W | 80 W (100 W variant) | Continuous when lit; the dominant load |
| Pump (24 V brushless) | 24 V | 5 W | 15 W | 15 W is a **sub-second startup inrush**; ~5 W running, low duty |
| ~~Fan (12 V PWM)~~ | — | **0 W** | **0 W** | **Removed — no fan in V1** ([ECO-001](ECO-001-fan-removal.md)); was a plant-circulation fan, not electronics cooling |
| MCU + sensors | 3.3 / 5 V | 1.5 W | 4 W | ESP32-S3 Wi-Fi TX bursts dominate the peak |
| Status LED board | 5 V | 0.5 W | 1.5 W | PWM-dimmed at night |
| RTC + backup | 3.3 V | 0.05 W | 0.1 W | DS3231; coin cell when unpowered |

**Totals (compact 80 W build):**

- Transient peak (everything at max + pump inrush): **100 W**.
- **Continuous worst-case** (LED 80 + pump ~5 running + MCU 5, no fan): **90 W**.
- 100 W full-yield variant continuous worst-case: **110 W**.

The pump inrush is treated as transient: it is absorbed by the PSU's short-term peak capability plus
bulk input capacitance, not added to the continuous PSU rating. Headroom is sized against the
**continuous** worst-case, which is the load that actually heats the supply.

## 2. PSU recommendation

| Build | PSU | Continuous load | Headroom |
|---|---|---:|---:|
| **Compact (LED ≤80 W)** | 24 VDC **120 W**, certified external brick | 90 W | **33 %** ✔ ≥20 % |
| **Full-yield (LED 100 W)** | 24 VDC **150 W**, certified external brick | 110 W | **36 %** ✔ ≥20 % |

Requirements (spec §7.8, §11.4, §17.1):

- Certified **CE/UL-equivalent external** brick — **no AC mains inside the unit**.
- Output via a **locking/keyed DC connector rated for current** (not a friction barrel jack at this
  power — see §4).
- **Input fuse** inside the unit, **reverse-polarity protection**, **TVS** on the 24 V input
  (specified in [WI-EE-03 schematic](WI-EE-03-schematic.md)).

## 3. Rail plan (DC/DC)

All rails derived from the single 24 V input, with ≥20 % current headroom each (spec §7.8 rails):

| Rail | Source | Feeds | Max load | DC/DC sizing | Headroom |
|---|---|---|---:|---|---:|
| **24 V** | PSU direct | LED driver, 24 V pump | ~4.2 A (100 W LED + pump) | n/a (pass-through, fused) | — |
| **12 V** | 24→12 V buck | **Optional 12 V pump only** (no fan — ECO-001) | ~1.7 A with 12 V pump option; **0 A / omit** with 24 V pump | buck ≥ **2 A / 24 W** *(optional)* | covers pump option |
| **5 V** | 24→5 V buck | Sensors, status LEDs, 3.3 V feed | ~0.6 A | buck ≥ **2 A / 10 W** | comfortable |
| **3.3 V** | 5→3.3 V buck/LDO | MCU, RTC, logic | ~0.6 A (Wi-Fi TX) | reg ≥ **1 A** | ≥40 % |

Notes:

- A single-stage 24→3.3 V buck is acceptable instead of cascading 5→3.3 V; the cascade is chosen so
  the 5 V sensor rail and the MCU rail are independently sequenced and the 5 V buck absorbs most of
  the step-down loss.
- **The 12 V rail now serves only the optional 12 V pump alternate** (the fan is removed — ECO-001).
  The default build uses the 24 V pump, so the **12 V buck `U6` becomes optional / DNP** and may be
  omitted entirely; fit it (≥2 A) only if the 12 V pump alternate (see
  [alternates.csv](../bom/alternates.csv)) is used.

## 4. Connector current ratings (worst-case)

Sized to the worst-case load on each connector with margin (spec §7.10 ">1 A loads use locking
connectors / screw terminals"):

| Connector | Worst-case current | Selection | Rated |
|---|---:|---|---:|
| **PSU input (24 V)** | 6.25 A (150 W) | Locking DC (XT30) or 2-pin screw terminal | ≥10 A |
| **LED output (24 V)** | 4.2 A (100 W) | JST VH (2-pin) or screw terminal | ≥6 A |
| **Pump (24 V)** | 0.63 A peak | JST VH/XH (2-pin) | ≥3 A |
| ~~Fan (12 V, 4-pin)~~ | — | **Not fitted — no fan in V1 (ECO-001)**; header DNP | — |
| **Sensor buses (I²C/analog)** | <0.1 A | JST PH/XH, keyed | ≥1 A |
| **Status LED board** | 0.3 A | JST XH | ≥1 A |

These ratings carry into the [harness table (WI-EE-05)](../wiring/harness-table.csv) and the
[schematic connector definitions (WI-EE-03)](WI-EE-03-schematic.md).

## 5. Reconciliation note

This budget uses spec §7.8 typical/peak figures and the WI-EE-01 candidate parts. Per the work
item's acceptance criterion ("≥20 % headroom against **measured** peak loads from WI-EE-01"), the
LED/pump numbers must be **re-checked against the WI-EE-01 bench logs** once the breadboard PoC runs.
The PSU class is not expected to change (both recommendations now hold ≥33 % at the spec maxima — the
margin *improved* with the fan removed), but the rail-buck current selection should be confirmed
against measured pump inrush. **No fan is fitted in V1** ([ECO-001](ECO-001-fan-removal.md)); the
12 V rail/buck is optional (12 V pump alternate only).
