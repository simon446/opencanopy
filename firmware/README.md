# firmware/

Firmware for the OpenCanopy controller (ESP32-S3), written in **Rust** (`no_std`, `esp-hal`), plus its
host-side simulation and hardware-in-the-loop (HIL) harnesses. Owned by the **Firmware** track. See
spec §9–§10.

This is a Cargo **workspace**. The core control logic is a platform-agnostic `no_std` crate that builds
and unit-tests on the host with stable Rust — no ESP32 and no Xtensa toolchain required to validate
rules (spec §9.1, §10.1). Only the on-target `controller` binary needs the Espressif toolchain.

## Layout

- `Cargo.toml` — workspace manifest (added by the Firmware track, [WI-FW-01](../plan/work-items/02-firmware/WI-FW-01-project-build.md)).
- `rust-toolchain.toml` — pins the Espressif Rust channel (installed via `espup`).
- `.cargo/config.toml` — default target `xtensa-esp32s3-none-elf` + `espflash` runner.
- `control/` — **no_std, platform-agnostic** control logic: state machine, irrigation, light, climate,
  LED status, plant profile, and the `hal.rs` sensor/actuator/clock **traits** (the hardware seam).
  Builds and `cargo test`s on the host with stable Rust. Has **no** `esp-hal` dependency.
  - `src/`, `tests/`.
- `controller/` — **no_std `esp-hal` binary** for the ESP32-S3; provides concrete trait
  implementations binding `control` to real peripherals.
  - `src/` — `main.rs` plus `sensors/`, `actuators/`, `drivers/`.
- `sim/` — host scenario runner that drives the real `control` crate through simulated trait impls
  (no hardware). See `sim/README.md`.
  - `src/`, `scenarios/`, `models/`.
- `hil/` — hardware-in-the-loop fixtures and procedures for testing on real boards. See `hil/README.md`.
  - `fixtures/`.

## Toolchain

- **Host tests / sim:** stable Rust — `cargo test -p control`, `cargo fmt --check`, `cargo clippy`.
- **On-target build:** install the esp channel with `espup`, then `cargo build` (target set in
  `.cargo/config.toml`); flash/monitor with `espflash`. Xtensa is not on upstream stable Rust, which is
  why host-testable logic is isolated in `control/`.

## Principles (spec §9.1, §10.1)

Control rules are deterministic and offline-first, validated in simulation before any PCB exists, then
re-validated on hardware via HIL. The state machine and fault priorities (§9.3) are encoded as
compiler-checked Rust types. Pump output must fail safe (off) on reset/brownout. Optional Wi-Fi/MQTT
telemetry (§9.11) lives behind a default-off Cargo feature and is never required for control.
