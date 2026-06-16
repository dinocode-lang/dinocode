// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/stack/operations.rs
//  Desc:       Stack operations for MemoryManager
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::DinoRef,
};
use std::alloc::{alloc, dealloc, Layout};
    
impl MemoryManager {
    #[inline(always)]
    fn stack_has_space(&self, additional: usize) -> bool {
        let used = unsafe { self.stack_sp.offset_from(self.stack_ptr) } as usize;
        used + additional <= self.stack_capacity
    }

    pub(crate) fn stack_grow(&mut self, new_capacity: usize) {
        let new_layout = Layout::array::<DinoRef>(new_capacity)
            .expect("Stack capacity overflow");
        let new_ptr = unsafe { alloc(new_layout) } as *mut DinoRef;
        
        let used = unsafe { self.stack_sp.offset_from(self.stack_ptr) } as usize;
        if used > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(self.stack_ptr, new_ptr, used);
            }
        }
        
        if self.stack_capacity > 0 {
            let old_layout = Layout::array::<DinoRef>(self.stack_capacity)
                .expect("Stack capacity overflow");
            unsafe { dealloc(self.stack_ptr as *mut u8, old_layout); }
        }
        
        self.stack_ptr = new_ptr;
        self.stack_sp = unsafe { new_ptr.add(used) };
        self.stack_start_ptr = new_ptr;
        self.stack_capacity = new_capacity;
    }

    pub fn stack_push(&mut self, value: DinoRef) {
        if !self.stack_has_space(1) {
            let new_capacity = (self.stack_capacity * 2).max(256);
            self.stack_grow(new_capacity);
        }
        
        unsafe {
            std::ptr::write(self.stack_sp, value);
            self.stack_sp = self.stack_sp.add(1);
        }
    }

    pub fn stack_pop(&mut self) -> Option<DinoRef> {
        if self.stack_depth() == 0 {
            return None;
        }
        unsafe {
            self.stack_sp = self.stack_sp.sub(1);
            Some(std::ptr::read(self.stack_sp))
        }
    }

    #[inline(always)]
    pub fn stack_depth(&self) -> usize {
        unsafe { self.stack_sp.offset_from(self.stack_ptr) as usize }
    }
    
    pub fn stack_extend_from_slice(&mut self, values: &[DinoRef]) {
        for &value in values {
            self.stack_push(value);
        }
    }

    pub fn stack(&self) -> &[DinoRef] {
        let depth = self.stack_depth();
        unsafe { std::slice::from_raw_parts(self.stack_ptr, depth) }
    }

    pub fn move_sp(&mut self, new_sp: usize) {
        unsafe { self.stack_sp = self.stack_ptr.add(new_sp); }
    }

    pub fn stack_pop_n(&mut self, count: usize) -> bool {
        let current_depth = self.stack_depth();
        if count > current_depth {
            return false;
        }
        unsafe {
            self.stack_sp = self.stack_sp.sub(count);
        }
        true
    }

    pub fn stack_peek(&self, offset_from_top: usize) -> Option<DinoRef> {
        let current_depth = self.stack_depth();
        if offset_from_top >= current_depth {
            return None;
        }
        unsafe {
            let ptr = self.stack_sp.sub(offset_from_top + 1);
            Some(std::ptr::read(ptr))
        }
    }

    pub fn stack_peek_top(&self) -> Option<DinoRef> {
        self.stack_peek(0)
    }

    pub fn stack_get(&self, index: usize) -> Option<DinoRef> {
        unsafe {
            let ptr = self.stack_ptr.add(index);
            Some(std::ptr::read(ptr))
        }
    }

    pub fn stack_pop_n_with_result(&mut self, count: usize) -> Option<DinoRef> {
        let current_depth = self.stack_depth();
        if count > current_depth {
            return None;
        }
        unsafe {
            let result_ptr = self.stack_sp.sub(count);
            let result = std::ptr::read(result_ptr);
            self.stack_sp = result_ptr;
            Some(result)
        }
    }

    pub fn stack_insert(&mut self, index: usize, value: DinoRef) {
        let depth = self.stack_depth();
        if !self.stack_has_space(1) {
            let new_capacity = (self.stack_capacity * 2).max(256);
            self.stack_grow(new_capacity);
        }
        
        unsafe {
            let src = self.stack_ptr.add(index);
            let dst = src.add(1);
            let count = depth - index;
            if count > 0 {
                std::ptr::copy(src, dst, count);
            }
            std::ptr::write(src, value);
            self.stack_sp = self.stack_sp.add(1);
        }
    }

    pub unsafe fn stack_set_unchecked(&mut self, index: usize, value: DinoRef) {
        unsafe {
            let ptr = self.stack_ptr.add(index);
            std::ptr::write(ptr, value);
        }
    }

    pub fn stack_set(&mut self, index: usize, value: DinoRef) -> bool {
        let depth = self.stack_depth();
        if index >= depth {
            return false;
        }
        unsafe {
            let ptr = self.stack_ptr.add(index);
            std::ptr::write(ptr, value);
        }
        true
    }
}
