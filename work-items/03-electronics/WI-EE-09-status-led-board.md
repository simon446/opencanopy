# WI-EE-09 — Status LED board

| Field | Value |
|---|---|
| Track | Electronics |
| Milestone | (part of M4) |
| Depends on | WI-EE-03 |
| Spec refs | §7.11, §3.5 |
| Status | Not started |

## Objective

Design the separate front-panel status LED PCB driving the 5 indicators behind a diffuser strip.

## Deliverables

- [ ] Small front-panel PCB with 5 indicator positions (RGB or separate colored LEDs).
- [ ] Current-limited; PWM dimming for night mode.
- [ ] Connector back to the controller board.
- [ ] Mechanical fit with the diffuser ([WI-ME-01](../04-mechanical/WI-ME-01-assembly-cad.md) front UI).

## Acceptance criteria

- 5 dimmable positions; supports steady/slow-pulse/fast-blink/double-blink/off patterns (§7.11).
- Layout supports colorblind-safe use (position + pattern), aligned with [WI-FW-08](../02-firmware/WI-FW-08-led-status.md).
