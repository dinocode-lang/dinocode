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
        heap::pool::allocator::ObjectPool,
        stack::{
            Stack,
            call_frames::CallStack,
        },
        heap::arena::Arena,
    },
    types::UserFunction,
    native::{
        get_native_registry,
        call_native_function,
    },
    runtime::context::Runtime,
};

#[derive(Debug)]
pub struct MemoryManager {
    pub arena: Arena,
    pub(crate) object_pool: ObjectPool,
    pub stack: Stack,
    pub call_stack: CallStack,
        
    pub(crate) arena_bytes_since_gc: usize,
    pub(crate) pool_allocs_since_gc: usize,
    pub(crate) arena_gc_threshold: usize,
    pub(crate) pool_gc_threshold: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        let mut manager = Self {
            arena: Arena::new(),
            object_pool: ObjectPool::new(8192),
            stack: Stack::new(8192),
            call_stack: CallStack::new(1024, 1000),
                        
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
        
        // Native class constructors don't need these
        let functions: Vec<UserFunction> = Vec::new();
        let mut ip: usize = 0;
        let mut runtime = Runtime::new(self, &functions, &mut ip);
        
        for &function_id in bootstrap_list {            
            let result = match call_native_function(&mut runtime, function_id, 0, 0) {
                Ok(class_ref) => {
                    #[cfg(feature = "logging")]
                    {
                        let name = registry.get_name_by_id(function_id).unwrap_or("<unknown>");
                        log::debug!("      Loaded class: {} (ID: {}) -> stack[{}]", name, function_id, runtime.memory.stack.depth());
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
            
            runtime.memory.stack.push(result);
        }
        
        self.arena.set_globals();
        
        #[cfg(feature = "logging")]
        log::info!("Bootstrap complete. {} items loaded into global stack.", bootstrap_list.len());
    }
}

impl Drop for MemoryManager {
    fn drop(&mut self) {
    }
}
