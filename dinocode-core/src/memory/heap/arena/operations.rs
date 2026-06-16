// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/heap/arena/operations.rs
//  Desc:       Heap operations for immutable data types
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::{
        MemoryManager,
        types::heap_header::{HAS_HASH, IS_INTERNED},
    },
    types::{
        DinoRef,
        dinoref::value_type,
    },
    utils::{
        bigint::{string_to_bigint_bits, bigint_bits_to_string},
        StringInterner,
    },
};

impl MemoryManager {

    pub fn set_globals(&mut self) {
        self.globals_start = self.arena.len();
        self.string_interner.set_globals();
        #[cfg(feature = "logging")]
        log::debug!("Globals sealed at offset {}", self.globals_start);
    }

    // String Operations

    pub fn get_string(&self, offset: u32) -> &str {
        let offset = offset as usize;
        if offset + 24 > self.arena.len() { 
            #[cfg(feature = "logging")]
            log::warn!(" Invalid arena offset: {} > {}", offset + 24, self.arena.len());
            return "<invalid arena offset>"; 
        }
        
        let len = u32::from_le_bytes(
            self.arena[offset + 8..offset + 12].try_into().unwrap()
        ) as usize;
        
        if offset + 24 + len > self.arena.len() { 
            #[cfg(feature = "logging")]
            log::warn!(" Invalid string len: offset={}, len={}, arena_len={}", offset, len, self.arena.len());
            return "<invalid string len>"; 
        }
        
        let bytes = &self.arena[offset + 24..offset + 24 + len];
        std::str::from_utf8(bytes).unwrap_or("<invalid utf8>")
    }

    pub fn get_const_bytes(&self, offset: u32) -> &[u8] {
        let offset = offset as usize;
        if offset + 24 > self.arena.len() { return &[]; }
        
        let len = u32::from_le_bytes(
            self.arena[offset + 8..offset + 12].try_into().unwrap()
        ) as usize;
        
        if offset + 24 + len > self.arena.len() { return &[]; }
        &self.arena[offset + 24..offset + 24 + len]
    }

    pub fn get_const_hash(&self, offset: u32) -> u64 {
        let offset = offset as usize;
        if offset + 24 > self.arena.len() { return 0; }
        
        u64::from_le_bytes(
            self.arena[offset + 16..offset + 24].try_into().unwrap()
        )
    }

    pub fn get_const_len(&self, offset: u32) -> usize {
        let offset = offset as usize;
        if offset + 12 > self.arena.len() { return 0; }
        
        u32::from_le_bytes(
            self.arena[offset + 8..offset + 12].try_into().unwrap()
        ) as usize
    }



    fn allocate(&mut self, size: usize) -> u32 {
        let offset = self.arena.len();
        self.arena.resize(offset + size, 0);
        offset as u32
    }

    pub fn alloc_string(&mut self, s: &str) -> DinoRef {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let total_size = 8 + 4 + 4 + 8 + len;  // header + len_field + pad + hash + data
                
        self.record_arena_alloc(total_size);
        
        let offset = self.allocate(total_size);
        
        self.write_header(
            offset as usize,
            value_type::STRING,
            0,
            (total_size - 8) as u32
        );
        
        let len_bytes = (len as u32).to_le_bytes();
        self.arena[offset as usize + 8..offset as usize + 12].copy_from_slice(&len_bytes);
        
        let zero_bytes = [0u8; 4];
        self.arena[offset as usize + 12..offset as usize + 16].copy_from_slice(&zero_bytes);
        
        let hash_bytes = [0u8; 8];
        self.arena[offset as usize + 16..offset as usize + 24].copy_from_slice(&hash_bytes);
        
        self.arena[offset as usize + 24..offset as usize + 24 + len].copy_from_slice(bytes);

        DinoRef::string(offset)
    }

    pub fn ensure_const_hash(&mut self, dinoref: DinoRef) -> u64 {
        let vtype = dinoref.decode_type();
        if vtype != value_type::STRING && vtype != value_type::BIGINT { return 0; }
        let handle = dinoref.decode_index() as usize;

        if self.has_hash(handle) {
            return self.get_const_hash(handle as u32);
        }

        let hash = {
            let bytes = self.get_const_bytes(handle as u32);
            StringInterner::compute_hash(bytes)
        };

        let hash_bytes = hash.to_le_bytes();
        self.arena[handle + 16..handle + 24].copy_from_slice(&hash_bytes);
        self.set_has_hash(handle);

        hash
    }

    pub fn alloc_const_string(&mut self, s: &str) -> DinoRef {
        let hash = Self::hash_string(s);
        
        if let Some(dinoref) = self.get_interned_string_by_hash(hash) {
            return dinoref;
        }
        
        let bytes = s.as_bytes();
        let len = bytes.len();
        let total_size = 8 + 4 + 4 + 8 + len;  // header + len_field + pad + hash + data
        
        let offset = self.allocate(total_size);
        
        self.write_header(
            offset as usize,
            value_type::STRING,
            HAS_HASH | IS_INTERNED,
            (total_size - 8) as u32
        );
        
        let len_bytes = (len as u32).to_le_bytes();
        self.arena[offset as usize + 8..offset as usize + 12].copy_from_slice(&len_bytes);
        
        let zero_bytes = [0u8; 4];
        self.arena[offset as usize + 12..offset as usize + 16].copy_from_slice(&zero_bytes);
        
        let hash_bytes = hash.to_le_bytes();
        self.arena[offset as usize + 16..offset as usize + 24].copy_from_slice(&hash_bytes);
        
        self.arena[offset as usize + 24..offset as usize + 24 + len].copy_from_slice(bytes);
        
        let dinoref = DinoRef::string(offset);
        self.intern_string_by_hash(hash, dinoref);
        
        dinoref
    }

    // BigInt Operations

    pub fn alloc_bigint_from_bits(&mut self, bits: &[u8]) -> DinoRef {
        let len = bits.len();
        let total_size = 8 + 4 + 4 + 8 + len;  // header + len_field + pad + hash + data
        
        self.record_arena_alloc(total_size);
        
        let offset = self.allocate(total_size);
        
        self.write_header(
            offset as usize,
            value_type::BIGINT,
            0,
            (total_size - 8) as u32
        );
        
        let len_bytes = (len as u32).to_le_bytes();
        self.arena[offset as usize + 8..offset as usize + 12].copy_from_slice(&len_bytes);
        
        let zero_bytes = [0u8; 4];
        self.arena[offset as usize + 12..offset as usize + 16].copy_from_slice(&zero_bytes);
        
        let hash_bytes = [0u8; 8];
        self.arena[offset as usize + 16..offset as usize + 24].copy_from_slice(&hash_bytes);
        
        self.arena[offset as usize + 24..offset as usize + 24 + len].copy_from_slice(bits);
        
        DinoRef::bigint(offset)
    }
    
    pub fn alloc_bigint(&mut self, val: i64) -> Result<DinoRef, String> {
         let bits = string_to_bigint_bits(&val.to_string())?;
         Ok(self.alloc_bigint_from_bits(&bits))
    }

    pub fn alloc_bigint_str(&mut self, s: &str) -> Result<DinoRef, String> {
         let bits = string_to_bigint_bits(s)?;
         Ok(self.alloc_bigint_from_bits(&bits))
    }

    pub fn get_bigint_string(&self, offset: u32) -> String {
        let bits = self.get_const_bytes(offset);
        bigint_bits_to_string(bits)
    }
}
