// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/frames/function.rs
//  Desc:       Frame for parsing function calls and methods.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct FuncFrame { 
    pub depth: u32, 
    pub output_len: usize, 
    pub is_method: bool, 
    pub is_dollar: bool,
    pub is_explicit_call: bool,
    pub name: Option<String> // Store function name for debugging
}
