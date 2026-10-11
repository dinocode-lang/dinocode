// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/interpreter/core/execution/utils/unary_ops.rs
//  Desc:       Unary operations
//
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::{DinoRef, value_type},
    errors::{Result, RuntimeError},
};

#[inline(always)]
pub fn unary_not(memory: &mut MemoryManager) -> Result<()> {
    let value = unsafe { memory.stack.peek_unchecked(0) };
    let truthy = value.try_as_bool(memory)?;
    
    unsafe {
        memory.stack.pop_unchecked();
        memory.stack.push_unchecked(if truthy { DinoRef::FALSE } else { DinoRef::TRUE });
    }
    Ok(())
}

#[inline(always)]
pub fn unary_neg(memory: &mut MemoryManager) -> Result<()> {
    let value = unsafe { memory.stack.peek_unchecked(0) };
    let result = match value.decode_type() {
        value_type::INT => {
            DinoRef::int(-value.as_int())
        },
        value_type::FLOAT => {
            let f_val = value.as_float();
            DinoRef::float(-f_val)
        },
        _ => return Err(RuntimeError::ExpectedNumeric(value.type_name())),
    };
    
    unsafe {
        memory.stack.pop_unchecked();
        memory.stack.push_unchecked(result);
    }
    Ok(())
}
