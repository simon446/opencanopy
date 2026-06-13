# WI-FW-10 — Logging & connectivity

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-10 |
| Depends on | WI-FW-07 |
| Spec refs | §9.10, §9.11 |
| Status | Not started |

## Objective

Implement local rolling logs and the optional, offline-first connectivity surfaces. No control loop
may depend on connectivity.

## Deliverables

- [ ] Ring log capturing sensor readings (5–15 min), watering events, faults, LED derating,
      reservoir-low events, firmware + calibration versions (§9.10).
- [ ] ≥7 days onboard persistence; export over USB/serial.
- [ ] Optional Wi-Fi/MQTT/Home Assistant telemetry behind a Cargo **feature** (`telemetry`, using
      `esp-wifi`), default-off; control loop builds and runs with the feature disabled (§9.11).
- [ ] Test confirming all control paths function with connectivity disabled.

## Acceptance criteria

- Logs are exportable via serial and survive ≥7 days (spec §10 / §21 "Logs exportable").
- Disabling all connectivity changes no control behavior.
