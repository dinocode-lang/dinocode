// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/gc/mark_sweep.rs
//  Desc:       Mark & Sweep GC (Pool only)
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::{
        DinoRef,
        dinoref::value_type,
    },
};

impl MemoryManager {
    pub fn mark_and_sweep(&mut self) {
        self.mark_pool_roots();
        self.object_pool.sweep();
        self.compact_and_evacuate_heap();

        #[cfg(feature = "logging")]
        log::debug!("GC completed: arena {} → {} bytes", 
                 self.arena.len(), self.arena.len());
    }

    fn mark_pool_roots(&mut self) {
        let depth = self.stack_depth();
        for i in 0..depth {
             let val = unsafe { *self.stack_ptr.add(i) };
             self.mark_value(val);
        }
    }

    fn mark_value(&mut self, value: DinoRef) {
        match value.decode_type() {
            value_type::ARRAY => {
                let handle = value.decode_index();
                if self.object_pool.bitmap.get(handle as usize)
                && !self.object_pool.mark_bitmap.get(handle as usize) {
                    self.object_pool.mark_bitmap.set(handle as usize);
                    self.mark_array_children(handle);
                }
            },

            value_type::OBJECT => {
                let handle = value.get_object_id();

                if self.object_pool.bitmap.get(handle as usize) && !self.object_pool.mark_bitmap.get(handle as usize) {
                    self.object_pool.mark_bitmap.set(handle as usize);
                    self.mark_object_children(handle);
                }
            },
            _ => { }
        }
    }

    fn mark_array_children(&mut self, handle: u32) {
        let len = self.get_array_len(handle);
        for i in 0..len {
            let elem = self.get_array_element(handle, i);
            self.mark_value(elem);
        }
    }

    fn mark_object_children(&mut self, handle: u32) {
        let slot = self.object_pool.get_slot(handle);
        if slot.kind != value_type::OBJECT { return; }

        unsafe {
             let object = &slot.data.object;
             let cap = object.capacity;
             let entries = object.entries;

             for i in 0..cap {
                 let entry = &*entries.add(i as usize);
                 if entry.key != 0 {
                     self.mark_value(entry.value);
                 }
             }
        }
    }
}
