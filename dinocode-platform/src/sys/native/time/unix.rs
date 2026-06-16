// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/native/time/unix.rs
//  Desc:       Unix (Linux/Mac) specific local time implementation
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::time::{SystemTime, Duration, UNIX_EPOCH};
use crate::time::to_timestamp_ms;

pub fn local_now() -> SystemTime {
    #[repr(C)]
    struct TimeSpec {
        tv_sec: i64,
        tv_nsec: i64,
    }
    
    #[repr(C)]
    struct Tm {
        tm_sec: i32,
        tm_min: i32,
        tm_hour: i32,
        tm_mday: i32,
        tm_mon: i32,
        tm_year: i32,
        tm_wday: i32,
        tm_yday: i32,
        tm_isdst: i32,
        tm_gmtoff: i64,
        tm_zone: *const i8,
    }
    
    unsafe extern "C" {
        fn clock_gettime(clock_id: i32, tp: *mut TimeSpec) -> i32;
        fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
    }
    
    const CLOCK_REALTIME: i32 = 0;
    
    let mut ts = TimeSpec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        clock_gettime(CLOCK_REALTIME, &mut ts);
    }
    
    let mut tm = Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null(),
    };
    
    unsafe {
        localtime_r(&ts.tv_sec, &mut tm);
    }
    
    let year = tm.tm_year as i64 + 1900;
    let month = tm.tm_mon as i64 + 1;
    let day = tm.tm_mday as i64;
    let hour = tm.tm_hour as i64;
    let minute = tm.tm_min as i64;
    let second = tm.tm_sec as i64;
    let millisecond = ts.tv_nsec / 1_000_000;
    
    let timestamp_ms = to_timestamp_ms(year, month, day, hour, minute, second, millisecond);
    UNIX_EPOCH + Duration::from_millis(timestamp_ms as u64)
}
