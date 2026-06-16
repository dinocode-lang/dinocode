// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/wasm/time.rs
//  Desc:       WASM time implementation mimicking std::time
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::time::Duration;
use js_sys::Date;
use wasm_bindgen::prelude::*;
use crate::time::to_timestamp_ms;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

impl SystemTime {
    pub fn now() -> Self {
        SystemTime(Date::now() as u64)
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, std::convert::Infallible> {
        Ok(Duration::from_millis(self.0.saturating_sub(earlier.0)))
    }
}

impl std::ops::Add<Duration> for SystemTime {
    type Output = SystemTime;
    fn add(self, dur: Duration) -> SystemTime {
        SystemTime(self.0 + dur.as_millis() as u64)
    }
}

impl Instant {
    pub fn now() -> Self {
        Instant(now() as u64)
    }

    pub fn elapsed(&self) -> Duration {
        let now_ms = now() as u64;
        Duration::from_millis(now_ms.saturating_sub(self.0))
    }
}

pub fn local_now() -> SystemTime {
    let date = Date::new_0();
    
    let year = date.get_full_year() as i64;
    let month = (date.get_month() + 1) as i64; // JS months are 0-11
    let day = date.get_date() as i64;
    let hour = date.get_hours() as i64;
    let minute = date.get_minutes() as i64;
    let second = date.get_seconds() as i64;
    let millisecond = date.get_milliseconds() as i64;
    
    let timestamp_ms = to_timestamp_ms(year, month, day, hour, minute, second, millisecond);
    SystemTime(timestamp_ms as u64)
}
