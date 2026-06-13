# WI-FW-10 — Logging & connectivity

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-10 |
| Depends on | WI-FW-07 |
| Spec refs | §9.10, §9.11, §16.1 (RTC), §23 (DR-05) |
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
- [ ] **Time source:** read the battery-backed RTC (§16.1, DS3231/RV-3028-class) as the authoritative
      wall clock for the light schedule and log timestamps; on RTC-invalid use the §10.3 safe-schedule
      fallback. Optional **NTP sync only when** the `telemetry` feature is enabled and a network is
      present — never required (offline-first, §23 DR-05). Coordinate the planting-epoch / grow-cycle
      age read with [WI-FW-07](WI-FW-07-safety-state-machine.md) boot.
- [ ] Test confirming all control paths function with connectivity disabled.

## Acceptance criteria

- Logs are exportable via serial and survive ≥7 days (spec §10 / §21 "Logs exportable").
- Disabling all connectivity changes no control behavior.
- Scheduling and log timestamps use the RTC; after a power cycle with no network, the photoperiod
  resumes on the correct wall-clock (or the documented safe fallback) — not a reset clock (§23 DR-05).
