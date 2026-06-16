// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/utils/type_conversion.rs
//  Desc:       Type conversion utilities for 'as' and 'is' expressions.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::collections::HashMap;
use dinocode_core::{
    types::TypeId,
    utils::suggestions::SuggestionEngine,
};

pub struct TypeResolver {
    cache: HashMap<String, TypeId>,
}

impl TypeResolver {
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        
        for &name in TypeId::valid_type_names() {
            if let Some(type_id) = TypeId::from_str(name) {
                cache.insert(name.to_string(), type_id);
            }
        }
        
        Self { cache }
    }

    pub fn resolve_type(&mut self, type_str: &str) -> Option<u32> {
        let lower = type_str.to_lowercase();
        
        if let Some(&type_id) = self.cache.get(&lower) {
            return Some(type_id.as_index());
        }
        
        if let Some(type_id) = TypeId::from_str(&lower) {
            self.cache.insert(lower, type_id);
            Some(type_id.as_index())
        } else {
            None
        }
    }

    pub fn suggest_type(&self, invalid_type: &str) -> Option<String> {
        let valid_types: Vec<String> = TypeId::valid_type_names()
            .iter()
            .map(|&name| name.to_string())
            .collect();
        
        let mut engine = SuggestionEngine::new();
        engine.find_best_suggestion(invalid_type, &valid_types)
    }
}
