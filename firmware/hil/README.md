# firmware/hil/

Hardware-in-the-loop (HIL) test harness for OpenCanopy. Runs the firmware on a real ESP32-S3 board
with instrumented actuators and simulated/sensed inputs, to verify behavior that pure simulation
cannot fully guarantee. Owned by the **Firmware** track, coordinated with **Electronics** (spec §10.4).

## Layout

- `fixtures/` — bench fixtures, harness adapters, signal injectors, and instrumentation configs that
  connect a board under test to controlled stimuli (e.g. forced leak signal, reservoir-low signal,
  fan-stall, over-temperature injection).

## Purpose

HIL proves the safety-critical paths on real silicon: pump fail-off on reset/brownout, leak-triggered
pump lockout, fan-failure detection, and LED-status correctness under injected faults. HIL pass is a
V1 release gate (spec §21 *Electronics*/*Firmware*).
