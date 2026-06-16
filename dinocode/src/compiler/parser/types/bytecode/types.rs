// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/bytecode/types.rs
//  Desc:       Bytecode type definitions for the DinoCode parser.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    memory::MemoryManager,
    types::{
        DinoRef,
        UserFunction,
    },
};

#[derive(Debug)]
pub struct Bytecode {
    pub instructions: Vec<u32>,
    pub memory_manager: MemoryManager,
    pub const_pool: Vec<DinoRef>,
    pub functions: Vec<UserFunction>,
    pub global_count: u32,
    pub main_function: Option<DinoRef>,
}
