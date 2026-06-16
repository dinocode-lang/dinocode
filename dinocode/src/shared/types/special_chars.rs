// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/types/special_chars.rs
//  Desc:       Special character constants for lexical analysis
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub const SPECIAL_CHARS: &[u8] = &[
    b'_', 
    0xC3, 0xB1, // ñ
    0xC3, 0x91, // Ñ
    0xC3, 0xA1, // á
    0xC3, 0xA9, // é
    0xC3, 0xAD, // í
    0xC3, 0xB3, // ó
    0xC3, 0xBA, // ú
    0xC3, 0xBC, // ü
    0xC3, 0x81, // Á
    0xC3, 0x89, // É
    0xC3, 0x8D, // Í
    0xC3, 0x93, // Ó
    0xC3, 0x9A, // Ú
];

pub const SPECIAL_CHARS_DOLLAR: &[u8] = &[
    b'_', b'$', 
    0xC3, 0xB1, // ñ
    0xC3, 0x91, // Ñ
    0xC3, 0xA1, // á
    0xC3, 0xA9, // é
    0xC3, 0xAD, // í
    0xC3, 0xB3, // ó
    0xC3, 0xBA, // ú
    0xC3, 0xBC, // ü
    0xC3, 0x81, // Á
    0xC3, 0x89, // É
    0xC3, 0x8D, // Í
    0xC3, 0x93, // Ó
    0xC3, 0x9A, // Ú
];
