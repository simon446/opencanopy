<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# Component sourcing — pump, grow light, status LEDs, cables

**Owner:** Electronics (component selection + BOM is this track's), coordinating **firmware** (driver
interfaces) and **mechanical** (3D models for the enclosure). **Date:** 2026-06-14.
**Spec refs:** §7.2 (light), §7.3 (pump), §7.11 (status LEDs), §16.1/§16.3 (BOM), §17.1 (no mains inside).

This picks **real, orderable parts** for the four field components the maintainer flagged, against the
interfaces firmware already expects, and lists where mechanical can get a **3D/STEP model** for each.
It also surfaces two integration findings that need firmware + a project decision (§6).

## 0. What firmware already expects (the constraints we source to)

| Component | Firmware/electronics interface (already built) |
|---|---|
| Pump | 2-wire **24 V DC**, low-side N-FET switched (`Q1`, GPIO10), INA219 current sense across `R_SHUNT`, `D2` flyback. On/off today; PWM-capable. Calibrated `pump_ml_per_sec`. |
| Grow light | **PWM-dimmable** driver (GPIO14), `led_ppfd_25/50/75/100` calibration map, PPF ~150 µmol/s / PPE ≥2.5, full-spectrum + 660 nm, **driver DC-fed from the 24 V rail** (no mains inside — §17.1/S1). |
| Status LEDs | 5× **WS2812B addressable** RGB, single-wire (GPIO21/RMT), 5 V (4.5 V via series diode). `led_status` renders colour+pattern. |
| Cables | The chosen locking/keyed connectors ([connector-spec.md](../wiring/connector-spec.md)) + wire gauges from the [harness table](../wiring/harness-table.csv). |

## 1. Status LEDs — **settled**

**Worldsemi WS2812B-2020** (2.0 × 2.0 mm addressable RGB). 5 on the front-panel board (PCB2).

