// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/builtins/bigint/view.rs
//  Desc:       Wrapper for bigint memory operations
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::manager::MemoryManager,
    types::{DinoRef, dinoref::value_type},
    errors::{Result, RuntimeError},
    utils::bigint::BigInt,
};

#[derive(Clone, Copy)]
pub struct BigIntView(pub usize);

impl BigIntView {
    #[inline(always)]
    pub fn as_id(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn as_ref(&self) -> DinoRef {
        DinoRef::bigint(self.0)
    }

    #[inline(always)]
    pub fn from_ref(dinoref: DinoRef) -> Result<Self> {
        if !dinoref.is_bigint() {
            return Err(RuntimeError::ExpectedInstance("bigint"));
        }
        Ok(Self(dinoref.as_index()))
    }

    #[inline(always)]
    pub fn from_ref_unchecked(dinoref: DinoRef) -> Self {
        Self(dinoref.as_index())
    }
    
    pub fn new(val: &BigInt, memory: &mut MemoryManager) -> Self {
        let mag = val.magnitude();
        let len = 1 + mag.len(); // sign + magnitude
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
        arena.data[offset] = value_type::BIGINT as u8;
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
        arena.data[offset + 12..offset + 16].copy_from_slice(&[0u8; 4]);
        
        // Zero hash bytes
        arena.data[offset + 16..offset + 24].copy_from_slice(&[0u8; 8]);
        
        // Write bigint bytes
        arena.data[offset + 24] = val.sign;
        arena.data[offset + 25..offset + 25 + mag.len()].copy_from_slice(mag);

        Self(offset)
    }

    #[inline(always)]
    pub fn get<'a>(&self, memory: &'a MemoryManager) -> BigInt<'a> {
        let offset = self.0;
        let len = u32::from_le_bytes(
            memory.arena.data[offset + 8..offset + 12].try_into().unwrap()
        ) as usize;
        let bytes = &memory.arena.data[offset + 24..offset + 24 + len];
        BigInt::from_slice(bytes[0], &bytes[1..])
    }
}
