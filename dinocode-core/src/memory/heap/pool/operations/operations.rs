// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/heap/pool/operations.rs
//  Desc:       Operations for manipulating pool objects (Arrays/Objects)
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        MemoryManager,
        types::pool_types::{
            NativeArray,
            NativeObject,
            ObjectEntry,
        },
    },
    types::{
        DinoRef,
        value_type,
    },
    prototypes::{
        array::Array as ProtoArray,
        object::Object as ProtoObject,
    },
    errors::{RuntimeError, Result, RuntimeErrorType},
};
use std::alloc::{alloc, dealloc, Layout};

impl MemoryManager {
    // Helpers
    
    pub fn get_key_hash(&mut self, key: DinoRef) -> u64 {
        if key.is_const() {
            self.ensure_const_hash(key)
        } else {
            Self::hash_u64(key.raw())
        }
    }

    pub fn hash_u64(key: u64) -> u64 {
        let mut x = key;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }

    unsafe fn resize_object(object: &mut NativeObject, new_cap: u32) {
        let old_cap = object.capacity as usize;
        let old_entries = object.entries;
        
        let new_layout = Layout::array::<ObjectEntry>(new_cap as usize).unwrap();
        unsafe {
            let new_entries = alloc(new_layout) as *mut ObjectEntry;
            
            for i in 0..new_cap {
                std::ptr::write(new_entries.add(i as usize), ObjectEntry { hash: 0, key: 0, value: DinoRef::NONE, flags: 0 });
            }
            
            for i in 0..old_cap {
                let old_entry = &*old_entries.add(i);
                if old_entry.key != 0 {
                    let mut idx = (old_entry.hash as usize) % (new_cap as usize);
                    loop {
                        let new_entry = &mut *new_entries.add(idx);
                        if new_entry.key == 0 {
                            *new_entry = *old_entry;
                            break;
                        }
                        idx = (idx + 1) % (new_cap as usize);
                    }
                }
            }
            
            let old_layout = Layout::array::<ObjectEntry>(old_cap).unwrap();
            dealloc(old_entries as *mut u8, old_layout);
            
            object.entries = new_entries;
        }
        object.capacity = new_cap;
    }

    // Public allocation methods

    pub fn alloc_array_capacity(&mut self, capacity: u32) -> u32 {
        self.record_pool_alloc();
        
        let handle = self.object_pool.alloc_slot().expect("Pool full");
        
        let cap = if capacity == 0 { 4 } else { capacity };
        let layout = Layout::array::<DinoRef>(cap as usize).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut DinoRef;
        
        let array = NativeArray {
            capacity: cap,
            count: 0,
            elements: ptr,
        };
        
        let slot = self.object_pool.get_slot_mut(handle);
        slot.kind = value_type::ARRAY;
        slot.proto = DinoRef::NONE;
        slot.data.array = array;
        
        // Set Array prototype
        if let Some(stack_idx) = ProtoArray::get_bootstrap_index() {
            let proto_ref = unsafe { self.get_global_variable_unchecked(stack_idx) };
            self.set_proto(handle, proto_ref);
        }
        
        handle
    }
    
    pub fn alloc_object_capacity(&mut self, capacity: u32) -> u32 {
        self.record_pool_alloc();
        
        let handle = self.object_pool.alloc_slot().expect("Pool full");
        
        let cap = if capacity == 0 { 8 } else { capacity };
        let layout = Layout::array::<ObjectEntry>(cap as usize).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut ObjectEntry;
        
        for i in 0..cap {
            unsafe {
                let entry_ptr = ptr.add(i as usize);
                std::ptr::write(entry_ptr, ObjectEntry { hash: 0, key: 0, value: DinoRef::NONE, flags: 0 });
            }
        }
        
        let object = NativeObject {
            capacity: cap,
            count: 0,
            entries: ptr,
        };
        
        let slot = self.object_pool.get_slot_mut(handle);
        slot.kind = value_type::OBJECT;
        slot.proto = DinoRef::NONE;
        slot.data.object = object;
        
        // Set Object prototype
        if let Some(stack_idx) = ProtoObject::get_bootstrap_index() {
            let proto_ref = unsafe { self.get_global_variable_unchecked(stack_idx) };
            self.set_proto(handle, proto_ref);
        }
        
        handle
    }

    pub fn free_pool_object(&mut self, handle: u32) {
        self.object_pool.free_slot(handle);
    }


    #[inline(always)]
    pub fn has_object_type(&self, handle: u32, object_type: u16) -> bool {
        let slot = self.object_pool.get_slot(handle);
        slot.kind == value_type::OBJECT && slot.subkind == object_type
    }

    // Other Operations

