<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# WI-EE-03 — Schematic & protection (design capture)

**Status:** Design captured (sheets, nets, pin map, protection, fail-safe pump drive) + ERC and
§11.1 review checklist worked against the design. **KiCad schematic entry from this capture, and the
automated CLI ERC run, are pending the KiCad source files** (`electronics/pcb/kicad/`).
**Spec refs:** §7.9, §7.10, §11.1, §17.1, §9.6, §11.4.
**Pin map (firmware consumes this):** [pin-map.csv](pin-map.csv) → HAL [WI-FW-02](../../plan/work-items/02-firmware/WI-FW-02-hal-mocks.md).

> This document is the engineering capture the KiCad schematic is drawn from: every sheet, net, part,
> and protection element is specified here. It is the artifact the §11.1 design review signs off.

## 1. Sheet structure

| Sheet | Contents |
|---|---|
| **1 Power** | 24 V input, fuse, reverse-polarity, TVS, bulk caps; 24→12 V, 24→5 V, 5→3.3 V regulators; per-rail test points |
| **2 MCU** | ESP32-S3-WROOM-1 (N8R8), decoupling, EN/BOOT, USB-CDC, UART header, strapping |
| **3 Sensors** | I²C bus (SHT40, DS3231 RTC, INA219), moisture ADC front end, reservoir, leak comparator, optional LED NTC |
| **4 Outputs** | Pump MOSFET + fail-off, LED dim interface, flyback/snubbers; fan PWM/tach **DNP** (no fan in V1, ECO-001) |
| **5 Connectors/Expansion** | Locking/keyed field connectors; status-LED connector; camera/PAR/load-cell/pH/EC headers (unpopulated) |

## 2. Power & protection (sheet 1) — §17.1, §11.4

Input chain, in order from the 24 V barrel/locking jack:

1. **F1 input fuse** — slow-blow, in the 24 V line. 6.3 A for the 120 W build, 8 A for the 150 W
   (100 W LED) build. Sized above continuous worst-case (90 W→3.8 A / 110 W→4.6 A, no fan — ECO-001)
   and below connector/trace ratings.
2. **Reverse-polarity protection** — P-channel MOSFET in the high side (low forward drop, preferred
   over a series Schottky at these currents). Gate clamped with a Zener so V_GS stays in spec.
3. **TVS** — `SMBJ28A` (28 V stand-off, ~45 V clamp) across 24 V→GND after the fuse, to clamp PSU
   transients and inductive kickback below the regulators' and MOSFETs' V_DS rating.
4. **Bulk capacitance** — low-ESR electrolytic + ceramics on 24 V to absorb pump inrush (§WI-EE-02).

Regulators (each with input/output ceramics and a **test point** on the output, §7.9):

| Rail | Topology | Rating | Feeds |
|---|---|---:|---|
| 12 V *(optional/DNP)* | 24→12 V synchronous buck | ≥2 A | optional 12 V pump only (no fan — ECO-001) |
| 5 V | 24→5 V synchronous buck | ≥2 A | sensors, status LEDs, 3.3 V |
| 3.3 V | 5→3.3 V buck/LDO | ≥1 A | MCU, RTC, logic |

## 3. MCU (sheet 2)

- **ESP32-S3-WROOM-1 N8R8** (octal flash + PSRAM). GPIO26–37 reserved for flash/PSRAM — **not used**
  for I/O. Strapping pins (0, 3, 45, 46) left at their default states.
- USB-CDC on GPIO19/20 (native USB-JTAG) for flash + log capture; UART0 (GPIO43/44) on a debug
  header as a fallback.
- **EN/reset** and **BOOT** buttons accessible during assembly, hidden in normal use (§7.9).
- Brownout detector and watchdog are firmware-enabled (§11.4) on this part; decoupling per WROOM
  datasheet.

Full GPIO assignment is in [pin-map.csv](pin-map.csv). Analog sensors are all on **ADC1** (GPIO1–10)
so they keep working while Wi-Fi is active (ADC2 is unusable with the radio on).

## 4. Sensors (sheet 3)

- **I²C bus** (GPIO8/9, 4.7 kΩ pull-ups to 3V3): SHT40 (0x44), DS3231 RTC (0x68) with CR2032 backup,
  INA219 pump-current monitor. Bus length kept short; pull-ups sized for the populated capacitance.
- **Moisture front end**: capacitive probe analog → series resistor + RC filter → GPIO4 (ADC1_CH3).
  Kept **physically away** from the LED/pump switching node (§7.10) — see [WI-EE-04 layout](WI-EE-04-pcb-layout.md).
- **Reservoir**: float switch → GPIO5 (internal pull-up, closed=low); optional analog level → GPIO6.
- **Leak**: conductive trace pad → comparator (fixed threshold) → GPIO7 (active-high). Hardware
  comparator gives a clean, debounced edge; firmware **latches** the leak fault (manual clear, §11.4).
- **Optional LED-heatsink NTC** → GPIO2 (ADC1_CH1) — the DR-09 secondary thermal input; the LED
  driver's own thermal foldback remains primary.

## 5. Outputs (sheet 4) — fail-safe pump drive is the safety-critical element

### 5.1 Pump MOSFET — fails OFF (§9.6, §11.4)

