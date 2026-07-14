// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/range.rs
//  Desc:       Range prototype - methods available on range objects (n..m)
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    memory::types::object_types,
    types::DinoRef,
    errors::{
        Result,
        RuntimeError,
    },
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    symbol,
};

crate::register_module! {
    name: init_range,
    classes: [Range]
}

#[dinoclass]
pub struct Range;

#[dinomethods]
impl Range {
    #[key]
    pub const START: () = ();

    #[key]
    pub const STOP: () = ();

    #[key]
    pub const STEP_VAL: () = ();

    #[raw]
    pub fn step(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument("step"));
        }

        let (this, step_ref) = {
            let stack = runtime.memory.stack();
            (stack[args_start], stack[args_start + 1])
        };

        if !this.is_object() {
            return Err(RuntimeError::ExpectedInstance("range"));
        }
        let handle = this.get_object_id();

        let step_val = step_ref.try_as_int(&mut runtime.memory)?;
        if step_val == 0 {
            return Err(RuntimeError::InvalidArgumentValue { 
                func: "step", 
                message: "step value must not be zero" 
            }); 
        }

        let _ = runtime.memory.set_object_property(handle, Self::STEP_VAL(), DinoRef::int(step_val), 0);

        Ok(this)
    }

    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Ok(DinoRef::FALSE);
        }

        let (this, other) = {
            let stack = runtime.memory.stack();
            (stack[args_start], stack[args_start + 1])
        };

        if !this.is_object() {
            return Ok(DinoRef::FALSE);
        }

        let handle_this = this.get_object_id();

        if let Ok(num) = other.try_as_int(&mut runtime.memory) {
            let start_ref = runtime.get_property_by_id(handle_this, Self::START())?;
            let stop_ref = runtime.get_property_by_id(handle_this, Self::STOP())?;
            let step_ref = runtime.get_property_by_id(handle_this, Self::STEP_VAL())?;
            
            let start = start_ref.try_as_int(&mut runtime.memory)?;
            let stop = stop_ref.try_as_int(&mut runtime.memory)?;
            let step = step_ref.try_as_int(&mut runtime.memory)?;

            if step > 0 {
                if num >= start && num < stop && (num - start) % step == 0 {
                    return Ok(DinoRef::TRUE);
                }
            } else if step < 0 {
                if num <= start && num > stop && (start - num) % (-step) == 0 {
                    return Ok(DinoRef::TRUE);
                }
            }
            return Ok(DinoRef::FALSE);
        }

        Ok(DinoRef::FALSE)
    }
}

impl Range {
    pub fn create_instance(runtime: &mut Runtime, start: i64, stop: i64, step: i64) -> DinoRef {
        let handle = runtime.memory.alloc_object_capacity(4);
        let slot = runtime.memory.object_pool.get_slot_mut(handle);
        slot.subkind = object_types::RANGE;

        let _ = runtime.memory.set_object_property(handle, Self::START(), DinoRef::int(start), 0);
        let _ = runtime.memory.set_object_property(handle, Self::STOP(), DinoRef::int(stop), 0);
        let _ = runtime.memory.set_object_property(handle, Self::STEP_VAL(), DinoRef::int(step), 0);

        if let Some(stack_idx) = Self::get_bootstrap_index() {
            let proto_ref = unsafe { runtime.memory.get_global_variable_unchecked(stack_idx) };
            runtime.memory.object_pool.get_slot_mut(handle).proto = proto_ref;
        }

        DinoRef::object(handle)
    }
}
