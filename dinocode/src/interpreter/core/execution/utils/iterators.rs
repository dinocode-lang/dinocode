// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/interpreter/core/execution/utils/iterators.rs
//  Desc:       Iterator operations
//
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{DinoRef, value_type, opcode},
    errors::{Result, RuntimeError},
    builtins::{
        string::view::StringView,
        range::prototype::Range,
    },
    memory::{
        MemoryManager,
        types::object_types,
    },
};

#[inline(always)]
pub fn for_init(memory: &mut MemoryManager, var_idx: u32) -> Result<u8> {
    let iterable = unsafe { memory.stack.peek_unchecked(0) };
    
    let (target, index, count, step, next_opcode) = match iterable.decode_type() {
        value_type::ARRAY => {
            let len = iterable.as_array_unchecked().len(memory);
            (
                iterable,
                DinoRef::ZERO,
                DinoRef::int(len as i64),
                DinoRef::ONE,
                opcode::FOR_ITER_ARRAY,
            )
        },
        value_type::STRING => {
            let len = iterable.as_string_unchecked().len(memory);
            (
                iterable,
                DinoRef::ZERO,
                DinoRef::int(len as i64),
                DinoRef::ONE,
                opcode::FOR_ITER_STRING,
            )
        },
        value_type::OBJECT => {
            let object = iterable.as_object_unchecked();
            if object.has_type(object_types::RANGE, memory) {
                let index = object.get(Range::START(), memory)?;
                let count = object.get(Range::STOP(), memory)?;
                let step = object.get(Range::STEP_VAL(), memory)?;
                (
                    DinoRef::NONE,
                    index,
                    count,
                    step,
                    opcode::FOR_ITER_RANGE,
                )
            } else {
                return Err(RuntimeError::NotIterable(None));
            }
        },
        _ => {
            return Err(RuntimeError::NotIterable(Some(iterable.type_name())));
        }
    };
    
    unsafe {
        memory.stack.set_local_unchecked(var_idx, target);
        memory.stack.set_local_unchecked(var_idx + 1, index);
        memory.stack.set_local_unchecked(var_idx + 2, count);
        memory.stack.set_local_unchecked(var_idx + 3, step);
        memory.stack.pop_unchecked(); 
    }
    
    Ok(next_opcode)
}

#[inline(always)]
pub fn for_iter_array(memory: &mut MemoryManager, var_idx: u32) -> Result<bool> {
    let index = unsafe { memory.stack.get_local_unchecked(var_idx + 1) };
    let count = unsafe { memory.stack.get_local_unchecked(var_idx + 2) };
    
    let idx_val = index.as_int();
    let cnt_val = count.as_int();
    
    if idx_val >= cnt_val {
        memory.stack.push(DinoRef::FALSE);
        Ok(false)
    } else {
        let target = unsafe { memory.stack.get_local_unchecked(var_idx) };
        let elem = target.as_array_unchecked().get(idx_val as u32, memory)?;
        
        let new_index = DinoRef::int(idx_val.wrapping_add(1));
        unsafe { memory.stack.set_local_unchecked(var_idx + 1, new_index); }
        memory.stack.push(elem);
        Ok(true)
    }
}

#[inline(always)]
pub fn for_iter_range(memory: &mut MemoryManager, var_idx: u32) -> Result<bool> {
    let mut index = unsafe { memory.stack.get_local_unchecked(var_idx + 1) };
    let count = unsafe { memory.stack.get_local_unchecked(var_idx + 2) };
    let step = unsafe { memory.stack.get_local_unchecked(var_idx + 3) };
    
    let idx_val = index.as_int();
    let cnt_val = count.as_int();
    let stp_val = step.as_int();
    let exhausted = if stp_val > 0 { idx_val >= cnt_val } else { idx_val <= cnt_val };
    
    if exhausted {
        memory.stack.push(DinoRef::FALSE);
        Ok(false)
    } else {
        index = DinoRef::int(idx_val.wrapping_add(stp_val));
        unsafe { memory.stack.set_local_unchecked(var_idx + 1, index); }
        memory.stack.push(DinoRef::int(idx_val));
        Ok(true)
    }
}

#[inline(always)]
pub fn for_iter_string(memory: &mut MemoryManager, var_idx: u32) -> Result<bool> {
    let index = unsafe { memory.stack.get_local_unchecked(var_idx + 1) };
    let count = unsafe { memory.stack.get_local_unchecked(var_idx + 2) };
    
    let idx_val = index.as_int();
    let cnt_val = count.as_int();
    
    if idx_val >= cnt_val {
        memory.stack.push(DinoRef::FALSE);
        Ok(false)
    } else {
        let target = unsafe { memory.stack.get_local_unchecked(var_idx) };
        let string = target.as_string_unchecked();
        
        if let Some(ch) = string.char_at(idx_val, memory) {
            let char_str = ch.to_string();
            let str_ref = StringView::new(&char_str, memory).as_ref();
            
            let new_index = DinoRef::int(idx_val + 1);
            unsafe { memory.stack.set_local_unchecked(var_idx + 1, new_index); }
            
            memory.stack.push(str_ref);
            Ok(true)
        } else {
            memory.stack.push(DinoRef::FALSE);
            Ok(false)
        }
    }
}
