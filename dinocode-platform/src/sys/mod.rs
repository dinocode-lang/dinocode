// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/sys/mod.rs
//  Desc:       System selector
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    pub mod io;
    pub mod time;
    pub mod process;
    pub mod thread;
}
#[cfg(not(target_arch = "wasm32"))]
pub use native as platform;

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    pub mod io;
    pub mod time;
    pub mod process;
    pub mod thread;
}
#[cfg(target_arch = "wasm32")]
pub use wasm as platform;
