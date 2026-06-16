// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/time_format.rs
//  Desc:       Time formatting utility
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_platform::time::{SystemTime, UNIX_EPOCH, Duration};

pub struct DinoTime {
    pub year: i64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
}

impl DinoTime {
    pub fn from_system_time(t: SystemTime) -> Self {
        let epoch_secs = t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let second = epoch_secs % 60;
        let minute = (epoch_secs / 60) % 60;
        let hour = (epoch_secs / 3600) % 24;

        let days = (epoch_secs / 86400) as i64 + 719468;
        let era = (if days >= 0 { days } else { days - 146096 }) / 146097;
        let doe = (days - era * 146097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = (yoe as i64) + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if m <= 2 { y + 1 } else { y };

        Self { year, month: m, day: d, hour, minute, second }
    }

    pub fn from_timestamp_ms(timestamp_ms: i64) -> Self {
        let duration = Duration::from_millis(timestamp_ms as u64);
        let datetime = UNIX_EPOCH + duration;
        Self::from_system_time(datetime)
    }

    pub fn to_timestamp_ms(year: i64, month: i64, day: i64) -> i64 {
        let mut y = year;
        let mut m = month;
        if m < 3 {
            y -= 1;
            m += 12;
        }
        let d = day;
        let days = d + (153 * m - 457) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 719469;
        days * 86400 * 1000
    }

    pub fn format(&self, pattern: &str) -> String {
        pattern
            .replace("%Y", &format!("{:04}", self.year))
            .replace("%m", &format!("{:02}", self.month))
            .replace("%d", &format!("{:02}", self.day))
            .replace("%H", &format!("{:02}", self.hour))
            .replace("%M", &format!("{:02}", self.minute))
            .replace("%S", &format!("{:02}", self.second))
    }
}
