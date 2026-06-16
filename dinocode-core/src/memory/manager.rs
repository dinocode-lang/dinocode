// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/manager.rs
//  Desc:       MemoryManager structure
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        types::CallFrame,
        heap::pool::allocator::ObjectPool,
    },
    types::DinoRef,
    utils::StringInterner,
    native::{
        get_native_registry,
        call_native_function,
    },
};

#[derive(Debug)]
pub struct MemoryManager {
    pub(crate) arena: Vec<u8>,
    pub(crate) object_pool: ObjectPool,
    pub(crate) string_interner: StringInterner,
    pub(crate) globals_start: usize,
    
    pub(crate) stack_ptr: *mut DinoRef,
    pub(crate) stack_sp: *mut DinoRef,
    pub(crate) stack_capacity: usize,
    pub(crate) stack_start_ptr: *mut DinoRef,
    
    pub(crate) call_stack: Vec<CallFrame>,
    pub(crate) bp: usize,
    pub(crate) recursion_depth: usize,
    pub(crate) max_recursion_depth: usize,
        
    pub(crate) arena_bytes_since_gc: usize,
    pub(crate) pool_allocs_since_gc: usize,
    pub(crate) arena_gc_threshold: usize,
    pub(crate) pool_gc_threshold: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        let stack_size = 8192;
        let layout = std::alloc::Layout::array::<DinoRef>(stack_size)
            .expect("Stack capacity overflow");
        let stack_ptr = unsafe { std::alloc::alloc(layout) } as *mut DinoRef;
        
        let mut manager = Self {
            arena: Vec::with_capacity(1024 * 1024),
            object_pool: ObjectPool::new(8192),
            string_interner: StringInterner::new(),
            globals_start: 0,
            
            stack_ptr,
            stack_sp: stack_ptr,
            stack_capacity: stack_size,
            stack_start_ptr: stack_ptr,
            
            call_stack: Vec::with_capacity(1024),
            bp: 0,
            recursion_depth: 0,
            max_recursion_depth: 1000,
                        
            arena_bytes_since_gc: 0,
            pool_allocs_since_gc: 0,
            arena_gc_threshold: 16 * 1024 * 1024,
            pool_gc_threshold: 1000,
        };
        
        manager.bootstrap();
        
        manager
    }
    
    pub fn bootstrap(&mut self) {
        let registry = get_native_registry();
        let bootstrap_list = registry.get_bootstrap_list();
                
        for &function_id in bootstrap_list {            
            let result = match call_native_function(self, function_id, 0, 0) {
                Ok(class_ref) => {
                    #[cfg(feature = "logging")]
                    {
                        let name = registry.get_name_by_id(function_id).unwrap_or("<unknown>");
                        log::debug!("      Loaded class: {} (ID: {}) -> stack[{}]", name, function_id, self.stack_depth());
                    }
                    class_ref
                }
                Err(_e) => {
                    #[cfg(feature = "logging")]
                    {
                        let name = registry.get_name_by_id(function_id).unwrap_or("<unknown>");
                        log::error!("  Failed to bootstrap class {}: {:?}", name, _e);
                    }
                    continue;
                }
            };
            
            self.stack_push(result);
        }
        
        self.set_globals();
        
        #[cfg(feature = "logging")]
        log::info!("Bootstrap complete. {} items loaded into global stack.", bootstrap_list.len());
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
        if self.stack_capacity > 0 {
            let layout = std::alloc::Layout::array::<DinoRef>(self.stack_capacity)
                .expect("Stack capacity overflow");
            unsafe { std::alloc::dealloc(self.stack_ptr as *mut u8, layout); }
        }
    }
}
