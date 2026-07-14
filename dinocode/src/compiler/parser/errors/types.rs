// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/errors/types.rs
//  Desc:       Parser errors types
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
//  ═══════════════════════════════════════════════════════════

use dinocode_core::utils::parsers::numeric::NumericParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseErrorType {
    ExpectedToken(&'static str),
    ExpectedIdentifier(&'static str),
    ExpectedExpression(&'static str),
    ExpectedIndentedBlock(&'static str),
    MismatchedDelimiter(&'static str),
    InvalidControlFlow(&'static str),
    FunctionResolutionError(&'static str),
    UnexpectedToken(&'static str),
    InvalidOperator(&'static str),
    NumericParse(NumericParseError),
    Custom(String),

    MultipleReturnValues,
    NativePropertyAssignment,
    UnsupportedBackdotOperator,
    MultipleMainFunction,
    PrefixIncrementDecrementNotSupported,
    EmptyMatchComparison,
    InvalidAssignmentTarget,
    UndefinedVariable { name: String, suggestion: Option<String> },
    UnknownType { name: String, suggestion: Option<String> },
    MatchCorrespondenceError { expected_values: usize, actual_values: usize },
    MissingTokenValue,
}
