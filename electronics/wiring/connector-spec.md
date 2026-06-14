<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# Connector specification — electronics → mechanical handoff (WI-EE-05)

**Purpose:** the **authoritative list of connectors** the electronics track has chosen, for the
mechanical team to design cable routing, strain relief, panel cut-outs, and the wiring harness
around. Every field connection on the controller board is here with its board-side part, the
**mating cable-side** part the harness is built from, pinout, ratings, and which zone it lives in.

Cross-references (kept consistent by CI): board-side parts are in
[`bom.csv`](../bom/bom.csv) (`CN_*`), pin assignments in [`pin-map.csv`](../analysis/pin-map.csv) and
[`harness-table.csv`](harness-table.csv), connectivity in the
[netlist](../pcb/netlist/controller_netlist.py). **No fan in V1** ([ECO-001](../analysis/ECO-001-fan-removal.md)):
`J_FAN` is a DNP footprint with **no cable** — listed but struck.

## 1. Connector choices (why these)

| Family | Used for | Why |
|---|---|---|
| **XT30** (board + inline) | 24 V power input | Locking, 30 A-rated, polarized; far above the 6.25 A worst case; common, cheap. |
| **JST VH** (3.96 mm) | LED, pump (24 V / higher current) | Locking ramp, ≥10 A/pin, keyed; right pitch for 18–20 AWG power. |
| **JST PH** (2.0 mm) | moisture, reservoir, leak, temp/RH (signal/low-power) | Keyed, positive-latch, compact for 26–28 AWG sensor cable. |
| **JST XH** (2.5 mm) | status-LED board (5 V + data) | Keyed, latched, robust for the front-panel cable. |
| 2.54 mm header | debug UART | Service-only, hidden; cheap pin header. |
| USB-C receptacle | flashing / log | Native USB-CDC; user-facing service port. |

All field connectors are **locking/keyed** — no loose Dupont (spec §7.9, risk S8). Anything carrying
>1 A uses the VH/XT30 (locking, current-rated) family.

## 2. Connector table (board-side + mating cable-side)

| Ref | Function | Zone | Board-side (on PCB) | Mating cable-side (harness builds this) | Pins | Wire | Worst-case I |
|---|---|---|---|---|---:|---|---:|
| **J_PWR** | 24 V PSU input | dry | XT30PW (right-angle PCB) | XT30 inline female + 18 AWG | 2 | 18 AWG | 6.25 A |
| **J_LED** | grow light | dry→light | JST `B4PS-VH` | JST `VHR-4N` + `SVH-21T-P1.1` crimps | 4 | 18/26 AWG | 4.2 A |
| **J_PUMP** | submersible pump | dry→wet | JST `B2PS-VH` | JST `VHR-2N` + `SVH-21T-P1.1` | 2 | 20 AWG | 0.63 A |
| ~~J_FAN~~ | ~~fan~~ | — | **DNP — no fan in V1 (ECO-001)** | **no cable** | (4) | — | — |
| **J_MOIST** | capacitive moisture probe | dry→pot | JST `B3B-PH-K-S` | JST `PHR-3` + `SPH-002T-P0.5S` | 3 | 26 AWG | <0.1 A |
| **J_RES** | reservoir float (+ opt. level) | dry→wet | JST `B3B-PH-K-S` | JST `PHR-3` + `SPH-002T-P0.5S` | 3 | 26 AWG | <0.1 A |
| **J_LEAK** | leak trace/rope on tray | dry→wet | JST `B2B-PH-K-S` | JST `PHR-2` + `SPH-002T-P0.5S` | 2 | 26 AWG | <0.1 A |
| **J_SENS** | temp/RH (SHT40) + I²C | dry | JST `B4B-PH-K-S` | JST `PHR-4` + `SPH-002T-P0.5S` | 4 | 28 AWG | <0.05 A |
| **J_STATUS** | front-panel LED board | dry | JST `B3B-XH-A` | JST `XHP-3` + `SXH-001T-P0.6` | 3 | 26 AWG | 0.3 A |
| **J_DBG** | debug UART (service) | dry | 1×4 2.54 mm header | 2.54 mm socket / pogo jig | 4 | — | <0.05 A |
| **J_USB** | USB-C flash/log | dry (panel) | USB-C receptacle (`GCT USB4105`) | USB-C cable | — | — | 0.5 A |

> "Zone" `dry→wet` means the connector sits in the **dry** electronics bay but its cable runs into the
> **wet** zone — these are the ones that need drip loops (§3).

## 3. Pinouts (board-side, pin 1 keyed/marked)

```
J_PWR  (XT30, 2):   1=+24V   2=GND
J_LED  (VH, 4):     1=+24V   2=GND   3=LED_DIM(PWM/0-10V)   4=LED_NTC(opt)
J_PUMP (VH, 2):     1=PUMP_+24V       2=PUMP_RET(switched -)
J_MOIST(PH, 3):     1=+3V3   2=GND   3=MOIST_SIG(analog)
J_RES  (PH, 3):     1=RES_LOW_SW      2=GND   3=RES_LEVEL_ADC(opt)
J_LEAK (PH, 2):     1=LEAK_SENSE      2=GND
J_SENS (PH, 4):     1=+3V3   2=GND   3=SDA   4=SCL
J_STATUS(XH, 3):    1=+5V    2=GND   3=STATUS_DATA
J_DBG  (hdr, 4):    1=+3V3   2=UART_TX  3=UART_RX  4=GND
```

Full per-pin polarity/voltage/colour is in [`harness-table.csv`](harness-table.csv); silkscreen on
the PCB repeats name + polarity + voltage at each connector (spec §7.9, risk S9).

## 4. Mechanical requirements this imposes (for the mechanical team)

1. **Drip loops** on every `dry→wet` cable (`J_PUMP`, `J_MOIST`, `J_RES`, `J_LEAK`) — the cable must
   dip **below** the connector so water runs off, not in (spec §8.5, §17.1; risks S3/S4).
2. **Strain relief** (clamp or grommet) at each removable module end so cable strain never reaches the
   PCB pad. Dry-zone runs (`J_LED`, `J_SENS`, `J_STATUS`) get a service loop + clamp.
3. **DR-08 routing rule:** the pump **supply tube** must never cross the electronics bay; the
   `J_PUMP` electrical run drip-loops below the electronics. Tube and `J_PUMP` cable route separately.
4. **Panel access:** `J_USB` and the boot/reset buttons (SW1/SW2) need a service opening; hidden in
   normal use. `J_DBG` is internal service-only.
5. **Keying clearance:** all listed housings are latched — leave finger access to release each latch
   for servicing the removable wet-zone modules.
6. **Connector→channel mapping:** harness labels (`pump`, `LED`, `moisture`, `reservoir`, `leak`,
   `status`) must match the mechanical cable channels in
   [WI-ME-07](../../plan/work-items/04-mechanical/WI-ME-07-cable-tube-routing.md).

## 5. Status-LED board side

The front-panel board ([WI-EE-09](../analysis/WI-EE-09-status-led-board.md)) carries the **mating**
`J_STA` (`B3B-XH-A`) at its edge; one keyed `J_STATUS`↔`J_STA` cable (5 V / GND / DATA) clips behind
the front UI. LED **pitch and board outline** are owned by the mechanical front-UI CAD
([WI-ME-01](../../plan/work-items/04-mechanical/WI-ME-01-assembly-cad.md)) and the diffuser windows —
the electronics side is otherwise complete.
