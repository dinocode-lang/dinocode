// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/wasm/thread.rs
//  Desc:       WASM thread implementation
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::time::Duration;
use std::cell::{Cell, RefCell};
use wasm_bindgen::prelude::*;
use js_sys::Atomics;

thread_local! {
    static SLEEP_HOOK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
    static STEP_HOOK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
    static STEP_ACTIVE: Cell<bool> = Cell::new(false);
    static STEP_START: Cell<i32> = Cell::new(-1);
    static STEP_LIMIT: Cell<i32> = Cell::new(-1);
}

pub fn set_sleep_hook(f: js_sys::Function) {
    SLEEP_HOOK.with(|h| *h.borrow_mut() = Some(f));
}

pub fn set_step_hook(f: js_sys::Function) {
    STEP_HOOK.with(|h| *h.borrow_mut() = Some(f));
}

pub fn enable_step() {
    STEP_ACTIVE.with(|a| a.set(true));
    STEP_START.with(|s| s.set(-1));
    STEP_LIMIT.with(|l| l.set(-1));
}

pub fn disable_step() {
    STEP_ACTIVE.with(|a| a.set(false));
    STEP_START.with(|s| s.set(-1));
    STEP_LIMIT.with(|l| l.set(-1));
}

pub fn sleep(duration: Duration) {
    SLEEP_HOOK.with(|h| {
        if let Some(f) = h.borrow().as_ref() {
            let ms = duration.as_millis() as u32;
            
            let buffer = js_sys::SharedArrayBuffer::new(8);
            let int32_array = js_sys::Int32Array::new(&buffer);
            
            // [0] = 0 (not ready)
            // [1] = duration in ms
            int32_array.set_index(0, 0);
            int32_array.set_index(1, ms as i32);
            
            let _ = f.call1(&JsValue::NULL, &buffer.into());
            let _ = Atomics::wait(&int32_array, 0, 0);
        }
    });
}

pub fn step(ip: usize) {
    STEP_HOOK.with(|h| {
        if let Some(f) = h.borrow().as_ref() {
            if !STEP_ACTIVE.with(|a| a.get()) {
                return;
            }
            let start = STEP_START.with(|s| s.get());
            let limit = STEP_LIMIT.with(|l| l.get());
            if start >= 0 && limit >= 0 && (ip as i64) >= (start as i64) && (ip as i64) <= (limit as i64) {
                return;
            }

            let buffer = js_sys::SharedArrayBuffer::new(12);
            let int32_array = js_sys::Int32Array::new(&buffer);

            // [0] = 0 (not ready)
            // [1] = current ip
            // [2] = line end ip
            int32_array.set_index(0, 0);
            int32_array.set_index(1, ip as i32);
            int32_array.set_index(2, -1);
            
            let _ = f.call1(&JsValue::NULL, &buffer.into());
            let _ = Atomics::wait(&int32_array, 0, 0);

            let new_limit = int32_array.get_index(2);
            STEP_START.with(|s| s.set(ip as i32));
            STEP_LIMIT.with(|l| l.set(new_limit));
        }
    });
}
