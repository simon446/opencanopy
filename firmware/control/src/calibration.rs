//! Persistent calibration storage. Spec §9.9, §7.6.
//!
//! Separates hidden/developer calibration (this struct) from user configuration (there is none in
//! V1 — it is a no-config appliance). The on-target build persists this to flash via
//! `esp-storage` + `sequential-storage`; here we provide a **dependency-free fixed-layout codec
//! with a CRC32** so the encode/decode/validate logic is host-testable offline and identical on
//! target. (Swapping in `postcard` on-device is a `controller/` binding concern; the schema and
//! the fail-safe rules live here.)
//!
//! Fail-safe rule (§7.6): missing or corrupt calibration must **disable auto-watering and raise a
//! fault** rather than act on implausible data. A capacitive raw count means nothing without the
//! per-media dry/wet points, so we never guess them.

/// Calibration schema (§9.9). `led_ppfd_map` is flattened to its four measured grid points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Calibration {
    /// Bumped whenever the schema or values change; stamped into logs (§9.10).
    pub version: u16,
    pub moisture_raw_dry: u16,
    pub moisture_raw_wet: u16,
    /// Measured canopy PPFD at 25/50/75/100 % LED power (§9.9 `led_ppfd_map`).
    pub led_ppfd_25: u16,
    pub led_ppfd_50: u16,
    pub led_ppfd_75: u16,
    pub led_ppfd_100: u16,
    pub reservoir_low_adc: u16,
}

/// Why the calibration was rejected by [`Calibration::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalError {
    /// Bad magic, wrong length, or CRC mismatch — storage corrupt.
    Corrupt,
    /// No calibration record present (first boot / erased flash).
    Missing,
    /// Decoded cleanly but values are implausible (e.g. dry≈wet).
    Implausible,
}

/// Where a loaded calibration came from. Only [`CalSource::Valid`] lets the firmware trust the
/// moisture reading (§7.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalSource {
    Valid,
    Missing,
    Corrupt,
    Implausible,
}

/// Result of loading from storage: the calibration to use plus whether the moisture reading may be
/// trusted. V1 is passive (no pump) — this no longer gates any actuation, only whether the moisture
/// monitor reports a value or raises SENSOR_FAULT.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadedCalibration {
    pub cal: Calibration,
    pub source: CalSource,
    /// False whenever calibration is missing/corrupt/implausible — the moisture monitor must then
    /// treat moisture as untrustworthy and raise SENSOR_FAULT rather than report a bogus value
    /// (§7.6). Only a fully valid record trusts the moisture reading.
    pub moisture_trusted: bool,
}

const MAGIC: u32 = 0x4F43_414C; // "OCAL"
/// Encoded record length in bytes. Layout: magic(4) version(2) dry(2) wet(2)
/// ppfd25/50/75/100(2×4) reservoir(2) crc(4). The `pump_ml_per_sec` field was removed when the pump
/// was dropped from V1 (ECO-003, passive watering); an old 28/30-byte record now fails the length
/// check below → fail-safe.
pub const RECORD_LEN: usize = 4 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 2 + 4;

impl Calibration {
    /// Conservative engineering defaults. NOTE: these are *placeholders for non-moisture fields*;
    /// they are NOT a valid moisture calibration, so loading defaults still distrusts moisture.
    pub const DEFAULTS: Calibration = Calibration {
        version: 4, // schema v4: pump_ml_per_sec removed (ECO-003, passive watering)
        // Deliberately equal so `validate()` reports Implausible if defaults are ever used as-is —
        // forces a real dev calibration before the moisture monitor will trust a reading (§7.6).
        moisture_raw_dry: 0,
        moisture_raw_wet: 0,
        led_ppfd_25: 120,
        led_ppfd_50: 240,
        led_ppfd_75: 360,
        led_ppfd_100: 480,
        reservoir_low_adc: 600,
    };

    /// Validate plausibility (§7.6). Independent of the codec/CRC.
    pub fn validate(&self) -> Result<(), CalError> {
        // Moisture span must be real and correctly ordered, with enough separation to normalize.
        if self.moisture_raw_wet <= self.moisture_raw_dry
            || self.moisture_raw_wet - self.moisture_raw_dry < 100
        {
            return Err(CalError::Implausible);
        }
        // LED map must be monotonic non-decreasing and non-trivial.
        if !(self.led_ppfd_25 <= self.led_ppfd_50
            && self.led_ppfd_50 <= self.led_ppfd_75
            && self.led_ppfd_75 <= self.led_ppfd_100
            && self.led_ppfd_100 > 0)
        {
            return Err(CalError::Implausible);
        }
        Ok(())
    }

