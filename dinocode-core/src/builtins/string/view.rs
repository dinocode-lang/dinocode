// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/builtins/string/view.rs
//  Desc:       Wrapper for string memory operations
//
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::manager::MemoryManager,
    memory::types::heap_header::{HAS_HASH, IS_INTERNED},
    types::{
        DinoRef,
        dinoref::value_type,
    },
    errors::{Result, RuntimeError},
};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub struct StringView(pub usize);

impl StringView {
    #[inline(always)]
    pub fn as_id(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        DinoRef::string(self.0)
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_string() {
            return Err(RuntimeError::ExpectedInstance("string"));
        }
        Ok(Self(dinoref.as_index()))
    }

    #[inline(always)]
    pub fn from_ref_unchecked(dinoref: DinoRef) -> Self {
        Self(dinoref.as_index())
    }
    
    pub fn new(s: &str, memory: &mut MemoryManager) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let total_size = 8 + 4 + 4 + 8 + len;
        
        memory.record_arena_alloc(total_size);
        
        let offset = {
            let arena = &mut memory.arena;
            let offset = arena.data.len();
            arena.data.resize(offset + total_size, 0);
            offset
        };
        
        let arena = &mut memory.arena;
        
        // Write header
        arena.data[offset] = value_type::STRING as u8;
        arena.data[offset + 1] = 0;
        arena.data[offset + 2] = 0;
        arena.data[offset + 3] = 0;
        arena.data[offset + 4] = 0;
        
        let size_bytes = ((total_size - 8) as u32).to_le_bytes();
        arena.data[offset + 5..offset + 9].copy_from_slice(&size_bytes);
        
        // Write length
        let len_bytes = (len as u32).to_le_bytes();
        arena.data[offset + 8..offset + 12].copy_from_slice(&len_bytes);
        
        // Zero reserved bytes
        let zero_bytes = [0u8; 4];
        arena.data[offset + 12..offset + 16].copy_from_slice(&zero_bytes);
        
        // Zero hash bytes
        let hash_bytes = [0u8; 8];
        arena.data[offset + 16..offset + 24].copy_from_slice(&hash_bytes);
        
        // Write string bytes
        arena.data[offset + 24..offset + 24 + len].copy_from_slice(bytes);

