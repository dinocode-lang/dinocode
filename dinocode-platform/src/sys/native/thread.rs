// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/native/thread.rs
//  Desc:       Native thread implementation
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub use std::time::Duration;

pub fn sleep(duration: Duration) {
    std::thread::sleep(duration);
}
