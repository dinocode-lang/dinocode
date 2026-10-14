// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/stack/call_frames.rs
//  Desc:       Call frame management for function invocations
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════
use crate::{
    memory::{
        types::CallFrame,
        stack::Stack,
    },
    types::{
        DinoRef,
        UserFunction,
    },
    errors::{RuntimeError, Result},
};
use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
pub struct CallStack {
    ptr: *mut CallFrame,
    sp: *mut CallFrame,
    capacity: usize,
    max_recursion_depth: usize,
}

impl CallStack {
    pub fn new(capacity: usize, max_recursion_depth: usize) -> Self {
        let layout = Layout::array::<CallFrame>(capacity)
            .expect("CallStack capacity overflow");
        let ptr = unsafe { alloc(layout) } as *mut CallFrame;

        Self {
            ptr,
            sp: ptr,
            capacity,
            max_recursion_depth,
        }
    }
    
    #[inline(always)]
    pub fn len(&self) -> usize {
        unsafe { self.sp.offset_from(self.ptr) as usize }
    }
    
    pub fn get_unchecked(&self, index: usize) -> CallFrame {
        unsafe { std::ptr::read(self.ptr.add(index)) }
    }
    
    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, frame: CallFrame) {
        unsafe {
            std::ptr::write(self.sp, frame);
            self.sp = self.sp.add(1);
        }
    }
    
    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> CallFrame {
        unsafe {
            self.sp = self.sp.sub(1);
            std::ptr::read(self.sp)
        }
    }
    
    pub fn push(
        &mut self,
        return_address: u32,
        function_id: u32,
        function: &UserFunction,
        args_start: usize,
        argc: usize,
        fn_slot: u8,
        stack: &mut Stack,
    ) -> Result<()> {
        if self.len() >= self.max_recursion_depth {
            return Err(RuntimeError::RecursionLimitExceeded);
        }

        let actual_args = argc.min(function.param_count as usize);

        let frame = CallFrame {
            return_address,
            old_bp: stack.bp,
            function_id,
            args_start,
            args_count: actual_args,
            return_count: function.return_count,
            fn_slot,
        };
        
        unsafe { self.push_unchecked(frame); }
        
        stack.bp = args_start;
        
        let new_sp = stack.bp + actual_args + function.local_count as usize;
        if new_sp > stack.capacity {
            stack.grow(new_sp.next_power_of_two());
        }
        
        let current_depth = stack.depth();
        let locals_start = stack.bp + actual_args;
        
        if new_sp > current_depth {
            let write_start = current_depth.max(locals_start);
            let count = new_sp - write_start;
            
            unsafe {
                let ptr = stack.ptr.add(write_start);
                for i in 0..count {
                    std::ptr::write(ptr.add(i), DinoRef::NONE);
                }
            }
        }
        
        unsafe { stack.sp = stack.ptr.add(new_sp); }

        Ok(())
    }
    
    pub fn pop(&mut self, stack: &mut Stack) -> u32 {
        let frame = unsafe { self.pop_unchecked() };
        unsafe {
            stack.bp = frame.old_bp;
            stack.sp = stack.ptr.add(frame.args_start - frame.fn_slot as usize);
            stack.push_unchecked(DinoRef::NONE);
        }
        frame.return_address
    }
    
    pub fn pop_with_ref(&mut self, stack: &mut Stack) -> u32 {
        let frame = unsafe { self.pop_unchecked() };
        unsafe {
            let return_value = stack.pop_unchecked();
            stack.bp = frame.old_bp;
            stack.sp = stack.ptr.add(frame.args_start - frame.fn_slot as usize);
            stack.push_unchecked(return_value);
        }
        frame.return_address
    }
    
    pub fn pop_with_self(&mut self, stack: &mut Stack) -> u32 {
        let frame = unsafe { self.pop_unchecked() };
        unsafe {
            let self_value = stack.get_local_unchecked(0);
            stack.bp = frame.old_bp;
            stack.sp = stack.ptr.add(frame.args_start - frame.fn_slot as usize);
            stack.push_unchecked(self_value);
        }
        frame.return_address
    }
}

impl Drop for CallStack {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<CallFrame>(self.capacity)
                .expect("CallStack capacity overflow");
            unsafe { dealloc(self.ptr as *mut u8, layout); }
        }
    }
}