- **Buy:** LCSC **C965555** (JLCPCB-stocked for assembly). [LCSC](https://www.lcsc.com/product-detail/C965555.html) · [JLCPCB](https://jlcpcb.com/partdetail/Worldsemi-WS2812B2020/C965555)
- **Operating range 3.7–5.3 V** — this **validates our 5 V→4.5 V series-diode design** (`D_LVD`): at 4.5 V the data V_IH ≈ 0.7·VDD = 3.15 V, so the 3.3 V data line has margin.
- **3D / footprint:** [SnapEDA/SnapMagic](https://www.snapeda.com/parts/WS2812B-2020/Worldsemi/view-part/) (KiCad + STEP) and the KiCad library. *Mechanical needs the PCB2 outline, not the individual LED model.*
- **Alternate:** discrete RGB + TLC59116 I²C driver (already in `alternates.csv`).

## 2. Pump — TOPSFLO TL-B series (brushless DC, submersible)

**TOPSFLO TL-B03H** (24 V brushless DC micro centrifugal, **IP68 submersible**, seal-less, food-grade,
>20 000 h, continuous-rated). Family page: [topsflo.com](https://www.topsflo.com/brushless-dc-pump/24v-dc-water-pump-tl-b03h.html) · [TL-B10 datasheet PDF](https://pdf.directindustry.com/pdf/topsflo-industry-technology-co-limited/tl-b10-12v-24v-brushless-dc-water-pump/160944-733491.html)

- **Specs (TL-B03H):** 24 V, max lift **2.5 m**, max flow **6.5 L/min (390 L/h)**, food-grade, IP68.
  At our operating point (~1 m head, 80–240 L/h) it runs well **under 5 W** — matches §7.3.
- **Firmware fit:** plain 2-wire 24 V DC motor → low-side FET switch (`Q1`) + `D2` flyback + INA219
  current sense exactly as built. Brushless = quiet + continuous-rated (vs a brushed pump). ✔
- **Pick the exact model** (TL-B03H / TL-B04 / TL-B10) off the head–flow curve once the reservoir
  height + tubing run are fixed by mechanical; all share the same 2-wire interface.
- **3D / datasheet:** TOPSFLO provides datasheets (directindustry) and **CAD/STEP on request**
  (`info@topsflo.com`) — request the STEP for the chosen model for mechanical.
- **Alternate:** a generic 24 V brushless DC submersible (cheaper, but no datasheet/3D — fails the
  "real data" bar we hold the light to; acceptable for the pump as a fallback only).

## 3. Grow light — LED engine + **DC-DC dimmable driver** (the cross-domain crux)

This is the one part where a single off-the-shelf fixture does **not** drop in, because **§17.1
forbids mains inside the unit** (risk **S1**, Critical). Every consumer grow bar (Spider Farmer, ECO
Farm, Yearld, etc.) is a **mains-input** fixture — they can't go in the dry bay. So the driver inside
the unit must be **DC-DC, fed from the 24 V rail**.

### 3.1 LED engine

**Samsung LM301H / LM301H EVO** (or LM301B) horticulture board, 3000–3500 K white + **660 nm** deep
red (Osram/Epistar), ~60 W, **PPE 2.7–3.1 µmol/J**. The Samsung diode has a full
[datasheet](https://download.led.samsung.com/led/file/resource/2022/05/Data_Sheet_LM301H_CRI80_Rev.5.4.pdf)
(SPD, PPE) — the real spectral/efficacy data §16.3 requires. **Panel form** (distributed emitters) for
the ≥0.6 uniformity at 150 mm that PL-06 needs. Bare boards: Kingbrite / Bavagreen / ECO Farm
(specify a ~60 W board; finalise the SKU against a published PPFD map for DR-01/PL-06).

### 3.2 Driver — Mean Well **LDD-1000H** (×N), DC-DC constant-current, **logic-level PWM**

[LDD-H series](https://www.ledsupply.com/led-drivers/mean-well-ldd-h-series-cc-step-down-mode) · [Mean Well](https://www.meanwell-web.com/en/ldd-series)

- **Input 9–56 VDC** → fed from our **24 V rail** (no mains, S1-compliant). ✔
- **PWM dim is logic-level:** DIM "ON" = **2.5–6 VDC**, frequency **100 Hz–1 kHz**. Our **3.3 V MCU PWM
  drives it directly — no op-amp, no level shifter.** ✔✔ (This lets us **delete the 0-10 V op-amp path**
  `U9` — see §6.)
- **Step-down** (output < input): from 24 V the usable string is ≤ ~21 V, so ~**21 W per LDD-1000H** →
  budget **~3× LDD-1000H for 60 W**, with the LED board split into ≤21 V strings. Final count is set
  during the LED-board electrical layout (board Vf/current).
- **3D / datasheet:** Mean Well publishes the LDD-H datasheet + **STEP model**; 31.8 × 20.3 × 12.2 mm.

### 3.3 The simpler alternative (needs a project decision)

A single **Mean Well HLG-60H-C AB** (70 W constant-current, **3-in-1 dim: 0-10 V / PWM / resistance**,
IP65/67, [STEP model](https://www.meanwell.com/Upload/PDF/HLG-60H-C/HLG-60H-C-SPEC.PDF)) is **one part**
and a much cleaner driver — **but it is AC-mains input**, so it can only live **outside** the unit
(like the 24 V PSU). That keeps mains external (still S1-compliant) but **moves the driver out of the
dry bay** and means a second external brick + the dim signal entering the unit.

| Option | Driver | Mains? | Firmware fit | Complexity |
|---|---|---|---|---|
| **A (recommended)** | N× LDD-1000H from 24 V rail | none inside | **best** — 3.3 V PWM direct | 3 modules + board split |
| B | 1× external HLG-60H-C AB | mains, **external only** | needs 10 V dim level-shift | 1 module, architecture change |

**Recommendation:** Option A (LDD-H) keeps the committed "DC driver in dry bay" architecture and is the
best firmware fit. Option B is simpler hardware but reopens the driver-placement decision — flag to the
**Project & Repo** track (§6).

## 4. Cables

The connectors are already chosen ([connector-spec.md](../wiring/connector-spec.md), with mfr part
numbers in [bom.csv](bom.csv)). What's left is the **wire**:

- **Silicone-insulated stranded** hook-up wire (flexible, heat/oil-resistant, good in the wet zone):
  **18 AWG** for 24 V power (`J_PWR`/`J_LED`), **20 AWG** pump (`J_PUMP`), **26–28 AWG** for signal/sensor
  runs. E.g. BNTECHGO / generic silicone wire (UL1007/UL3239-class).
- **Pre-crimped or self-crimped** to the JST VH/PH/XH + XT30 housings already specified; wet-zone runs
  get **drip loops + strain relief** per the connector spec.
- **3D:** the connector STEP models (JST, Amass XT30) carry the geometry mechanical needs; bulk wire is
  modelled as a routed path in the cable conduit (WI-ME-07).

## 5. 3D models for mechanical (where to get each STEP)

| Item | 3D / STEP source |
|---|---|
| Pump (TOPSFLO TL-B0x) | TOPSFLO (datasheet + STEP on request, `info@topsflo.com`) |
| LED board (LM301H) | flat board — model from the vendor's board dimensions; pair with the heatsink (WI-ME-05) |
| LED driver (LDD-1000H) | Mean Well STEP download (also HLG-60H STEP if Option B) |
| Status LEDs / PCB2 | SnapEDA WS2812B-2020 (component); mechanical uses the **PCB2 outline** from WI-EE-09 |
| Connectors (JST VH/PH/XH, XT30) | JST / Amass STEP via the manufacturer or SnapEDA |
| Float / leak / moisture probe | supplier model (request with the chosen part) |

The **big geometry** items mechanical actually needs to design around are the **pump**, the **LED
board + heatsink**, and the **LED driver(s)** — prioritise those STEP files.

## 6. Action items this surfaces

- **Firmware — grow-LED PWM frequency.** `controller/src/hw.rs` drives the grow-LED PWM on a **25 kHz**
  LEDC timer (inherited from the now-deleted fan). The LDD-H (and most LED drivers) accept PWM dimming
  only at **100 Hz–1 kHz** → **move the grow-LED channel to ~1 kHz** (a separate LEDC timer). Logic is
  host-testable; no silicon needed. *Until then dimming won't work with a real driver.*
- **Electronics — drop the 0-10 V op-amp path.** With the LDD-H taking 3.3 V PWM directly, the optional
  `U9` (MCP6001) + RC are unnecessary (and MCP6001's 6 V max never could make 0-10 V anyway). They are
  already DNP; remove from the netlist/BOM in a follow-up.
- **Project & Repo — driver placement decision (Option A vs B, §3.3):** DC-DC LDD array in the dry bay
  vs. a single external mains HLG. Affects the spec's "remote driver in upper dry bay" wording and the
  enclosure. Electronics recommends **A**; this is a locked-requirement change to log in the risk register.
- **Plant Science / DR-01:** finalise the LED-board SKU against a **published PPFD map** so PL-06's
  uniformity + the DLI target are met by the actual part (not just the model candidate).

## Sources

- [Mean Well LDD-H (LEDSupply)](https://www.ledsupply.com/led-drivers/mean-well-ldd-h-series-cc-step-down-mode) · [Mean Well LDD series](https://www.meanwell-web.com/en/ldd-series)
- [Mean Well HLG-60H-C spec](https://www.meanwell.com/Upload/PDF/HLG-60H-C/HLG-60H-C-SPEC.PDF)
- [Samsung LM301H datasheet](https://download.led.samsung.com/led/file/resource/2022/05/Data_Sheet_LM301H_CRI80_Rev.5.4.pdf) · [Samsung LM301H EVO](https://led.samsung.com/lighting/mid-power-leds/3030-leds/lm301h-evo/)
- [TOPSFLO TL-B03H](https://www.topsflo.com/brushless-dc-pump/24v-dc-water-pump-tl-b03h.html) · [TL-B10 datasheet](https://pdf.directindustry.com/pdf/topsflo-industry-technology-co-limited/tl-b10-12v-24v-brushless-dc-water-pump/160944-733491.html)
- [WS2812B-2020 (LCSC C965555)](https://www.lcsc.com/product-detail/C965555.html) · [SnapEDA 3D](https://www.snapeda.com/parts/WS2812B-2020/Worldsemi/view-part/)
