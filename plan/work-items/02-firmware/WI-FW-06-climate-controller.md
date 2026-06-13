# WI-FW-06 — Climate / fan controller

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-06 |
| Depends on | WI-FW-03, WI-PL-04 |
| Spec refs | §9.7, §5.3, §5.4 |
| Status | Done |

## Objective

Implement VPD calculation and the fan/LED climate response: per-stage fan minimums, RH/VPD/temp
boosts, and fan-tach fault detection.

## Deliverables

- [x] VPD calculator (temp + RH → kPa) with unit tests against reference vectors from [WI-PL-04](../01-plant-science/WI-PL-04-vpd-climate-model.md).
- [x] Per-stage fan minimums, lights-on vs lights-off duty cycling (§9.7).
- [x] Fan boosts: RH>75% +15%, RH>85% +30% (amber), VPD<0.5 +20%, temp>28/30/32 °C escalation (§9.7).
- [x] Fan-tach-missing → FAN_FAULT.
- [x] "Do not blast seedlings" — circulation, not wind stress — encoded in stage minimums.
- [x] Unit tests for duty behavior across temp/RH/VPD.

## Acceptance criteria

- Fan/VPD tests pass (spec §10.2 "VPD calculator" + "Fan controller").
- Climate controller never attempts to cool below ambient (open-frame constraint).

## Implementation

- `control/src/climate_controller.rs`: Tetens `svp_kpa`/`vpd_kpa` (dependency-free `exp` in
  `control/src/math.rs`), `vpd_band`, per-stage fan minimums (lights-on + periodic lights-off
  bursts), additive boosts (highest RH/temp tier + independent VPD), temp escalation to max-fan +
  LED-derate request at >32 °C, and fan-tach-missing → `FAN_FAULT`. The VPD calculator is asserted
  against all 11 reference vectors in `docs/vpd-climate-model.md` §1.1 to 4 decimal places. Never
  cools below ambient — only fan duty + a derate request.
