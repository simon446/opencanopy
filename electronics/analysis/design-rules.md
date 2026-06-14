<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# PCB design rules & net classes (WI-EE-04 layout recipe)

The deterministic rule set for laying out the controller board, so the KiCad placement/routing step
is mechanical rather than a judgement call. Pairs with the floorplan/stackup in
[WI-EE-04](WI-EE-04-pcb-layout.md) and the trace-width proof in
[WI-EE-06 / pcb-verification](../test/pcb-verification.md). Widths assume **1 oz** outer copper, 10 °C
rise (IPC-2221, the basis of [`trace_width.py`](trace_width.py)); on inner layers use 1.5× width.

## 1. Stackup (4-layer)

| Layer | Use | Copper |
|---|---|---|
| L1 top | components, signal, 24 V/pump/LED power fills | 1 oz |
| L2 | **solid GND plane** (return reference) | 1 oz |
| L3 | power distribution (5 V, 3V3 pours; 12 V if fitted) | 1 oz |
| L4 bottom | signal, **analog sensor routing**, test points | 1 oz |

The continuous L2 ground plane is the single biggest win for moisture/ADC quality and high-current
returns (§7.10). 2-layer is a documented cost fallback (then GND is a bottom pour + careful returns).

## 2. Net classes

Assign every net to a class on import; these drive KiCad's width/clearance rules and the DRC.

| Net class | Nets | Track width | Min clearance | Notes |
|---|---|---:|---:|---|
| **PWR_24V** | `+24V_IN`, `+24V_FUSED`, `+24V`, `PUMP_+`, `PUMP_RET` | **pour / ≥2.5 mm** | **0.5 mm** | 4.2 A worst case (LED feed). Filled pour on L1; ≥0.5 mm creepage to logic (condensation margin even in the dry bay). |
| **PWR_12V** *(DNP)* | `+12V`, `PH12` | 0.9 mm | 0.4 mm | only if 12 V pump option fitted; 1.7 A. |
| **PWR_5V** | `+5V`, `PH5` | 0.5 mm | 0.3 mm | 0.6 A; pour on L3. |
| **PWR_3V3** | `+3V3` | 0.5 mm | 0.3 mm | 0.6 A; pour on L3. |
| **GND** | `GND` | plane | — | solid L2; star-tie analog/power returns at the bulk-cap ground. |
| **ANALOG** | `MOIST_ADC`, `MOIST_SIG`, `LEAK_SENSE`, `LEAK_REF`, `LED_NTC`, `RES_LEVEL_ADC` | 0.25 mm | 0.25 mm | route on L4 over solid GND; **keep ≥3 mm from pump/LED switching nodes**; no crossing the 24 V pour. |
| **SIGNAL** | I²C (`SDA`,`SCL`), `PUMP_PWM`, `LED_DIM*`, `STATUS_DATA*`, `UART_*`, `FAN_*` (DNP) | 0.2 mm | 0.2 mm | default digital. Keep I²C pair short; series R on `STATUS_DATA`/gate nets at the source. |
| **USB** | `USB_DM`, `USB_DP` | 0.25 mm | 0.2 mm | route as a ~90 Ω differential pair, length-matched, short. |

## 3. Clearances & rules (DRC)

| Rule | Value | Why |
|---|---:|---|
| Min track / min clearance (global) | 0.2 mm / 0.2 mm | standard 2-layer-capable fab; cheap. |
| 24 V ↔ logic/analog clearance | **≥0.5 mm** | creepage with possible condensation in the bay (§7.10). |
| Min via | 0.3 mm drill / 0.6 mm pad | standard; use multiple vias to stitch the GND plane and for power. |
| Min annular ring | 0.15 mm | fab floor. |
| Pump-FET / regulator copper | **pour both sides + stitching vias** | the pour is the heatsink (§7.10, WI-EE-06 thermal). |
| Edge clearance | 0.5 mm | handling / routing. |
| Silkscreen | name + polarity + voltage + warning at every field connector | spec §7.9, risk S9. |

## 4. Placement constraints (from the floorplan, §2 WI-EE-04)

- **High-current switching domain** (24 V in, F1, Q2, bulk caps, DC/DC regulators, pump FET) kept on
  one side; the **analog sensor domain** (moisture front end, leak comparator, I²C, NTC) on the other.
  They never share copper or return paths.
- **RTC (U3)** placed **away from the regulators and the LED-connector heat path** (DS3231 oscillator
  drift is temperature-dependent — DR-05/DR-09, WI-EE-10 §6).
- **Bulk caps** (`C_BULK1/2/3`) adjacent to the 24 V input and the pump FET to absorb inrush.
- **Test points** on every rail and control net (§5 WI-EE-04) on an accessible layer.
- **Mounting holes / board outline** coordinated with the mechanical electronics-bay before fab.

## 5. How this is applied

1. Import [`controller.net`](../pcb/netlist/controller.net) into the KiCad PCB.
2. Create the net classes in **Board Setup → Net Classes** with the §2 widths/clearances and assign
   nets by the patterns above (KiCad supports net-class assignment by net-name pattern).
3. Set the §3 global/rule values in **Board Setup → Constraints / Custom Rules**.
4. Place per §4, route, pour GND (L2) and the power planes (L3), stitch with vias.
5. Run `kicad-cli pcb drc --exit-code-violations` (already wired into the `eda` CI job) until clean.

Steps 1–5 are the **KiCad-GUI residual** — this document removes the design judgement from them; what
remains is the mechanical act of placing and routing in the tool.
