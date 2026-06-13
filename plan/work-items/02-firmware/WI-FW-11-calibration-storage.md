# WI-FW-11 — Calibration storage

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | (supports M3, M6) |
| Depends on | WI-FW-02 |
| Spec refs | §9.9, §7.6 |
| Status | Not started |

## Objective

Implement persistent calibration storage that separates hidden/developer calibration from user
config, with safe handling of missing/corrupt data.

## Deliverables

- [ ] Flash-backed calibration store (e.g. `esp-storage` + `sequential-storage`, no ESP-IDF NVS) for
      the §9.9 schema: `moisture_raw_dry/wet`, `pump_ml_per_sec`, `fan_min_pwm`, `led_ppfd_map`,
      `reservoir_low_adc`. Serialize with a `no_std` codec (e.g. `postcard`).
- [ ] Safe defaults + validation: missing/corrupt calibration disables auto-watering and raises fault
      rather than acting on bad data (§7.6 fail-safe).
- [ ] Calibration version stamped into logs.
- [ ] Unit tests for defaults, missing, and corrupt calibration (spec §10.2 "Calibration store (flash)").

## Acceptance criteria

- Defaults/missing/corrupt calibration tests pass.
- Implausible moisture reading disables auto-watering and shows fault (§7.6).
