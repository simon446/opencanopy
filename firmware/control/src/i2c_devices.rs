//! Pure I2C device protocol logic for the bus peripherals (§7.5, §16.1): SHT40 (temp/RH) and
//! DS3231 (RTC). Spec/pin map: `electronics/analysis/pin-map.csv` (I2C0 on GPIO8/9, shared by both).
//! The INA219 pump-current monitor was removed with the pump (ECO-003, passive watering).
//!
//! This is the part of "the I2C driver" that's actually error-prone — register addresses, command
//! bytes, CRC, BCD decoding, and raw→engineering conversions — and it's kept **pure and
//! host-testable** here. The `controller/` esp-hal binding only does the bus transaction
//! (`write_read`) and hands the bytes to these functions, so the protocol logic is validated
//! off-hardware (and the same code runs on-target and in the Wokwi custom-chip mock).

use crate::hal::{TempRh, WallTime};

// ===================================================================================== SHT40 ====

/// SHT40-AD1B I2C address (BOM U2, §7.5).
pub const SHT40_ADDR: u8 = 0x44;
/// "Measure T & RH, high precision" command (datasheet 0xFD).
pub const SHT40_CMD_MEASURE_HIGH: u8 = 0xFD;
/// Soft-reset command.
pub const SHT40_CMD_SOFT_RESET: u8 = 0x94;

/// SHT4x CRC-8: polynomial 0x31, init 0xFF (datasheet §4.4). Used to validate each 16-bit word.
pub fn sht4x_crc(data: &[u8]) -> u8 {
    let mut crc: u8 = 0xFF;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Parse a 6-byte SHT40 measurement response: `[T_hi, T_lo, T_crc, RH_hi, RH_lo, RH_crc]`.
/// Returns the converted temp/RH, or `None` if either CRC fails (→ sensor fault upstream, §7.6).
pub fn sht40_parse(resp: &[u8; 6]) -> Option<TempRh> {
    if sht4x_crc(&resp[0..2]) != resp[2] || sht4x_crc(&resp[3..5]) != resp[5] {
        return None;
    }
    let t_raw = u16::from_be_bytes([resp[0], resp[1]]);
    let rh_raw = u16::from_be_bytes([resp[3], resp[4]]);
    // Datasheet conversions.
    let temp_c = -45.0 + 175.0 * (t_raw as f32) / 65535.0;
    let rh = -6.0 + 125.0 * (rh_raw as f32) / 65535.0;
    Some(TempRh {
        temp_c,
        rh_pct: crate::math::clampf(rh, 0.0, 100.0),
    })
}

// ==================================================================================== DS3231 ====

/// DS3231 RTC I2C address (BOM U3, §16.1).
pub const DS3231_ADDR: u8 = 0x68;
/// Timekeeping registers start at 0x00 (sec,min,hour,day,date,month,year).
pub const DS3231_REG_SECONDS: u8 = 0x00;
/// Status register (0x0F); bit 7 = OSF (oscillator-stop flag → time invalid).
pub const DS3231_REG_STATUS: u8 = 0x0F;
pub const DS3231_OSF_MASK: u8 = 0x80;

fn bcd_to_dec(b: u8) -> u32 {
    ((b >> 4) & 0x0F) as u32 * 10 + (b & 0x0F) as u32
}

/// Days from 1970-01-01 to the given civil date (Howard Hinnant's algorithm), integer-only.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400; // [0, 399]
    let mp = (if m > 2 { m - 3 } else { m + 9 }) as i64;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
    era * 146097 + doe - 719468
}

