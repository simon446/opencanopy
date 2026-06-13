# WI-PL-04 — VPD & climate response model

| Field | Value |
|---|---|
| Track | Plant Science |
| Milestone | M1-05 |
| Depends on | WI-PL-01 |
| Spec refs | §5.3, §5.4, §9.7 |
| Status | Done |

## Objective

Specify the VPD calculation and the open-frame climate-response rules (fan boosts, LED derating,
RH guardrails) so the climate controller behaves like a monitor-and-nudge system, not an HVAC.

## Deliverables

- [x] VPD definition (temp + RH → kPa) with the exact formula to implement and reference test vectors.
- [x] VPD decision table (<0.4 / 0.5–1.2 / 1.2–1.6 / >1.6 kPa) → fan/water/LED actions (§5.4).
- [x] RH guardrail table (>85 / 70–85 / 55–70 / <40 %) → actions (§5.4).
- [x] Temperature response table (§5.3) including fruit-set protection (avoid sustained >32 °C canopy).
- [x] Explicit non-goal: firmware does **not** hold an air-temp setpoint and cannot cool below ambient.

## Acceptance criteria

- VPD formula + test vectors enable a deterministic unit test in [WI-FW-06](../02-firmware/WI-FW-06-climate-controller.md).
- Temp/RH/VPD action tables match spec §5.3/§5.4/§9.7 exactly.
