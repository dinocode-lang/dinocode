// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/heap/pool/allocator.rs
//  Desc:       Object Pool allocator with Bitmap GC
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::alloc::{alloc, dealloc, Layout};
use super::bitmap::Bitmap;
use crate::{
    memory::types::{
        pool_types::{NativeArray, NativeObject, ObjectEntry, PoolSlot},
        object_types,
    },
    types::{
        DinoRef,
        dinoref::value_type,
    },
};

#[derive(Debug)]
pub struct ObjectPool {
    slots: *mut PoolSlot,
    pub capacity: usize,
    pub bitmap: Bitmap,
    pub mark_bitmap: Bitmap,
}

impl ObjectPool {
    pub fn new(initial_capacity: usize) -> Self {
        let layout = Layout::array::<PoolSlot>(initial_capacity).unwrap();
        let slots = unsafe { alloc(layout) } as *mut PoolSlot;
        
        Self {
            slots,
            capacity: initial_capacity,
            bitmap: Bitmap::new(initial_capacity),
            mark_bitmap: Bitmap::new(initial_capacity),
        }
    }

    pub fn alloc_slot(&mut self) -> Option<u32> {
        if let Some(index) = self.bitmap.find_first_free() {
            if index < self.capacity {
                self.bitmap.set(index);
                return Some(index as u32);
            }
        }
        
        None
    }
    
    pub fn grow(&mut self) {
        let new_capacity = self.capacity * 2;
        let layout = Layout::array::<PoolSlot>(new_capacity).unwrap();
        let new_slots = unsafe { alloc(layout) } as *mut PoolSlot;
        
        if self.capacity > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(self.slots, new_slots, self.capacity);
                let old_layout = Layout::array::<PoolSlot>(self.capacity).unwrap();
                dealloc(self.slots as *mut u8, old_layout);
            }
        }
        
        self.slots = new_slots;
        self.bitmap.resize(new_capacity);
        self.mark_bitmap.resize(new_capacity);
        self.capacity = new_capacity;
    }
    
    pub fn sweep(&mut self) {
        for i in 0..self.capacity {
            if self.bitmap.get(i) && !self.mark_bitmap.get(i) {
                self.free_slot(i as u32);
            }
        }
        // Clear marks for next cycle
        self.mark_bitmap.clear_all();
    }

    #[inline(always)]
    pub fn get_slot(&self, index: u32) -> &PoolSlot {
        unsafe { &*self.slots.add(index as usize) }
    }

    #[inline(always)]
    pub fn get_slot_mut(&mut self, index: u32) -> &mut PoolSlot {
        unsafe { &mut *self.slots.add(index as usize) }
    }
    
    pub fn set_array(&mut self, index: u32, array: NativeArray) {
        let slot = self.get_slot_mut(index);
        slot.subkind = object_types::DEFAULT;
        slot.data.array = array;
    }
    
    pub fn set_object(&mut self, index: u32, object: NativeObject) {
        let slot = self.get_slot_mut(index);
        slot.subkind = object_types::DEFAULT;
        slot.data.object = object;
    }
    
    pub fn free_slot(&mut self, index: u32) {
        if !self.bitmap.get(index as usize) { return; }
        
        let slot = self.get_slot_mut(index);
        match slot.kind {
            value_type::ARRAY => {
                let array = unsafe { &mut slot.data.array };
                if array.capacity > 0 && !array.elements.is_null() {
                     let layout = Layout::array::<DinoRef>(array.capacity as usize).unwrap();
                     unsafe { dealloc(array.elements as *mut u8, layout); }
                }
            },
            value_type::OBJECT => {
                let object = unsafe { &mut slot.data.object };
                if object.capacity > 0 && !object.entries.is_null() {
                     let layout = Layout::array::<ObjectEntry>(object.capacity as usize).unwrap();
                     unsafe { dealloc(object.entries as *mut u8, layout); }
                }
            },
            _ => {}
        }
        
        self.bitmap.clear(index as usize);
    }
}

impl Drop for ObjectPool {
    fn drop(&mut self) {
        // Free all allocated slots first
        for i in 0..self.capacity {
            if self.bitmap.get(i) {
                self.free_slot(i as u32);
            }
        }
    
        if self.capacity > 0 {
            let layout = Layout::array::<PoolSlot>(self.capacity).unwrap();
            unsafe { dealloc(self.slots as *mut u8, layout); }
        }
    }
}
