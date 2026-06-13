<!-- SPDX-License-Identifier: CC-BY-4.0 -->
# WI-EE-08 — Hardware-in-loop (HIL) fixture & automated fault tests

**Status:** Fixture design + automated test matrix authored. **Execution BLOCKED on a fabricated
board and on [WI-FW-07 safety state machine](../../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md)
(*Not started*)** — the fault tests assert state-machine behaviour that isn't implemented yet.
**Spec refs:** §10.4, §11.2, §9.3, §9.9.
**Lives with:** [`firmware/hil/`](../../firmware/hil/) (fixture firmware/fixtures) — this doc is the
electronics-side fixture spec.

## 1. Fixture design (§10.4)

A bench fixture that presents the controller's real connectors with programmable stimuli and captures
its outputs, so fault scenarios run automatically and repeatably:

| Channel | Stimulus / capture | Drives controller's | Maps to |
|---|---|---|---|
| Programmable moisture | DAC or digipot on `MOIST_SIG` (or digital mock) | moisture ADC (GPIO4) | §10.4 |
| Reservoir switch sim | relay/FET across float-switch pins | `RES_LOW_SW` (GPIO5) | §10.4 |
| Leak switch sim | relay across leak comparator input | `LEAK_DET` path (GPIO7) | §10.4 |
| Pump dummy load | resistor/electronic load + current shunt on `J_PUMP` | pump MOSFET drain | §10.4 |
| Fan tach sim | square-wave gen into `FAN_TACH` | tach (GPIO13) | §10.4 |
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
| H6 | Fan PWM changes | command duty sweep | fan PWM output tracks; tach sane | NORMAL |
| H7 | Status LED patterns | drive each fault | correct colour/position/pattern (§7.11) | per fault |
| H8 | **Watchdog resets safely** | stall firmware loop | watchdog fires; on reset **pump OFF** (gate pull-down) | →BOOT, pump OFF |
| H9 | Sensor fault → safe | implausible moisture | auto-water disabled, fault shown (§7.6) | SENSOR_FAULT |
| H10 | Over-temp derate | inject high air-temp | LED derates per §9.5 ladder | OVER_TEMP |

H2, H3, H4, H8 are the **gating safety tests** (spec §15.5 M4-09): pump lockout proven *in hardware*,
not just simulation (this is exactly the DR-02 gap — sim proves logic, HIL proves the device).

## 3. Calibrations run on the fixture (§9.9)

The §9.9 calibrations (moisture dry/wet, pump ml/s, fan min PWM, LED PPFD map, reservoir low) are run
here and their values populate the [WI-FW-11 NVS schema](../../plan/work-items/02-firmware/WI-FW-11-calibration-storage.md).
Procedure + result table is in [bringup.md §E](bringup.md).

## 4. Blocking dependencies

| Blocker | Why it blocks | Tracks |
|---|---|---|
| Fabricated controller board | No DUT to bring up / wire to the fixture | follows [WI-EE-07](../pcb/fabrication/fab-notes.md) Gerber export |
| **WI-FW-07 safety state machine** | H1–H10 assert state/priority/fail-off behaviour that the firmware must implement first | [02-firmware](../../plan/work-items/02-firmware/WI-FW-07-safety-state-machine.md) — *Not started* |
| WI-FW-08/04/05/06 controllers | LED/fan/pump command tests need the controllers | 02-firmware |

When WI-FW-07 lands and a board is fabricated, this matrix runs unchanged; the fixture itself can be
built and dry-validated against the firmware **simulator/HAL mocks** ahead of silicon.

## 5. Deliverable status

| Deliverable | State |
|---|---|
| Bring-up per §11.2 | procedure ✔ ([bringup.md](bringup.md)); execution ⛔ blocked (board + FW-07) |
| Pump-in-water / fan / LED / status verify | procedure ✔; execution ⛔ blocked |
| 24 h dry + 24 h wet soak | procedure ✔; execution ⛔ blocked |
| HIL fixture (§10.4) | design ✔ (this doc); build ⛔ pending |
| Automated HIL fault tests | matrix ✔ (H1–H10); run ⛔ blocked on WI-FW-07 |
| §9.9 calibrations → WI-FW-11 | procedure ✔; values ⛔ blocked |
