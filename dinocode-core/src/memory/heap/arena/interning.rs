// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/heap/arena/interning.rs
//  Desc:       String interning integration with the MemoryManager
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::{cell::RefCell, collections::HashMap};
use crate::{
    memory::MemoryManager,
    types::DinoRef,
    utils::StringInterner,
};

impl MemoryManager {
    #[inline(always)]
    pub fn hash_string(value: &str) -> u64 {
        StringInterner::compute_hash(value.as_bytes())
    }
    
    #[inline(always)]
    pub fn get_interned_string(&self, value: &str) -> Option<DinoRef> {
        let hash = Self::hash_string(value);
        self.string_interner.get(hash).map(|entry| entry.dinoref)
    }
    
    #[inline(always)]
    pub fn get_interned_string_by_hash(&self, hash: u64) -> Option<DinoRef> {
        self.string_interner.get(hash).map(|entry| entry.dinoref)
    }
    
    pub fn intern_string(&mut self, value: &str, dinoref: DinoRef) -> Result<(), DinoRef> {
        let hash = Self::hash_string(value);
        let string_len = value.len();
        
        if !self.string_interner.insert(hash, dinoref, string_len, &self.arena) {
            if let Some(entry) = self.string_interner.get(hash) {
                return Err(entry.dinoref);
            }
        }
        
        Ok(())
    }
    
    #[inline(always)]
    pub fn intern_string_by_hash(&mut self, hash: u64, dinoref: DinoRef) {
        self.string_interner.insert(hash, dinoref, 0, &self.arena);
    }
    
    #[inline(always)]
    pub fn has_interned_string(&self, value: &str) -> bool {
        self.get_interned_string(value).is_some()
    }
    
    #[inline(always)]
    pub fn has_interned_hash(&self, hash: u64) -> bool {
        self.string_interner.get(hash).is_some()
    }
    
        
    pub fn get_string_hash_table(&self) -> &HashMap<u64, DinoRef> {
        thread_local! {
            static HASH_TABLE_CACHE: RefCell<HashMap<u64, DinoRef>> = RefCell::new(HashMap::new());
        }
        
        HASH_TABLE_CACHE.with(|cache| {
            let mut hash_table = cache.borrow_mut();
            hash_table.clear();
            for entry in &self.string_interner.entries {
                if !entry.is_none() {
                    hash_table.insert(entry.string_hash, entry.dinoref);
                }
            }
            unsafe {
                std::mem::transmute::<&HashMap<u64, DinoRef>, &HashMap<u64, DinoRef>>(&*hash_table)
            }
        })
    }
}
