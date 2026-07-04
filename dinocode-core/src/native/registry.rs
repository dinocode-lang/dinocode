// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/native/registry.rs
//  Desc:       Global registry for native functions and classes
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::collections::HashMap;
use crate::{
    types::DinoRef,
    memory::MemoryManager,
    errors::{RuntimeError, Result},
};

pub type NativeFnWrapper = fn(&mut MemoryManager, usize, usize) -> Result<DinoRef>;

pub struct NativeFunctionRegistry {
    functions: Vec<NativeFnWrapper>,
    flags: Vec<u8>,
    bootstrap_required: Vec<u32>,
    next_id: u32,
    name_to_id: Option<HashMap<String, u32>>,
    class_name_to_global_index: Option<HashMap<String, u32>>,
}

impl NativeFunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: Vec::with_capacity(64),
            flags: Vec::with_capacity(64),
            bootstrap_required: Vec::new(),
            next_id: 0,
            name_to_id: Some(HashMap::with_capacity(64)),
            class_name_to_global_index: Some(HashMap::with_capacity(32)),
        }
    }

    pub fn register(&mut self, name: &str, function: NativeFnWrapper) -> u32 {
        self.register_with_flags(name, function, 0)
    }

    pub fn register_with_flags(&mut self, name: &str, function: NativeFnWrapper, flags: u8) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.functions.push(function);
        self.flags.push(flags);

        if flags == 0 {
            if let Some(ref mut map) = self.name_to_id {
                map.insert(name.to_string(), id);
            }
        }

        if (flags & 2) != 0 {
            let global_index = self.bootstrap_required.len() as u32;
            self.bootstrap_required.push(id);

            if let Some(ref mut map) = self.class_name_to_global_index {
                map.insert(name.to_string(), global_index);
            }
        }

        id
    }

    pub fn get_by_id(&self, id: u32) -> Option<NativeFnWrapper> {
        self.functions.get(id as usize).copied()
    }

    pub fn get_flag_by_id(&self, id: u32) -> Option<u8> {
        self.flags.get(id as usize).copied()
    }

    pub fn bootstrap_count(&self) -> u32 {
        self.bootstrap_required.len() as u32
    }

    pub fn get_bootstrap_list(&self) -> &[u32] {
        &self.bootstrap_required
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<u32> {
        let map = self.name_to_id.as_ref()?;
        let &id = map.get(name)?;
        if let Some(&flag) = self.flags.get(id as usize) {
            if (flag & 1) == 0 {
                return Some(id);
            }
        }
        None
    }

    pub fn get_id_by_name_unchecked(&self, name: &str) -> Option<u32> {
        self.name_to_id.as_ref()?.get(name).copied()
    }

    pub fn get_name_by_id(&self, id: u32) -> Option<&str> {
        self.name_to_id.as_ref()?
            .iter()
            .find(|(_, func_id)| **func_id == id)
            .map(|(name, _)| name.as_str())
    }

    pub fn list_functions(&self) -> Vec<&str> {
        self.name_to_id
            .as_ref()
            .map(|map| map.keys().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn is_native_function(&self, name: &str) -> Option<u32> {
        self.get_id_by_name(name)
    }

    pub fn get_bootstrap_global_index(&self, class_name: &str) -> Option<u32> {
        self.class_name_to_global_index.as_ref()?.get(class_name).copied()
    }

    pub fn free_info(&mut self) {
        self.name_to_id = None;
        self.class_name_to_global_index = None;
    }
}

impl Default for NativeFunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

static mut NATIVE_REGISTRY: Option<NativeFunctionRegistry> = None;
static REGISTRY_INIT: std::sync::Once = std::sync::Once::new();

pub fn get_native_registry() -> &'static mut NativeFunctionRegistry {
    unsafe {
        REGISTRY_INIT.call_once(|| {
            NATIVE_REGISTRY = Some(NativeFunctionRegistry::new());
        });
        #[allow(static_mut_refs)]
        NATIVE_REGISTRY.as_mut().unwrap()
    }
}

pub fn register_native_function(name: &str, function: NativeFnWrapper) -> u32 {
    get_native_registry().register(name, function)
}

pub fn register_native_class(
    class_name: &str,
    bootstrap_fn: NativeFnWrapper
) -> u32 {
    let registry = get_native_registry();
    let bootstrap_id = registry.register_with_flags(class_name, bootstrap_fn, 2);
    registry.get_bootstrap_list()
        .iter()
        .position(|&id| id == bootstrap_id)
        .expect("Class just registered but not found in bootstrap list") as u32
}

pub fn is_native_function(name: &str) -> Option<u32> {
    get_native_registry().is_native_function(name)
}

pub fn get_bootstrap_global_index(class_name: &str) -> Option<u32> {
    get_native_registry().get_bootstrap_global_index(class_name)
}

pub fn free_info() {
    get_native_registry().free_info();
}

pub fn call_native_function(
    memory: &mut MemoryManager,
    function_id: u32,
    args_start: usize,
    args_count: usize
) -> Result<DinoRef> {
    let registry = get_native_registry();

    if let Some(function) = registry.get_by_id(function_id) {
        function(memory, args_start, args_count)
    } else {
        let function_name = registry
            .get_name_by_id(function_id)
            .unwrap_or("<unknown>");
        Err(RuntimeError::NativeFunctionNotFound(function_name))
    }
}
