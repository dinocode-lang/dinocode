// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/errors/types/base_error.rs
//  Desc:       Base error types and categories.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::compiler::parser::errors::ParseErrorType;
use dinocode_core::utils::parsers::numeric::NumericParseError;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ErrorCategory {
    Lexer,
    Parser,
    Runtime,
    Compiler,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum BaseErrorType {
    NumericParse(NumericParseError),
    UnexpectedChar,
    UnexpectedTokenAfterDot,
    UnexpectedBlankAfterDot,
    UnexpectedDollarCall,
    DollarCallWithSpace,
    OperatorNotAllowed,
    InvalidOperator,
    IncompleteRedirection,
    UnexpectedSemicolon,
    ReservedKeywordAsIdentifier,
    
    Custom(String),
    Parser(ParseErrorType),
}

pub type LexErrorType = BaseErrorType;
