// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/native/time/windows.rs
//  Desc:       Windows-specific local time implementation
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::time::to_timestamp_ms;

pub fn local_now() -> SystemTime {
    #[repr(C)]
    struct SystemTimeWindows {
        w_year: u16,
        w_month: u16,
        w_day_of_week: u16,
        w_day: u16,
        w_hour: u16,
        w_minute: u16,
        w_second: u16,
        w_milliseconds: u16,
    }
    
    unsafe extern "system" {
        fn GetLocalTime(lp_system_time: *mut SystemTimeWindows);
    }
    
    let mut st = SystemTimeWindows {
        w_year: 0,
        w_month: 0,
        w_day_of_week: 0,
        w_day: 0,
        w_hour: 0,
        w_minute: 0,
        w_second: 0,
        w_milliseconds: 0,
    };
    
    unsafe {
        GetLocalTime(&mut st);
    }
    
    let year = st.w_year as i64;
    let month = st.w_month as i64;
    let day = st.w_day as i64;
    let hour = st.w_hour as i64;
    let minute = st.w_minute as i64;
    let second = st.w_second as i64;
    let millisecond = st.w_milliseconds as i64;
    
    let timestamp_ms = to_timestamp_ms(year, month, day, hour, minute, second, millisecond);
    UNIX_EPOCH + Duration::from_millis(timestamp_ms as u64)
}
