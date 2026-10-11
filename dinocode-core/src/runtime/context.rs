// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/runtime/context.rs
//  Desc:       Runtime context for execution
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::UserFunction,
};

pub struct Runtime<'a, 'b> {
    pub memory: &'a mut MemoryManager,
    pub functions: &'b Vec<UserFunction>,
    pub ip: &'a mut usize,
}

impl<'a, 'b> Runtime<'a, 'b> {
    pub fn new(
        memory: &'a mut MemoryManager,
        functions: &'b Vec<UserFunction>,
        ip: &'a mut usize,
    ) -> Self {
        Self { memory, functions, ip }
    }
}

