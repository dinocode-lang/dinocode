// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/errors/types/error_info.rs
//  Desc:       Error information and severity types.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    shared::errors::types::base_error::{BaseErrorType, ErrorCategory, LexErrorType},
    compiler::parser::errors::ParseErrorType,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub category: ErrorCategory,
    pub error_type: BaseErrorType,
    pub message: String,
    pub suggestion: Option<String>,
    pub details: Option<String>,
    pub severity: ErrorSeverity,
}

impl ErrorInfo {
    pub fn new(
        category: ErrorCategory,
        error_type: BaseErrorType,
        message: String,
        suggestion: Option<String>,
        details: Option<String>,
        severity: ErrorSeverity,
    ) -> Self {
        Self {
            category,
            error_type,
            message,
            suggestion,
            details,
            severity,
        }
    }
    
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
    
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    pub fn custom(category: ErrorCategory, message: String, suggestion: Option<String>, details: Option<String>) -> Self {
        Self {
            category,
            error_type: BaseErrorType::Custom(message.clone()),
            message,
            suggestion,
            details,
            severity: ErrorSeverity::Error,
        }
    }
}

pub type LexErrorInfo = ErrorInfo;
pub type ParseErrorInfo = ErrorInfo;

pub fn lex_error(error_type: LexErrorType, message: String, suggestion: Option<String>, details: Option<String>) -> LexErrorInfo {
    ErrorInfo::new(
        ErrorCategory::Lexer,
        error_type,
        message,
        suggestion,
        details,
        ErrorSeverity::Error,
    )
}

pub fn parse_error(error_type: ParseErrorType, message: String, suggestion: Option<String>, details: Option<String>) -> ParseErrorInfo {
    ErrorInfo::new(
        ErrorCategory::Parser,
        BaseErrorType::Parser(error_type),
        message,
        suggestion,
        details,
        ErrorSeverity::Error,
    )
}

impl BaseErrorType {
    pub fn get_severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }
}
