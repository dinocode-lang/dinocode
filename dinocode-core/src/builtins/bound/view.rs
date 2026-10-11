// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/bound/view.rs
//  Desc:       Wrapper for bound function memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        manager::MemoryManager,
        types::{
            object_types,
            property_flags::PROP_NORMAL,
        }
    },
    runtime::context::Runtime,
    types::{DinoRef, Symbol},
    errors::{Result, RuntimeError},
    builtins::{
        object::view::ObjectView,
        array::view::ArrayView,
        bound::prototype::BoundFunction,
    },
};

#[derive(Clone, Copy)]
pub struct BoundFunctionView(pub u32);

impl BoundFunctionView {
    #[inline(always)]
    pub fn as_id(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        DinoRef::object(self.0)
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_object() {
            return Err(RuntimeError::ExpectedInstance("bound function"));
        }
        Ok(Self(dinoref.get_object_id()))
    }

    #[inline(always)]
    pub fn from_ref_unchecked(dinoref: DinoRef) -> Self {
        Self(dinoref.get_object_id())
    }

    pub fn new_from_stack(size: usize, memory: &mut MemoryManager) -> Result<Self> {
        let depth = memory.stack.depth();
        let start = depth - size;
        let fn_ref = unsafe { memory.stack.get_unchecked(start) };
        let bound_args = if size > 1 {
            unsafe { std::slice::from_raw_parts(memory.stack.ptr.add(start + 1), size - 1) }
        } else {
            &[]
        };
        Self::new_from_slice(fn_ref, bound_args, memory)
    }

    pub fn new_from_slice(fn_ref: DinoRef, bound_args: &[DinoRef], memory: &mut MemoryManager) -> Result<Self> {
        let bound_args = ArrayView::new_from_slice(bound_args, memory)?;

        let pairs = [
            Symbol::FN, fn_ref,
            Symbol::ARGS, bound_args.as_ref()
        ];
        
        let proto = BoundFunction::as_ref(memory);
        let object = ObjectView::new_from_slice(proto, &pairs, PROP_NORMAL, memory)?;

        let slot = memory.object_pool.get_slot_mut(object.as_id());
        slot.subkind = object_types::BOUND_FN;

        Ok(Self(object.as_id()))
    }

    pub fn call(&self, args_start: usize, args_count: usize, runtime: &mut Runtime) -> Result<DinoRef> {
        let object = ObjectView::from_id(self.0);
        let func = object.get(Symbol::FN, &mut runtime.memory)?;
        let bound_args = object.get(Symbol::ARGS, &mut runtime.memory)?;
        let array = bound_args.as_array_unchecked();
        let bound_count = array.len(&runtime.memory) as usize;
        
        let args_count = args_count - 1;   // Exclude BOUND_FN wrapper object
        let total_args = bound_count + args_count;

        let current_size = runtime.memory.stack.depth();
        let target_size = args_start + total_args;
        
        if target_size > current_size {
            let diff = target_size - current_size;
            runtime.memory.stack.push_n(diff, DinoRef::NONE);
        }

        if bound_count > 0 && args_count > 0 {
            let fn_args_start = args_start + 1; // Skip BOUND_FN wrapper object
            for i in (0..args_count).rev() {
                unsafe {
                    let val = runtime.memory.stack.get_unchecked(fn_args_start + i);
                    runtime.memory.stack.set_unchecked(args_start + bound_count + i, val);
                }
            }
        }

        for i in 0..bound_count {
            unsafe {
                let val = array.get_unchecked(i as u32, &runtime.memory);
                runtime.memory.stack.set_unchecked(args_start + i, val);
            }
        }

        let func = func.as_function_unchecked();
        func.virtual_call(args_start, total_args, runtime)
    }
}
