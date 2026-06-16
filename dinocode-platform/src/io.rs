// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/io.rs
//  Desc:       IO abstraction
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::sys::platform;

#[cfg(target_arch = "wasm32")]
use js_sys;

pub fn print(s: &str) {
    platform::io::print(s);
}

pub fn println(s: &str) {
    platform::io::println(s);
}

pub fn flush() {
    platform::io::flush();
}

pub fn read_line() -> Result<String, String> {
    platform::io::read_line().map_err(|e| e.to_string())
}

pub fn input(prompt: &str) -> Result<String, String> {
    platform::io::input(prompt).map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
pub fn set_print_hook(f: js_sys::Function) {
    platform::io::set_print_hook(f);
}

#[cfg(target_arch = "wasm32")]
pub fn set_input_hook(f: js_sys::Function) {
    platform::io::set_input_hook(f);
}
