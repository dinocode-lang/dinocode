// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/frames/function_def.rs
//  Desc:       Frame for parsing user-defined functions or methods.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct FunctionDefFrame {
    pub name: String,
    pub start_ip: usize,
    pub end_ip: usize,
    pub param_count: u32,
    pub return_count: u32,
    pub initial_depth: u32,
    pub function_id: u32,
    pub is_global: bool,
    pub is_method: bool,
}
