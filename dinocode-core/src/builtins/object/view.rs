// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/object/view.rs
//  Desc:       Wrapper for object memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        manager::MemoryManager,
        types::{
            pool_types::{NativeObject, ObjectEntry},
            PropertyFlags,
            property_flags::{PROP_NORMAL, PROP_PROTOTYPE},
        },
    },
    types::{DinoRef, value_type, Symbol},
    errors::{Result, RuntimeError},
    utils::hash::DinoHash,
    runtime::context::Runtime,
    builtins::bound::view::BoundFunctionView,
};
use std::{
    ptr::{write, write_bytes},
    alloc::{alloc, dealloc, Layout}
};

#[derive(Clone, Copy)]
pub struct ObjectView {
    pub id: u32,
    pub is_class: bool,
}

impl ObjectView {
    #[inline(always)]
    pub fn as_id(&self) -> u32 {
        self.id
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        if self.is_class {
            DinoRef::class(self.id)
        } else {
            DinoRef::object(self.id)
        }
    }

    #[inline(always)]
    pub fn as_object_ref(&self) -> DinoRef {
        DinoRef::object(self.id)
    }

    #[inline(always)]
    pub fn as_class_ref(&self) -> DinoRef {
        DinoRef::class(self.id)
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_object() {
            return Err(RuntimeError::ExpectedInstance("object"));
        }
        Ok(Self {
            id: dinoref.get_object_id(),
            is_class: dinoref.is_class(),
        })
    }

    #[inline(always)]
    pub fn from_ref_unchecked(dinoref: DinoRef) -> Self {
        Self {
            id: dinoref.get_object_id(),
            is_class: dinoref.is_class(),
        }
    }

    #[inline(always)]
    pub fn from_id(id: u32) -> Self {
        Self {
            id,
            is_class: false,
        }
    }

    #[inline(always)]
    pub fn set_proto(&self, proto: DinoRef, memory: &mut MemoryManager) {
        let slot = memory.object_pool.get_slot_mut(self.id);
        slot.proto = proto;
    }

    #[inline(always)]
    pub fn set_subkind(&self, subkind: u16, memory: &mut MemoryManager) {
        let slot = memory.object_pool.get_slot_mut(self.id);
        slot.subkind = subkind;
    }

    pub fn new_from_stack(proto: DinoRef, size: usize, flags: u8, memory: &mut MemoryManager) -> Result<Self> {
        let depth = memory.stack.depth();
        let start = depth - size;
        let elements = unsafe { std::slice::from_raw_parts(memory.stack.ptr.add(start), size) };
        Self::new_from_slice(proto, elements, flags, memory)
    }

    pub fn new_from_slice(proto: DinoRef, pairs: &[DinoRef], flags: u8, memory: &mut MemoryManager) -> Result<Self> {
        memory.record_pool_alloc();
        let handle = memory.object_pool.alloc_slot().expect("Pool full");
        let pair_count = pairs.len() / 2;
        let cap = (pair_count as u32).next_power_of_two().max(8);

        let layout = Layout::array::<ObjectEntry>(cap as usize).unwrap();
        let entries_ptr = unsafe { alloc(layout) } as *mut ObjectEntry;
        unsafe { write_bytes(entries_ptr, 0, cap as usize); }

        let mut actual_count = 0;
        for i in (0..pairs.len()).step_by(2) {
            let key = pairs[i];
            let value = pairs[i + 1];

            let key_hash = DinoHash::any(key, memory);
            let key_raw = key.raw();

            let start_idx = (key_hash as usize) % (cap as usize);
            for j in 0..cap {
                let idx = (start_idx + j as usize) % (cap as usize);
                let entry = unsafe { &mut *entries_ptr.add(idx) };
                if entry.key == 0 {
                    entry.key = key_raw;
                    entry.hash = key_hash;
                    entry.value = value;
                    entry.flags = flags;
                    actual_count += 1;
                    break;
                }
            }
        }

        let slot = memory.object_pool.get_slot_mut(handle);
        slot.kind = value_type::OBJECT;
        slot.proto = proto;
        slot.data.object.capacity = cap;
        slot.data.object.count = actual_count as u32;
        slot.data.object.entries = entries_ptr;

        Ok(Self { id: handle, is_class: false })
    }

