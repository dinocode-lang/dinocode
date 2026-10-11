// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/object/prototype.rs
//  Desc:       Object prototype
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::types::property_flags::PROP_NORMAL,
    runtime::context::Runtime,
    types::DinoRef,
    errors::{Result, RuntimeError},
    builtins::array::view::ArrayView,
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    getter,
    symbol,
};

crate::register_module! {
    name: init_object,
    classes: [Object]
}

#[dinoclass]
pub struct Object;

#[dinomethods]
impl Object {

    #[raw]
    pub fn keys(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = this.as_object()?;
        let keys = object.keys(&mut runtime.memory);
        let array = ArrayView::new_from_slice(&keys, &mut runtime.memory)?;
        Ok(array.as_ref())
    }
    
    #[raw]
    pub fn values(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = this.as_object()?;
        let values = object.values(&runtime.memory);
        let array = ArrayView::new_from_slice(&values, &mut runtime.memory)?;
        Ok(array.as_ref())
    }

    #[raw]
    pub fn get_attr(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "get_attr", expected: 1, got: args_count - 1 }); }
        let raw_ref = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = raw_ref.as_object()?;
        object.get_attr(args_start, runtime)
    }
    
    #[raw]
    pub fn get(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "get", expected: 1, got: args_count - 1 }); }
        
        let (raw_ref, key) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1),
            )
        };
        let object = raw_ref.as_object()?;
        Ok(object.get(key, &mut runtime.memory)?)
    }
    
    #[raw]
    pub fn set(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::MissingArgument { func: "set", expected: 2, got: args_count - 1 }); }
        
        let (raw_ref, key, val) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1),
                runtime.memory.stack.get_unchecked(args_start + 2)
            )
        };

        let object = raw_ref.as_object()?;
        object.set(key, val, PROP_NORMAL, &mut runtime.memory)?;
        
        Ok(val)
    }
    
    #[raw]
    #[getter]
    pub fn len(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = this.as_object()?;
        let count = object.len(&runtime.memory);
        Ok(DinoRef::int(count as i64))
    }
    
    #[raw]
    #[symbol(name="in", alias)]
    pub fn has(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        let prop = Object::get(runtime, args_start, args_count);
        Ok(DinoRef::bool(prop.is_ok()))
    }
    
    #[raw]
    pub fn delete(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument { func: "delete", expected: 1, got: args_count - 1 }); }
        
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = this.as_object()?;
        let key = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        
        let deleted = object.delete(key, &mut runtime.memory);
        Ok(DinoRef::bool(deleted))
    }
    
    #[raw]
    pub fn clear(runtime: &mut Runtime, args_start: usize, _args_count: usize) -> Result<DinoRef> {
        let this = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = this.as_object()?;
        object.clear(&mut runtime.memory);
        Ok(DinoRef::NONE)
    }

    #[raw]
    #[symbol(name="call")]
    pub fn call(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        let raw_ref = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let object = raw_ref.as_object()?;
        object.call(args_start, args_count, runtime)
    }
}
