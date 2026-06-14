# WI-EE-09 — Status LED board

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | (part of M4) |
| Depends on | WI-EE-03 |
| Spec refs | §7.11, §3.5 |
| Status | Electrical design **complete** — 5×WS2812 chain captured in the [netlist](../../../electronics/pcb/netlist/controller_netlist.py) (decoupling, data series-R, 5V→4.5V series diode for WS2812 V_IH); [report](../../../electronics/analysis/WI-EE-09-status-led-board.md). **Residual:** KiCad board entry/fab (GUI) + LED pitch/outline, which pend the mechanical front-UI CAD ([WI-ME-01](../04-mechanical/WI-ME-01-assembly-cad.md)) |

## Objective

Design the separate front-panel status LED PCB driving the 5 indicators behind a diffuser strip.

## Deliverables

- [x] Small front-panel PCB with 5 indicator positions (RGB or separate colored LEDs). *(5× WS2812B-2020 primary; discrete RGB + TLC59116 alternate.)*
- [x] Current-limited; PWM dimming for night mode. *(Per-LED + global brightness; ≤1.5 W; data series-R + decoupling specified.)*
- [x] Connector back to the controller board. *(J_STATUS 3-pin 5V/GND/DATA, in harness.)*
- [ ] Mechanical fit with the diffuser ([WI-ME-01](../04-mechanical/WI-ME-01-assembly-cad.md) front UI). *(Pitch/outline pend WI-ME-01 front-UI CAD.)*

## Acceptance criteria

- 5 dimmable positions; supports steady/slow-pulse/fast-blink/double-blink/off patterns (§7.11).
- Layout supports colorblind-safe use (position + pattern), aligned with [WI-FW-08](../02-firmware/WI-FW-08-led-status.md).
