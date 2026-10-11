// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/interpreter/core/execution/utils/dispatch.rs
//  Desc:       Type dispatch operations
//
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{DinoRef, value_type},
    errors::{Result, RuntimeError},
    builtins::{
        object::view::ObjectView,
        array::prototype::Array,
        string::{
            view::StringView,
            prototype::String,
        },
        function::prototype::Function,
    },
    memory::{
        manager::MemoryManager,
        types::property_flags::PROP_NORMAL,
    },
    runtime::context::Runtime,
};

pub fn as_object(dinoref: DinoRef, memory: &mut MemoryManager) -> Result<ObjectView> {
    let dinoref = match dinoref.decode_type() {
        value_type::OBJECT => dinoref,
        value_type::ARRAY => Array::as_ref(memory),
        value_type::STRING => String::as_ref(memory),
        value_type::FUNCTION => Function::as_ref(memory),
        _ => return Err(RuntimeError::ExpectedInstance("object"))
    };
    Ok(dinoref.as_object_unchecked())
}

pub fn get_index(object: DinoRef, index: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef> {
    match object.decode_type() {
        value_type::OBJECT => {
            let object = object.as_object_unchecked();
            object.get(index, memory)
        }
        value_type::ARRAY => {
            let array = object.as_array_unchecked();
            let idx = index.try_as_int(memory)?;
            array.get(idx as u32, memory)
        }
        value_type::STRING => {
            let string = object.as_string_unchecked();
            let idx = index.try_as_int(memory)?;
            if let Some(char) = string.char_at(idx, memory) {
                Ok(StringView::new(&char.to_string(), memory).as_ref())
            } else {
                Err(RuntimeError::IndexOutOfBounds)
            }
        }
        _ => Err(RuntimeError::IndexOutOfBounds),
    }
}

pub fn set_index(object: DinoRef, index: DinoRef, value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef> {
    match object.decode_type() {
        value_type::OBJECT => {
            let object = object.as_object_unchecked();
            object.set(index, value, PROP_NORMAL, memory)?;
            Ok(value)
        }
        value_type::ARRAY => {
            let array = object.as_array_unchecked();
            let idx = index.try_as_int(memory)?;
            array.set(idx as u32, value, memory)?;
            Ok(value)
        }
        value_type::STRING => {
            Err(RuntimeError::Immutable("string"))
        }
        _ => Err(RuntimeError::IndexOutOfBounds),
    }
}

pub fn call(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<()> {
    let callee = unsafe { runtime.memory.stack.get_unchecked(args_start - 1) };

    match callee.decode_type() {
        value_type::FUNCTION => {
            let func = callee.as_function_unchecked();
            func.call(args_start, args_count, runtime)?;
        }
        value_type::OBJECT => {
            let object = callee.as_object_unchecked();
            object.call(args_start, args_count, runtime)?;
        }
        _ => Err(RuntimeError::CallNotFunction(callee.type_name()))?,
    }

    Ok(())
}
