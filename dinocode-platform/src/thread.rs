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

pub fn step(ip: usize) {
    platform::thread::step(ip);
}

#[cfg(target_arch = "wasm32")]
pub fn set_sleep_hook(f: js_sys::Function) {
    platform::thread::set_sleep_hook(f);
}

#[cfg(target_arch = "wasm32")]
pub fn set_step_hook(f: js_sys::Function) {
    platform::thread::set_step_hook(f);
}

#[cfg(target_arch = "wasm32")]
pub fn enable_step() {
    platform::thread::enable_step();
}

#[cfg(target_arch = "wasm32")]
pub fn disable_step() {
    platform::thread::disable_step();
}
