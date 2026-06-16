// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/frames/object.rs
//  Desc:       Frame for parsing object instantiation and access.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct ObjectFrame { 
    pub depth: u32, 
    pub output_len: usize, 
    pub create: bool 
}
