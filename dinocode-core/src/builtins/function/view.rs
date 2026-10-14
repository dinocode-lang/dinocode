// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/function/view.rs
//  Desc:       Wrapper for function memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    types::DinoRef,
    errors::{Result, RuntimeError},
    runtime::context::Runtime,
    native::registry::call_native_function,
};

#[derive(Clone, Copy)]
pub struct FunctionView {
    pub id: u32,
    pub is_native: bool,
}

impl FunctionView {
    #[inline(always)]
    pub fn as_id(&self) -> u32 {
        self.id
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        if self.is_native {
            DinoRef::native_fn(self.id)
        } else {
            DinoRef::function(self.id)
        }
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_function() {
            return Err(RuntimeError::ExpectedInstance("function"));
        }
        Ok(Self {
            id: dinoref.get_function_id(),
            is_native: dinoref.is_native_fn(),
        })
    }

    #[inline(always)]
    pub fn from_ref_unchecked(dinoref: DinoRef) -> Self {
        Self {
            id: dinoref.get_function_id(),
            is_native: dinoref.is_native_fn(),
        }
    }

    #[inline(always)]
    pub fn call(&self, args_start: usize, args_count: usize, runtime: &mut Runtime) -> Result<DinoRef> {
        if self.is_native {
            let res = call_native_function(runtime, self.id, args_start, args_count)?;
            runtime.memory.stack.move_sp(args_start - 1); // back to function ref
            unsafe { runtime.memory.stack.push_unchecked(res) };
            return Ok(res);
        } else if let Some(func) = runtime.functions.get(self.id as usize) {
            let start_ip = func.start_ip;
            runtime.memory.call_stack.push(
                (*runtime.ip + 1) as u32,
                self.id,
                func,
                args_start,
                args_count,
                1,
                &mut runtime.memory.stack,
            )?;
            *runtime.ip = start_ip as usize - 1;
        } else {
            Err(RuntimeError::FunctionNotFound)?
        }
        Ok(DinoRef::NONE)
    }

    #[inline(always)]
    pub fn virtual_call(&self, args_start: usize, args_count: usize, runtime: &mut Runtime) -> Result<DinoRef> {
        if self.is_native {
            let sp_before = runtime.memory.stack.depth();
            let res = call_native_function(runtime, self.id, args_start, args_count)?;
            if runtime.memory.stack.depth() == sp_before {
                runtime.memory.stack.move_sp(args_start);
                unsafe { runtime.memory.stack.push_unchecked(res) };
            }
            return Ok(res);
        } else if let Some(func) = runtime.functions.get(self.id as usize) {
            let start_ip = func.start_ip;
            runtime.memory.call_stack.push(
                (*runtime.ip + 1) as u32,
                self.id,
                func,
                args_start,
                args_count,
                0,
                &mut runtime.memory.stack,
            )?;
            *runtime.ip = start_ip as usize - 1;
        } else {
            Err(RuntimeError::FunctionNotFound)?
        }
        Ok(DinoRef::NONE)
    }
}