    #[inline(always)]
    pub fn has_type(&self, object_type: u16, memory: &MemoryManager) -> bool {
        let slot = memory.object_pool.get_slot(self.id);
        slot.kind == value_type::OBJECT && slot.subkind == object_type
    }

    pub(crate) unsafe fn resize_instance_object(object: &mut NativeObject, new_cap: u32) {
        let old_cap = object.capacity as usize;
        let old_entries = object.entries;

        unsafe {
            let new_layout = Layout::array::<ObjectEntry>(new_cap as usize).unwrap();
            let new_entries = alloc(new_layout) as *mut ObjectEntry;

            write_bytes(new_entries, 0, new_cap as usize);

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

    pub fn set(&self, key: DinoRef, value: DinoRef, flags: u8, memory: &mut MemoryManager) -> Result<()> {
        let key_hash = DinoHash::any(key, memory);
        let key_raw = key.raw();
        let key_is_const = key.is_const();
        let key_type = key.decode_type();
        let final_flags = if self.is_class { flags | PROP_PROTOTYPE } else { flags };

        let (object_count, object_capacity, _) = {
            let slot = memory.object_pool.get_slot(self.id);
            unsafe { (slot.data.object.count, slot.data.object.capacity, slot.data.object.entries) }
        };

        if object_count as f64 > (object_capacity as f64 * 0.75) {
            let new_cap = object_capacity * 2;
            let slot = memory.object_pool.get_slot_mut(self.id);
            unsafe { Self::resize_instance_object(&mut slot.data.object, new_cap); }
        }

        let (object_capacity, object_entries) = {
            let slot = memory.object_pool.get_slot(self.id);
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
                    write(entry_ptr, ObjectEntry { hash: key_hash, key: key_raw, value, flags: final_flags });
                    let slot = memory.object_pool.get_slot_mut(self.id);
                    slot.data.object.count += 1;
                    return Ok(());
                }

                if entry.hash == key_hash {
                    let mut matched = entry.key == key_raw;
                    if !matched && key_is_const {
                        let entry_key_ref = DinoRef::from_raw(entry.key);
                        if entry_key_ref.decode_type() == key_type {
                            let str1 = memory.arena.get_bytes(key.as_index());
                            let str2 = memory.arena.get_bytes(entry_key_ref.as_index());
                            if str1 == str2 {
                                matched = true;
                            }
                        }
                    }
                    if matched {
                        (*entry_ptr).value = value;
                        (*entry_ptr).flags = final_flags;
                        return Ok(());
                    }
                }
            }
        }
        Err(RuntimeError::InternalError("Object property map is full"))
    }

