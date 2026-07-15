// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/function.rs
//  Desc:       Function prototype - methods available on functions
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    types::{
        DinoRef,
        Symbol,
    },
    errors::{
        Result,
        RuntimeError,
    },
    prototypes::array::Array,
    memory::types::object_types,
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    symbol,
};

crate::register_module! {
    name: init_function,
    classes: [Function]
}

#[dinoclass]
pub struct Function;

#[dinomethods]
impl Function {
    #[raw]
    pub fn bind(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument("bind"));
        }

        let fn_ref = runtime.memory.stack()[args_start];

        if !fn_ref.is_function() {
            return Err(RuntimeError::ExpectedInstance("function"));
        }

        let bound_count = if args_count > 1 { args_count - 1 } else { 0 };
        Ok(Self::create_bound_instance(runtime, fn_ref, args_start + 1, bound_count))
    }

    #[raw]
    #[symbol(name="call")]
    pub fn call(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 {
            return Err(RuntimeError::StackUnderflow);
        }

        let bound_obj = runtime.memory.stack()[args_start];
        if !bound_obj.is_object() {
            return Err(RuntimeError::ExpectedInstance("bound function object"));
        }

        let obj_id = bound_obj.get_object_id();

        let fn_ref = runtime.get_property_by_id(obj_id, Symbol::FN)?;
        let bound_args = runtime.get_property_by_id(obj_id, Symbol::ARGS)?;
        
        let bound_handle = bound_args.decode_index();
        let bound_count = runtime.memory.get_array_len(bound_handle) as usize;
        let passed_count = args_count - 1;
        let total_args = bound_count + passed_count;

        let current_size = runtime.memory.stack_depth();
        let target_size = args_start + total_args;
        
        if target_size > current_size {
            let diff = target_size - current_size;
            for _ in 0..diff {
                runtime.memory.stack_push(DinoRef::NONE);
            }
        }

        if bound_count >= 1 {
            for i in (1..=passed_count).rev() {
                let val = runtime.memory.stack()[args_start + i];
                unsafe {
                    runtime.memory.stack_set_unchecked(args_start + bound_count + i - 1, val);
                }
            }
        } else {
            for i in 1..=passed_count {
                let val = runtime.memory.stack()[args_start + i];
                unsafe {
                    runtime.memory.stack_set_unchecked(args_start + i - 1, val);
                }
            }
        }

        for i in 0..bound_count {
            let val = runtime.memory.get_array_element(bound_handle, i as u32);
            unsafe {
                runtime.memory.stack_set_unchecked(args_start + i, val);
            }
        }

        let sp_before = runtime.memory.stack_depth();

        if fn_ref.is_user_function() {
            let fid = fn_ref.get_function_id();
            if let Some(func) = runtime.functions.get(fid as usize) {
                let start_ip = func.start_ip;
                runtime.memory.push_call_frame((*runtime.ip + 1) as u32, fid, func, args_start, total_args)?;
                *runtime.ip = start_ip as usize - 1;
                return Ok(DinoRef::NONE);
            }
            return Err(RuntimeError::FunctionNotFound);
        } else if fn_ref.is_native_fn() {
            let fid = fn_ref.get_function_id();
            let res = crate::native::call_native_function(runtime, fid, args_start, total_args)?;
            if runtime.memory.stack_depth() == sp_before {
                runtime.memory.stack_pop_n(target_size.saturating_sub(current_size));
            }
            return Ok(res);
        }

        Err(RuntimeError::FunctionNotFound)
    }
}

impl Function {
    pub fn create_bound_instance(runtime: &mut Runtime, fn_ref: DinoRef, args_start: usize, bound_count: usize) -> DinoRef {
        let handle = runtime.memory.alloc_object_capacity(4);
        runtime.memory.object_pool.get_slot_mut(handle).subkind = object_types::BOUND_FN;

        if let Some(stack_idx) = Function::get_bootstrap_index() {
            let proto_ref = unsafe { runtime.memory.get_global_variable_unchecked(stack_idx) };
            runtime.memory.set_proto(handle, proto_ref);
        }

        let _ = runtime.memory.set_object_property(handle, Symbol::FN, fn_ref, 0);

        let bound_args = Array::create_instance(runtime, args_start, bound_count);
        let _ = runtime.memory.set_object_property(handle, Symbol::ARGS, bound_args, 0);

        DinoRef::object(handle)
    }
}
