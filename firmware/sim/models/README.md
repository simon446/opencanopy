# sim/models/ — plant/environment models

The simulator's environment models (spec §10.3). Implemented in [`../src/models.rs`](../src/models.rs);
this file documents the equations and their parameters.

> **These are engineering estimates, not measured reality.** Per WI-FW-09 and spec §23 (DR-02) they
> **must be re-parameterized from bench data** ([WI-QA-09](../../../plan/work-items/05-validation-qa/WI-QA-09-bench-characterization.md))
> before the simulator is trusted to gate a live-plant grow loop. Passing scenarios proves the
> **control logic**, not that the model matches a real plant.

## Models (all per 5-minute control tick)

| Behavior (§10.3) | Equation | Constant | Value |
|---|---|---|---|
| Moisture declines faster under light / high VPD | `Δmoist = base · vpd_factor · dt_min` | `DECLINE_LIGHT_PCT_PER_MIN` | 0.012 %/min |
| | | `DECLINE_DARK_PCT_PER_MIN` | 0.004 %/min |
| | `vpd_factor = 1.5 if VPD>1.2 else 1.0` | | |
| Pump raises moisture after a delay | water matures after `SOAK_MS`, then `Δ% = ml / POT_ML_PER_PCT` | `POT_ML_PER_PCT` | 15 mL/% |
| | | `SOAK_MS` | 8 min |
| Reservoir drains when the pump runs | `reservoir -= run_s · ml_per_s` (immediate) | `RESERVOIR_LOW_ML` | 300 mL |
| LED adds heat | `air_T = room_T + led% · LED_HEAT_GAIN_C − fan% · FAN_COOL_C` | `LED_HEAT_GAIN_C` | +4 °C @100% |
| Fan disperses heat + humidity | `air_RH = room_RH − fan% · FAN_DEHUMIDIFY_PCT` | `FAN_COOL_C` | −1 °C @100% |
| | | `FAN_DEHUMIDIFY_PCT` | −6 %RH @100% |
| Leak / sensor faults | injected via `sim::Inject` | — | — |

## Fault injection (`sim::Inject`)

- `leak` — leak sensor reads wet.
- `moisture_stuck_pct` — probe frozen at a fixed reported % (stuck-wet / stuck-dry scenarios).
- `moisture_error` — forces a `SensorError` from the probe.
- `pump_disconnected` — pump motor runs but moves no water (no rise, no drawdown).
- `fan_tach_zero` — tach reads 0 while commanded on → `FAN_FAULT`.

## Important modeling note (no-rise vs. pump effect)

The no-rise pump-fault threshold in `control` (`MIN_RISE_PCT = 1.0`) must stay **below** the smallest
modeled pulse effect so healthy watering never false-faults. With `POT_ML_PER_PCT = 15`, the seedling
20–50 mL pulse raises moisture ≈1.3–3.3 %, comfortably above 1.0 %. If a future bench calibration
changes `POT_ML_PER_PCT` or the per-stage pulse sizes, re-check this margin.
