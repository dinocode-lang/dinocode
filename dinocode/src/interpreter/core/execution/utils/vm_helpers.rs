// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/interpreter/core/execution/utils/vm_helpers.rs
//  Desc:       VM helper functions
//
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::DinoRef,
    builtins::{
        string::view::StringView,
        array::view::ArrayView,
    },
    memory::MemoryManager,
    errors::Result,
};

#[inline(always)]
pub fn args_to_array(memory: &mut MemoryManager, main_args: &[String]) -> Result<DinoRef> {
    let dinoref_args: Vec<DinoRef> = main_args.iter()
        .map(|arg| StringView::new(arg, memory).as_ref())
        .collect();
    
    ArrayView::new_from_slice(&dinoref_args, memory)
        .map(|array| array.as_ref())
}

pub fn get_call_stack_ips(memory: &MemoryManager, main_function: Option<DinoRef>, error_ip: usize) -> Vec<usize> {
    let mut exclude_ids = Vec::new();
    
    if let Some(main_ref) = main_function {
        exclude_ids.push(main_ref.get_function_id());
    }
    
    let len = memory.call_stack.len();
    let mut ips: Vec<usize> = (0..len)
        .rev()
        .map(|i| memory.call_stack.get_unchecked(i))
        .filter(|frame| !exclude_ids.contains(&frame.function_id))
        .take(9)
        .map(|frame| frame.return_address as usize)
        .collect();
    
    ips.reverse();
    ips.push(error_ip);
    
    ips
}