    /// Normalize a raw capacitive count to 0..=100 (watering-model §3). Returns `None` when the
    /// calibration is invalid or the reading is implausibly outside the calibrated span — both of
    /// which mean "do not auto-water" (§7.6). The caller treats `None` as a moisture sensor fault.
    pub fn normalize_moisture(&self, raw: u16) -> Option<f32> {
        self.validate().ok()?;
        let dry = self.moisture_raw_dry as f32;
        let wet = self.moisture_raw_wet as f32;
        let pct = (raw as f32 - dry) / (wet - dry) * 100.0;
        // Allow a small margin beyond [dry,wet]; far outside means a disconnected/shorted probe.
        if !(-20.0..=120.0).contains(&pct) {
            return None;
        }
        Some(crate::math::clampf(pct, 0.0, 100.0))
    }

    /// Commanded LED power (%) needed to hit a target PPFD, via piecewise-linear interpolation of
    /// the measured `led_ppfd_map` (§9.9). Clamped to `[0, 100]`.
    pub fn percent_for_ppfd(&self, target_ppfd: u16) -> u8 {
        let pts = [
            (0u8, 0u16),
            (25, self.led_ppfd_25),
            (50, self.led_ppfd_50),
            (75, self.led_ppfd_75),
            (100, self.led_ppfd_100),
        ];
        let target = target_ppfd as f32;
        if target <= 0.0 {
            return 0;
        }
        if target >= self.led_ppfd_100 as f32 {
            return 100;
        }
        for w in pts.windows(2) {
            let (p0, y0) = (w[0].0 as f32, w[0].1 as f32);
            let (p1, y1) = (w[1].0 as f32, w[1].1 as f32);
            if target >= y0 && target <= y1 && y1 > y0 {
                let frac = (target - y0) / (y1 - y0);
                return crate::math::clampf(p0 + frac * (p1 - p0), 0.0, 100.0) as u8;
            }
        }
        100
    }

    /// Encode to the fixed-layout record (with CRC32 trailer).
    pub fn encode(&self) -> [u8; RECORD_LEN] {
        let mut b = [0u8; RECORD_LEN];
        let mut i = 0;
        b[i..i + 4].copy_from_slice(&MAGIC.to_le_bytes());
        i += 4;
        b[i..i + 2].copy_from_slice(&self.version.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.moisture_raw_dry.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.moisture_raw_wet.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.led_ppfd_25.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.led_ppfd_50.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.led_ppfd_75.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.led_ppfd_100.to_le_bytes());
        i += 2;
        b[i..i + 2].copy_from_slice(&self.reservoir_low_adc.to_le_bytes());
        i += 2;
        let crc = crc32(&b[..i]);
        b[i..i + 4].copy_from_slice(&crc.to_le_bytes());
        b
    }

    /// Decode and CRC-check a record. Distinguishes corrupt storage from implausible values.
    pub fn decode(bytes: &[u8]) -> Result<Calibration, CalError> {
        if bytes.len() != RECORD_LEN {
            return Err(CalError::Corrupt);
        }
        let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic != MAGIC {
            return Err(CalError::Corrupt);
        }
        let body = &bytes[..RECORD_LEN - 4];
        let stored_crc = u32::from_le_bytes([
            bytes[RECORD_LEN - 4],
            bytes[RECORD_LEN - 3],
            bytes[RECORD_LEN - 2],
            bytes[RECORD_LEN - 1],
        ]);
        if crc32(body) != stored_crc {
            return Err(CalError::Corrupt);
        }
        let u16a = |i: usize| u16::from_le_bytes([bytes[i], bytes[i + 1]]);
        let cal = Calibration {
            version: u16a(4),
            moisture_raw_dry: u16a(6),
            moisture_raw_wet: u16a(8),
            led_ppfd_25: u16a(10),
            led_ppfd_50: u16a(12),
            led_ppfd_75: u16a(14),
            led_ppfd_100: u16a(16),
            reservoir_low_adc: u16a(18),
        };
        cal.validate()?;
        Ok(cal)
    }
}

