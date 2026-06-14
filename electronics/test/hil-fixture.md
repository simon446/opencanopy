<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-08 — Hardware-in-loop (HIL) fixture & automated fault tests

**Status:** Fixture design + automated test matrix authored; **logic half now dry-validated against
the firmware host-tests** (see §2.1). **Hardware execution BLOCKED only on a fabricated board** —
[WI-FW-07 safety state machine](../../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md) is
now **Done** and host-tested, so the state-machine behaviour these fault tests assert is implemented
and proven off-silicon; what remains is proving it **on the device**.
**Spec refs:** §10.4, §11.2, §9.3, §9.9.
**Lives with:** [`firmware/hil/`](../../firmware/hil/) (fixture firmware/fixtures) — this doc is the
electronics-side fixture spec.
**No fan in V1** ([ECO-001](../analysis/ECO-001-fan-removal.md)) — the fan PWM/tach channel (H6) is
struck.

## 1. Fixture design (§10.4)

A bench fixture that presents the controller's real connectors with programmable stimuli and captures
its outputs, so fault scenarios run automatically and repeatably:

| Channel | Stimulus / capture | Drives controller's | Maps to |
|---|---|---|---|
| Programmable moisture | DAC or digipot on `MOIST_SIG` (or digital mock) | moisture ADC (GPIO4) | §10.4 |
| Reservoir switch sim | relay/FET across float-switch pins | `RES_LOW_SW` (GPIO5) | §10.4 |
| Leak switch sim | relay across leak comparator input | `LEAK_DET` path (GPIO7) | §10.4 |
| Pump dummy load | resistor/electronic load + current shunt on `J_PUMP` | pump MOSFET drain | §10.4 |
| ~~Fan tach sim~~ | **N/A — no fan in V1 (ECO-001)** | GPIO13 unused | — |
| LED dim dummy | scope/ADC on `LED_DIM` | LED dim (GPIO14) | §10.4 |
| Current measurement | shunt + ADC on pump/24 V | INA219 cross-check | §10.4 |
| UART log capture | host serial on `J_DBG`/USB-CDC | logs (§9.10) | §10.4 |

The fixture host (PC or a second MCU) sets stimuli, reads the controller's UART log + captured
outputs, and asserts the expected state/output per case.

## 2. Automated test matrix (§10.4 + §9.3 priorities)

| # | Scenario | Stimulus | Expected (asserted) | State (§9.3) |
|---|---|---|---|---|
| H1 | Boot self-test | power-on | SELF_TEST runs, pump confirmed OFF (§9.4) | BOOT→SELF_TEST→NORMAL |
| H2 | **Leak → pump lockout** | assert leak while watering | pump output **never enables**; latched until manual clear (§11.4) | LEAK_DETECTED (highest) |
| H3 | **Low-water → no pump** | reservoir-low asserted | pump **never enables** | LOW_WATER |
| H4 | Leak priority over low-water | both asserted | LEAK_DETECTED wins (§9.3 priority) | LEAK_DETECTED |
| H5 | LED dim command changes | command 25/50/75/100 % | LED_DIM output tracks command | NORMAL |
| ~~H6~~ | ~~Fan PWM changes~~ | **N/A — no fan in V1 (ECO-001)** | — | — |
| H7 | Status LED patterns | drive each fault | correct colour/position/pattern (§7.11) | per fault |
| H8 | **Watchdog resets safely** | stall firmware loop | watchdog fires; on reset **pump OFF** (gate pull-down) | →BOOT, pump OFF |
| H9 | Sensor fault → safe | implausible moisture | auto-water disabled, fault shown (§7.6) | SENSOR_FAULT |
| H10 | Over-temp derate | inject high air-temp | LED derates per §9.5 ladder | OVER_TEMP |

H2, H3, H4, H8 are the **gating safety tests** (spec §15.5 M4-09): pump lockout proven *in hardware*,
not just simulation (this is exactly the DR-02 gap — sim proves logic, HIL proves the device).

## 2.1 Logic half — dry-validated against firmware host-tests (no silicon)

With [WI-FW-07](../../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md) **Done**, each H-case's
*logic* is now provable off-silicon by an existing `cargo test` in the host-tested `control` crate.
This **closes the logic half of the DR-02 gap ahead of fabrication**; the hardware column is what the
board run still has to prove (real fail-off voltage, the watchdog actually firing, real PPFD/current).

