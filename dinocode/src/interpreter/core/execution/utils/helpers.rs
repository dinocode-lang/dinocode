// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/interpreter/core/execution/utils/helpers.rs
//  Desc:       Utility helpers for execution
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    runtime::context::Runtime,
    types::DinoRef,
    errors::Result,
};
use std::cmp::Ordering;

#[inline(always)]
pub fn try_magic_method(
    obj: DinoRef,
    other: DinoRef,
    symbol: DinoRef,
    runtime: &mut Runtime,
) -> Result<Option<DinoRef>> {
    if let Ok(magic_method) = runtime.get_property(obj, symbol) {
        let current_depth = runtime.memory.stack_depth();
        runtime.memory.stack_push(obj);
        runtime.memory.stack_push(other);
        let res = runtime.call_function(magic_method, current_depth, 2)?;
        runtime.memory.stack_pop_n(2);
        return Ok(Some(res));
    }
    Ok(None)
}

#[inline(always)]
pub fn strings_equal(a: DinoRef, b: DinoRef, memory: &dinocode_core::memory::MemoryManager) -> bool {
    if a.raw() == b.raw() {
        return true;
    }
    
    let a_idx = a.decode_index();
    let b_idx = b.decode_index();
    
    let a_len = memory.get_const_len(a_idx);
    let b_len = memory.get_const_len(b_idx);
    
    if a_len != b_len {
        return false;
    }
    
    let a_offset = a_idx as usize;
    let b_offset = b_idx as usize;
    
    if memory.is_interned(a_offset) && memory.is_interned(b_offset) {
        return false;
    }
    
    if memory.has_hash(a_offset) && memory.has_hash(b_offset) {
        let a_hash = memory.get_const_hash(a_idx);
        let b_hash = memory.get_const_hash(b_idx);
        if a_hash != b_hash {
            return false;
        }
    }
    
    memory.get_const_bytes(a_idx) == memory.get_const_bytes(b_idx)
}

#[inline(always)]
pub fn strings_compare(a: DinoRef, b: DinoRef, memory: &dinocode_core::memory::MemoryManager) -> Ordering {
    if a.raw() == b.raw() {
        return Ordering::Equal;
    }
    
    let a_idx = a.decode_index();
    let b_idx = b.decode_index();
    
    let a_len = memory.get_const_len(a_idx);
    let b_len = memory.get_const_len(b_idx);
    
    if a_len != b_len {
        return a_len.cmp(&b_len);
    }
    
    let a_offset = a_idx as usize;
    let b_offset = b_idx as usize;
    
    if memory.is_interned(a_offset) && memory.is_interned(b_offset) {
        return Ordering::Less;
    }
    
    if memory.has_hash(a_offset) && memory.has_hash(b_offset) {
        let a_hash = memory.get_const_hash(a_idx);
        let b_hash = memory.get_const_hash(b_idx);
        if a_hash != b_hash {
            return a_hash.cmp(&b_hash);
        }
    }
    
    memory.get_string(a_idx).cmp(memory.get_string(b_idx))
}
