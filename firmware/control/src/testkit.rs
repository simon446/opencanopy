//! Host mock implementations of the [`crate::hal`] traits, with injectable readings and
//! fault-injection hooks. Spec §10.1, WI-FW-02.
//!
//! These let the entire control stack run on the host with simulated sensors/actuators and a
//! simulated clock — the seam that makes the §10.2 unit tests and the §10.3 simulator possible.
//! They are `no_std`-compatible (pure `core`) so the same mocks can run on-device for bring-up.
//! The simulator (`sim/`) builds its plant/environment models on top of these.

use crate::hal::{
    Clock, GrowLed, LeakSensor, LedHeatSensor, LedId, MoistureSensor, ReservoirSensor, Rtc,
    SensorError, StatusLeds, TempRh, TempRhSensor, WallTime,
};
use crate::led_status::{LedColor, LedPattern};

/// A clock whose time is set explicitly by the test/simulator.
#[derive(Debug, Clone, Copy, Default)]
pub struct MockClock {
    pub now_ms: u64,
}
impl MockClock {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn advance_ms(&mut self, dt: u64) {
        self.now_ms += dt;
    }
}
impl Clock for MockClock {
    fn now_ms(&self) -> u64 {
        self.now_ms
    }
}

/// A settable RTC; can be marked invalid to exercise the §9.4 fallback.
#[derive(Debug, Clone, Copy)]
pub struct MockRtc {
    pub time: WallTime,
}
impl Default for MockRtc {
    fn default() -> Self {
        MockRtc {
            time: WallTime::INVALID,
        }
    }
}
impl Rtc for MockRtc {
    fn wall_time(&self) -> WallTime {
        self.time
    }
}

/// Air temp/RH sensor with an injectable fault.
#[derive(Debug, Clone, Copy)]
pub struct MockTempRh {
    pub reading: TempRh,
    pub fault: Option<SensorError>,
}
impl Default for MockTempRh {
    fn default() -> Self {
        MockTempRh {
            reading: TempRh {
                temp_c: 24.0,
                rh_pct: 60.0,
            },
            fault: None,
        }
    }
}
impl TempRhSensor for MockTempRh {
    fn read(&mut self) -> Result<TempRh, SensorError> {
        match self.fault {
            Some(e) => Err(e),
            None => Ok(self.reading),
        }
    }
}

/// Capacitive moisture probe returning an injectable raw count. Set `fault` to force an error, or
/// hold `raw` constant to emulate a stuck probe (the [`crate::moisture_monitor::MoistureValidator`]
/// detects the stuck condition over time).
#[derive(Debug, Clone, Copy, Default)]
pub struct MockMoisture {
    pub raw: u16,
    pub fault: Option<SensorError>,
}
impl MoistureSensor for MockMoisture {
    fn read_raw(&mut self) -> Result<u16, SensorError> {
        match self.fault {
            Some(e) => Err(e),
            None => Ok(self.raw),
        }
    }
}

/// Reservoir sensor: raw ADC + an explicit low flag for the safety-critical signal.
#[derive(Debug, Clone, Copy, Default)]
pub struct MockReservoir {
    pub adc: u16,
    pub low: bool,
    pub fault: Option<SensorError>,
}
impl ReservoirSensor for MockReservoir {
    fn read_adc(&mut self) -> Result<u16, SensorError> {
        match self.fault {
            Some(e) => Err(e),
            None => Ok(self.adc),
        }
    }
}

/// Leak sensor with injectable wet state.
#[derive(Debug, Clone, Copy, Default)]
pub struct MockLeak {
    pub wet: bool,
}
impl LeakSensor for MockLeak {
    fn is_wet(&mut self) -> bool {
        self.wet
    }
}

/// Optional LED heat-sink NTC.
#[derive(Debug, Clone, Copy, Default)]
pub struct MockLedHeat {
    pub temp_c: Option<f32>,
}
impl LedHeatSensor for MockLedHeat {
    fn temp_c(&mut self) -> Option<f32> {
        self.temp_c
    }
}

/// Grow-LED mock recording the last commanded power.
#[derive(Debug, Clone, Copy, Default)]
pub struct MockLed {
    pub power_pct: u8,
}
impl GrowLed for MockLed {
    fn set_power(&mut self, pct: u8) {
        self.power_pct = pct;
    }
}

/// Status-LED mock storing the last (color, pattern) for each of the 4 LEDs.
#[derive(Debug, Clone, Copy)]
pub struct MockStatusLeds {
    pub states: [(LedColor, LedPattern); 4],
}
impl Default for MockStatusLeds {
    fn default() -> Self {
        MockStatusLeds {
            states: [(LedColor::Off, LedPattern::Off); 4],
        }
    }
}
impl MockStatusLeds {
    fn idx(id: LedId) -> usize {
        match id {
            LedId::Water => 0,
            LedId::Moisture => 1,
            LedId::Light => 2,
            LedId::System => 3,
        }
    }
    pub fn get(&self, id: LedId) -> (LedColor, LedPattern) {
        self.states[Self::idx(id)]
    }
}
impl StatusLeds for MockStatusLeds {
    fn set(&mut self, id: LedId, color: LedColor, pattern: LedPattern) {
        self.states[Self::idx(id)] = (color, pattern);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_advances() {
        let mut c = MockClock::new();
        c.advance_ms(5000);
        assert_eq!(c.now_ms(), 5000);
    }

    #[test]
    fn moisture_fault_injection() {
        let mut m = MockMoisture {
            raw: 2000,
            fault: None,
        };
        assert_eq!(m.read_raw(), Ok(2000));
        m.fault = Some(SensorError::Stuck);
        assert_eq!(m.read_raw(), Err(SensorError::Stuck));
    }

    #[test]
    fn status_leds_store_per_position() {
        let mut leds = MockStatusLeds::default();
        leds.set(LedId::Water, LedColor::Red, LedPattern::FastBlink);
        assert_eq!(
            leds.get(LedId::Water),
            (LedColor::Red, LedPattern::FastBlink)
        );
        assert_eq!(leds.get(LedId::System), (LedColor::Off, LedPattern::Off));
    }

    #[test]
    fn leak_injection() {
        let mut l = MockLeak { wet: false };
        assert!(!l.is_wet());
        l.wet = true;
        assert!(l.is_wet());
    }
}
