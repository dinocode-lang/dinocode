// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/time.rs
//  Desc:       Time abstraction
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::sys::platform;
pub use std::time::Duration;

pub type SystemTime = platform::time::SystemTime;
pub type Instant = platform::time::Instant;
pub use platform::time::{UNIX_EPOCH, local_now};

pub fn to_timestamp_ms(year: i64, month: i64, day: i64, hour: i64, minute: i64, second: i64, millisecond: i64) -> i64 {
    let mut y = year;
    let mut m = month;
    if m < 3 {
        y -= 1;
        m += 12;
    }
    let d = day;
    let days = d + (153 * m - 457) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 719469;
    (days * 86400 + hour * 3600 + minute * 60 + second) * 1000 + millisecond
}
