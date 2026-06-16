// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/native/time/mod.rs
//  Desc:       Time module with OS-specific implementations
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[cfg(target_os = "windows")]
mod windows;

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;

pub use std::time::{SystemTime, Instant, UNIX_EPOCH, Duration};

pub fn local_now() -> SystemTime {
    #[cfg(target_os = "windows")]
    {
        windows::local_now()
    }
    
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        unix::local_now()
    }
}
