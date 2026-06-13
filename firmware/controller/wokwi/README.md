# controller/wokwi/ — ESP32-S3 emulator smoke test

A **smoke test** that runs the real firmware binary on an emulated ESP32-S3 (Wokwi), as a CI-level
complement to the host unit tests and the `sim/` logic simulator. It proves the binary **boots,
links esp-hal, and runs the genuine control loop without panicking on emulated silicon**, with
serial output asserted by the Wokwi CLI.

This is *not* a replacement for hardware-in-the-loop. The emulator does not reproduce analog signal
fidelity (capacitive-moisture ADC curves, pump current, fan-tach timing) — that stays with WI-EE-08
bring-up / WI-QA-05 fault injection.

## What it does

The `emulator` Cargo feature (`src/emulator.rs`) boots the real `control::app_state::App` with a
valid calibration and runs ~1 simulated day of the 5-min control loop, driven by synthesized
sensors (moisture dries out → triggers watering → recovers), printing telemetry over UART. The
Wokwi scenario waits for the boot banner, confirms calibration loaded, and waits for the completion
sentinel; the CI command fails the run on any `panicked` text.

## Run locally (needs the esp toolchain + a Wokwi token)

```sh
cd firmware/controller
cargo build --release --features emulator       # -> target/xtensa-esp32s3-none-elf/release/controller
export WOKWI_CLI_TOKEN=...                       # from https://wokwi.com (free for open source)
curl -L https://wokwi.com/ci/install.sh | sh     # installs wokwi-cli
cd wokwi
wokwi-cli --timeout 30000 --fail-text "panicked" --scenario scenario.yaml .
```

## CI

The `emulator-smoke` job in [`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml) builds
the ELF with the esp toolchain and runs the scenario. It is **non-blocking** (`continue-on-error`)
and self-skips with a NOTICE if `WOKWI_CLI_TOKEN` is not configured as a repo secret.

## Status / honesty

Authored by the firmware track but **not verified on the host** (no Xtensa toolchain offline). It is
proven the first time it runs in CI with a token, or at bring-up. The board model is minimal (just
the dev board + serial monitor); modeling the analog sensors as Wokwi custom chips for closed-loop
emulation is a possible later step — though closed-loop behavior is already covered better by `sim/`.
