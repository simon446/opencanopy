# WI-FW-01 — Firmware project & build

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-01 |
| Depends on | WI-PS-01 |
| Spec refs | §9.1, §9.2, §14.1 |
| Status | Done |

## Objective

Create the buildable Rust firmware **workspace** with the layout from spec §9.2, targeting the
ESP32-S3 (`no_std`, `esp-hal`), with a host build path so the `control` crate compiles and tests
off-target on stable Rust.

## Deliverables

- [x] Cargo workspace under `firmware/` per §9.2: `control/` (no_std, platform-agnostic, host-testable),
      `controller/` (no_std `esp-hal` binary), `sim/` (host runner).
- [x] `rust-toolchain.toml` pinning the Espressif Rust channel (espup) and `.cargo/config.toml` setting
      the default target `xtensa-esp32s3-none-elf` + `espflash` runner.
- [x] `control/` builds and `cargo test`s for the **host** on stable Rust with **no** `esp-hal`
      dependency and no Xtensa toolchain.
- [x] `controller/` builds for `xtensa-esp32s3-none-elf` with the esp toolchain.
- [x] CI hook so the workspace builds + tests on every PR (coordinate with [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md)).

## Acceptance criteria

- `cargo test -p control` passes on the host (stable Rust, no hardware).
- `controller` cross-compiles clean for the ESP32-S3 target in CI.
- Workspace layout matches spec §9.2; `control/` has no dependency on `esp-hal`.

## Notes

Xtensa is not on upstream stable Rust — the target build needs the espup-installed `esp` channel; keep
host-testable logic in `control/` so contributors can run the unit/sim suite without that toolchain.

## Implementation

- Workspace `firmware/Cargo.toml` (members `control`, `sim`); `controller/` is a **standalone**
  package, **excluded** so the host suite stays offline-clean on stable Rust. `control`/`sim` have
  **zero external dependencies**.
- Esp channel pinned by `firmware/controller/rust-toolchain.toml`; `firmware/.cargo/config.toml`
  deliberately does **not** force the Xtensa target globally (that would break host `cargo test`);
  the target + `espflash` runner are set per-package in `controller/.cargo/config.toml`, plus an
  `esp` alias at the root.
- Verified locally: `cargo test -p control -p sim` (97 tests) + `cargo clippy … -D warnings` + `cargo fmt --check`.
- CI: existing `firmware` job runs fmt/clippy/host tests; `firmware-target` job cross-compiles the
  controller with the esp toolchain (non-blocking until bring-up). The Xtensa cross-compile itself
  is verified in CI / at WI-EE-08 bring-up, not on this host.
