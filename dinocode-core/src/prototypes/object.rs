// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/prototypes/object.rs
//  Desc:       Object prototype - methods available on all object instances
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    runtime::context::Runtime,
    memory::types::pool_types::ObjectEntry,
    types::{
        DinoRef,
        value_type,
    },
    errors::{
        Result,
        RuntimeError,
    },
    prototypes::array::Array,
};
use dinocode_macros::{
    dinoclass,
    dinomethods,
    raw,
    getter,
    symbol,
};

crate::register_module! {
    name: init_object,
    classes: [Object]
}

#[dinoclass]
pub struct Object;

#[dinomethods]
impl Object {
    #[raw]
    pub fn keys(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        let handle = this.get_object_id();
        let keys = runtime.memory.get_object_keys(handle);
        Ok(Array::create_from_slice(runtime, &keys))
    }
    
    #[raw]
    pub fn values(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        let handle = this.get_object_id();
        let values = runtime.memory.get_object_values(handle);
        Ok(Array::create_from_slice(runtime, &values))
    }
    
    #[raw]
    pub fn get(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument("get")); }
        
        let stack = runtime.memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let key = stack[args_start + 1];
        
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        runtime.get_property(this, key)
    }
    
    #[raw]
    pub fn set(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 3 { return Err(RuntimeError::MissingArgument("set")); }
        
        let stack = runtime.memory.stack();
        if args_start + 2 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let key = stack[args_start + 1];
        let val = stack[args_start + 2];
        
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        runtime.set_property(this, key, val)?;
        
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
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        let handle = this.get_object_id();
        let slot = runtime.memory.object_pool.get_slot(handle);
        
        if slot.kind != value_type::OBJECT { 
            return Err(RuntimeError::ExpectedInstance("object")); 
        }
        
        let count = unsafe { slot.data.object.count };
        Ok(DinoRef::int(count as i64))
    }
    
    #[raw]
    #[symbol(name="in", alias)]
    pub fn has(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument("has")); }
        
        let stack = runtime.memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let key = stack[args_start + 1];
        
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        let has_property = runtime.get_property(this, key).is_ok();
        
        Ok(DinoRef::bool(has_property))
    }
    
    #[raw]
    pub fn delete(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 2 { return Err(RuntimeError::MissingArgument("delete")); }
        
        let stack = runtime.memory.stack();
        if args_start + 1 >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        let key = stack[args_start + 1];
        
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        let handle = this.get_object_id();
        let key_hash = runtime.memory.get_key_hash(key);
        let key_raw = key.raw();
        let key_type = key.decode_type();
        let key_is_const = key.is_const();
        
        let (object_capacity, object_entries) = {
            let slot = runtime.memory.object_pool.get_slot(handle);
            if slot.kind != value_type::OBJECT { 
                return Err(RuntimeError::ExpectedInstance("object")); 
            }
            unsafe { (slot.data.object.capacity, slot.data.object.entries) }
        };
        
        unsafe {
            let cap = object_capacity as usize;
            
            let start_idx = (key_hash as usize) % cap;
            for i in 0..cap {
                let idx = (start_idx + i) % cap;
                let entry_ptr = object_entries.add(idx);
                let entry = &*entry_ptr;
                
                if entry.hash == key_hash {
                    let mut matched = entry.key == key_raw;
                    if !matched && key_is_const {
                        let entry_key_ref = DinoRef::from_raw(entry.key);
                        if entry_key_ref.decode_type() == key_type {
                            let str1 = runtime.memory.get_const_bytes(key.decode_index());
                            let str2 = runtime.memory.get_const_bytes(entry_key_ref.decode_index());
                            if str1 == str2 {
                                matched = true;
                            }
                        }
                    }
                    if matched {
                        let entry_mut = &mut *entry_ptr;
                        entry_mut.hash = 0;
                        entry_mut.key = 0;
                        entry_mut.value = DinoRef::NONE;
                        entry_mut.flags = 0;
                        let slot = runtime.memory.object_pool.get_slot_mut(handle);
                        slot.data.object.count -= 1;
                        return Ok(DinoRef::TRUE);
                    }
                }
                
            }
        }
        
        Ok(DinoRef::FALSE)
    }
    
    #[raw]
    pub fn clear(runtime: &mut Runtime, args_start: usize, args_count: usize) -> Result<DinoRef> {
        if args_count < 1 { return Err(RuntimeError::StackUnderflow); }
        
        let stack = runtime.memory.stack();
        if args_start >= stack.len() { 
            return Err(RuntimeError::StackUnderflow); 
        }
        
        let this = stack[args_start];
        if !this.is_object() { return Err(RuntimeError::ExpectedInstance("object")); }
        
        let handle = this.get_object_id();
        let slot = runtime.memory.object_pool.get_slot_mut(handle);
        
        if slot.kind != value_type::OBJECT { 
            return Err(RuntimeError::ExpectedInstance("object")); 
        }
        
        unsafe {
            let object = &mut slot.data.object;
            let cap = object.capacity as usize;
            
            for i in 0..cap {
                let entry_ptr = object.entries.add(i);
                std::ptr::write(entry_ptr, ObjectEntry { 
                    hash: 0,
                    key: 0, 
                    value: DinoRef::NONE, 
                    flags: 0 
                });
            }
            
            object.count = 0;
        }
        
        Ok(DinoRef::NONE)
    }
}

impl Object {
    pub fn create_instance(runtime: &mut Runtime, args_start: usize, count: usize) -> DinoRef {
        let pair_count = (count / 2) as u32;
        let cap = pair_count.next_power_of_two().max(8);
        let handle = runtime.memory.alloc_object_capacity(cap);

        let stack_snapshot: Vec<DinoRef> = runtime.memory.stack()[args_start..args_start + count].to_vec();
        for chunk in stack_snapshot.chunks(2) {
            if chunk.len() == 2 {
                let _ = runtime.memory.set_object_property(handle, chunk[0], chunk[1], 0);
            }
        }

        DinoRef::object(handle)
    }

    pub fn create_from_slice(runtime: &mut Runtime, properties: &[DinoRef]) -> DinoRef {
        let count = (properties.len() / 2) as u32;
        let cap = count.next_power_of_two().max(8);
        let handle = runtime.memory.alloc_object_capacity(cap);

        for chunk in properties.chunks(2) {
            if chunk.len() == 2 {
                let _ = runtime.memory.set_object_property(handle, chunk[0], chunk[1], 0);
            }
        }

        DinoRef::object(handle)
    }
}
