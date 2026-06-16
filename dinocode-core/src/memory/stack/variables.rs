// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/stack/variables.rs
//  Desc:       Variable access on the VM stack
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    types::DinoRef,
};
    
impl MemoryManager {
    #[inline(always)]
    pub unsafe fn get_variable_unchecked(&self, var_idx: u32) -> DinoRef {
        let index = self.bp + var_idx as usize;
        unsafe { *self.stack_ptr.add(index) }
    }

    #[inline(always)]
    pub unsafe fn set_variable_unchecked(&mut self, var_idx: u32, value: DinoRef) {
        let index = self.bp + var_idx as usize;
        unsafe { *self.stack_ptr.add(index) = value; }
    }

    #[inline(always)]
    pub unsafe fn get_global_variable_unchecked(&self, global_idx: u32) -> DinoRef {
        unsafe { *self.stack_ptr.add(global_idx as usize) }
    }

    #[inline(always)]
    pub unsafe fn set_global_variable_unchecked(&mut self, global_idx: u32, value: DinoRef) {
        unsafe { *self.stack_ptr.add(global_idx as usize) = value; }
    }

    #[inline(always)]
    pub unsafe fn get_local_variable_unchecked(&self, var_idx: u32) -> DinoRef {
        unsafe { *self.stack_ptr.add(self.bp + var_idx as usize) }
    }

    #[inline(always)]
    pub unsafe fn set_local_variable_unchecked(&mut self, var_idx: u32, value: DinoRef) {
        unsafe { *self.stack_ptr.add(self.bp + var_idx as usize) = value; }
    }
}
