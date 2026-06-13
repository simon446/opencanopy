//! Local rolling logs. Spec §9.10, §9.11, §23 (DR-05).
//!
//! A fixed-capacity ring buffer captures the events §9.10 requires (sensor readings, watering,
//! faults, LED derating, reservoir-low, firmware + calibration versions). It is allocation-free
//! (`no_std`) and **independent of connectivity** — no control path depends on it, and disabling
//! Wi-Fi/MQTT changes nothing here (WI-FW-10 acceptance). The on-target build persists the ring to
//! flash for ≥7 days and exports it over USB/serial; this module owns the in-memory format.

use crate::safety_controller::SystemState;

/// One log record. `ts_unix_s` is the RTC wall-clock time (§16.1); when the RTC is invalid the
/// monotonic boot-ms is stored with `ts_valid = false` so timestamps are never silently wrong.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogEntry {
    pub ts_unix_s: u64,
    pub ts_valid: bool,
    pub kind: LogKind,
}

/// The event payloads §9.10 enumerates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogKind {
    /// Periodic sensor snapshot (every 5–15 min, §9.10).
    Sensors {
        temp_c: f32,
        rh_pct: f32,
        vpd_kpa: f32,
        moisture_pct: i16, // -1 = invalid/no reading
        reservoir_low: bool,
        light_pct: u8,
        fan_pct: u8,
    },
    /// A watering pulse completed.
    Watering {
        dose_ml: u16,
        run_seconds_x10: u16,
        daily_total_ml: u16,
    },
    /// A fault was entered.
    Fault { state: SystemState },
    /// LED thermal derating applied.
    LedDerate { factor_pct: u8, air_temp_c: f32 },
    /// Reservoir-low edge.
    ReservoirLow,
    /// Versions stamped at boot and on calibration change (§9.10).
    Versions { firmware: u16, calibration: u16 },
}

/// Fixed-capacity ring log. `N` records; oldest overwritten when full.
#[derive(Debug, Clone)]
pub struct RingLog<const N: usize> {
    buf: [Option<LogEntry>; N],
    head: usize,
    len: usize,
    /// Count of records dropped (overwritten) — surfaced so truncation is never silent.
    dropped: u32,
}

impl<const N: usize> Default for RingLog<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> RingLog<N> {
    pub const fn new() -> Self {
        RingLog {
            buf: [None; N],
            head: 0,
            len: 0,
            dropped: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn capacity(&self) -> usize {
        N
    }
    pub fn dropped(&self) -> u32 {
        self.dropped
    }

    /// Append a record, overwriting the oldest when full.
    pub fn push(&mut self, entry: LogEntry) {
        if self.len == N {
            self.dropped += 1; // overwriting an unread oldest record
        }
        self.buf[self.head] = Some(entry);
        self.head = (self.head + 1) % N;
        if self.len < N {
            self.len += 1;
        }
    }

    /// Iterate oldest → newest (export order).
    pub fn iter(&self) -> impl Iterator<Item = LogEntry> + '_ {
        let start = (self.head + N - self.len) % N;
        (0..self.len).filter_map(move |i| self.buf[(start + i) % N])
    }

    /// Most recent entry, if any.
    pub fn last(&self) -> Option<LogEntry> {
        if self.len == 0 {
            None
        } else {
            self.buf[(self.head + N - 1) % N]
        }
    }
}

/// Default onboard capacity. Sized for ≥7 days: a sensor snapshot every 10 min = 144/day ≈ 1008/7d;
/// rounding up with headroom for watering/fault events gives ~2048 records (§9.10). The on-target
/// flash partition is sized to match; the host/sim uses the same type at this capacity.
pub type OnboardLog = RingLog<2048>;

#[cfg(test)]
mod tests {
    use super::*;

    fn sensors() -> LogKind {
        LogKind::Sensors {
            temp_c: 24.0,
            rh_pct: 60.0,
            vpd_kpa: 1.05,
            moisture_pct: 42,
            reservoir_low: false,
            light_pct: 62,
            fan_pct: 28,
        }
    }

    #[test]
    fn ring_keeps_order_until_full() {
        let mut log: RingLog<4> = RingLog::new();
        for i in 0..3 {
            log.push(LogEntry {
                ts_unix_s: i,
                ts_valid: true,
                kind: sensors(),
            });
        }
        assert_eq!(log.len(), 3);
        let ts: Vec<u64> = log.iter().map(|e| e.ts_unix_s).collect();
        assert_eq!(ts, [0, 1, 2]);
        assert_eq!(log.dropped(), 0);
    }

    #[test]
    fn ring_overwrites_oldest_and_counts_drops() {
        let mut log: RingLog<3> = RingLog::new();
        for i in 0..5 {
            log.push(LogEntry {
                ts_unix_s: i,
                ts_valid: true,
                kind: sensors(),
            });
        }
        assert_eq!(log.len(), 3);
        let ts: Vec<u64> = log.iter().map(|e| e.ts_unix_s).collect();
        assert_eq!(ts, [2, 3, 4]); // oldest two dropped
        assert_eq!(log.dropped(), 2);
        assert_eq!(log.last().unwrap().ts_unix_s, 4);
    }

    #[test]
    fn captures_required_event_kinds() {
        let mut log: OnboardLog = RingLog::new();
        log.push(LogEntry {
            ts_unix_s: 1,
            ts_valid: true,
            kind: LogKind::Versions {
                firmware: 1,
                calibration: 3,
            },
        });
        log.push(LogEntry {
            ts_unix_s: 2,
            ts_valid: true,
            kind: sensors(),
        });
        log.push(LogEntry {
            ts_unix_s: 3,
            ts_valid: true,
            kind: LogKind::Watering {
                dose_ml: 100,
                run_seconds_x10: 263,
                daily_total_ml: 100,
            },
        });
        log.push(LogEntry {
            ts_unix_s: 4,
            ts_valid: true,
            kind: LogKind::Fault {
                state: SystemState::PumpFault,
            },
        });
        log.push(LogEntry {
            ts_unix_s: 5,
            ts_valid: true,
            kind: LogKind::LedDerate {
                factor_pct: 50,
                air_temp_c: 33.0,
            },
        });
        log.push(LogEntry {
            ts_unix_s: 6,
            ts_valid: true,
            kind: LogKind::ReservoirLow,
        });
        assert_eq!(log.len(), 6);
    }

    #[test]
    fn invalid_rtc_timestamps_are_flagged() {
        let mut log: RingLog<2> = RingLog::new();
        log.push(LogEntry {
            ts_unix_s: 12345,
            ts_valid: false,
            kind: sensors(),
        });
        assert!(!log.last().unwrap().ts_valid);
    }
}
