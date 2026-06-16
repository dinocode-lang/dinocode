// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/types/buffer.rs
//  Desc:       Buffer type definitions for the lexer
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufType {
    None,
    Number,
    HexNumber,
    BitNumber,
    ScientificNumber,
    String,
    FString,
    Identifier,
    Operator,
    Comment,
    MultiLineComment,
    DollarCall,
}
