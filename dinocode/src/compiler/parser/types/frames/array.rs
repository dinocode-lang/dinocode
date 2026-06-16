// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/frames/array.rs
//  Desc:       Frame for parsing array instantiation and access.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct ArrayFrame { 
    pub depth: u32, 
    pub output_len: usize, 
    pub create: bool 
}
