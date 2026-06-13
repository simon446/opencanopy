# controller/wokwi/ — ESP32-S3 emulator smoke test

A **manual, on-silicon smoke test**: runs the real firmware binary (real esp-hal drivers) on an
emulated ESP32-S3 (Wokwi) and asserts on its serial output. It complements — does not replace — the
cheap host validation (`cargo test -p control`, incl. the I2C mock-bus tests) and the xtensa
cross-compile. Full analog/water fidelity stays with HIL (WI-EE-08).

> **Wokwi runs cost cloud-simulation minutes**, so this is a **manual** CI job (`workflow_dispatch`)
> and should be run sparingly. Validate driver logic with `cargo test` first.

## What the diagram exercises

`diagram.json` wires (to the committed pin map):

- **Native DS1307 RTC** on I2C0 (SDA=GPIO8, SCL=GPIO9) — a stand-in for the DS3231 (same 0x68
  address + BCD time registers). Proves the **real I2C bus + `read_ds3231` driver** end-to-end.
- **4.7 kΩ I2C pull-ups** to 3V3 (per pin-map note) on SDA/SCL.
- **Moisture voltage-divider** on the ADC pin (GPIO4).
- **UART0 → serial monitor** wiring (required for Wokwi to capture output).

## Expected result (validated)

```
boot: state=NORMAL rtc_valid=true            <- I2C bus + RTC driver work on silicon
[t=0m] … [t=35m]  stage=S2 light=1 led=61%   <- light schedule running off the valid RTC
REAL-DRIVER SMOKE TEST COMPLETE: ran 8 ticks
```

`temp`/`pump_mA` read `-99`/`-1` because the SHT40 and INA219 are absent (no native Wokwi parts),
and the firmware correctly **fail-safes to SENSOR_FAULT** — the §7.6 behavior, shown on silicon.

Known limitation: Wokwi's ESP32-S3 **ADC** simulation does not return the divider voltage, so
`moist` reads invalid (`-1`). The ADC read + moisture-calibration logic is already host-tested
(`control::calibration`, `irrigation_controller::MoistureValidator`); validating the analog probe
itself is an HIL task. SHT40/INA219 could be added as Wokwi **custom chips** (Rust→WASM, I2C device
API) for fuller bus coverage — deferred since the driver logic is host-tested with mocks.

## Run

```sh
cd firmware/controller && cargo build --release --features emulator
export WOKWI_CLI_TOKEN=...            # from https://wokwi.com/dashboard/ci
cd wokwi
wokwi-cli --timeout 45000 --expect-text "REAL-DRIVER SMOKE TEST COMPLETE" --fail-text "panicked" .
```

CI: the manual `emulator-smoke` job in [`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml)
(`workflow_dispatch` only).