- **Low-side N-channel logic-level MOSFET**, fully enhanced at V_GS = 2.5 V (e.g.
  DMN3404/CSD18-class), V_DS ≥ 40 V, R_DS(on) low enough that I²R rise is negligible at <1 A.
- **External 10 kΩ gate-to-GND pull-down** — the safety requirement: on MCU reset, crash, or
  Hi-Z GPIO, the gate is pulled to 0 V → **pump OFF**. This does not rely on firmware.
- **100 Ω series gate resistor** to tame edges; optional small gate driver if PWM frequency is high.
- **Flyback Schottky** across the pump (cathode to +V) for the inductive motor; TVS/snubber across
  the FET drain as a second clamp.

### 5.2 Fan — **DNP (no fan in V1, [ECO-001](ECO-001-fan-removal.md))**

- The fan-drive provision is kept on the board as an **unpopulated (DNP) option** so a future fan can
  be driven without a respin, but **V1 fits no fan**: 12 V PWM (GPIO12, 25 kHz) into a 4-pin header,
  tach (GPIO13) pulled up to 3V3 / PCNT, flyback `D3` across the motor — **all DNP**. GPIO12/13 are
  left unused (pin-map: RESERVED). The LED is **passively cooled** (WI-EE-10), so no fan is required.

### 5.3 LED dim interface

- **PWM (GPIO14)** directly to a PWM-input driver, **or** PWM→RC low-pass→op-amp buffer to generate
  an isolated **0–10 V** dim signal, matching the selected driver (§7.2). The driver's **thermal
  foldback is the primary LED protection (DR-09)**; firmware air-temp derate (§9.5) is secondary.
- All MOSFET gates are logic-level at 3.3 V drive, or driven, per §7.10.

## 6. Connectors & expansion (sheet 5) — §7.9

- All field connectors **locking/keyed** (JST VH/XH/PH, Molex, or screw terminals for >1 A) — no
  loose Dupont. Current ratings per [WI-EE-02 §4](power-budget.md). Silkscreen carries name, polarity,
  voltage, and warning marks.
- **Status-LED connector** (J_STATUS): 3-pin (5 V, GND, data) to the front-panel board
  ([WI-EE-09](WI-EE-09-status-led-board.md)).
- **Expansion headers, unpopulated**: camera (GPIO47/48), load-cell HX711 (GPIO15/16, the DR-04
  option), PAR/pH/EC (GPIO17/18 + shared I²C). Per §4.3 these are footprints/headers only in V1.

## 7. ERC expectations (M4-01/M4-04)

The schematic must pass KiCad ERC with no unconnected pins, no power-input-not-driven, no
output-output conflicts. Net-class power nets (24 V, 12 V, 5 V, 3V3, GND) are tagged for the
width/clearance rules used by [WI-EE-04](WI-EE-04-pcb-layout.md) and
[design-rules.md](design-rules.md). The ERC run itself executes once the KiCad source exists and can
be wired into CI (`kicad-cli sch erc`, spec §10.5).

**The schematic is formally captured as a netlist** —
[`electronics/pcb/netlist/controller_netlist.py`](../pcb/netlist/controller_netlist.py) — which is the
machine-readable source of truth (every component + pin-level net) and ships an **ERC stand-in** that
runs in CI today: no floating nets, no double-driven pins, the fail-OFF pump gate, full BOM coverage,
and a check that every MCU net matches [pin-map.csv](pin-map.csv). The KiCad schematic is entered by
importing the generated [`controller.net`](../pcb/netlist/controller.net), after which `kicad-cli sch
erc` supersedes the stand-in.

## 8. §11.1 design-review checklist (worked against this capture)

| Review item (§11.1) | Status in this design |
|---|---|
| Schematic review | ✔ captured (sheets 1–5 above) |
| Power budget review | ✔ [WI-EE-02](power-budget.md) — rails ≥20 % headroom |
| Connector/pinout review | ✔ [pin-map.csv](pin-map.csv) + §6; locking/keyed, labeled |
| Protection review | ✔ fuse + reverse-polarity + TVS + flyback present (§2, §5.1) |
| Wet/dry isolation review | ✔ PCB in upper dry bay; field connectors only to wet-zone sensors (§7.9, §17.1) |
| Firmware pin-mapping review | ✔ ADC1-only analog, fail-off pump gate, RMT status LED — matches [WI-FW-02] HAL |
| PCB DRC/ERC | ⏳ runs against KiCad source (CLI in CI) |
| Trace-current review | → [WI-EE-06](../test/pcb-verification.md) |
| Thermal review (MOSFET/regulator) | → [WI-EE-06](../test/pcb-verification.md) + copper-pour plan in [WI-EE-04](WI-EE-04-pcb-layout.md) |
| Assembly review with mechanical CAD | ⏳ pends mechanical track (WI-ME-*) |

## 9. Deliverable status

| Deliverable | State |
|---|---|
| Full schematic capture (MCU, buses, pump, LED dim, status connector, expansion; fan-drive DNP) | ✔ specified; KiCad entry pending |
| Protection (fuse, reverse-polarity, TVS, flyback) | ✔ specified |
| Pump MOSFET gate pull-down → fails OFF | ✔ specified (10 kΩ pull-down, hardware-guaranteed) |
| Logic-level MOSFETs at 3.3 V / gate driver | ✔ specified |
| ERC clean + §11.1 review sign-off | review worked here; **automated ERC pending KiCad source** |
