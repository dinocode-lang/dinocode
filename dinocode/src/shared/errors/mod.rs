pub mod types;
pub mod services;
pub mod utils;

pub use types::{
    ErrorCategory,
    ErrorSeverity,
    BaseErrorType,
    ErrorInfo,
    LexErrorType,
    LexErrorInfo,
    ParseErrorInfo,
    lex_error,
    parse_error,
};
pub use crate::compiler::lexer::errors::LexError;
pub use crate::compiler::parser::errors::{ParseError, ParseErrorType};
pub use services::{
    ErrorService,
    get_global_service,
    resolve_error,
    resolve_custom_error,
    resolve_lex_error,
    resolve_parse_error,
    resolve_custom_lex_error,
};
pub use utils::{
    pretty_format_error,
    pretty_lex_error_from_info,
    pretty_lex_error,
    pretty_lex_error_custom,
    pretty_parse_error_from_info,
    pretty_parse_error,
    pretty_runtime_error_from_info,
};
