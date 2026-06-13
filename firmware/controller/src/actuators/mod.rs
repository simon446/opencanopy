//! Actuator bindings: esp-hal peripherals → `control::hal` actuator traits (§9.6, §9.7, §9.8).
//! Method bodies are bring-up stubs (WI-EE-08); the trait shapes bind the verified logic.

use control::hal::{Fan, GrowLed, LedId, Pump, StatusLeds};
use control::led_status::{LedColor, LedPattern};

/// Pump drive via a low-side MOSFET GPIO. The gate has a hardware pull-down (WI-EE-03) so the pump
/// is OFF on reset/brownout; this binding additionally drives it low in its constructor.
pub struct PumpGpio {
    // held: output pin (+ optional current-sense ADC)
}
impl Pump for PumpGpio {
    fn set(&mut self, _on: bool) {
        // drive gate high/low
    }
    fn current_ma(&self) -> Option<u16> {
        None // populate if the sense resistor is fitted (enables the disconnected-pump fault, §10.3)
    }
}

/// Circulation fan: LEDC PWM out + tachometer capture in (§9.7).
pub struct VentFan {
    // held: LEDC channel + tach input/counter
}
impl Fan for VentFan {
    fn set_duty(&mut self, _pct: u8) {
        // set LEDC duty from percent
    }
    fn tach_rpm(&self) -> Option<u16> {
        None // Some(0) while commanded on → FAN_FAULT in control
    }
}

/// Dimmable grow LED on an LEDC PWM channel (§9.5). Percent is commanded power; PPFD mapping is
/// the `led_ppfd_map` calibration in `control`.
pub struct GrowLedPwm {
    // held: LEDC channel
}
impl GrowLed for GrowLedPwm {
    fn set_power(&mut self, _pct: u8) {
        // set LEDC duty from percent
    }
}

/// Driver for the 5 front status LEDs (§9.8). Renders (color, pattern) to whatever the status-LED
/// board uses (discrete RGB or addressable); blink patterns are timed here from a periodic tick.
pub struct StatusLedDriver {
    // held: GPIOs / addressable LED bus + pattern phase
}
impl StatusLeds for StatusLedDriver {
    fn set(&mut self, _id: LedId, _color: LedColor, _pattern: LedPattern) {
        // latch desired (color, pattern); a timer ISR animates the blink phases
    }
}
