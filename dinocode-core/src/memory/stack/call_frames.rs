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
        MemoryManager,
        types::CallFrame,
    },
    types::{
        DinoRef,
        UserFunction,
    },
    errors::{RuntimeError, Result},
};

impl MemoryManager {
    #[inline(always)]
    pub fn call_stack(&self) -> &Vec<CallFrame> {
        &self.call_stack
    }
    
    #[inline(always)]
    pub fn push_call_frame(
        &mut self,
        return_address: u32,
        function_id: u32,
        function: &UserFunction,
        args_start: usize,
        argc: usize,
        fn_slot: u8,
    ) -> Result<()> {
        if self.recursion_depth >= self.max_recursion_depth {
            return Err(RuntimeError::RecursionLimitExceeded);
        }

        let actual_args = argc.min(function.param_count as usize);

        let frame = CallFrame {
            return_address,
            old_bp: self.stack.bp,
            function_id,
            args_start,
            args_count: actual_args,
            return_count: function.return_count,
            fn_slot,
        };

        self.call_stack.push(frame);
        self.recursion_depth += 1;
        
        self.stack.bp = args_start;
        
        let new_sp = self.stack.bp + actual_args + function.local_count as usize;
        if new_sp > self.stack.capacity {
            self.stack.grow(new_sp.next_power_of_two());
        }
        
        let current_depth = self.stack.depth();
        let locals_start = self.stack.bp + actual_args;
        
        if new_sp > current_depth {
            let write_start = current_depth.max(locals_start);
            let count = new_sp - write_start;
            
            unsafe {
                let ptr = self.stack.ptr.add(write_start);
                for i in 0..count {
                    std::ptr::write(ptr.add(i), DinoRef::NONE);
                }
            }
        }
        
        unsafe { self.stack.sp = self.stack.ptr.add(new_sp); }

        Ok(())
    }

    #[inline(always)]
    pub fn pop_call_frame(&mut self) -> Option<u32> {
        if let Some(frame) = self.call_stack.pop() {
            if self.recursion_depth > 0 {
                self.recursion_depth -= 1;
            }
            
            self.stack.bp = frame.old_bp;
            unsafe { self.stack.sp = self.stack.ptr.add(frame.args_start - frame.fn_slot as usize); }

            self.stack.push(DinoRef::NONE);
        
            Some(frame.return_address)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn pop_call_frame_with_ref(&mut self) -> Option<u32> {
        if let Some(frame) = self.call_stack.pop() {
            if self.recursion_depth > 0 {
                self.recursion_depth -= 1;
            }
            
            let return_value = self.stack.pop().unwrap_or(DinoRef::NONE);

            self.stack.bp = frame.old_bp;
            
            unsafe { self.stack.sp = self.stack.ptr.add(frame.args_start - frame.fn_slot as usize); }
            
            self.stack.push(return_value);
            
            Some(frame.return_address)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn pop_call_frame_with_self(&mut self) -> Option<u32> {
        if let Some(frame) = self.call_stack.pop() {
            if self.recursion_depth > 0 {
                self.recursion_depth -= 1;
            }
            
            let self_value = unsafe { self.stack.get_local_unchecked(0) };

            self.stack.bp = frame.old_bp;
            
            unsafe { self.stack.sp = self.stack.ptr.add(frame.args_start - frame.fn_slot as usize); }
            
            self.stack.push(self_value);
            
            Some(frame.return_address)
        } else {
            None
        }
    }
}
