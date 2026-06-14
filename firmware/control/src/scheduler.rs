//! Small time helpers shared by the controllers: day-index for the daily-cap reset and night
//! detection. Kept here so the time conventions live in one place (§9.4, §9.6).

use crate::hal::WallTime;

const DAY_S: u64 = 86_400;

/// A monotonic day index used to reset the daily watering cap. Uses the RTC's wall-clock day when
/// valid; otherwise falls back to whole days since boot so the cap still resets without a clock
/// (§9.6 — the cap must function on the safe-schedule fallback too).
pub fn day_index(rtc: WallTime, utc_offset_s: i32, boot_ms: u64, now_ms: u64) -> u32 {
    if rtc.valid {
        (((rtc.unix_s as i64 + utc_offset_s as i64).rem_euclid(i64::MAX)) as u64 / DAY_S) as u32
    } else {
        (now_ms.saturating_sub(boot_ms) / 1000 / DAY_S) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_index_increments_with_wall_clock() {
        let d0 = day_index(
            WallTime {
                valid: true,
                unix_s: 0,
            },
            0,
            0,
            0,
        );
        let d1 = day_index(
            WallTime {
                valid: true,
                unix_s: DAY_S,
            },
            0,
            0,
            0,
        );
        assert_eq!(d1, d0 + 1);
    }

    #[test]
    fn day_index_falls_back_to_boot_days() {
        let d0 = day_index(WallTime::INVALID, 0, 0, 0);
        let d2 = day_index(WallTime::INVALID, 0, 0, 2 * DAY_S * 1000 + 5);
        assert_eq!(d2, d0 + 2);
    }
}
