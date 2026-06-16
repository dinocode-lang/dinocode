// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/types/modes.rs
//  Desc:       Parse mode definitions for the lexer
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseMode {
    Normal,
    InFString,
    InString,
    InFStringExpr(usize),
    InBlockString(usize),
}
