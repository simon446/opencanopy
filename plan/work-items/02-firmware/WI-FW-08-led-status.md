# WI-FW-08 — LED status module

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-08 |
| Depends on | WI-FW-07 |
| Spec refs | §9.8, §3.5, §7.11 |
| Status | Done |

## Objective

Map system/fault state to the **4** front status LEDs with colorblind-safe patterns (position +
blink, never color alone). *(Was 5 — [ECO-003](../../../docs/ECO-003-v1-redesign.md) dropped the
Climate LED: with no fan / no active climate control, climate warnings fold into the System LED.)*

## Deliverables

- [x] State→(color, pattern) mapping for Water, Moisture, Light, System LEDs (§9.8). *(Climate LED
      removed, ECO-003 — climate/VPD warnings tint System amber; LED-thermal shows on the Light LED.)*
- [x] Patterns: steady, slow pulse, fast blink, double blink, off; night-mode PWM dimming.
- [x] System LED heartbeat preserved even when other LEDs are off at night.
- [x] Unit tests for state-to-pattern mapping.

## Acceptance criteria

- Pattern-map tests pass (spec §10.2 "LED status").
- Every warning/fault is distinguishable by position + pattern without relying on color.

## Notes

Drives the physical board built in [WI-EE-09](../03-electronics/WI-EE-09-status-led-board.md); legend
documented in [WI-DOC-05](../06-documentation/WI-DOC-05-led-legend-troubleshooting.md).

## Implementation

- `control/src/led_status.rs`: `render()` mapping system + per-subsystem health to the 4 LEDs as
  (color, pattern), with night-mode dimming of green LEDs while keeping warnings/faults visible and
  the System heartbeat alive; sensor faults use the distinctive double-blink. Colorblind-safe by
  position + pattern. Host-tested incl. "every warning distinguishable without color" and RTC
  fallback → System amber.