    #[inline(always)]
    pub fn get(&self, key: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef> {
        self.get_details(key, memory).map(|(val, _)| val)
    }

    pub fn get_details(&self, key: DinoRef, memory: &mut MemoryManager) -> Result<(DinoRef, PropertyFlags)> {
        let key_hash = DinoHash::any(key, memory);
        let key_raw = key.raw();
        let key_is_const = key.is_const();
        let key_type = key.decode_type();

        let mut current_handle = self.id;
        let mut depth = 0;

        loop {
            if depth > 100 { return Err(RuntimeError::InternalError("prototype chain too deep")); }

            let (proto, object_capacity, object_entries) = {
                let slot = memory.object_pool.get_slot(current_handle);
                (slot.proto, unsafe { slot.data.object.capacity }, unsafe { slot.data.object.entries })
            };

            unsafe {
                let cap = object_capacity as usize;
                let start_idx = (key_hash as usize) % cap;
                for i in 0..cap {
                    let idx = (start_idx + i) % cap;
                    let entry = &*object_entries.add(idx);
                    if entry.key == 0 { break; }
                    if entry.hash == key_hash {
                        if entry.key == key_raw {
                            return Ok((entry.value, PropertyFlags::from(entry.flags)));
                        }
                        if key_is_const {
                            let entry_key_ref = DinoRef::from_raw(entry.key);
                            if entry_key_ref.decode_type() == key_type {
                                let str1 = memory.arena.get_bytes(key.as_index());
                                let str2 = memory.arena.get_bytes(entry_key_ref.as_index());
                                if str1 == str2 {
                                    return Ok((entry.value, PropertyFlags::from(entry.flags)));
                                }
                            }
                        }
                    }
                }
            }
            
            if proto.is_object() {
                current_handle = proto.get_object_id();
            } else {
                let name_str = key.to_key_string(memory)?;
                return Err(RuntimeError::PropertyNotFound(name_str));
            }
            depth += 1;
        }
    }

    pub fn keys(&self, memory: &MemoryManager) -> Vec<DinoRef> {
        let slot = memory.object_pool.get_slot(self.id);
        let mut keys = Vec::new();
        unsafe {
            let object = &slot.data.object;
            let cap = object.capacity as usize;
            for i in 0..cap {
                let entry = &*object.entries.add(i);
                if entry.key != 0 {
                    keys.push(DinoRef::from_raw(entry.key));
                }
            }
        }
        keys
    }

    pub fn values(&self, memory: &MemoryManager) -> Vec<DinoRef> {
        let slot = memory.object_pool.get_slot(self.id);
        let mut values = Vec::new();
        unsafe {
            let object = &slot.data.object;
            let cap = object.capacity as usize;
            for i in 0..cap {
                let entry = &*object.entries.add(i);
                if entry.key != 0 {
                    values.push(entry.value);
                }
            }
        }
        values
    }

    #[inline(always)]
    pub fn len(&self, memory: &MemoryManager) -> u32 {
        let slot = memory.object_pool.get_slot(self.id);
        unsafe { slot.data.object.count }
    }

    #[inline(always)]
    pub fn clear(&self, memory: &mut MemoryManager) {
        let slot = memory.object_pool.get_slot_mut(self.id);
        unsafe {
            let object = &mut slot.data.object;
            let cap = object.capacity as usize;
            write_bytes(object.entries, 0, cap);
            object.count = 0;
        }
    }

    pub fn delete(&self, key: DinoRef, memory: &mut MemoryManager) -> bool {
        let key_hash = DinoHash::any(key, memory);
        let key_raw = key.raw();
        let key_type = key.decode_type();
        let key_is_const = key.is_const();
        
        let (object_capacity, object_entries) = {
            let slot = memory.object_pool.get_slot(self.id);
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
                            let str1 = memory.arena.get_bytes(key.as_index());
                            let str2 = memory.arena.get_bytes(entry_key_ref.as_index());
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
                        let slot = memory.object_pool.get_slot_mut(self.id);
                        slot.data.object.count -= 1;
                        return true;
                    }
                }
            }
        }
        
        false
    }

    pub fn call(&self, args_start: usize, args_count: usize, runtime: &mut Runtime) -> Result<DinoRef> {
        let args_start = args_start - 1;
        let args_count = args_count + 1;
        let (symbol, result) = if self.is_class {
            let raw_ref = self.as_class_ref();
            let instance = ObjectView::new_from_slice(raw_ref, &[], PROP_NORMAL, &mut runtime.memory)?;
            let instance_ref = instance.as_ref();
            unsafe { runtime.memory.stack.set_unchecked(args_start, instance_ref); }
            (Symbol::NEW, instance_ref)
        } else {
            (Symbol::CALL, DinoRef::NONE)
        };

        if let Ok(method_ref) = self.get(symbol, &mut runtime.memory) {
            let method = method_ref.as_function_unchecked();
            method.virtual_call(args_start, args_count, runtime)
        } else {
            runtime.memory.stack.move_sp(args_start);
            unsafe { runtime.memory.stack.push_unchecked(result); }
            Ok(result)
        }
    }

    pub fn get_attr(&self, args_start: usize, runtime: &mut Runtime) -> Result<DinoRef> {
        let raw_ref = unsafe { runtime.memory.stack.get_unchecked(args_start) };
        let key = unsafe { runtime.memory.stack.get_unchecked(args_start + 1) };
        let (value, flags) = self.get_details(key, &mut runtime.memory)?;
        
        if flags.is_getter() {
            let func = value.as_function_unchecked();
            return func.virtual_call(args_start, 1, runtime);
        }

        let value = if flags.is_from_prototype() && value.is_function() && !self.is_class {
            let bound_fn = BoundFunctionView::new_from_slice(value, &[raw_ref], &mut runtime.memory)?;
            bound_fn.as_ref()
        } else {
            value
        };
        
        unsafe {
            runtime.memory.stack.move_sp(args_start);
            runtime.memory.stack.push_unchecked(value);
        }
        Ok(value)
    }
}
