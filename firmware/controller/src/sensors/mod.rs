//! Sensor bindings: esp-hal peripherals → `control::hal` sensor traits (§7.5, §7.6, §10.1).
//! Method bodies are bring-up stubs (WI-EE-08); the trait shapes are what bind the verified logic.

use control::hal::{MoistureSensor, ReservoirSensor, SensorError, TempRh, TempRhSensor};
use control::hal::LeakSensor;

/// SHT-class air temperature + humidity over I2C (§7.5). Range-checks before returning so an
/// implausible reading surfaces as a fault (§7.6) rather than a bad VPD input.
pub struct ShtAirSensor {
    // held: shared I2C bus handle
}
impl TempRhSensor for ShtAirSensor {
    fn read(&mut self) -> Result<TempRh, SensorError> {
        // measure → convert; reject NaN / out-of-range temp or RH∉[0,100].
        Err(SensorError::NotPresent) // TODO(WI-EE-08)
    }
}

/// Capacitive substrate probe on an ADC channel. Returns the **raw** count; normalization to %
/// is a calibration concern in `control` (never assume raw==moisture, watering-model §3).
pub struct MoistureAdc {
    // held: ADC driver + pin
}
impl MoistureSensor for MoistureAdc {
    fn read_raw(&mut self) -> Result<u16, SensorError> {
        Err(SensorError::NotPresent) // TODO(WI-EE-08): one-shot ADC read
    }
}

/// Reservoir level on an ADC channel (§9.9 `reservoir_low_adc`).
pub struct ReservoirAdc {
    // held: ADC driver + pin
}
impl ReservoirSensor for ReservoirAdc {
    fn read_adc(&mut self) -> Result<u16, SensorError> {
        Err(SensorError::NotPresent) // TODO(WI-EE-08)
    }
}

/// Leak/spill sensor as a GPIO (conductive or float switch). Conservative: read errors → "wet".
pub struct LeakGpio {
    // held: input pin
}
impl LeakSensor for LeakGpio {
    fn is_wet(&mut self) -> bool {
        false // TODO(WI-EE-08): read tray sensor; default-safe on uncertainty handled upstream
    }
}