/// Decode the 7 timekeeping registers + the status register into a [`WallTime`].
/// `time_regs` = `[sec, min, hour, weekday, date, month, year]` (BCD, 24-hour). The OSF bit in
/// `status_reg` being set means the oscillator stopped (dead battery / first power-up) → `valid =
/// false`, which triggers the firmware's safe-schedule fallback (§9.4).
pub fn ds3231_parse(time_regs: &[u8; 7], status_reg: u8) -> WallTime {
    let osf_set = status_reg & DS3231_OSF_MASK != 0;
    let sec = bcd_to_dec(time_regs[0] & 0x7F);
    let min = bcd_to_dec(time_regs[1] & 0x7F);
    // Hour: assume 24-hour mode (bit 6 = 0). Mask bits 5..0 for the tens/units.
    let hour = bcd_to_dec(time_regs[2] & 0x3F);
    let date = bcd_to_dec(time_regs[4] & 0x3F);
    // Century bit (reg5 bit7) extends the year; we treat years as 2000-based.
    let century = if time_regs[5] & 0x80 != 0 { 100 } else { 0 };
    let month = bcd_to_dec(time_regs[5] & 0x1F);
    let year = 2000 + century + bcd_to_dec(time_regs[6]);

    // Basic range sanity — implausible values also mean "not valid".
    let plausible =
        sec < 60 && min < 60 && hour < 24 && (1..=12).contains(&month) && (1..=31).contains(&date);

    if osf_set || !plausible {
        return WallTime::INVALID;
    }
    let days = days_from_civil(year as i64, month, date);
    let unix = days * 86_400 + hour as i64 * 3600 + min as i64 * 60 + sec as i64;
    if unix < 0 {
        WallTime::INVALID
    } else {
        WallTime {
            valid: true,
            unix_s: unix as u64,
        }
    }
}

// ============================================================================ BUS DRIVERS =======
//
// The full I2C *driving* logic (transaction sequences: address, write command, read registers,
// parse) lives here, generic over a minimal [`I2cBus`] trait so it is host-testable with a mock
// bus. `controller/` provides a thin adapter implementing `I2cBus`/`DelayMs` for the esp-hal `I2c`
// and `Delay` — no logic there. This keeps `control` dependency-free (no `embedded-hal`) while still
// validating addressing, register reads, NAK handling, and parsing in cheap `cargo` tests.

/// Minimal blocking I2C controller interface (mirrors the embedded-hal shape). `controller/` impls
/// this for esp-hal's `I2c`; tests impl it for a mock device bus.
pub trait I2cBus {
    type Error;
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn write_read(&mut self, addr: u8, bytes: &[u8], buf: &mut [u8]) -> Result<(), Self::Error>;
}

/// Minimal blocking millisecond delay (for the SHT40 conversion wait).
pub trait DelayMs {
    fn delay_ms(&mut self, ms: u32);
}

/// Why a device transaction failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    /// Bus-level failure (NAK / device absent / wiring).
    Bus,
    /// Device responded but the payload was invalid (CRC / implausible).
    Protocol,
}

/// Read SHT40 temp/RH: write the high-precision measure command, wait for the conversion, read the
/// 6-byte response, CRC-check and convert. A missing device (NAK) → `Bus`; a bad CRC → `Protocol`.
pub fn read_sht40<I: I2cBus, D: DelayMs>(
    i2c: &mut I,
    delay: &mut D,
) -> Result<TempRh, DeviceError> {
    i2c.write(SHT40_ADDR, &[SHT40_CMD_MEASURE_HIGH])
        .map_err(|_| DeviceError::Bus)?;
    delay.delay_ms(10);
    let mut buf = [0u8; 6];
    i2c.read(SHT40_ADDR, &mut buf)
        .map_err(|_| DeviceError::Bus)?;
    sht40_parse(&buf).ok_or(DeviceError::Protocol)
}

