// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/heap/pool/bitmap.rs
//  Desc:       Bitmap for managing pool allocation state
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct Bitmap {
    data: Vec<u64>,
    size: usize,
}

impl Bitmap {
    pub fn new(size: usize) -> Self {
        let u64_count = (size + 63) / 64;
        Self {
            data: vec![0; u64_count],
            size,
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        let u64_count = (new_size + 63) / 64;
        self.data.resize(u64_count, 0);
        self.size = new_size;
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> bool {
        if index >= self.size { return false; }
        let word_idx = index / 64;
        let bit_idx = index % 64;
        (self.data[word_idx] & (1 << bit_idx)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize) {
        if index >= self.size { return; }
        let word_idx = index / 64;
        let bit_idx = index % 64;
        self.data[word_idx] |= 1 << bit_idx;
    }

    #[inline(always)]
    pub fn clear(&mut self, index: usize) {
        if index >= self.size { return; }
        let word_idx = index / 64;
        let bit_idx = index % 64;
        self.data[word_idx] &= !(1 << bit_idx);
    }

    pub fn find_first_free(&self) -> Option<usize> {
        for (i, &word) in self.data.iter().enumerate() {
            if word != u64::MAX {
                // Found a word with at least one zero
                let zeros = !word;
                let trailing = zeros.trailing_zeros();
                let index = i * 64 + trailing as usize;
                if index < self.size {
                    return Some(index);
                }
            }
        }
        None
    }
    
    pub fn clear_all(&mut self) {
        for word in self.data.iter_mut() {
            *word = 0;
        }
    }
}
