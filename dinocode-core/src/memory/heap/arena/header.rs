// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/heap/arena/header.rs
//  Desc:       Header operations for MemoryManager
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::memory::{
    MemoryManager,
    types::heap_header::{IS_FORWARDED, HAS_HASH, IS_INTERNED},
};

// Arena header layout (8 bytes):
//   [0..2]  type_tag : u16 LE  (same value as value_type::STRING / BIGINT)
//   [2]     flags    : u8
//   [3]     padding  : u8
//   [4..8]  size     : u32 LE  (payload bytes, excluding the 8-byte header)

impl MemoryManager {
    
    // Header writing operations

    #[inline(always)]
    pub fn write_header(&mut self, offset: usize, type_tag: u16, flags: u8, size: u32) {
        let tag_bytes = type_tag.to_le_bytes();
        self.arena[offset]     = tag_bytes[0];
        self.arena[offset + 1] = tag_bytes[1];
        self.arena[offset + 2] = flags;
        self.arena[offset + 3] = 0;

        let size_bytes = size.to_le_bytes();
        self.arena[offset + 4..offset + 8].copy_from_slice(&size_bytes);
    }

    // Header reading operations

    #[inline(always)]
    pub fn get_type_tag(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.arena[offset], self.arena[offset + 1]])
    }

    #[inline(always)]
    pub fn get_flags(&self, offset: usize) -> u8 {
        self.arena[offset + 2]
    }

    #[inline(always)]
    pub fn get_size(&self, offset: usize) -> u32 {
        u32::from_le_bytes([
            self.arena[offset + 4],
            self.arena[offset + 5],
            self.arena[offset + 6],
            self.arena[offset + 7]
        ])
    }

    #[inline(always)]
    pub fn get_total_size(&self, offset: usize) -> usize {
        8 + self.get_size(offset) as usize
    }

    // Forwarding operations

    #[inline(always)]
    pub fn is_forwarded(&self, offset: usize) -> bool {
        self.arena[offset + 2] & IS_FORWARDED != 0
    }

    #[inline(always)]
    pub fn set_forwarded(&mut self, offset: usize, new_offset: u32) {
        self.arena[offset + 2] |= IS_FORWARDED;

        // Store forwarding address in first 4 bytes of data (after 8-byte header)
        let data_start = offset + 8;
        let addr_bytes = new_offset.to_le_bytes();
        self.arena[data_start..data_start + 4].copy_from_slice(&addr_bytes);
    }

    #[inline(always)]
    pub fn get_forwarding_address(&self, offset: usize) -> u32 {
        if self.is_forwarded(offset) {
            let data_start = offset + 8;
            u32::from_le_bytes([
                self.arena[data_start],
                self.arena[data_start + 1],
                self.arena[data_start + 2],
                self.arena[data_start + 3]
            ])
        } else {
            0
        }
    }

    // Hash and interning operations

    #[inline(always)]
    pub fn has_hash(&self, offset: usize) -> bool {
        self.arena[offset + 2] & HAS_HASH != 0
    }

    #[inline(always)]
    pub fn set_has_hash(&mut self, offset: usize) {
        self.arena[offset + 2] |= HAS_HASH;
    }

    #[inline(always)]
    pub fn is_interned(&self, offset: usize) -> bool {
        self.arena[offset + 2] & IS_INTERNED != 0
    }

    #[inline(always)]
    pub fn set_interned(&mut self, offset: usize) {
        self.arena[offset + 2] |= IS_INTERNED;
    }

}
