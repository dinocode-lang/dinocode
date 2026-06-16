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
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use js_sys::Atomics;

thread_local! {
    static SLEEP_HOOK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
}

pub fn set_sleep_hook(f: js_sys::Function) {
    SLEEP_HOOK.with(|h| *h.borrow_mut() = Some(f));
}

pub fn sleep(duration: Duration) {
    SLEEP_HOOK.with(|h| {
        if let Some(f) = h.borrow().as_ref() {
            let ms = duration.as_millis() as u32;
            
            let buffer = js_sys::SharedArrayBuffer::new(8);
            let int32_array = js_sys::Int32Array::new(&buffer);
            
            // index 0 = 0 (not ready)
            // index 1 = duration in ms
            int32_array.set_index(0, 0);
            int32_array.set_index(1, ms as i32);
            
            let _ = f.call1(&JsValue::NULL, &buffer.into());
            let _ = Atomics::wait(&int32_array, 0, 0);
        }
    });
}
