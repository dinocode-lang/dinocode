// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/wasm/io.rs
//  Desc:       WASM IO implementation
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use wasm_bindgen::prelude::*;
use web_sys::console;
use std::cell::RefCell;

thread_local! {
    static PRINT_HOOK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
    static INPUT_HOOK: RefCell<Option<js_sys::Function>> = RefCell::new(None);
}

pub fn set_print_hook(f: js_sys::Function) {
    PRINT_HOOK.with(|h| *h.borrow_mut() = Some(f));
}

pub fn set_input_hook(f: js_sys::Function) {
    INPUT_HOOK.with(|h| *h.borrow_mut() = Some(f));
}

fn exec_print_hook(s: &str) -> bool {
    PRINT_HOOK.with(|h| {
        if let Some(f) = h.borrow().as_ref() {
            let _ = f.call1(&JsValue::NULL, &JsValue::from_str(s));
            return true;
        }
        return false;
    })
}

pub fn print(s: &str) {
    if !exec_print_hook(s) {
        console::log_1(&JsValue::from_str(s));
    }
}

pub fn println(s: &str) {
    let s2 = format!("{}\n", s);
    if !exec_print_hook(&s2) {
        console::log_1(&JsValue::from_str(s));
    }
}

pub fn flush() {}

pub fn read_line() -> std::io::Result<String> {
    input("")
}

pub fn input(prompt: &str) -> std::io::Result<String> {
    INPUT_HOOK.with(|h| {
        if let Some(f) = h.borrow().as_ref() {
            match f.call1(&JsValue::NULL, &JsValue::from_str(prompt)) {
                Ok(val) => Ok(val.as_string().unwrap_or_default()),
                Err(_) => Ok(String::new()),
            }
        } else {
            let result = web_sys::window()
                .and_then(|w| w.prompt_with_message(prompt).ok())
                .flatten()
                .unwrap_or_default();
            Ok(result)
        }
    })
}
