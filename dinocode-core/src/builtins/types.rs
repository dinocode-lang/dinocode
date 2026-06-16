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
use crate::types::DinoRef;
use crate::memory::MemoryManager;
use crate::errors::{Result, RuntimeError, RuntimeErrorType};
use crate::utils::type_conversions::TypeConverter;

crate::register_module! {
    name: init_types,
    functions: [int, float, number, bigint, bool, str, id]
}

#[dinof(raw)]
pub fn int(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("int".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_int(arg, memory)
}

#[dinof(raw)]
pub fn float(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("float".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_float(arg, memory)
}

#[dinof(raw)]
pub fn number(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("number".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_number(arg, memory)
}


#[dinof(raw)]
pub fn bigint(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("bigint".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_bigint(arg, memory)
}

#[dinof(raw)]
pub fn bool(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("bool".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_bool(arg, memory)
}

#[dinof(raw)]
pub fn str(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("str".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    TypeConverter::to_string(arg, memory)
}

#[dinof(raw)]
pub fn id(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
    if args_count == 0 {
        return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("id".into())));
    }
    let arg = memory.stack().get(args_start).copied()
        .ok_or(RuntimeError::StackUnderflow)?;
    Ok(DinoRef::int(arg.payload() as i64))
}
