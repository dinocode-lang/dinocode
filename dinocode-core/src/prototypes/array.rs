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
    memory::MemoryManager,
    types::{
        DinoRef,
        value_type,
    },
    errors::{
        Result,
        RuntimeError,
        RuntimeErrorType,
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
    pub fn push(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 {
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("push".into())));
        }
        
        let stack = memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let value = stack[args_start + 1];

        if !this.is_array() {
            return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance));
        }
        
        let handle = this.decode_index();
        let len = memory.get_array_len(handle);
        
        memory.set_array_element(handle, len, value)?;
        
        Ok(DinoRef::int((len + 1) as i64))
    }
    
    #[raw]
    pub fn pop(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let val = memory.array_pop(handle);
        Ok(val)
    }
    
    #[raw]
    pub fn get(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("get".into()))); }
        
        let (this, idx_ref) = {
            let stack = memory.stack();
            if args_start + 1 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1])
        };
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let idx = idx_ref.try_as_int(memory)?;
        
        if idx < 0 { return Ok(DinoRef::NONE); }
        
        let handle = this.decode_index();
        let val = memory.get_array_element(handle, idx as u32);
        
        Ok(val)
    }
    
    #[raw]
    pub fn set(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("set".into()))); }
        
        let (this, idx_ref, val) = {
            let stack = memory.stack();
            if args_start + 2 >= stack.len() { 
                return Err(RuntimeError::StackUnderflow); 
            }
            (stack[args_start], stack[args_start + 1], stack[args_start + 2])
        };
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let idx = idx_ref.try_as_int(memory)?;
        
        if idx < 0 { return Err(RuntimeError::Typed(RuntimeErrorType::IndexOutOfBounds)); }

        let handle = this.decode_index();
        memory.set_array_element(handle, idx as u32, val)?;
        Ok(val)
    }
    
    #[raw]
    #[getter]
    pub fn len(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_array_len(handle);
        Ok(DinoRef::int(len as i64))
    }
    
    #[raw]
    pub fn clear(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let slot = memory.object_pool.get_slot_mut(handle);
        if slot.kind != value_type::ARRAY { 
            return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); 
        }
        
        slot.data.array.count = 0;
        
        Ok(DinoRef::ZERO)
    }
    
    #[raw]
    pub fn is_empty(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_array_len(handle);
        Ok(DinoRef::bool(len == 0))
    }
    
    #[raw]
    pub fn first(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_array_len(handle);
        
        if len == 0 {
            return Ok(DinoRef::NONE);
        }
        
        let val = memory.get_array_element(handle, 0);
        Ok(val)
    }
    
    #[raw]
    pub fn last(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_array_len(handle);
        
        if len == 0 {
            return Ok(DinoRef::NONE);
        }
        
        let val = memory.get_array_element(handle, len - 1);
        Ok(val)
    }
    
    #[raw]
    #[symbol(name="in", alias)]
    pub fn contains(memory: &mut MemoryManager, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { 
            return Err(RuntimeError::Typed(RuntimeErrorType::MissingArgument("contains".into()))); 
        }
        
        let stack = memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let value = stack[args_start + 1];
        
        if !this.is_array() { return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedArrayInstance)); }
        
        let handle = this.decode_index();
        let len = memory.get_array_len(handle);
        
        if len == 0 {
            return Ok(DinoRef::bool(false));
        }
        
        for i in 0..len {
            let element = memory.get_array_element(handle, i);
            
            if element == value {
                return Ok(DinoRef::TRUE);
            }
        }
        
        Ok(DinoRef::FALSE)
    }
}

impl Array {
    pub fn create_instance(memory: &mut MemoryManager, args_start: usize, count: usize) -> DinoRef {
        let cap = (count as u32).max(1);
        let handle = memory.alloc_array_capacity(cap);

        let elements: Vec<DinoRef> = memory.stack()[args_start..args_start + count].to_vec();

        let slot = memory.object_pool.get_slot_mut(handle);
        let array = unsafe { &mut slot.data.array };

        unsafe {
            for (i, val) in elements.iter().enumerate() {
                std::ptr::write(array.elements.add(i), *val);
            }
            array.count = count as u32;
        }

        DinoRef::array(handle)
    }

    pub fn create_from_slice(memory: &mut MemoryManager, elements: &[DinoRef]) -> DinoRef {
        let cap = elements.len() as u32;
        let handle = memory.alloc_array_capacity(cap.max(1));

        let slot = memory.object_pool.get_slot_mut(handle);
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
