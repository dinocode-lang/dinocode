// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/utils/headers.rs
//  Desc:       Header skipping utilities for file preprocessing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub fn get_header_len(source: &[u8]) -> usize {
    let mut i = 0;
    let len = source.len();
    
    // Skip BOM
    if len >= 3
    && source[0] == 0xEF
    && source[1] == 0xBB
    && source[2] == 0xBF {
        i = 3;
    }
    i
}