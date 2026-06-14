# WI-FW-11 — Calibration storage

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | (supports M3, M6) |
| Depends on | WI-FW-02 |
| Spec refs | §9.9, §7.6 |
| Status | Done |

## Objective

Implement persistent calibration storage that separates hidden/developer calibration from user
config, with safe handling of missing/corrupt data.

## Deliverables

- [x] Flash-backed calibration store (e.g. `esp-storage` + `sequential-storage`, no ESP-IDF NVS) for
      the §9.9 schema: `moisture_raw_dry/wet`, `led_ppfd_map`, `reservoir_low_adc`. Serialize with a
      `no_std` codec (e.g. `postcard`). *(`fan_min_pwm` removed with the fan (ECO-001) and
      `pump_ml_per_sec` removed with the pump (ECO-003); the record is now schema **v4**, 24 bytes,
      so any old 28/30-byte blob fails the length check → fail-safe. Spec §9.9 still lists the dropped
      fields and needs the Project track's pass.)*
- [x] Safe defaults + validation: missing/corrupt calibration disables auto-watering and raises fault
      rather than acting on bad data (§7.6 fail-safe).
- [x] Calibration version stamped into logs.
- [x] Unit tests for defaults, missing, and corrupt calibration (spec §10.2 "Calibration store (flash)").

## Acceptance criteria

- Defaults/missing/corrupt calibration tests pass.
- Implausible moisture reading disables auto-watering and shows fault (§7.6).

## Implementation

- `control/src/calibration.rs`: the §9.9 schema, a dependency-free fixed-layout codec with a
  CRC32 trailer (host-testable; `postcard`/`sequential-storage` are the on-target swap-in), plus
  `validate()`/`load()` implementing the §7.6 fail-safe — missing/corrupt/implausible calibration
  disables auto-watering and raises a fault rather than acting on bad data. Calibration `version`
  is stamped into the log at boot. Host-tested (defaults, missing, corrupt-CRC, truncated,
  implausible). Flash IO is the `controller/` binding (esp-storage), verified at bring-up.