/// Read the DS3231 wall clock: timekeeping registers from 0x00 + the status register (OSF bit).
/// Any bus error (or OSF/implausible time) yields [`WallTime::INVALID`] → safe-schedule fallback.
pub fn read_ds3231<I: I2cBus>(i2c: &mut I) -> WallTime {
    let mut t = [0u8; 7];
    if i2c
        .write_read(DS3231_ADDR, &[DS3231_REG_SECONDS], &mut t)
        .is_err()
    {
        return WallTime::INVALID;
    }
    let mut status = [0u8; 1];
    if i2c
        .write_read(DS3231_ADDR, &[DS3231_REG_STATUS], &mut status)
        .is_err()
    {
        return WallTime::INVALID;
    }
    ds3231_parse(&t, status[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- SHT40 ----
    #[test]
    fn sht4x_crc_datasheet_vector() {
        // Datasheet check value: CRC of {0xBE, 0xEF} = 0x92.
        assert_eq!(sht4x_crc(&[0xBE, 0xEF]), 0x92);
    }

    #[test]
    fn sht40_parse_roundtrips_known_value() {
        // Construct a response for ~25 °C / ~50 %RH and valid CRCs, then check the decode.
        // t_raw for 25°C: (25+45)/175*65535 = 26214; rh_raw for 50%: (50+6)/125*65535 = 29360.
        let t = 26214u16.to_be_bytes();
        let rh = 29360u16.to_be_bytes();
        let resp = [t[0], t[1], sht4x_crc(&t), rh[0], rh[1], sht4x_crc(&rh)];
        let v = sht40_parse(&resp).expect("valid CRC");
        assert!((v.temp_c - 25.0).abs() < 0.1, "temp {}", v.temp_c);
        assert!((v.rh_pct - 50.0).abs() < 0.1, "rh {}", v.rh_pct);
    }

    #[test]
    fn sht40_parse_rejects_bad_crc() {
        let t = 26214u16.to_be_bytes();
        let rh = 29360u16.to_be_bytes();
        let mut resp = [t[0], t[1], sht4x_crc(&t), rh[0], rh[1], sht4x_crc(&rh)];
        resp[2] ^= 0xFF; // corrupt temp CRC
        assert!(sht40_parse(&resp).is_none());
    }

    #[test]
    fn sht40_clamps_rh() {
        // rh_raw = 65535 → 119% → clamps to 100.
        let t = 26214u16.to_be_bytes();
        let rh = 65535u16.to_be_bytes();
        let resp = [t[0], t[1], sht4x_crc(&t), rh[0], rh[1], sht4x_crc(&rh)];
        assert_eq!(sht40_parse(&resp).unwrap().rh_pct, 100.0);
    }

    // ---- DS3231 ----
    #[test]
    fn ds3231_decodes_bcd_time() {
        // 2026-06-14 08:30:15, Sunday(1), 24h mode. BCD encode.
        let regs = [
            0x15, // sec 15
            0x30, // min 30
            0x08, // hour 08 (24h)
            0x01, // weekday
            0x14, // date 14
            0x06, // month 06
            0x26, // year 26 -> 2026
        ];
        let wt = ds3231_parse(&regs, 0x00);
        assert!(wt.valid);
        // Verify against an independent unix computation for 2026-06-14T08:30:15Z.
        // days_from_civil(2026,6,14) * 86400 + 8*3600 + 30*60 + 15
        let expect = super::days_from_civil(2026, 6, 14) as u64 * 86_400 + 8 * 3600 + 30 * 60 + 15;
        assert_eq!(wt.unix_s, expect);
    }

    #[test]
    fn ds3231_osf_means_invalid() {
        let regs = [0x00, 0x00, 0x08, 0x01, 0x14, 0x06, 0x26];
        let wt = ds3231_parse(&regs, DS3231_OSF_MASK); // oscillator stopped
        assert!(!wt.valid);
    }

    #[test]
    fn ds3231_implausible_is_invalid() {
        let regs = [0x99, 0x99, 0x99, 0x09, 0x99, 0x99, 0x99]; // garbage BCD
        assert!(!ds3231_parse(&regs, 0x00).valid);
    }

    #[test]
    fn days_from_civil_epoch_anchor() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(2000, 1, 1), 10957);
    }

    // ====================================================== BUS-DRIVER INTEGRATION TESTS =========
    //
    // A mock I2C bus that simulates the two real devices on the shared bus, so the full driver
    // transaction sequences (address, command, register read, parse, NAK) run in `cargo` — no
    // hardware, no Wokwi.

    struct MockBus {
        sht40: Option<[u8; 6]>,        // None = device absent (NAK)
        ds3231: Option<([u8; 7], u8)>, // (time regs, status reg)
        writes: Vec<(u8, Vec<u8>)>,    // recorded writes
    }
    impl MockBus {
        fn new() -> Self {
            MockBus {
                sht40: None,
                ds3231: None,
                writes: Vec::new(),
            }
        }
    }
    impl I2cBus for MockBus {
        type Error = ();
        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), ()> {
            self.writes.push((addr, bytes.to_vec()));
            match addr {
                SHT40_ADDR => self.sht40.map(|_| ()).ok_or(()),
                DS3231_ADDR => self.ds3231.map(|_| ()).ok_or(()),
                _ => Err(()),
            }
        }
        fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), ()> {
            match addr {
                SHT40_ADDR => {
                    let r = self.sht40.ok_or(())?;
                    buf[..6].copy_from_slice(&r);
                    Ok(())
                }
                _ => Err(()),
            }
        }
        fn write_read(&mut self, addr: u8, bytes: &[u8], buf: &mut [u8]) -> Result<(), ()> {
            match (addr, bytes.first().copied()) {
                (DS3231_ADDR, Some(DS3231_REG_SECONDS)) => {
                    let (t, _) = self.ds3231.ok_or(())?;
                    buf[..7].copy_from_slice(&t);
                    Ok(())
                }
                (DS3231_ADDR, Some(DS3231_REG_STATUS)) => {
                    let (_, s) = self.ds3231.ok_or(())?;
                    buf[0] = s;
                    Ok(())
                }
                _ => Err(()),
            }
        }
    }
    struct NoDelay;
    impl DelayMs for NoDelay {
        fn delay_ms(&mut self, _ms: u32) {}
    }

    fn valid_sht40(temp_raw: u16, rh_raw: u16) -> [u8; 6] {
        let t = temp_raw.to_be_bytes();
        let rh = rh_raw.to_be_bytes();
        [t[0], t[1], sht4x_crc(&t), rh[0], rh[1], sht4x_crc(&rh)]
    }

    #[test]
    fn sht40_driver_reads_value_over_bus() {
        let mut bus = MockBus::new();
        bus.sht40 = Some(valid_sht40(26214, 29360)); // ~25 °C / 50 %
        let v = read_sht40(&mut bus, &mut NoDelay).unwrap();
        assert!((v.temp_c - 25.0).abs() < 0.1 && (v.rh_pct - 50.0).abs() < 0.1);
        // The driver must issue the measure command on the wire.
        assert!(bus
            .writes
            .iter()
            .any(|(a, b)| *a == SHT40_ADDR && b == &[SHT40_CMD_MEASURE_HIGH]));
    }

    #[test]
    fn sht40_driver_nak_is_bus_error() {
        let mut bus = MockBus::new(); // sht40 absent
        assert_eq!(read_sht40(&mut bus, &mut NoDelay), Err(DeviceError::Bus));
    }

    #[test]
    fn sht40_driver_bad_crc_is_protocol_error() {
        let mut bus = MockBus::new();
        let mut r = valid_sht40(26214, 29360);
        r[2] ^= 0xFF; // corrupt temp CRC
        bus.sht40 = Some(r);
        assert_eq!(
            read_sht40(&mut bus, &mut NoDelay),
            Err(DeviceError::Protocol)
        );
    }

    #[test]
    fn ds3231_driver_reads_time_over_bus() {
        let mut bus = MockBus::new();
        bus.ds3231 = Some(([0x15, 0x30, 0x08, 0x01, 0x14, 0x06, 0x26], 0x00));
        let wt = read_ds3231(&mut bus);
        assert!(wt.valid);
        let expect = super::days_from_civil(2026, 6, 14) as u64 * 86_400 + 8 * 3600 + 30 * 60 + 15;
        assert_eq!(wt.unix_s, expect);
    }

    #[test]
    fn ds3231_driver_absent_is_invalid() {
        let mut bus = MockBus::new();
        assert!(!read_ds3231(&mut bus).valid);
    }

    #[test]
    fn ds3231_driver_osf_is_invalid() {
        let mut bus = MockBus::new();
        bus.ds3231 = Some(([0x00, 0x00, 0x08, 0x01, 0x14, 0x06, 0x26], DS3231_OSF_MASK));
        assert!(!read_ds3231(&mut bus).valid);
    }
}
