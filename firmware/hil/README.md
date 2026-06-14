# firmware/hil/

Hardware-in-the-loop (HIL) test harness for OpenCanopy. Runs the firmware on a real ESP32-S3 board
with instrumented actuators and simulated/sensed inputs, to verify behavior that pure simulation
cannot fully guarantee. Owned by the **Firmware** track, coordinated with **Electronics** (spec §10.4).

## Layout

- `fixtures/` — bench fixtures, harness adapters, signal injectors, and instrumentation configs that
  connect a board under test to controlled stimuli (e.g. forced leak signal, reservoir-low signal,
  abnormal-moisture signal, over-temperature injection).

## Purpose

V1 is passive (no pump, ECO-003) and fan-less (ECO-001): the grow LED is the only actuator and the
firmware **monitors and warns**. HIL proves the safety-critical paths on real silicon: the LED forced
off on reset/brownout, over-temperature LED cut-back (the grow LED is the only thermal lever), and the
warning/LED-status correctness for leak/overflow, reservoir-low, abnormal-moisture, and sensor-fault
under injected faults. HIL pass is a V1 release gate (spec §21 *Electronics*/*Firmware*).
