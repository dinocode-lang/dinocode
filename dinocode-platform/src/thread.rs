// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/thread.rs
//  Desc:       Thread abstraction
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::sys::platform;
pub use std::time::Duration;

pub fn sleep(duration: Duration) {
    platform::thread::sleep(duration);
}

#[cfg(target_arch = "wasm32")]
pub fn set_sleep_hook(f: js_sys::Function) {
    platform::thread::set_sleep_hook(f);
}
