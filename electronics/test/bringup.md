<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-08 — Board bring-up procedure

**Status:** Procedure authored (ready to execute). **Execution BLOCKED only on a fabricated board** —
[WI-FW-07 safety state machine](../../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md) is
now **Done** and host-tested, so the firmware that steps 6–13 flash *exists* and its logic is already
dry-validated off-silicon (122 host tests + Wokwi). The `blocked` rows below await **silicon**, not
firmware. **No fan in V1** ([ECO-001](../analysis/ECO-001-fan-removal.md)) — fan steps struck.
**Spec refs:** §11.2, §9.9, §11.4.

> Fill the **Result** column as each step is performed. Do **not** advance past a failed safety step
> (continuity, fail-off, leak lockout). Power throughout step 3+ comes from a **current-limited bench
> supply** set to the rail's expected draw + margin.

## A. Power-off checks (no firmware needed)

| # | Step (§11.2) | Pass criterion | Result |
|---|---|---|---|
| 1 | Visual inspection | No bridges, correct polarity, parts seated | `pending` |
| 2a | Continuity: 24 V↔GND | **No short** | `pending` |
| 2b | Continuity: 12 V↔GND | No short | `pending` |
| 2c | Continuity: 5 V↔GND | No short | `pending` |
| 2d | Continuity: 3.3 V↔GND | No short | `pending` |
| 2e | **Pump gate→GND** | ~10 kΩ pull-down present (fail-OFF guarantee, §9.6) | `pending` |

## B. Power-on, rails (current-limited supply)

| # | Step | Pass criterion | Result |
|---|---|---|---|
| 3 | Apply 24 V via current-limited supply, watch current | No fault current; reverse-polarity + fuse intact | `pending` |
| 4 | Verify rails **unloaded** | 24/12/5/3.3 V within ±5 % at test points | `pending` |
| 5 | Verify regulator temperatures | All regulators stable, within rating (cross-check [WI-EE-06 T5](pcb-verification.md)) | `pending` |

## C. MCU & I/O (requires firmware — **blocked on WI-FW-07 build**)

| # | Step | Pass criterion | Result |
|---|---|---|---|
| 6 | Flash test firmware (USB-CDC) | Boots; runs SELF_TEST (§9.4) | `blocked` |
| 7 | Verify USB/UART | Log stream over USB-CDC and UART header | `blocked` |
| 8 | Verify each sensor bus | I²C enumerates SHT40/DS3231/INA219; ADC1 reads moisture/leak/reservoir | `blocked` |
| 9 | Verify each output with **dummy load** | pump/LED-dim lines toggle as commanded | `blocked` (board) |
| 10 | **Pump MOSFET with real pump in water** | Pumps when commanded; **off on reset/crash** (fail-OFF) | `blocked` (board) |
| ~~11~~ | ~~Fan PWM + tach~~ | **N/A — no fan in V1 (ECO-001)**; GPIO12/13 left unused | `n/a` |
| 12 | LED dimming with driver | Dim command changes output; driver foldback honoured (DR-09) | `blocked` |
| 13 | Status LED board | All 5 positions + all patterns (§7.11) | `blocked` |

## D. Soak

| # | Step | Pass criterion | Result |
|---|---|---|---|
| 14 | 24 h **dry** burn-in (no water) | No resets/thermal faults; logs clean | `blocked` |
| 15 | 24 h **wet-bay** test (water, no plant) | No leaks; leak lockout armed; rails stable | `blocked` |

## E. Calibration (§9.9 — values populate [WI-FW-11 NVS schema](../../plan/work-items/02-firmware/WI-FW-11-calibration-storage.md))

| Calibration | Method (§9.9) | Output key | Result |
|---|---|---|---|
| Moisture dry/wet | dev mode, chosen media (4-point per §7.6) | `moisture_raw_dry`, `moisture_raw_wet` | `blocked` |
| Pump ml/s | run into cylinder 30 s | `pump_ml_per_sec` | `blocked` |
| Reservoir low point | fill–drain | `reservoir_low_adc` | `blocked` |
| ~~Fan min PWM~~ | ~~lowest reliable spinning duty~~ | ~~`fan_min_pwm`~~ | **N/A — no fan in V1 (ECO-001)** |
| LED PPFD map | PAR grid @25/50/75/100 % | `led_ppfd_map` | `blocked` (needs PL-06 + light) |
| Temp/RH sanity | vs reference | (validation note) | `blocked` |
| Leak | wet test | (verify lockout latch) | `blocked` |

## Acceptance (spec §15.5 M4-08)

Rails, MCU, sensors, outputs all pass → sign here. **Hardware fail-off (step 10) and leak lockout
(step 15 + HIL) are gating** and must pass before any live-plant loop.
