// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/types/heap_header.rs
//  Desc:       Memory types and constants for heap management
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub const IS_FORWARDED: u8 = 0x80;
pub const HAS_HASH: u8 = 0x40;
pub const IS_INTERNED: u8 = 0x20;
