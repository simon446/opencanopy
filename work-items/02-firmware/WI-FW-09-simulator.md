# WI-FW-09 — Plant/environment simulator & scenarios

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-09 |
| Depends on | WI-FW-05, WI-FW-06, WI-FW-07 |
| Spec refs | §10.3 |
| Status | Not started |

## Objective

Build a plant/environment simulator sufficient to validate the full control loop without hardware,
and implement the 11 required scenarios.

## Deliverables

- [ ] Simulator (`firmware/sim/`) modeling: moisture decline (faster under light/high VPD), pump→
      moisture rise after delay, reservoir drawdown, fan→RH effect, LED→heat, injectable leak/sensor
      faults (§10.3).
- [ ] All 11 required scenarios implemented as automated tests:
      normal seedling, normal fruiting, reservoir empty, sensor stuck wet, sensor stuck dry, pump
      disconnected, leak, hot room, humid night, RTC invalid, power loss mid-watering.

## Acceptance criteria

- All 11 §10.3 scenarios pass with the documented expected results.
- Simulator runs in CI (no hardware) — coordinate with [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md).
