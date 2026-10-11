// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/range/prototype.rs
//  Desc:       Range prototype
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
            return Err(RuntimeError::MissingArgument { func: "step", expected: 1, got: args_count - 1 });
        }

        let (this, step_ref) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        let step_val = step_ref.try_as_int(&mut runtime.memory)?;
        if step_val == 0 {
            return Err(RuntimeError::InvalidArgumentValue { 
                func: "step", 
                message: "step value must not be zero" 
            }); 
        }

        let object = this.as_object()?;
        object.set(Self::STEP_VAL(), DinoRef::int(step_val), PROP_NORMAL, &mut runtime.memory)?;

        Ok(this)
    }

    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Ok(DinoRef::FALSE);
        }

        let (this, other) = unsafe {
            (
                runtime.memory.stack.get_unchecked(args_start),
                runtime.memory.stack.get_unchecked(args_start + 1)
            )
        };

        if let Ok(num) = other.try_as_int(&mut runtime.memory) {
            let object = this.as_object()?;
            let start_ref = object.get(Self::START(), &mut runtime.memory)?;
            let stop_ref = object.get(Self::STOP(), &mut runtime.memory)?;
            let step_ref = object.get(Self::STEP_VAL(), &mut runtime.memory)?;
            
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
