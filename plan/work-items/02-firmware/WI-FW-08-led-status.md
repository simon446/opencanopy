# WI-FW-08 — LED status module

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-08 |
| Depends on | WI-FW-07 |
| Spec refs | §9.8, §3.5, §7.11 |
| Status | Not started |

## Objective

Map system/fault state to the 5 front status LEDs with colorblind-safe patterns (position + blink,
never color alone).

## Deliverables

- [ ] State→(color, pattern) mapping for Water, Moisture, Light, Climate, System LEDs (§9.8).
- [ ] Patterns: steady, slow pulse, fast blink, double blink, off; night-mode PWM dimming.
- [ ] System LED heartbeat preserved even when other LEDs are off at night.
- [ ] Unit tests for state-to-pattern mapping.

## Acceptance criteria

- Pattern-map tests pass (spec §10.2 "LED status").
- Every warning/fault is distinguishable by position + pattern without relying on color.

## Notes

Drives the physical board built in [WI-EE-09](../03-electronics/WI-EE-09-status-led-board.md); legend
documented in [WI-DOC-05](../06-documentation/WI-DOC-05-led-legend-troubleshooting.md).