| # | Logic dry-validated by (host test) | What still needs the board |
|---|---|---|
| H1 | `app_state::boots_to_normal_with_valid_calibration`, `…missing_calibration_disables_watering_and_faults`; `safety_controller::boot_with_bad_calibration_does_not_reach_normal` (pump forced off at boot, §9.4) | SELF_TEST runs on real peripherals; pump line measured OFF |
| **H2** | `app_state::leak_gate_overrides_everything`; `safety_controller::leak_latches_until_manual_clear`; `hal_integration::injected_leak_keeps_pump_trait_deenergized` | MOSFET drain measured dead while leak asserted; latch across power-cycle |
| **H3** | `safety_controller::sensor_fault_and_low_water_block_pump` | reservoir-low float → pump line OFF on the DUT |
| **H4** | `safety_controller::leak_beats_everything`, `…priority_ladder_each_rung` | both inputs asserted on hardware → LEAK wins |
| H5 | `light_controller::commanded_power_combines_schedule_ramp_and_derate` (+ `app_state` led_pct path) | LED_DIM output measured at 25/50/75/100 % |
| ~~H6~~ | — (no fan in V1, ECO-001) | — |
| H7 | `led_status::sensor_fault_is_distinct_double_blink`, `…night_mode_dims_greens_but_keeps_system_heartbeat`, `…rtc_fallback_shows_amber_system_pulse` | WS2812 colour/pattern on the real status board (RMT driver — see gaps) |
| **H8** | boot forces pump off (`safety_controller::boot`); **hardware fail-off is firmware-independent** — gate pull-down `R1`, [WI-EE-03 §5.1](../analysis/WI-EE-03-schematic.md) | watchdog (RWDT) actually fires & resets; pump measured OFF through the reset |
| H9 | `app_state::missing_calibration_disables_watering_and_faults`; `hal_integration::injected_moisture_fault_disables_watering` | implausible moisture on real ADC → SENSOR_FAULT |
| H10 | `app_state::over_temp_cuts_led_and_pump`; `light_controller::air_temp_derate_thresholds` | inject real air-temp; LED derate measured (no fan branch — LED-off is the live mitigation) |

So **9 of 10 cases** have their logic proven off-silicon today (H6 is gone). Only the *device-physics*
half — fail-off voltage, watchdog firing, real PPFD/current/LED-status — waits on a fabricated board.

## 2.2 Open integration gap — INA219 over-current → pump fault (DR-04)

The BOM carries the **INA219 pump-current monitor as "required V1"** (`U4`, DR-04: clog/dry-run
detection). The firmware **reads** it (`controller/src/hw.rs` `read_ina219_ma`, logged each tick) but
the current does **not yet feed pump-fault arbitration**: `SafetyInputs.pump_fault`
(`control/src/app_state.rs`) is driven only by the irrigation controller's **flow-based** no-rise /
watering-limit logic — `SensorFrame` has no pump-current field, so an **over-current/clog never
latches `PUMP_FAULT`**. Test **H-INA** to add when the path is wired (firmware item, host-testable
now; no silicon needed):

| # | Scenario | Stimulus | Expected (asserted) | State |
|---|---|---|---|---|
| H-INA | Pump over-current / clog | shunt current above the DR-04 threshold | `PUMP_FAULT` latches; pump locked out | PUMP_FAULT |

This is flagged to the firmware track; the HIL fixture's current shunt on `J_PUMP` already provides
the stimulus, so H-INA runs on the same rig once the firmware threshold lands.

## 3. Calibrations run on the fixture (§9.9)

The §9.9 calibrations (moisture dry/wet, pump ml/s, LED PPFD map, reservoir low — **fan-min-PWM
struck, no fan in V1, ECO-001**) are run here and their values populate the
[WI-FW-11 NVS schema](../../plan/work-items/02-firmware/WI-FW-11-calibration-storage.md).
Procedure + result table is in [bringup.md §E](bringup.md).

## 4. Blocking dependencies

| Blocker | Why it blocks | Tracks | State |
|---|---|---|---|
| Fabricated controller board | No DUT to bring up / wire to the fixture | follows [WI-EE-07](../pcb/fabrication/fab-notes.md) Gerber export | **the only remaining blocker** |
| ~~WI-FW-07 safety state machine~~ | ~~H1–H10 assert state/priority/fail-off behaviour~~ | [02-firmware](../../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md) | **✔ Done** — logic dry-validated (§2.1) |
| ~~WI-FW-08/04/05/06 controllers~~ | ~~LED/pump command tests need the controllers~~ | 02-firmware | **✔ Done** |

WI-FW-07 has landed: the matrix logic is **dry-validated against the firmware host-tests today**
(§2.1). When a board is fabricated, the same matrix runs unchanged to prove the device-physics half.

## 5. Deliverable status

| Deliverable | State |
|---|---|
| Bring-up per §11.2 | procedure ✔ ([bringup.md](bringup.md)); execution ⛔ blocked on **board only** (FW-07 done) |
| Pump-in-water / LED / status verify | procedure ✔; execution ⛔ blocked on board (fan struck — ECO-001) |
| 24 h dry + 24 h wet soak | procedure ✔; execution ⛔ blocked on board |
| HIL fixture (§10.4) | design ✔ (this doc); build ⛔ pending |
| Automated HIL fault tests | matrix ✔ (H1–H10); **logic half ✔ dry-validated (§2.1)**; device run ⛔ blocked on board |
| §9.9 calibrations → WI-FW-11 | procedure ✔; values ⛔ blocked on board (fan-min-PWM struck — ECO-001) |
