# sim/models/ — plant/environment models

The simulator's environment models (spec §10.3; **ECO-003 passive-watering** revision). Implemented
in [`../src/models.rs`](../src/models.rs); this file documents the equations and their parameters.

> **These are engineering estimates, not measured reality.** Per WI-FW-09 and spec §23 (DR-02) they
> **must be re-parameterized from bench data** ([WI-QA-09](../../../plan/work-items/05-validation-qa/WI-QA-09-bench-characterization.md))
> before the simulator is trusted to gate a live-plant grow loop. Passing scenarios proves the
> **control logic** (monitor + warn), not that the model matches a real plant.
>
> **No pump, no fan in V1 (ECO-003 / ECO-001).** Watering is passive: a base reservoir feeds the
> substrate through a capillary wick. The firmware never actuates water — it monitors and warns. The
> only actuator is the grow LED.

## Models (all per 5-minute control tick)

| Behavior (§10.3) | Equation | Constant | Value |
|---|---|---|---|
| Substrate dries by day, rehydrates at night (working wick + water) | `Δmoist = −DAY_DRY` (lights on) / `+NIGHT_WET` (lights off) | `DAY_DRY_PCT_PER_TICK` | 0.02 %/tick |
| | a gentle diurnal sawtooth — stays in-band and never plateaus (so it's never mistaken for a stuck probe) | `NIGHT_WET_PCT_PER_TICK` | 0.04 %/tick |
| Substrate dries out when the wick fails or the reservoir is empty | `Δmoist = −NOWATER_DROP` | `NOWATER_DROP_PCT_PER_TICK` | 0.05 %/tick |
| Reservoir drains by transpiration (through the wick) | `reservoir -= ET`; `ET = base · vpd_factor · dt_min` | `ET_LIGHT_ML_PER_MIN` | 0.30 mL/min |
| | `vpd_factor = 1.5 if VPD>1.2 else 1.0` | `ET_DARK_ML_PER_MIN` | 0.10 mL/min |
| Reservoir low-water mark (refill warning) | `reservoir ≤ RESERVOIR_LOW_ML` | `RESERVOIR_LOW_ML` | 500 mL |
| LED adds heat | `air_T = room_T + led% · LED_HEAT_GAIN_C` | `LED_HEAT_GAIN_C` | +4 °C @100% |
| Air RH tracks the room | `air_RH = room_RH` (no fan to disturb it) | — | — |
| Leak / sensor faults | injected via `sim::Inject` | — | — |

## Fault injection (`sim::Inject`)

- `leak` — catch-tray reads wet (flood/overflow).
- `moisture_stuck_pct` — probe frozen at a fixed reported % (stuck-sensor scenario).
- `moisture_error` — forces a `SensorError` from the probe.
- `wick_failure` — the wick can't replenish the substrate **even with a full reservoir** (clogged /
  poor contact): the substrate dries out → `MOISTURE_LOW`, while the reservoir stays full. This is
  the passive-watering failure mode that replaces the old pump "disconnected" fault.

## Important modeling note (sawtooth, not plateau)

A passive substrate held at a *constant* equilibrium would read an unchanging raw count and trip the
firmware's stuck-sensor detector (`STUCK_WINDOW_MS = 6 h`). Real substrate always drifts, so the
model uses a small diurnal sawtooth (dry by day, rehydrate at night) that keeps moving by more than a
raw count within any 6-hour window. If a future bench calibration changes the moisture span or the
drift rates, re-check that the healthy case never looks "stuck".
