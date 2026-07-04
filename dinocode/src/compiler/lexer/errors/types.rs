// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/errors/types.rs
//  Desc:       Lexer errors types
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::utils::parsers::numeric::NumericParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorType {
    NumericParse(NumericParseError),
    UnexpectedToken(&'static str),
    InvalidOperator(&'static str),
    Custom(String),
    UnexpectedLogicalContinuation,
    UnexpectedTokenAfterDot,
    UnexpectedBlankAfterDot,
    DollarCallWithSpace,
    ReservedKeywordAsIdentifier,
    UnterminatedMultilineComment,
    EmptyLiteralBlockString,
}
