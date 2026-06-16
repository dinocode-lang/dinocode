// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/gc/pressure.rs
//  Desc:       Pressure tracking for arena and pool allocations
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::memory::MemoryManager;
impl MemoryManager {
    #[inline(always)]
    pub(crate) fn record_arena_alloc(&mut self, bytes: usize) -> bool {
        self.arena_bytes_since_gc += bytes;
        
        if self.should_trigger_gc() {
            self.trigger_gc();
            true
        } else {
            false
        }
    }
    
    #[inline(always)]
    pub(crate) fn record_pool_alloc(&mut self) -> bool {
        self.pool_allocs_since_gc += 1;
        
        if self.should_trigger_gc() {
            self.trigger_gc();
            true
        } else {
            false
        }
    }
    
    #[inline(always)]
    fn should_trigger_gc(&self) -> bool {
        if self.arena_bytes_since_gc >= self.arena_gc_threshold {
            return true;
        }
        if self.pool_allocs_since_gc >= self.pool_gc_threshold {
            return true;
        }
        
        let arena_pressure = self.arena_bytes_since_gc as f64 / self.arena_gc_threshold as f64;
        let pool_pressure = self.pool_allocs_since_gc as f64 / self.pool_gc_threshold as f64;
        
        arena_pressure > 0.5 && pool_pressure > 0.5
    }
    
    #[inline(never)]
    pub(crate) fn trigger_gc(&mut self) {
        #[cfg(feature = "logging")]
        log::debug!("GC triggered: arena={} bytes, pool={} allocs", 
                 self.arena_bytes_since_gc, self.pool_allocs_since_gc);
        
        self.mark_and_sweep();
        self.reset_gc_pressure();
    }
    
    #[inline(always)]
    fn reset_gc_pressure(&mut self) {
        self.arena_bytes_since_gc = 0;
        self.pool_allocs_since_gc = 0;
    }
    
    /// Public to manually trigger GC if needed
    pub fn trigger_gc_if_needed(&mut self) {
        if self.should_trigger_gc() {
            self.trigger_gc();
        }
    }
}
