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
use std::sync::atomic::{AtomicU64, Ordering};
use dinocode_platform::time::{SystemTime, UNIX_EPOCH};
use dinocode_platform::process;

static HASH_SEED: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
pub fn get_hash_seed() -> u64 {
    let seed = HASH_SEED.load(Ordering::Relaxed);
    if seed == 0 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let pid = process::id() as u64;

        let stack_ptr = &seed as *const _ as u64;

        let mut final_seed = (now ^ (pid << 32))
            .wrapping_add(stack_ptr)
            .wrapping_add(0x9e3779b97f4a7c15);

        final_seed ^= final_seed >> 33;
        final_seed = final_seed.wrapping_mul(0xff51afd7ed558ccd);
        final_seed ^= final_seed >> 33;

        match HASH_SEED.compare_exchange(0, final_seed, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => final_seed,
            Err(val) => val,
        }
    } else {
        seed
    }
}

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

    #[inline(always)]
    pub fn compute_hash(bytes: &[u8]) -> u64 {
        let mut hash = get_hash_seed();
        let mut i = 0;
        
        while i + 8 <= bytes.len() {
            let chunk = u64::from_le_bytes([
                bytes[i], bytes[i+1], bytes[i+2], bytes[i+3],
                bytes[i+4], bytes[i+5], bytes[i+6], bytes[i+7]
            ]);
            hash = hash.wrapping_mul(33).wrapping_add(chunk);
            i += 8;
        }
        
        while i < bytes.len() {
            hash = hash.wrapping_mul(33).wrapping_add(bytes[i] as u64);
            i += 1;
        }
        
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^ (hash >> 33)
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
                let new_offset = dinoref.decode_index() as usize;
                let ex_offset = entry.dinoref.decode_index() as usize;
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