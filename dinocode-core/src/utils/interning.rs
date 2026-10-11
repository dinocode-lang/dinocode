// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/interning.rs
//  Desc:       String interning utilities
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::types::DinoRef;

#[derive(Debug)]
pub struct StringInterner {
    pub entries: Vec<StringEntry>,
    pub slots: Vec<u32>,
    pub mask: u64,
    pub count: usize,
    pub globals_count: usize,
}

impl StringInterner {
    pub const INITIAL_CAPACITY: usize = 64;
    pub const MAX_LOAD_FACTOR: f32 = 0.85;
    pub const EMPTY: u32 = u32::MAX;

    #[inline(always)]
    pub fn new() -> Self {
        let capacity = Self::INITIAL_CAPACITY;
        Self {
            entries: Vec::with_capacity(capacity),
            slots: vec![Self::EMPTY; capacity],
            mask: (capacity as u64) - 1,
            count: 0,
            globals_count: 0,
        }
    }

    #[inline(always)]
    pub fn set_globals(&mut self) {
        self.globals_count = self.entries.len();
    }

    pub fn truncate_to_globals(&mut self) {
        self.entries.truncate(self.globals_count);
        self.count = self.globals_count;
        self.slots.fill(Self::EMPTY);
        
        for i in 0..self.count {
            let entry = &self.entries[i];
            let mut pos = (entry.string_hash & self.mask) as usize;
            while self.slots[pos] != Self::EMPTY {
                pos = (pos + 1) & (self.slots.len() - 1);
            }
            self.slots[pos] = i as u32;
        }
    }

    pub fn insert(&mut self, hash: u64, dinoref: DinoRef, string_len: usize, arena: &[u8]) -> bool {
        if self.count * 8 >= self.slots.len() * 7 {
            self.resize();
        }

        let mut pos = (hash & self.mask) as usize;
        let slots_len = self.slots.len();

        loop {
            let index = self.slots[pos];
            if index == Self::EMPTY {
                let new_index = self.entries.len() as u32;
                self.entries.push(StringEntry {
                    dinoref,
                    string_hash: hash,
                    string_len,
                });
                self.slots[pos] = new_index;
                self.count += 1;
                return true;
            }

            let entry = &self.entries[index as usize];
            if entry.string_hash == hash && entry.string_len == string_len {
                let new_offset = dinoref.as_index();
                let ex_offset = entry.dinoref.as_index();
                let new_bytes = &arena[new_offset + 24..new_offset + 24 + string_len.min(arena.len().saturating_sub(new_offset + 24))];
                let ex_bytes  = &arena[ex_offset  + 24..ex_offset  + 24 + string_len.min(arena.len().saturating_sub(ex_offset  + 24))];
                
                if new_bytes == ex_bytes {
                    return false;
                }
            }

            pos = (pos + 1) & (slots_len - 1);
        }
    }

    pub fn get(&self, hash: u64) -> Option<&StringEntry> {
        let mut pos = (hash & self.mask) as usize;
        let slots_len = self.slots.len();

        loop {
            let index = self.slots[pos];
            if index == Self::EMPTY { return None; }

            let entry = &self.entries[index as usize];
            if entry.string_hash == hash {
                return Some(entry);
            }

            pos = (pos + 1) & (slots_len - 1);
        }
    }

    pub fn resize(&mut self) {
        let new_capacity = self.slots.len() * 2;
        self.slots = vec![Self::EMPTY; new_capacity];
        self.mask = (new_capacity as u64) - 1;

        for i in 0..self.entries.len() {
            let hash = self.entries[i].string_hash;
            let mut pos = (hash & self.mask) as usize;
            while self.slots[pos] != Self::EMPTY {
                pos = (pos + 1) & (new_capacity - 1);
            }
            self.slots[pos] = i as u32;
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.slots.fill(Self::EMPTY);
        self.count = 0;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StringEntry {
    pub dinoref: DinoRef,
    pub string_hash: u64,
    pub string_len: usize,
}

impl StringEntry {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            dinoref: DinoRef::NONE,
            string_hash: 0,
            string_len: 0,
        }
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.dinoref.is_none()
    }
}