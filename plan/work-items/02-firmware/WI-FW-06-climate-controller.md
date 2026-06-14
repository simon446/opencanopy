# WI-FW-06 — Climate monitor (fan removed)

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-06 |
| Depends on | WI-FW-03, WI-PL-04 |
| Spec refs | §9.7, §5.3, §5.4 |
| Status | Done (descoped: fan removed) |

> **SCOPE CHANGE (2026-06-14): the circulation fan was removed from the V1 design** by mechanical and
> electronics. The climate controller no longer commands any actuator — it is now a **monitor**: VPD
> calculation + temp/RH health flags for the Climate LED, plus an LED-derate request (cutting the
> grow LED is the only remaining way to shed heat). `FAN_FAULT`, the fan-tach input, per-stage fan
> minimums, the fan duty boosts, and `fan_min_pwm` were all removed from the firmware. Spec §9.7
> (fan controller), §9.3 (`FAN_FAULT` state) and §9.9 (`fan_min_pwm`) still describe a fan and need a
> coordinated revision by the spec owner.

## Objective

Implement VPD calculation and the temperature/humidity climate response. (Originally also drove a
circulation fan; with the fan removed the response is monitoring + an LED-derate request only.)

## Deliverables

- [x] VPD calculator (temp + RH → kPa) with unit tests against reference vectors from [WI-PL-04](../01-plant-science/WI-PL-04-vpd-climate-model.md).
- [x] Temp/RH/VPD health classification → Climate LED amber/red (RH>85 amber, RH>90 red, VPD-stress
      amber, temp >30 amber / >32 red, cold <16 amber, outside preferred RH band amber).
- [x] LED-derate request at temp >32 °C (the only heat lever without a fan).
- [x] Unit tests for the classification across temp/RH/VPD.
- [x] ~~Per-stage fan minimums, lights-on vs lights-off duty cycling~~ — removed with the fan.
- [x] ~~Fan boosts (RH/VPD/temp duty) and fan-tach-missing → FAN_FAULT~~ — removed with the fan.

## Acceptance criteria

- VPD + climate-classification tests pass (spec §10.2 "VPD calculator").
- Climate controller commands no actuator and never attempts to cool (open-frame, fan-less constraint).

## Implementation

- `control/src/climate_controller.rs`: Tetens `svp_kpa`/`vpd_kpa` (dependency-free `exp` in
  `control/src/math.rs`), `vpd_band`, and `evaluate` → `{vpd_kpa, vpd_band, climate_amber,
  climate_red, request_led_derate}`. No fan duty, no tach, no `FAN_FAULT`. The VPD calculator is
  asserted against all 11 reference vectors in `docs/vpd-climate-model.md` §1.1 to 4 decimal places.
  Over-temp protection now rests entirely on the LED derate/cut (`safety_controller` `OverTemp` gate
  forces `led_max_factor = 0.0`).
