// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/type_conversions.rs
//  Desc:       Native type conversion functions
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_macros::dinof;
use crate::{
    memory::MemoryManager,
    types::DinoRef,
    errors::{
        Result,
        RuntimeError,
    },
    utils::conversions::TypeConverter,
};

crate::register_module! {
    name: init_types,
    functions: [int, float, number, bigint, bool, str, id]
}

#[dinof(raw)]
pub fn int(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("int"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    
    let base = if args_count > 1 {
        let base_arg = memory.stack().get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        Some(base_arg.try_as_int(memory)? as u32)
    } else {
        None
    };
    
    TypeConverter::to_int_lax(arg, base, memory)
}

#[dinof(raw)]
pub fn float(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("float"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_float_lax(arg, memory)
}

#[dinof(raw)]
pub fn number(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("number"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_number_lax(arg, memory)
}


#[dinof(raw)]
pub fn bigint(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("bigint"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    
    let base = if args_count > 1 {
        let base_arg = memory.stack().get(args_start + 1).copied()
            .ok_or(RuntimeError::StackUnderflow)?;
        Some(base_arg.try_as_int(memory)? as u32)
    } else {
        None
    };
    
    TypeConverter::to_bigint_lax(arg, base, memory)
}

#[dinof(raw)]
pub fn bool(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("bool"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_bool(arg, memory)
}

#[dinof(raw)]
pub fn str(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("str"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_string(arg, memory)
}

#[dinof(raw)]
pub fn id(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::MissingArgument("id"));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    Ok(DinoRef::int(arg.payload() as i64))
}