        Self(offset)
    }

    pub fn new_const(s: &str, memory: &mut MemoryManager) -> Self {
        use crate::utils::hash::DinoHash;
        let hash = DinoHash::bytes(s.as_bytes());
        
        if let Some(existing) = memory.arena.string_interner.get(hash) {
            return Self(existing.dinoref.as_index());
        }
        
        let bytes = s.as_bytes();
        let len = bytes.len();
        let total_size = 8 + 4 + 4 + 8 + len;
        
        let offset = {
            let arena = &mut memory.arena;
            let offset = arena.data.len();
            arena.data.resize(offset + total_size, 0);
            offset
        };
        
        let arena = &mut memory.arena;
        
        // Write header with interned flag
        arena.data[offset] = value_type::STRING as u8;
        arena.data[offset + 1] = (u32::from(HAS_HASH) | u32::from(IS_INTERNED)) as u8;
        arena.data[offset + 2] = 0;
        arena.data[offset + 3] = 0;
        arena.data[offset + 4] = 0;
        
        let size_bytes = ((total_size - 8) as u32).to_le_bytes();
        arena.data[offset + 5..offset + 9].copy_from_slice(&size_bytes);
        
        // Write length
        let len_bytes = (len as u32).to_le_bytes();
        arena.data[offset + 8..offset + 12].copy_from_slice(&len_bytes);
        
        // Zero reserved bytes
        let zero_bytes = [0u8; 4];
        arena.data[offset + 12..offset + 16].copy_from_slice(&zero_bytes);
        
        // Write hash bytes
        let hash_bytes = hash.to_le_bytes();
        arena.data[offset + 16..offset + 24].copy_from_slice(&hash_bytes);
        
        // Write string bytes
        arena.data[offset + 24..offset + 24 + len].copy_from_slice(bytes);
        
        let dinoref = DinoRef::string(offset);
        memory.arena.string_interner.insert(hash, dinoref, 0, &memory.arena.data);
        
        Self(offset)
    }

    #[inline(always)]
    pub fn get<'a>(&self, memory: &'a MemoryManager) -> &'a str {
        let offset = self.0;
        let len = self.len(memory);
        let bytes = &memory.arena.data[offset + 24..offset + 24 + len];
        std::str::from_utf8(bytes).unwrap()
    }

    #[inline(always)]
    pub fn len(&self, memory: &MemoryManager) -> usize {
        let offset = self.0;
        u32::from_le_bytes(memory.arena.data[offset + 8..offset + 12].try_into().unwrap()) as usize
    }

    #[inline(always)]
    pub fn is_empty(&self, memory: &MemoryManager) -> bool {
        self.len(memory) == 0
    }

    #[inline(always)]
    pub fn char_at(&self, index: i64, memory: &MemoryManager) -> Option<char> {
        self.get(memory).chars().nth(index as usize)
    }

    #[inline(always)]
    pub fn contains(&self, substr: &str, memory: &MemoryManager) -> bool {
        self.get(memory).contains(substr)
    }

    #[inline(always)]
    pub fn starts_with(&self, prefix: &str, memory: &MemoryManager) -> bool {
        self.get(memory).starts_with(prefix)
    }

    #[inline(always)]
    pub fn ends_with(&self, suffix: &str, memory: &MemoryManager) -> bool {
        self.get(memory).ends_with(suffix)
    }

    #[inline(always)]
    pub fn to_uppercase(&self, memory: &MemoryManager) -> String {
        self.get(memory).to_uppercase()
    }

    #[inline(always)]
    pub fn to_lowercase(&self, memory: &MemoryManager) -> String {
        self.get(memory).to_lowercase()
    }

    #[inline(always)]
    pub fn concat(&self, other: &str, memory: &mut MemoryManager) -> String {
        let s1 = self.get(memory);
        format!("{}{}", s1, other)
    }

    pub fn trim(&self, chars_to_trim: Option<&str>, memory: &MemoryManager) -> String {
        let s = self.get(memory);
        if let Some(chars) = chars_to_trim {
            s.trim_matches(|c: char| chars.contains(c)).to_string()
        } else {
            s.trim().to_string()
        }
    }

    #[inline(always)]
    pub fn repeat(&self, count: usize, memory: &MemoryManager) -> String {
        self.get(memory).repeat(count)
    }

    pub fn split(&self, delimiter: Option<&str>, memory: &MemoryManager) -> Vec<String> {
        let s = self.get(memory);
        if let Some(del) = delimiter {
            if del.is_empty() {
                s.chars().map(|c| c.to_string()).collect()
            } else {
                s.split(del).map(|s| s.to_string()).collect()
            }
        } else {
            s.split_whitespace().map(|s| s.to_string()).collect()
        }
    }

    #[inline(always)]
    pub fn replace(&self, from: &str, to: &str, memory: &MemoryManager) -> String {
        self.get(memory).replace(from, to)
    }

    pub fn substr(&self, start: usize, end: Option<usize>, memory: &MemoryManager) -> Option<String> {
        let s = self.get(memory);
        let chars: Vec<char> = s.chars().collect();

        if start >= chars.len() {
            return None;
        }

        let end_usize = end.unwrap_or(chars.len());
        if start >= end_usize {
            return None;
        }

        Some(chars[start..end_usize].iter().collect())
    }

    #[inline(always)]
    pub fn index_of(&self, search: &str, start: usize, memory: &MemoryManager) -> Option<usize> {
        let s = self.get(memory);
        let substr = s.get(start..)?;
        substr.find(search).map(|pos| start + pos)
    }

    #[inline(always)]
    pub fn last_index_of(&self, search: &str, end: usize, memory: &MemoryManager) -> Option<usize> {
        let s = self.get(memory);
        let substr = s.get(..end)?;
        substr.rfind(search)
    }

    pub fn pad_left(&self, width: usize, pad_char: char, memory: &MemoryManager) -> String {
        let s = self.get(memory);
        if s.len() >= width {
            s.to_string()
        } else {
            let padding = width - s.len();
            format!("{}{}", pad_char.to_string().repeat(padding), s)
        }
    }

    pub fn pad_right(&self, width: usize, pad_char: char, memory: &MemoryManager) -> String {
        let s = self.get(memory);
        if s.len() >= width {
            s.to_string()
        } else {
            let padding = width - s.len();
            format!("{}{}", s, pad_char.to_string().repeat(padding))
        }
    }

    #[inline(always)]
    pub fn hash(&self, memory: &mut MemoryManager) -> u64 {
        memory.arena.ensure_hash(self.0)
    }

    pub fn intern(&self, memory: &mut MemoryManager) -> Result<DinoRef> {
        let hash = memory.arena.ensure_hash(self.0);
        
        if let Some(existing) = memory.arena.string_interner.get(hash) {
            return Ok(existing.dinoref);
        }

        let dinoref = self.as_ref();
        memory.arena.set_interned(self.0);
        memory.arena.string_interner.insert(hash, dinoref, 0, &memory.arena.data);
        
        Ok(dinoref)
    }

    pub fn cmp(&self, other: Self, memory: &MemoryManager) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        }
        
        let a_len = self.len(memory);
        let b_len = other.len(memory);
        
        if a_len != b_len {
            return a_len.cmp(&b_len);
        }
        
        if memory.arena.is_interned(self.0) && memory.arena.is_interned(other.0) {
            return Ordering::Less;
        }
        
        if memory.arena.has_hash(self.0) && memory.arena.has_hash(other.0) {
            let a_hash = memory.arena.get_hash(self.0);
            let b_hash = memory.arena.get_hash(other.0);
            if a_hash != b_hash {
                return a_hash.cmp(&b_hash);
            }
        }
        
        self.get(memory).cmp(other.get(memory))
    }

    #[inline(always)]
    pub fn eq(&self, other: Self, memory: &MemoryManager) -> bool {
        self.cmp(other, memory).is_eq()
    }
}
