// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/array.rs
//  Desc:       Array prototype — methods available on array objects
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    types::{
        DinoRef,
        value_type,
    },
    errors::{
        Result,
        RuntimeError,
    },
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    getter,
    symbol,
};

crate::register_module! {
    name: init_array,
    classes: [Array]
}

#[dinoclass]
pub struct Array;

#[dinomethods]
impl Array {
    #[raw]
    pub fn push(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::MissingArgument("push"));
        }
        
        let stack = runtime.memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let value = stack[args_start + 1];

        if !this.is_array() {
            return Err(RuntimeError::ExpectedInstance("array"));
        }
        
        let handle = this.decode_index();
        let len = runtime.memory.get_array_len(handle);
        
        runtime.memory.set_array_element(handle, len, value)?;
        
        Ok(DinoRef::int((len + 1) as i64))
    }
    
    #[raw]
    pub fn pop(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let val = runtime.memory.array_pop(handle);
        Ok(val)
    }
    
    #[raw]
    pub fn get(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument("get")); }
        
        let (this, idx_ref) = {
            let stack = runtime.memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let idx = idx_ref.try_as_int(&mut runtime.memory)?;
        
        if idx < 0 { return Ok(DinoRef::NONE); }
        
        let handle = this.decode_index();
        let val = runtime.memory.get_array_element(handle, idx as u32);
        
        Ok(val)
    }
    
    #[raw]
    pub fn set(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::MissingArgument("set")); }
        
        let (this, idx_ref, val) = {
            let stack = runtime.memory.stack();
            if args_start + 2 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1], stack[args_start + 2])
        };
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let idx = idx_ref.try_as_int(&mut runtime.memory)?;
        
        if idx < 0 { return Err(RuntimeError::IndexOutOfBounds); }

        let handle = this.decode_index();
        runtime.memory.set_array_element(handle, idx as u32, val)?;
        Ok(val)
    }
    
    #[raw]
    #[getter]
    pub fn len(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let len = runtime.memory.get_array_len(handle);
        Ok(DinoRef::int(len as i64))
    }
    
    #[raw]
    pub fn clear(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let slot = runtime.memory.object_pool.get_slot_mut(handle);
        if slot.kind != value_type::ARRAY { 
            return Err(RuntimeError::ExpectedInstance("array")); 
        }
        
        slot.data.array.count = 0;
        
        Ok(DinoRef::ZERO)
    }
    
    #[raw]
    pub fn is_empty(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let len = runtime.memory.get_array_len(handle);
        Ok(DinoRef::bool(len == 0))
    }
    
    #[raw]
    pub fn first(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let len = runtime.memory.get_array_len(handle);
        
        if len == 0 {
            return Ok(DinoRef::NONE);
        }
        
        let val = runtime.memory.get_array_element(handle, 0);
        Ok(val)
    }
    
    #[raw]
    pub fn last(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let len = runtime.memory.get_array_len(handle);
        
        if len == 0 {
            return Ok(DinoRef::NONE);
        }
        
        let val = runtime.memory.get_array_element(handle, len - 1);
        Ok(val)
    }
    
    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { 
            return Err(RuntimeError::MissingArgument("contains")); 
        }
        
        let stack = runtime.memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let value = stack[args_start + 1];
        
        if !this.is_array() { return Err(RuntimeError::ExpectedInstance("array")); }
        
        let handle = this.decode_index();
        let len = runtime.memory.get_array_len(handle);
        
        if len == 0 {
            return Ok(DinoRef::bool(false));
        }
        
        for i in 0..len {
            let element = runtime.memory.get_array_element(handle, i);
            
            if element == value {
                return Ok(DinoRef::TRUE);
            }
        }
        
        Ok(DinoRef::FALSE)
    }
}

impl Array {
    pub fn create_instance(runtime: &mut Runtime, args_start: usize, count: usize) -> DinoRef {
        let cap = (count as u32).max(1);
        let handle = runtime.memory.alloc_array_capacity(cap);

        let elements: Vec<DinoRef> = runtime.memory.stack()[args_start..args_start + count].to_vec();

        let slot = runtime.memory.object_pool.get_slot_mut(handle);
        let array = unsafe { &mut slot.data.array };

        unsafe {
            for (i, val) in elements.iter().enumerate() {
                std::ptr::write(array.elements.add(i), *val);
            }
            array.count = count as u32;
        }

        DinoRef::array(handle)
    }

    pub fn create_from_slice(runtime: &mut Runtime, elements: &[DinoRef]) -> DinoRef {
        let cap = elements.len() as u32;
        let handle = runtime.memory.alloc_array_capacity(cap.max(1));

        let slot = runtime.memory.object_pool.get_slot_mut(handle);
        let array = unsafe { &mut slot.data.array };

        unsafe {
            for (i, val) in elements.iter().enumerate() {
                std::ptr::write(array.elements.add(i), *val);
            }
            array.count = cap;
        }

        DinoRef::array(handle)
    }
}
