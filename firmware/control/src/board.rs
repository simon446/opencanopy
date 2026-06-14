//! On-target frame-assembly helpers (host-tested). Turns raw board readings into a [`SensorFrame`],
//! applying the documented GPIO polarity. This logic used to live inline in the esp-hal binding
//! (`controller/hw.rs`), where cargo could never test it; pulling it here closes a real blind spot
//! — a swapped field or wrong pin polarity is silent and would only surface on hardware.
//!
//! The two boolean pin states are wrapped in newtypes so they **cannot be swapped** at the call
//! site (a compile error), and the polarity is documented + asserted once, here.

use crate::app_state::SensorFrame;
use crate::hal::{SensorError, TempRh, WallTime};

/// Raw state of the reservoir float-switch pin. Pin map `RES_LOW_SW`: internal pull-up, the switch
/// closes (pin LOW) when the reservoir is LOW. `true` = pin reads low.
///
/// ASSUMPTION to confirm at WI-EE-08 bring-up: the float's NO/NC orientation matches "closed = low
/// water". A host test cannot verify the physical wiring — only that the firmware applies *this*
/// documented convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReservoirFloatLow(pub bool);

/// Raw state of the leak-detect pin. Pin map: comparator output, **active-HIGH = leak**.
/// `true` = pin reads high.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeakPinHigh(pub bool);

/// Assemble a [`SensorFrame`] from raw board readings, applying GPIO polarity. The newtype
/// parameters make the reservoir/leak booleans impossible to transpose.
#[allow(clippy::too_many_arguments)]
pub fn build_sensor_frame(
    now_ms: u64,
    rtc: WallTime,
    temp_rh: Result<TempRh, SensorError>,
    moisture_raw: Result<u16, SensorError>,
    reservoir: ReservoirFloatLow,
    leak: LeakPinHigh,
    led_heat_c: Option<f32>,
) -> SensorFrame {
    SensorFrame {
        now_ms,
        rtc,
        temp_rh,
        moisture_raw,
        reservoir_low: reservoir.0, // RES_LOW_SW closed (LOW) == reservoir low
        leak: leak.0,               // active-HIGH == leak
        led_heat_c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_fields_and_polarity() {
        let f = build_sensor_frame(
            123,
            WallTime {
                valid: true,
                unix_s: 42,
            },
            Ok(TempRh {
                temp_c: 24.0,
                rh_pct: 60.0,
            }),
            Ok(2000),
            ReservoirFloatLow(true), // pin low => reservoir LOW
            LeakPinHigh(false),      // pin low => no leak
            Some(55.0),
        );
        assert_eq!(f.now_ms, 123);
        assert_eq!(f.rtc.unix_s, 42);
        assert_eq!(f.moisture_raw, Ok(2000));
        assert!(f.reservoir_low, "float pin low must map to reservoir_low");
        assert!(!f.leak, "leak pin low must map to no leak");
        assert_eq!(f.led_heat_c, Some(55.0));
    }

    #[test]
    fn leak_high_maps_to_leak_reservoir_high_maps_to_not_low() {
        let f = build_sensor_frame(
            0,
            WallTime::INVALID,
            Err(SensorError::Bus),
            Err(SensorError::NotPresent),
            ReservoirFloatLow(false), // pin high => reservoir NOT low
            LeakPinHigh(true),        // pin high => leak
            None,
        );
        assert!(f.leak);
        assert!(!f.reservoir_low);
        assert!(f.temp_rh.is_err());
        assert!(f.moisture_raw.is_err());
    }

    // End-to-end: a leak pin high, assembled into the frame and run through the App, must drive the
    // LEAK_DETECTED warning state — proving the whole controller-side path (board reading -> frame
    // -> control) on the host, not just the App in isolation. (V1 is passive: leak is a warning, not
    // a pump lockout — there is no pump.)
    #[test]
    fn assembled_leak_frame_drives_leak_state() {
        use crate::app_state::{App, AppConfig};
        use crate::calibration::Calibration;
        use crate::led_status::LedColor;
        use crate::safety_controller::SystemState;

        let cal = Calibration {
            version: 4,
            moisture_raw_dry: 1000,
            moisture_raw_wet: 3000,
            led_ppfd_25: 120,
            led_ppfd_50: 240,
            led_ppfd_75: 360,
            led_ppfd_100: 480,
            reservoir_low_adc: 600,
        }
        .encode();
        let mut app = App::boot(
            AppConfig::default(),
            Some(&cal),
            60,
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            true,
        );
        let frame = build_sensor_frame(
            0,
            WallTime {
                valid: true,
                unix_s: 8 * 3600,
            },
            Ok(TempRh {
                temp_c: 24.0,
                rh_pct: 60.0,
            }),
            Ok(1400), // dry-ish substrate
            ReservoirFloatLow(false),
            LeakPinHigh(true), // LEAK
            None,
        );
        let cmd = app.step(&frame);
        assert_eq!(cmd.state, SystemState::LeakDetected);
        // Flood warning reds the Water + System LEDs.
        assert_eq!(cmd.panel.water.color, LedColor::Red);
        assert_eq!(cmd.panel.system.color, LedColor::Red);
    }
}
