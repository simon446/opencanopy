# controller/ — ESP32-S3 on-target binary

The bare-metal firmware that runs on the device. It is a **thin binding layer**: every control rule
lives in the platform-agnostic [`control`](../control) crate (host-tested, spec §10.2). This binary
only initializes esp-hal peripherals, wires them to `control`'s `hal.rs` traits, and runs the 5-min
control loop.

## Build / flash (requires the Espressif toolchain)

Xtensa is **not** on upstream stable Rust. Install the esp channel once, then build from this
directory (the target + flasher come from `controller/.cargo/config.toml`):

```sh
cargo install espup espflash
espup install            # installs the `esp` toolchain pinned by controller/rust-toolchain.toml
cd firmware/controller
cargo build --release    # -> xtensa-esp32s3-none-elf
cargo run   --release    # flash + serial monitor via espflash
cargo build --release --features telemetry   # optional Wi-Fi/MQTT (default-off, §9.11)
```

Host crates (`control/`, `sim/`) do **not** need any of this — run `cargo test` from `firmware/`.

## Why a standalone package (excluded from the workspace)

esp-hal/esp-wifi only build for `xtensa-esp32s3-none-elf` and pull from crates.io. Keeping this crate
out of the host workspace (`firmware/Cargo.toml` `exclude`) means the host unit/sim suite never tries
to resolve or build them — it stays offline-clean on stable Rust (WI-FW-01 acceptance).

## Status / ownership

The module structure and control-loop sequencing are fixed here (`main.rs`, `sensors/`,
`actuators/`, `drivers/`). The esp-hal peripheral construction and the **pin map** are finalized
during electronics bring-up ([WI-EE-08](../../plan/work-items/03-electronics/WI-EE-08-bringup-hil.md));
binding method bodies marked `TODO(WI-EE-08)` are completed against real hardware there. Pump-off on
reset is guaranteed by a hardware gate pull-down ([WI-EE-03](../../plan/work-items/03-electronics/WI-EE-03-schematic.md))
plus the RWDT/brownout detector armed in `drivers::Platform`.