/// Load calibration from a stored byte slice (`None` = no record present). Applies the §7.6
/// fail-safe policy and returns whether the moisture reading may be trusted.
pub fn load(stored: Option<&[u8]>) -> LoadedCalibration {
    match stored {
        None => LoadedCalibration {
            cal: Calibration::DEFAULTS,
            source: CalSource::Missing,
            moisture_trusted: false,
        },
        Some(bytes) => match Calibration::decode(bytes) {
            Ok(cal) => LoadedCalibration {
                cal,
                source: CalSource::Valid,
                moisture_trusted: true,
            },
            Err(CalError::Corrupt) => LoadedCalibration {
                cal: Calibration::DEFAULTS,
                source: CalSource::Corrupt,
                moisture_trusted: false,
            },
            Err(_) => LoadedCalibration {
                cal: Calibration::DEFAULTS,
                source: CalSource::Implausible,
                moisture_trusted: false,
            },
        },
    }
}

/// IEEE 802.3 CRC-32 (reflected), table-less. Enough to catch flash bit-rot for a small record.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good() -> Calibration {
        Calibration {
            version: 4,
            moisture_raw_dry: 1234,
            moisture_raw_wet: 2870,
            led_ppfd_25: 120,
            led_ppfd_50: 240,
            led_ppfd_75: 360,
            led_ppfd_100: 480,
            reservoir_low_adc: 600,
        }
    }

    // §10.2 "Calibration store (flash) — defaults, missing/corrupt calibration".
    #[test]
    fn roundtrip_encode_decode() {
        let c = good();
        let bytes = c.encode();
        assert_eq!(bytes.len(), RECORD_LEN);
        assert_eq!(Calibration::decode(&bytes).unwrap(), c);
    }

    #[test]
    fn missing_distrusts_moisture() {
        let l = load(None);
        assert_eq!(l.source, CalSource::Missing);
        assert!(!l.moisture_trusted);
    }

    #[test]
    fn corrupt_crc_distrusts_moisture() {
        let mut bytes = good().encode();
        let n = bytes.len();
        bytes[n - 1] ^= 0xFF; // flip a CRC byte
        assert_eq!(Calibration::decode(&bytes), Err(CalError::Corrupt));
        let l = load(Some(&bytes));
        assert_eq!(l.source, CalSource::Corrupt);
        assert!(!l.moisture_trusted);
    }

    #[test]
    fn truncated_record_is_corrupt() {
        let bytes = good().encode();
        assert_eq!(Calibration::decode(&bytes[..10]), Err(CalError::Corrupt));
    }

    #[test]
    fn implausible_values_rejected() {
        let mut c = good();
        c.moisture_raw_wet = c.moisture_raw_dry; // zero span
        assert_eq!(c.validate(), Err(CalError::Implausible));
        // and through the codec it surfaces as implausible, not corrupt
        let bytes = c.encode();
        assert_eq!(Calibration::decode(&bytes), Err(CalError::Implausible));
        assert!(!load(Some(&bytes)).moisture_trusted);
    }

    #[test]
    fn defaults_alone_do_not_trust_moisture() {
        // §7.6: defaults are not a real moisture calibration; the reading must not be trusted.
        assert_eq!(Calibration::DEFAULTS.validate(), Err(CalError::Implausible));
    }

    #[test]
    fn normalize_moisture_maps_span() {
        let c = good();
        assert_eq!(c.normalize_moisture(1234), Some(0.0)); // dry point
        assert_eq!(c.normalize_moisture(2870), Some(100.0)); // wet point
        let mid = c.normalize_moisture((1234 + 2870) / 2).unwrap();
        assert!((mid - 50.0).abs() < 1.0);
    }

    #[test]
    fn normalize_rejects_implausible_reading() {
        let c = good();
        // Way below the dry count → disconnected probe → None (sensor fault, no watering).
        assert_eq!(c.normalize_moisture(0), None);
    }

    #[test]
    fn percent_for_ppfd_interpolates() {
        let c = good();
        assert_eq!(c.percent_for_ppfd(0), 0);
        assert_eq!(c.percent_for_ppfd(120), 25);
        assert_eq!(c.percent_for_ppfd(240), 50);
        assert_eq!(c.percent_for_ppfd(480), 100);
        assert_eq!(c.percent_for_ppfd(600), 100); // beyond max clamps
                                                  // halfway between 240 and 360 PPFD → ~62-63%
        let p = c.percent_for_ppfd(300);
        assert!((62..=63).contains(&p), "got {p}");
    }
}
