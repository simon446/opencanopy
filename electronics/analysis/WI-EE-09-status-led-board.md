<!-- SPDX-License-Identifier: CERN-OHL-S-2.0 -->
# WI-EE-09 — Status LED board (design)

**Status:** Design complete; 5×WS2812 chain captured in the
[netlist](../pcb/netlist/controller_netlist.py) (board PCB2). **Board build via the tscircuit flow
([ECO-002](ECO-002-pcb-toolchain.md)); LED pitch/outline pend the mechanical front-UI CAD
([WI-ME-01](../../plan/work-items/04-mechanical/WI-ME-01-assembly-cad.md)).**
**Spec refs:** §7.11, §3.5.
**Pairs with firmware:** [WI-FW-08 LED status](../../plan/work-items/02-firmware/WI-FW-08-led-status.md).

A small separate front-panel PCB behind a diffuser strip, driving the 5 system indicators (§3.5/§7.11).

## 1. Topology

**Primary: 5 × WS2812B-2020 addressable RGB**, daisy-chained on a single data line.

- **One controller wire** (`STATUS_DATA`, GPIO21 via RMT — already in [pin-map.csv](pin-map.csv)),
  plus 5 V and GND → the 3-pin `J_STATUS` connector ([harness](../wiring/harness-table.csv)).
- **RGB per position** → any colour, so the same physical layout serves the colourblind-safe scheme
  (colour *and* position *and* pattern, §7.11/§3.5).
- **PWM dimming for night mode** is intrinsic: per-LED 8-bit brightness in the WS2812, plus a global
  brightness scale in firmware → smooth night dimming without extra hardware.

**Alternate: 5 × discrete RGB LEDs + TLC59116 I²C constant-current driver** (see
[alternates.csv](../bom/alternates.csv)) — for builds that prefer no addressable-LED firmware path;
it shares the I²C bus and gives 16 PWM channels (5×3 = 15 used).

## 2. Current limiting & power

- WS2812B-2020: ~5 mA per colour-die typical at moderate brightness; ≤60 mA per device at full white.
  Five devices full-white worst case = 300 mA @ 5 V (1.5 W). Firmware caps brightness so the
  realistic draw is well under the **0.3 A / 1.5 W** budgeted on the 5 V rail
  ([power-budget.csv](power-budget.csv)).
- **Decoupling:** 100 nF per LED + a bulk cap at the connector. **Series resistor (~330 Ω)** on the
  data line at the source, and a level consideration: WS2812 V_IH ≈ 0.7·VDD = 3.5 V vs the 3.3 V
  GPIO. The first-LED margin is thin at 5 V; mitigations on the board: power the chain at **4.5–4.7 V**
  (small series diode from 5 V) to drop V_IH into range, or fit a single level shifter at the input.
  The alternate TLC59116 path avoids this entirely (I²C).
- The discrete/driver alternate uses per-channel current-set resistor (TLC59116 `Rext`) — no
  per-LED ballast resistors needed.

## 3. Patterns (firmware-driven, §7.11)

The board hardware supports all five patterns because brightness/colour are fully software-controlled
per LED; the pattern *logic* lives in [WI-FW-08](../../plan/work-items/02-firmware/WI-FW-08-led-status.md):

| Pattern | How |
|---|---|
| steady | constant brightness/colour |
| slow pulse | sinusoidal brightness ramp |
| fast blink | square on/off, fast |
| double blink | two short pulses + gap |
| off | brightness 0 |

Night mode = global brightness scale applied on top of any pattern.

## 4. Colourblind-safe layout (§3.5)

Five **fixed positions** in a known order behind the diffuser, so meaning is conveyed by **position +
pattern** as well as colour (a deuteranope reads "position 2, fast-blink" without relying on hue).
The legend is documented in `docs/led-status-legend.md` (Documentation track) and must match the
firmware map.

## 5. Mechanical interface

- Board sized to sit behind the **diffuser strip** in the front UI; LED pitch matched to the diffuser
  windows. Final outline + mount pattern pend [WI-ME-01](../../plan/work-items/04-mechanical/WI-ME-01-assembly-cad.md).
- Single keyed `J_STATUS` connector at the board edge facing the controller; cable clipped behind the
  front panel ([harness README](../wiring/README.md)).

## 6. Deliverable status

| Deliverable | State |
|---|---|
| 5-indicator front-panel PCB (RGB) | ✔ designed (WS2812 primary; discrete+TLC alt); in the netlist |
| Current-limited; PWM night dimming | ✔ specified (per-LED + global brightness; decoupling + data series-R) |
| Connector back to controller | ✔ `J_STATUS` 3-pin (5 V/GND/DATA), in harness |
| Mechanical fit with diffuser | ⏳ pends WI-ME-01 front-UI CAD |
| 5 dimmable positions, all patterns, colourblind-safe | ✔ supported in hardware; logic in WI-FW-08 |
