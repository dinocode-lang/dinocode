// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//
//  File:       src/utils/hash.rs
//  Desc:       Hashing utilities
//
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::sync::atomic::{AtomicU64, Ordering};
use dinocode_platform::time::{SystemTime, UNIX_EPOCH};
use dinocode_platform::process;
use crate::{
    types::DinoRef,
    memory::MemoryManager,
};

static HASH_SEED: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
fn get_hash_seed() -> u64 {
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

pub struct DinoHash;

impl DinoHash {
    #[inline(always)]
    pub fn bytes(data: &[u8]) -> u64 {
        let mut hash = get_hash_seed();
        let mut i = 0;
        
        while i + 8 <= data.len() {
            let chunk = u64::from_le_bytes([
                data[i], data[i+1], data[i+2], data[i+3],
                data[i+4], data[i+5], data[i+6], data[i+7]
            ]);
            hash = hash.wrapping_mul(33).wrapping_add(chunk);
            i += 8;
        }
        
        while i < data.len() {
            hash = hash.wrapping_mul(33).wrapping_add(data[i] as u64);
            i += 1;
        }
        
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^ (hash >> 33)
    }

    #[inline(always)]
    pub fn u64(key: u64) -> u64 {
        let seed = get_hash_seed();
        let mut x = key ^ seed;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }

    #[inline(always)]
    pub fn any(key: DinoRef, memory: &mut MemoryManager) -> u64 {
        if key.is_const() {
            memory.arena.ensure_hash(key.as_index())
        } else {
            Self::u64(key.raw())
        }
    }
}
