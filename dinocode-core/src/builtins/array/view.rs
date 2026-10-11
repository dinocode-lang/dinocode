// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/array/view.rs
//  Desc:       Wrapper for array memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        manager::MemoryManager,
        types::pool_types::NativeArray,
    },
    types::{DinoRef, value_type},
    errors::{Result, RuntimeError},
    builtins::array::prototype::Array,
};
use std::{
    ptr::{write, read},
    alloc::{alloc, Layout, realloc}
};

#[derive(Clone, Copy)]
pub struct ArrayView(pub u32);

impl ArrayView {
    #[inline(always)]
    pub fn as_id(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        DinoRef::array(self.0)
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_array() {
            return Err(RuntimeError::ExpectedInstance("array"));
        }
        Ok(Self(dinoref.decode_index()))
    }

    #[inline(always)]
    pub fn from_ref_unchecked(dinoref: DinoRef) -> Self {
        Self(dinoref.decode_index())
    }

    pub fn new_from_stack(size: usize, memory: &mut MemoryManager) -> Result<Self> {
        let depth = memory.stack.depth();
        let start = depth - size;
        let elements = unsafe { std::slice::from_raw_parts(memory.stack.ptr.add(start), size) };
        Self::new_from_slice(elements, memory)
    }

    pub fn new_from_slice(elements: &[DinoRef], memory: &mut MemoryManager) -> Result<Self> {
        memory.record_pool_alloc();
        let handle = memory.object_pool.alloc_slot().expect("Pool full");
        let cap = (elements.len() as u32).max(4);

        let layout = Layout::array::<DinoRef>(cap as usize).unwrap();
        let ptr = unsafe { alloc(layout) } as *mut DinoRef;

        unsafe {
            for (i, val) in elements.iter().enumerate() {
                write(ptr.add(i), *val);
            }
        }

        let proto = Array::as_ref(memory);
        let slot = memory.object_pool.get_slot_mut(handle);
        slot.kind = value_type::ARRAY;
        slot.proto = proto;
        slot.data.array = NativeArray {
            capacity: cap,
            count: elements.len() as u32,
            elements: ptr,
        };

        Ok(Self(handle))
    }

    #[inline(always)]
    pub fn len(&self, memory: &MemoryManager) -> u32 {
        let slot = memory.object_pool.get_slot(self.0);
        unsafe { slot.data.array.count }
    }

    #[inline(always)]
    pub fn get(&self, index: u32, memory: &MemoryManager) -> Result<DinoRef> {
        let slot = memory.object_pool.get_slot(self.0);
        unsafe {
            let array = &slot.data.array;
            if index >= array.count {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            Ok(read(array.elements.add(index as usize)))
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: u32, memory: &MemoryManager) -> DinoRef {
        let slot = memory.object_pool.get_slot(self.0);
        unsafe {
            let array = &slot.data.array;
            read(array.elements.add(index as usize))
        }
    }

    pub fn set(&self, index: u32, value: DinoRef, memory: &mut MemoryManager) -> Result<()> {
        let slot = memory.object_pool.get_slot_mut(self.0);

        let cap_needed = index + 1;
        let (current_cap, current_ptr) = unsafe {
            let array = &slot.data.array;
            (array.capacity, array.elements)
        };

        if cap_needed > current_cap {
            let new_cap = cap_needed.next_power_of_two().max(current_cap * 2);
            let old_layout = Layout::array::<DinoRef>(current_cap as usize).unwrap();
            let new_layout = Layout::array::<DinoRef>(new_cap as usize).unwrap();

            unsafe {
                let new_ptr = realloc(current_ptr as *mut u8, old_layout, new_layout.size()) as *mut DinoRef;
                if new_ptr.is_null() {
                    return Err(RuntimeError::InternalError("Out of memory"));
                }
                let slot = memory.object_pool.get_slot_mut(self.0);
                slot.data.array.elements = new_ptr;
                slot.data.array.capacity = new_cap;
            }
        }

        let slot = memory.object_pool.get_slot_mut(self.0);
        unsafe {
            let array = &mut slot.data.array;
            write(array.elements.add(index as usize), value);
            if index >= array.count {
                array.count = index + 1;
            }
        }

        Ok(())
    }

    #[inline(always)]
    pub fn push(&self, value: DinoRef, memory: &mut MemoryManager) -> Result<u32> {
        let slot = memory.object_pool.get_slot(self.0);
        let len = unsafe { slot.data.array.count };
        self.set(len, value, memory)?;
        Ok(len + 1)
    }

    #[inline(always)]
    pub fn pop(&self, memory: &mut MemoryManager) -> Result<DinoRef> {
        let slot = memory.object_pool.get_slot_mut(self.0);
        unsafe {
            let array = &mut slot.data.array;
            if array.count == 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            array.count -= 1;
            Ok(read(array.elements.add(array.count as usize)))
        }
    }

    #[inline(always)]
    pub fn clear(&self, memory: &mut MemoryManager) -> Result<()> {
        let slot = memory.object_pool.get_slot_mut(self.0);
        slot.data.array.count = 0;
        Ok(())
    }

    #[inline(always)]
    pub fn is_empty(&self, memory: &MemoryManager) -> bool {
        self.len(memory) == 0
    }

    #[inline(always)]
    pub fn first(&self, memory: &MemoryManager) -> Result<DinoRef> {
        let slot = memory.object_pool.get_slot(self.0);
        unsafe {
            let array = &slot.data.array;
            if array.count == 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            Ok(read(array.elements))
        }
    }

    #[inline(always)]
    pub fn last(&self, memory: &MemoryManager) -> Result<DinoRef> {
        let slot = memory.object_pool.get_slot(self.0);
        unsafe {
            let array = &slot.data.array;
            if array.count == 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            Ok(read(array.elements.add((array.count - 1) as usize)))
        }
    }

    pub fn contains(&self, value: DinoRef, memory: &MemoryManager) -> bool {
        let slot = memory.object_pool.get_slot(self.0);
        unsafe {
            let array = &slot.data.array;
            for i in 0..array.count {
                let element = read(array.elements.add(i as usize));
                if element == value {
                    return true;
                }
            }
        }
        false
    }
}
