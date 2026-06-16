// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/types/frames/class_def.rs
//  Desc:       Frame for parsing class definitions
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct ClassFrame {
    pub name: String,
    pub start_ip: usize,
    pub end_ip: usize,
    pub initial_depth: u32,
    pub method_count: u32,
}
