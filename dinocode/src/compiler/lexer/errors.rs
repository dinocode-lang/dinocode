// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/errors.rs
//  Desc:       Lexer-specific error type.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("{0}")]
    Tokenize(String),
}

impl LexError {
    pub fn print(&self) {
        match self {
            LexError::Tokenize(msg) => eprint!("{}", msg),
        }
    }
}
