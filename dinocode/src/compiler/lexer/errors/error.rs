// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/errors/error.rs
//  Desc:       Lexer errors formatting
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::formatter::{
    DinoError,
    ErrorType,
    FormatterColor,
};
use thiserror::Error;
use super::types::LexErrorType;

#[derive(Error, Debug, Clone)]
#[error("Lexer error at line {line}, col {column}")]
pub struct LexError {
    pub typ: LexErrorType,
    pub line: u32,
    pub column: u32,
}

impl LexError {
    pub fn new(typ: LexErrorType, line: u32, column: u32) -> Self {
        Self { typ, line, column }
    }
}

impl From<LexError> for DinoError<'static> {
    fn from(err: LexError) -> Self {
        let mut dino = DinoError::new(err.line, err.column).with_type(ErrorType::Lexer);
        match err.typ {
            LexErrorType::NumericParse(npe) => {
                dino = dino.add_message_owned(npe.message.clone(), FormatterColor::Default);
                if let Some(help) = &npe.help {
                    dino = dino.add_note_owned(help.clone(), FormatterColor::Default);
                }
                if let Some(info) = &npe.info {
                    dino = dino.add_info_owned(info.clone(), FormatterColor::Default);
                }
            }
            LexErrorType::UnexpectedToken(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default);
            }
            LexErrorType::UnexpectedTokenAfterDot => {
                dino = dino.add_message("unexpected token after '.'", FormatterColor::Default)
                    .add_note("dot access must be followed by an identifier", FormatterColor::Default);
            }
            LexErrorType::UnexpectedBlankAfterDot => {
                dino = dino.add_message("unexpected space after '.'", FormatterColor::Default)
                    .add_note("dot access cannot have spaces between the dot and the identifier", FormatterColor::Default);
            }
            LexErrorType::UnexpectedLogicalContinuation => {
                dino = dino.add_message("incomplete expression", FormatterColor::Default)
                    .add_note("ensure the expression ends properly", FormatterColor::Default);
            }
            LexErrorType::DollarCallWithSpace => {
                dino = dino.add_message("unexpected space after '$'", FormatterColor::Default)
                    .add_note("dollar calls cannot have spaces between '$' and '('", FormatterColor::Default);
            }
            LexErrorType::InvalidOperator(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default);
            }
            LexErrorType::ReservedKeywordAsIdentifier => {
                dino = dino.add_message("reserved keyword used as identifier", FormatterColor::Default);
            }
            LexErrorType::UnterminatedMultilineComment => {
                dino = dino.add_message("unterminated multiline comment", FormatterColor::Default)
                    .add_note("multiline comments must be closed with '*#'", FormatterColor::Default);
            }
            LexErrorType::EmptyLiteralBlockString => {
                dino = dino.add_message("literal block string cannot be empty", FormatterColor::Default)
                    .add_note("block strings must contain at least one character", FormatterColor::Default);
            }
            LexErrorType::Custom(msg) => {
                dino = dino.add_message_owned(msg, FormatterColor::Default);
            }
        }
        dino
    }
}
