// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/value_pool.rs
//  Desc:       Generic deduplication pool for compilation.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::collections::HashMap;
use super::suggestions::suggest_best_match;
use crate::{
    memory::MemoryManager,
    types::{DinoRef, opcode, UserFunction},
    native::{get_native_registry, registry::get_bootstrap_global_index},
};

#[derive(Debug, Clone)]
pub struct ValuePool {
    float_to_dinoref: HashMap<u64, DinoRef>,
    int_to_dinoref: HashMap<i64, DinoRef>,
    bigint_to_dinoref: HashMap<String, DinoRef>,
    const_pool: Vec<DinoRef>,
    dinoref_to_const_index: HashMap<DinoRef, u32>,
    var_names: Vec<String>,
    name_to_global_idx: HashMap<String, u32>,
    scope_mappings: Vec<Vec<u32>>,
    scope_global_to_local: Vec<HashMap<u32, u32>>,
    functions: Vec<UserFunction>,
    total_vars_created: u32,
}

impl Default for ValuePool {
    fn default() -> Self {
        Self::new()
    }
}

impl ValuePool {
    pub fn new() -> Self {
        Self {
            float_to_dinoref: HashMap::new(),
            int_to_dinoref: HashMap::new(),
            bigint_to_dinoref: HashMap::new(),
            const_pool: Vec::new(),
            dinoref_to_const_index: HashMap::new(),
            var_names: Vec::new(),
            name_to_global_idx: HashMap::new(),
            scope_mappings: vec![Vec::new()],
            scope_global_to_local: vec![HashMap::new()],
            functions: Vec::new(),
            total_vars_created: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            float_to_dinoref: HashMap::with_capacity(capacity / 8),
            int_to_dinoref: HashMap::with_capacity(capacity / 8),
            bigint_to_dinoref: HashMap::with_capacity(capacity / 16),
            const_pool: Vec::with_capacity(capacity / 4),
            dinoref_to_const_index: HashMap::with_capacity(capacity / 4),
            var_names: Vec::with_capacity(capacity / 4),
            name_to_global_idx: HashMap::with_capacity(capacity / 4),
            scope_mappings: vec![Vec::with_capacity(capacity / 8)],
            scope_global_to_local: vec![HashMap::with_capacity(capacity / 8)],
            functions: Vec::with_capacity(capacity / 16),
            total_vars_created: 0,
        }
    }

    pub fn get_or_create_string(&mut self, value: &str, memory_manager: &mut MemoryManager) -> u32 {
        if let Some(dinoref) = memory_manager.get_interned_string(value) {
            self.add_to_const_pool(dinoref)
        } else {
            let dinoref = memory_manager.alloc_const_string(value);
            
            let _ = memory_manager.intern_string(value, dinoref);
            self.add_to_const_pool(dinoref)
        }
    }

    pub fn get_or_create_float(&mut self, value: f64, _memory_manager: &mut MemoryManager) -> u32 {
        let key = value.to_bits();
        if let Some(&dinoref) = self.float_to_dinoref.get(&key) {
            self.add_to_const_pool(dinoref)
        } else {
            let dinoref = DinoRef::float(value);
            self.float_to_dinoref.insert(key, dinoref);
            self.add_to_const_pool(dinoref)
        }
    }

    pub fn get_or_create_int(&mut self, value: i64, _memory_manager: &mut MemoryManager) -> u32 {
        if let Some(&dinoref) = self.int_to_dinoref.get(&value) {
            self.add_to_const_pool(dinoref)
        } else {
            let dinoref = DinoRef::int(value);
            self.int_to_dinoref.insert(value, dinoref);
            self.add_to_const_pool(dinoref)
        }
    }

    pub fn get_or_create_bigint(&mut self, value: &str, memory_manager: &mut MemoryManager) -> Result<u32, String> {
        if let Some(&dinoref) = self.bigint_to_dinoref.get(value) {
            Ok(self.add_to_const_pool(dinoref))
        } else {
            let dinoref = memory_manager.alloc_bigint_str(value)?;
            self.bigint_to_dinoref.insert(value.to_string(), dinoref);
            Ok(self.add_to_const_pool(dinoref))
        }
    }

    pub fn get_or_create_native_fn(&mut self, native_id: u32) -> u32 {
        let dinoref = DinoRef::native_fn(native_id);
        self.add_to_const_pool(dinoref)
    }

    pub fn get_or_create_function(&mut self, function_id: u32) -> u32 {
        let dinoref = DinoRef::function(function_id);
        self.add_to_const_pool(dinoref)
    }

    pub fn add_to_const_pool(&mut self, dinoref: DinoRef) -> u32 {
        if let Some(&index) = self.dinoref_to_const_index.get(&dinoref) {
            index
        } else {
            let index = self.const_pool.len() as u32;
            self.const_pool.push(dinoref);
            self.dinoref_to_const_index.insert(dinoref, index);
            index
        }
    }

    pub fn get_or_create_var_name(&mut self, name: &str) -> u32 {
        let global_idx = if let Some(&idx) = self.name_to_global_idx.get(name) {
            idx
        } else {
            let new_idx = self.var_names.len() as u32;
            self.var_names.push(name.to_string());
            self.name_to_global_idx.insert(name.to_string(), new_idx);
            new_idx
        };
        
        let scope_count = self.scope_mappings.len();
        let current_mapping = &mut self.scope_mappings[scope_count - 1];
        let current_g2l = &mut self.scope_global_to_local[scope_count - 1];

        if let Some(&local_idx) = current_g2l.get(&global_idx) {
            return local_idx;
        }

        let local_idx = current_mapping.len() as u32;
        current_mapping.push(global_idx);
        current_g2l.insert(global_idx, local_idx);

        local_idx
    }

