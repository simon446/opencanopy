# WI-FW-04 — Light controller

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-04 |
| Depends on | WI-FW-03 |
| Spec refs | §9.5, §9.4 (RTC fallback), §5.2 |
| Status | Not started |

## Objective

Implement the photoperiod scheduler, intensity ramping, RTC-fallback schedule, and LED thermal
derating.

## Deliverables

- [ ] Per-stage photoperiod + intensity targets with 30-min ramps (§9.5).
- [ ] Default 06:00→22:00 schedule when RTC valid; 16 h-on/8 h-off-from-boot fallback when invalid (§9.4).
- [ ] Thermal derating: 30–32 °C → −20%, >32 °C → −30–60% + climate fault, >35 °C → off/min (§9.5).
- [ ] Optional LED heat-sink NTC derating ladder (60/70/80 °C) if sensor present.
- [ ] Unit tests for schedule, ramp, fallback, and each derate threshold.

## Acceptance criteria

- Schedule/ramp/derate tests pass (spec §10.2 "Light scheduler" + "LED derating").
- RTC-invalid path raises amber System LED but does not block watering.
