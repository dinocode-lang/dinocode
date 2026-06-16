// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/errors.rs
//  Desc:       Parse error types and definition.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
//  ═══════════════════════════════════════════════════════════

use thiserror::Error;
use crate::shared::types::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseErrorType {
    ExpectedRightParen,
    ExpectedLeftParen,
    ExpectedRightBrace,
    ExpectedLeftBrace,
    ExpectedRightBracket,
    ExpectedLeftBracket,
    ExpectedStringTerminator,
    ExpectedStringInitializer,
    ExpectedRightBraceExpr,
    ExpectedLeftBraceExpr,
    ExpectedExpression(String),

    MismatchedDelimiter(String),

    ExpectedIdentifier,
    MismatchedParentheses,
    MismatchedBrackets,
    MismatchedBraces,
    FunctionNotFound,
    FunctionNotInScope,
    CannotAccessParentScopeFunction,
    MultipleReturnValues,
    ReturnOutsideFunction,
    BreakOutsideLoop,
    ContinueOutsideLoop,
    ExpectedClassName,
    UnexpectedDollarCall,
    ExpectedFunctionName,
    UnexpectedTokenInParameterList,
    InvalidAssignmentTarget,
    InvalidTypeIndexForAsOperator,
    InvalidTypeIndexForIsOperator,
    UnexpectedIsOperator,
    UnexpectedInOperator,
    NativePropertyAssignment,
    UnsupportedBackdotOperator,
    ExpectedTypeIdentifierAfterIs,
    ExpectedTypeIdentifierAfterAs,
    UnexpectedOperatorInDollarCall,
    FunctionNotFoundInScope,
    UndefinedVariable { name: String, suggestion: Option<String> },
    UnknownType { name: String, suggestion: Option<String> },
    MultipleMainFunction,
    InvalidUnaryOperator,
    InvalidBinaryOperator,
    PrefixIncrementDecrementNotSupported,
    MatchCorrespondenceError { expected_values: usize, actual_values: usize },
    EmptyMatchComparison,
    ExpectedIndentedBlock(TokenType),
    
    Custom(String),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    Parse(String),
}

impl ParseError {
    pub fn print(&self) {
        match self {
            ParseError::Parse(msg) => eprint!("{}", msg),
        }
    }
}
