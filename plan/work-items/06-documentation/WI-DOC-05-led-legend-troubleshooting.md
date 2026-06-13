# WI-DOC-05 — LED status legend & troubleshooting

| Field | Value |
|---|---|
| Track | Documentation |
| Milestone | (release) |
| Depends on | WI-FW-08 |
| Spec refs | §3.5, §9.8, §14.3 |
| Status | Not started |

## Objective

Write the LED status legend (the primary UI) and a troubleshooting guide keyed to the indicators.

## Deliverables

- [ ] `docs/led-status-legend.md`: each of the 5 LEDs × green/amber/red × blink pattern → meaning (§9.8).
- [ ] Colorblind-safe explanation (position + pattern, not color alone).
- [ ] `docs/troubleshooting.md` mapping each fault LED to likely cause + user action.

## Acceptance criteria

- LED status legend complete (§21) and matches the firmware mapping in [WI-FW-08](../02-firmware/WI-FW-08-led-status.md).
