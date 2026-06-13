# WI-FW-06 — Climate / fan controller

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-06 |
| Depends on | WI-FW-03, WI-PL-04 |
| Spec refs | §9.7, §5.3, §5.4 |
| Status | Not started |

## Objective

Implement VPD calculation and the fan/LED climate response: per-stage fan minimums, RH/VPD/temp
boosts, and fan-tach fault detection.

## Deliverables

- [ ] VPD calculator (temp + RH → kPa) with unit tests against reference vectors from [WI-PL-04](../01-plant-science/WI-PL-04-vpd-climate-model.md).
- [ ] Per-stage fan minimums, lights-on vs lights-off duty cycling (§9.7).
- [ ] Fan boosts: RH>75% +15%, RH>85% +30% (amber), VPD<0.5 +20%, temp>28/30/32 °C escalation (§9.7).
- [ ] Fan-tach-missing → FAN_FAULT.
- [ ] "Do not blast seedlings" — circulation, not wind stress — encoded in stage minimums.
- [ ] Unit tests for duty behavior across temp/RH/VPD.

## Acceptance criteria

- Fan/VPD tests pass (spec §10.2 "VPD calculator" + "Fan controller").
- Climate controller never attempts to cool below ambient (open-frame constraint).
