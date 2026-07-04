// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/errors/error.rs
//  Desc:       Parser errors formatting
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
//  ═══════════════════════════════════════════════════════════

use thiserror::Error;
use dinocode_core::formatter::{DinoError, ErrorType, FormatterColor};
use crate::shared::types::Token;
use super::types::ParseErrorType;

#[derive(Error, Debug, Clone)]
#[error("Parser error at line {line}, col {column}")]
pub struct ParseError {
    pub typ: ParseErrorType,
    pub line: u32,
    pub column: u32,
}

impl ParseError {
    pub fn new(typ: ParseErrorType, line: u32, column: u32) -> Self {
        Self { typ, line, column }
    }

    pub fn from_token(typ: ParseErrorType, token: &Token) -> Self {
        let line = token.line.unwrap_or(0);
        let column = token.column.unwrap_or(0);
        Self { typ, line, column }
    }
}

impl From<ParseError> for DinoError<'static> {
    fn from(err: ParseError) -> Self {
        let mut dino = DinoError::new(err.line, err.column).with_type(ErrorType::Parser);
        match err.typ {
            ParseErrorType::ExpectedToken(token) => {
                dino = dino.add_message("expected '", FormatterColor::Default)
                    .add_message(token, FormatterColor::WhiteBold)
                    .add_message("'", FormatterColor::Default);
            }
            ParseErrorType::ExpectedIdentifier(what) => {
                dino = dino.add_message("expected ", FormatterColor::Default)
                    .add_message(what, FormatterColor::WhiteBold);
            }
            ParseErrorType::ExpectedExpression(msg) => {
                dino = dino.add_message("expected expression: ", FormatterColor::Default)
                    .add_message(msg, FormatterColor::WhiteBold);
            }
            ParseErrorType::ExpectedIndentedBlock(keyword) => {
                dino = dino.add_message("expected indented block after '", FormatterColor::Default)
                    .add_message(keyword, FormatterColor::WhiteBold)
                    .add_message("'", FormatterColor::Default);
            }
            ParseErrorType::MismatchedDelimiter(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default);
            }
            ParseErrorType::InvalidControlFlow(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default)
                    .add_note("check the context where this statement is used", FormatterColor::Green);
            }
            ParseErrorType::FunctionResolutionError(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default);
            }
            ParseErrorType::UnexpectedToken(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default);
            }
            ParseErrorType::InvalidOperator(msg) => {
                dino = dino.add_message(msg, FormatterColor::Default);
            }
            ParseErrorType::MultipleReturnValues => {
                dino = dino.add_message("multiple return values are not supported", FormatterColor::Default);
            }
            ParseErrorType::NativePropertyAssignment => {
                dino = dino.add_message("cannot assign to a native property", FormatterColor::Default);
            }
            ParseErrorType::UnsupportedBackdotOperator => {
                dino = dino.add_message("unsupported backdot operator in this context", FormatterColor::Default);
            }
            ParseErrorType::MultipleMainFunction => {
                dino = dino.add_message("multiple 'main' functions defined", FormatterColor::Default)
                    .add_note("only one 'main' function is allowed per program", FormatterColor::Green);
            }
            ParseErrorType::PrefixIncrementDecrementNotSupported => {
                dino = dino.add_message("prefix '++' / '--' is not supported", FormatterColor::Default)
                    .add_note("use postfix form: 'i++' or 'i--'", FormatterColor::Green);
            }
            ParseErrorType::EmptyMatchComparison => {
                dino = dino.add_message("empty match comparison", FormatterColor::Default);
            }
            ParseErrorType::InvalidAssignmentTarget => {
                dino = dino.add_message("invalid assignment target", FormatterColor::Default);
            }
            ParseErrorType::UndefinedVariable { name, suggestion } => {
                dino = dino.add_message("undefined variable '", FormatterColor::Default)
                    .add_message_owned(name, FormatterColor::WhiteBold)
                    .add_message("'", FormatterColor::Default);
                if let Some(s) = suggestion {
                    dino = dino.add_note("did you mean '", FormatterColor::Green)
                        .add_note_owned(s, FormatterColor::WhiteBold)
                        .add_note("'?", FormatterColor::Green);
                }
            }
            ParseErrorType::UnknownType { name, suggestion } => {
                dino = dino.add_message("unknown type '", FormatterColor::Default)
                    .add_message_owned(name, FormatterColor::WhiteBold)
                    .add_message("'", FormatterColor::Default);
                if let Some(s) = suggestion {
                    dino = dino.add_note("did you mean '", FormatterColor::Green)
                        .add_note_owned(s, FormatterColor::WhiteBold)
                        .add_note("'?", FormatterColor::Green);
                }
            }
            ParseErrorType::MatchCorrespondenceError { expected_values, actual_values } => {
                dino = dino.add_message("match expected ", FormatterColor::Default)
                    .add_message_owned(expected_values.to_string(), FormatterColor::WhiteBold)
                    .add_message(" values, got ", FormatterColor::Default)
                    .add_message_owned(actual_values.to_string(), FormatterColor::WhiteBold);
            }
            ParseErrorType::NumericParse(err) => {
                dino = dino.add_message_owned(err.message.clone(), FormatterColor::Default);
                if let Some(help) = err.help {
                    dino = dino.add_note_owned(help, FormatterColor::Green);
                }
                if let Some(info) = err.info {
                    dino = dino.add_info_owned(info, FormatterColor::BrightBlueBold);
                }
            }
            ParseErrorType::Custom(msg) => {
                dino = dino.add_message_owned(msg, FormatterColor::Default);
            }
        }
        dino
    }
}
