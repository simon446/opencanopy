//! OpenCanopy ESP32-S3 firmware binary. Boots and runs the deterministic control loop (§9.2)
//! against the real esp-hal peripheral drivers in [`hw`], wired to the committed pin map.
//!
//! All control policy lives in the platform-agnostic `control` crate (host-tested, §10.2); this
//! binary is the hardware binding. Build with `--features emulator` to run the loop with fast
//! simulated timing + serial telemetry for the Wokwi smoke test (`controller/wokwi/`); the default
//! build runs the production 5-minute cadence.
//!
//! STATUS: the esp-hal driver code is verified by cross-compilation and the Wokwi run (with mock
//! I2C chips); calibration values, the WS2812 status-LED RMT driver, and the fan-tach PCNT input
//! are completed/validated at WI-EE-08 bring-up against real hardware and the finalized pin map.

#![no_std]
#![no_main]

mod hw;

use control::app_state::FIRMWARE_VERSION;
use esp_backtrace as _;
use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    // Default clock config (forcing CpuClock::max reconfigures the PLL, which the Wokwi emulator
    // doesn't fully model and can hang boot; the default is fine for this controller).
    let peripherals = esp_hal::init(esp_hal::Config::default());
    println!("=== OpenCanopy firmware v{FIRMWARE_VERSION} (ESP32-S3) ===");
    hw::run(peripherals)
}
