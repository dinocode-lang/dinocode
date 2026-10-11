// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/array/prototype.rs
//  Desc:       Array prototype
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════
use crate::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::{Result, RuntimeError},
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    getter,
    symbol,
};

crate::register_module! {
    name: init_array,
    classes: [Array]
}

#[dinoclass]
pub struct Array;

#[dinomethods]
impl Array {
    #[raw]
    pub fn push(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "push", expected: 1, got: args_count - 1 });
        }
        let (this, value) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };
        let array = this.as_array()?;
        let new_len = array.push(value, &mut runtime.memory)?;
        Ok(DinoRef::int(new_len as i64))
    }

    #[raw]
    pub fn pop(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let array = this.as_array()?;
        array.pop(&mut runtime.memory)
    }

    #[raw]
    pub fn get(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "get", expected: 1, got: args_count - 1 }); }
        let (this, idx_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };
        let index = idx_ref.try_as_int(&mut runtime.memory)?;
        let array = this.as_array()?;
        array.get(index as u32, &runtime.memory)
    }

    #[raw]
    pub fn set(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::MissingArgument { func: "set", expected: 2, got: args_count - 1 }); }
        let (this, idx_ref, val) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1),
                runtime.memory.stack.get_unchecked(args_start + 2)
            )
        };
        let index = idx_ref.try_as_int(&mut runtime.memory)?;
        let array = this.as_array()?;
        array.set(index as u32, val, &mut runtime.memory)?;
        Ok(val)
    }

    #[raw]
    #[getter]
    pub fn len(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let array = this.as_array()?;
        Ok(DinoRef::int(array.len(&runtime.memory) as i64))
    }
    
    #[raw]
    pub fn clear(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let array = this.as_array()?;
        array.clear(&mut runtime.memory)?;
        Ok(DinoRef::ZERO)
    }

    #[raw]
    pub fn is_empty(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let array = this.as_array()?;
        Ok(DinoRef::bool(array.is_empty(&runtime.memory)))
    }

    #[raw]
    pub fn first(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let array = this.as_array()?;
        array.first(&runtime.memory)
    }

    #[raw]
    pub fn last(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let array = this.as_array()?;
        array.last(&runtime.memory)
    }

    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument { func: "contains", expected: 1, got: args_count - 1 });
        }
        let (this, value) = unsafe { 
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };
        let array = this.as_array()?;
        Ok(DinoRef::bool(array.contains(value, &runtime.memory)))
    }
}