    pub fn push_scope(&mut self) {
        self.scope_mappings.push(Vec::new());
        self.scope_global_to_local.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scope_mappings.len() > 1 {
            self.scope_mappings.pop();
            self.scope_global_to_local.pop();
        }
    }

    pub fn allocate_temp_var(&mut self) -> String {
        let name = format!("@temp_{}", self.total_vars_created);
        self.total_vars_created += 1;
        self.get_or_create_var_name(&name);
        name
    }

    pub fn resolve_var_scope(&self, name: &str) -> Option<(bool, u32)> {
        let global_idx = *self.name_to_global_idx.get(name)?;
        let bootstrap_offset = self.get_bootstrap_offset();

        if self.scope_mappings.len() == 1 {
            if let Some(&local_idx) = self.scope_global_to_local[0].get(&global_idx) {
                return Some((true, local_idx + bootstrap_offset));
            }
        } else {
            let last = self.scope_mappings.len() - 1;

            if let Some(&local_idx) = self.scope_global_to_local[last].get(&global_idx) {
                return Some((false, local_idx));
            }

            if let Some(&local_idx) = self.scope_global_to_local[0].get(&global_idx) {
                return Some((true, local_idx + bootstrap_offset));
            }
        }

        None
    }
    
    fn get_bootstrap_offset(&self) -> u32 {
        get_native_registry().bootstrap_count()
    }

    pub fn get_bootstrap_global_index_by_name(&self, name: &str) -> Option<u32> {
        get_bootstrap_global_index(name)
    }

    pub fn get_var_access_opcode(&self, name: &str) -> Option<(u8, u32)> {
        match self.resolve_var_scope(name) {
            Some((true, idx)) => Some((opcode::GET_GLOBAL, idx)),
            Some((false, idx)) => Some((opcode::GET_LOCAL, idx)),
            None => None,
        }
    }

    pub fn get_var_assign_opcode(&self, name: &str) -> Option<(u8, u32)> {
        match self.resolve_var_scope(name) {
            Some((true, idx)) => Some((opcode::SET_GLOBAL, idx)),
            Some((false, idx)) => Some((opcode::SET_LOCAL, idx)),
            None => None,
        }
    }

    pub fn get_const_pool(&self) -> &[DinoRef] {
        &self.const_pool
    }

    pub fn get_var_names(&self) -> &[String] {
        &self.var_names
    }

    pub fn get_global_scope(&self) -> &[u32] {
        self.scope_mappings.first().map_or(&[], |scope| scope.as_slice())
    }

    pub fn get_functions(&self) -> &[UserFunction] {
        &self.functions
    }

    pub fn register_function(&mut self, function: UserFunction) -> u32 {
        let index = self.functions.len() as u32;
        self.functions.push(function);
        index
    }

    pub fn register_function_placeholder(&mut self, is_main: bool) -> u32 {
        let index = self.functions.len() as u32;
        let function = UserFunction {
            is_main,
            start_ip: 0,
            end_ip: 0,
            param_count: 0,
            return_count: 1,
            local_count: 0,
        };
        self.functions.push(function);
        index
    }

    pub fn update_function(&mut self, function_id: u32, start_ip: usize, end_ip: usize, param_count: u32, return_count: u32) {
        if let Some(function) = self.functions.get_mut(function_id as usize) {
            function.start_ip = start_ip;
            function.end_ip = end_ip;
            function.param_count = param_count;
            function.return_count = return_count;
            function.local_count = self.scope_mappings.last()
                .map_or(0, |m| m.len() as u32);
        }
    }

    pub fn get_current_scope_depth(&self) -> u32 {
        (self.scope_mappings.len() - 1) as u32
    }

    pub fn resolve_var_scope_with_level(&self, name: &str) -> Option<(u32, u32)> {
        let global_idx = *self.name_to_global_idx.get(name)?;

        if self.scope_mappings.len() == 1 {
            if let Some(&local_idx) = self.scope_global_to_local[0].get(&global_idx) {
                return Some((0, local_idx));
            }
        } else {
            let last = self.scope_mappings.len() - 1;

            if let Some(&local_idx) = self.scope_global_to_local[last].get(&global_idx) {
                return Some((last as u32, local_idx));
            }

            if let Some(&local_idx) = self.scope_global_to_local[0].get(&global_idx) {
                return Some((0, local_idx));
            }
        }

        None
    }

    pub fn suggest_variable_name(&self, invalid_name: &str) -> Option<String> {
        let mut available_vars = Vec::new();
        
        if self.scope_mappings.len() > 1 {
            if let Some(current_mapping) = self.scope_mappings.last() {
                for &global_idx in current_mapping {
                    if let Some(var_name) = self.var_names.get(global_idx as usize) {
                        available_vars.push(var_name.clone());
                    }
                }
            }
        }
        
        if let Some(global_mapping) = self.scope_mappings.first() {
            for &global_idx in global_mapping {
                if let Some(var_name) = self.var_names.get(global_idx as usize) {
                    let var_name_str = var_name.clone();
                    if !available_vars.contains(&var_name_str) {
                        available_vars.push(var_name_str);
                    }
                }
            }
        }
        
        suggest_best_match(invalid_name, &available_vars)
    }
}
