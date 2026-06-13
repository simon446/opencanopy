# WI-FW-01 — Firmware project & build

| Field | Value |
|---|---|
| Track | Firmware |
| Milestone | M3-01 |
| Depends on | WI-PS-01 |
| Spec refs | §9.1, §9.2, §14.1 |
| Status | Not started |

## Objective

Create the buildable Rust firmware **workspace** with the layout from spec §9.2, targeting the
ESP32-S3 (`no_std`, `esp-hal`), with a host build path so the `control` crate compiles and tests
off-target on stable Rust.

## Deliverables

- [ ] Cargo workspace under `firmware/` per §9.2: `control/` (no_std, platform-agnostic, host-testable),
      `controller/` (no_std `esp-hal` binary), `sim/` (host runner).
- [ ] `rust-toolchain.toml` pinning the Espressif Rust channel (espup) and `.cargo/config.toml` setting
      the default target `xtensa-esp32s3-none-elf` + `espflash` runner.
- [ ] `control/` builds and `cargo test`s for the **host** on stable Rust with **no** `esp-hal`
      dependency and no Xtensa toolchain.
- [ ] `controller/` builds for `xtensa-esp32s3-none-elf` with the esp toolchain.
- [ ] CI hook so the workspace builds + tests on every PR (coordinate with [WI-PS-06](../00-project-setup/WI-PS-06-ci-pipeline.md)).

## Acceptance criteria

- `cargo test -p control` passes on the host (stable Rust, no hardware).
- `controller` cross-compiles clean for the ESP32-S3 target in CI.
- Workspace layout matches spec §9.2; `control/` has no dependency on `esp-hal`.

## Notes

Xtensa is not on upstream stable Rust — the target build needs the espup-installed `esp` channel; keep
host-testable logic in `control/` so contributors can run the unit/sim suite without that toolchain.
