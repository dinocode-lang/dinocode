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

use crate::{
    memory::{
        heap::arena::Arena,
        types::heap_header::{IS_FORWARDED, HAS_HASH, IS_INTERNED},
    },
    utils::hash::DinoHash,
};

// Arena header layout (8 bytes):
//   [0..2]  type_tag : u16 LE  (same value as value_type::STRING / BIGINT)
//   [2]     flags    : u8
//   [3]     padding  : u8
//   [4..8]  size     : u32 LE  (payload bytes, excluding the 8-byte header)

impl Arena {
    
    // Header writing operations

    #[inline(always)]
    pub fn write_header(&mut self, offset: usize, type_tag: u16, flags: u8, size: u32) {
        let tag_bytes = type_tag.to_le_bytes();
        self.data[offset]     = tag_bytes[0];
        self.data[offset + 1] = tag_bytes[1];
        self.data[offset + 2] = flags;
        self.data[offset + 3] = 0;

        let size_bytes = size.to_le_bytes();
        self.data[offset + 4..offset + 8].copy_from_slice(&size_bytes);
    }

    // Header reading operations

    #[inline(always)]
    pub fn get_type_tag(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.data[offset], self.data[offset + 1]])
    }

    #[inline(always)]
    pub fn get_flags(&self, offset: usize) -> u8 {
        self.data[offset + 2]
    }

    #[inline(always)]
    pub fn get_size(&self, offset: usize) -> u32 {
        u32::from_le_bytes([
            self.data[offset + 4],
            self.data[offset + 5],
            self.data[offset + 6],
            self.data[offset + 7]
        ])
    }

    #[inline(always)]
    pub fn get_total_size(&self, offset: usize) -> usize {
        8 + self.get_size(offset) as usize
    }

    // Forwarding operations

    #[inline(always)]
    pub fn is_forwarded(&self, offset: usize) -> bool {
        self.data[offset + 2] & IS_FORWARDED != 0
    }

    #[inline(always)]
    pub fn set_forwarded(&mut self, offset: usize, new_offset: u32) {
        self.data[offset + 2] |= IS_FORWARDED;

        // Store forwarding address in first 4 bytes of data (after 8-byte header)
        let data_start = offset + 8;
        let addr_bytes = new_offset.to_le_bytes();
        self.data[data_start..data_start + 4].copy_from_slice(&addr_bytes);
    }

    #[inline(always)]
    pub fn get_forwarding_address(&self, offset: usize) -> u32 {
        if self.is_forwarded(offset) {
            let data_start = offset + 8;
            u32::from_le_bytes([
                self.data[data_start],
                self.data[data_start + 1],
                self.data[data_start + 2],
                self.data[data_start + 3]
            ])
        } else {
            0
        }
    }

    // Hash and interning operations

    #[inline(always)]
    pub fn get_hash(&self, offset: usize) -> u64 {
        u64::from_le_bytes(
            self.data[offset + 16..offset + 24].try_into().unwrap()
        )
    }

    #[inline(always)]
    pub fn get_len(&self, offset: usize) -> usize {
        u32::from_le_bytes(
            self.data[offset + 8..offset + 12].try_into().unwrap()
        ) as usize
    }

    #[inline(always)]
    pub fn has_hash(&self, offset: usize) -> bool {
        self.data[offset + 2] & HAS_HASH != 0
    }

    #[inline(always)]
    pub fn set_has_hash(&mut self, offset: usize) {
        self.data[offset + 2] |= HAS_HASH;
    }

    #[inline(always)]
    pub fn is_interned(&self, offset: usize) -> bool {
        self.data[offset + 2] & IS_INTERNED != 0
    }

    #[inline(always)]
    pub fn set_interned(&mut self, offset: usize) {
        self.data[offset + 2] |= IS_INTERNED;
    }
    
    #[inline(always)]
    pub fn ensure_hash(&mut self, offset: usize) -> u64 {
        if self.has_hash(offset) {
            return self.get_hash(offset);
        }

        let hash = {
            let bytes = self.get_bytes(offset);
            DinoHash::bytes(bytes)
        };

        let hash_bytes = hash.to_le_bytes();
        self.data[offset + 16..offset + 24].copy_from_slice(&hash_bytes);
        self.set_has_hash(offset);

        hash
    }

}
