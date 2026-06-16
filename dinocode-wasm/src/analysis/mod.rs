// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/analysis/mod.rs
//  Desc:       Analysis module for WASM
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub mod analyzer;
pub mod cfg;
pub mod helpers;

pub use analyzer::BytecodeAnalyzer;