    pub fn set_object_property(&mut self, handle: u32, key: DinoRef, value: DinoRef, flags: u8) -> Result<()> {
        let key_hash = self.get_key_hash(key);
        let key_raw = key.raw();
        let key_is_const = key.is_const();
        let key_type = key.decode_type();

        let (object_count, object_capacity, _) = {
            let slot = self.object_pool.get_slot(handle);
            if slot.kind != value_type::OBJECT { 
                return Err(RuntimeError::Typed(RuntimeErrorType::ExpectedObjectInstance));
            }
            unsafe { (slot.data.object.count, slot.data.object.capacity, slot.data.object.entries) }
        };
        
        if object_count as f64 > (object_capacity as f64 * 0.75) {
            let new_cap = object_capacity * 2;
            let slot = self.object_pool.get_slot_mut(handle);
            unsafe { Self::resize_object(&mut slot.data.object, new_cap); }
        }
        
        let (object_capacity, object_entries) = {
            let slot = self.object_pool.get_slot(handle);
            unsafe { (slot.data.object.capacity, slot.data.object.entries) }
        };

        unsafe {
            let cap = object_capacity as usize;
            let entries = object_entries;
            let start_idx = (key_hash as usize) % cap;
            
            for i in 0..cap {
                let idx = (start_idx + i) % cap;
                let entry_ptr = entries.add(idx);
                let entry = &*entry_ptr;
                
                if entry.key == 0 {
                    std::ptr::write(entry_ptr, ObjectEntry { hash: key_hash, key: key_raw, value, flags });
                    let slot = self.object_pool.get_slot_mut(handle);
                    slot.data.object.count += 1;
                    return Ok(());
                }
                
                if entry.hash == key_hash {
                    let mut matched = entry.key == key_raw;
                    if !matched && key_is_const {
                        let entry_key_ref = DinoRef::from_raw(entry.key);
                        if entry_key_ref.decode_type() == key_type {
                            let str1 = self.get_const_bytes(key.decode_index());
                            let str2 = self.get_const_bytes(entry_key_ref.decode_index());
                            if str1 == str2 {
                                matched = true;
                            }
                        }
                    }
                    if matched {
                        (*entry_ptr).value = value;
                        (*entry_ptr).flags = flags;
                        return Ok(());
                    }
                }
            }
        }
        Err(RuntimeError::InternalError("Object property map is full".to_string()))
    }

    pub fn set_proto(&mut self, handle: u32, proto: DinoRef) {
        self.object_pool.get_slot_mut(handle).proto = proto;
    }
    
    pub fn get_proto(&self, handle: u32) -> DinoRef {
        self.object_pool.get_slot(handle).proto
    }

    pub fn get_property(&mut self, handle: u32, key: DinoRef) -> Option<DinoRef> {
        self.get_property_details(handle, key).map(|(val, _)| val)
    }

    pub fn get_property_details(&mut self, handle: u32, key: DinoRef) -> Option<(DinoRef, u8)> {
        let key_hash = self.get_key_hash(key);
        let key_raw = key.raw();
        let key_is_const = key.is_const();
        let key_type = key.decode_type();

        let mut current_handle = handle;
        let mut depth = 0;
        
        loop {
            if depth > 100 { return None; }
            
            let (kind, proto, object_capacity, object_entries) = {
                let slot = self.object_pool.get_slot(current_handle);
                (slot.kind, slot.proto, unsafe { slot.data.object.capacity }, unsafe { slot.data.object.entries })
            };
            
            if kind == value_type::OBJECT {
                unsafe {
                    let cap = object_capacity as usize;
                    let start_idx = (key_hash as usize) % cap;
                    for i in 0..cap {
                        let idx = (start_idx + i) % cap;
                        let entry = &*object_entries.add(idx);
                        if entry.key == 0 { break; } // Empty slot found, key not here
                        if entry.hash == key_hash {
                            if entry.key == key_raw { 
                                return Some((entry.value, entry.flags)); 
                            }
                            if key_is_const {
                                let entry_key_ref = DinoRef::from_raw(entry.key);
                                if entry_key_ref.decode_type() == key_type {
                                    let str1 = self.get_const_bytes(key.decode_index());
                                    let str2 = self.get_const_bytes(entry_key_ref.decode_index());
                                    if str1 == str2 {
                                        return Some((entry.value, entry.flags));
                                    }
                                }
                            }
                        }
                    }
                }
                // Not found in this object, check prototype
                if proto.is_none() { return None; }
                if proto.is_object() {
                    current_handle = proto.get_object_id();
                } else {
                    return None;
                }
            } else if kind == value_type::ARRAY {
                // Arrays don't have named properties, only methods on prototype
                if proto.is_none() { return None; }
                if proto.is_object() {
                    current_handle = proto.get_object_id();
                } else {
                    return None;
                }
            } else {
                return None;
            }
            depth += 1;
        }
    }

}
