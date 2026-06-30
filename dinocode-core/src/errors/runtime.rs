// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/errors/runtime.rs
//  Desc:       Runtime error types.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use thiserror::Error;
use colored::Colorize;
use crate::utils::parsers::numeric::NumericParseError;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeErrorType {
    PlainObjectNotIterable,
    NotIterable(String),
    ArrayIndexNotNumeric,
    MemberAccessNotObject,
    CallNotFunction(String),
    InvalidBinaryOperation { left: String, op: String, right: String },

    NumericParse(NumericParseError),
    CannotConvert { from: String, to: String, help: Option<String>, info: Option<String> },

    ExpectedInteger(String),
    ExpectedNumber(String),
    ExpectedNumericValue,
    ExpectedArrayInstance,
    ExpectedObjectInstance,
    ExpectedStringInstance,
    ExpectedRangeInstance,
    ExpectedTimeInstance,

    IndexOutOfBounds,
    NativeFunctionNotFound(String),
    PropertyNotFound(String),
    ReadInputFailed(String),

    MissingArgument(String),
    WrongArgType { func: String, expected: String },
    WrongArgCount { func: String, expected: String },
    InvalidArgumentValue { func: String, message: String },
    ExpectedArrayOrIterator(String),
    ExpectedIterator(String),
    ValueIsNaN,
    ValueIsInfinity,
    Panic { message: String, help: Option<String>, info: Option<String> },
}

impl RuntimeErrorType {
    pub fn get_info(&self) -> (String, Option<String>, Option<String>) {
        match self {
            RuntimeErrorType::PlainObjectNotIterable =>
                ("cannot iterate a plain object without an iterator method".to_string(),
                 Some("implement an iterator method or convert to an Array first".to_string()), None),

            RuntimeErrorType::NotIterable(t) =>
                (format!("'{}' is not iterable", t),
                 Some("call iter() on the value or pass an Array or iterator object".to_string()), None),

            RuntimeErrorType::ArrayIndexNotNumeric =>
                ("array index must be a numeric value".to_string(),
                 Some("use an integer expression as the index".to_string()), None),

            RuntimeErrorType::MemberAccessNotObject =>
                ("member access requires an object or array".to_string(),
                 Some("check that the value is not none before accessing its members".to_string()), None),

            RuntimeErrorType::CallNotFunction(t) =>
                (format!("attempted to call a '{}' value as a function", t),
                 Some("check that the value is a function before calling it".to_string()), None),

            RuntimeErrorType::InvalidBinaryOperation { left, op, right } =>
                (format!("operator '{}' cannot be applied to '{}' and '{}'", op, left, right),
                 Some("check that both operands have compatible types for this operation".to_string()), None),

            RuntimeErrorType::NumericParse(err) =>
                (err.message.clone(), err.help.clone(), err.info.clone()),

            RuntimeErrorType::CannotConvert { from, to, help, info } =>
                (format!("cannot convert '{}' to {}", from, to),
                 help.clone(),
                 info.clone()),

            RuntimeErrorType::ExpectedInteger(t) =>
                (format!("expected integer argument, got '{}'", t),
                 Some("pass an int or float value; floats will be truncated".to_string()), None),

            RuntimeErrorType::ExpectedNumber(t) =>
                (format!("expected numeric argument, got '{}'", t),
                 Some("pass an int or float value".to_string()), None),

            RuntimeErrorType::ExpectedNumericValue =>
                ("expected a numeric value for this operation".to_string(),
                 Some("check that the value is of type int or float".to_string()), None),

            RuntimeErrorType::ExpectedArrayInstance =>
                ("expected an Array instance".to_string(),
                 Some("this method can only be called on Array objects".to_string()), None),

            RuntimeErrorType::ExpectedObjectInstance =>
                ("expected an Object instance".to_string(),
                 Some("this method can only be called on Objects".to_string()), None),

            RuntimeErrorType::ExpectedStringInstance =>
                ("expected a String instance".to_string(),
                 Some("this method can only be called on String objects".to_string()), None),

            RuntimeErrorType::ExpectedRangeInstance =>
                ("expected a Range instance".to_string(),
                 Some("this method can only be called on Range objects".to_string()), None),

            RuntimeErrorType::ExpectedTimeInstance =>
                ("expected a Time instance".to_string(),
                 Some("this method can only be called on Time objects".to_string()), None),

            RuntimeErrorType::IndexOutOfBounds =>
                ("index out of bounds".to_string(),
                 Some("check that the index is within the valid range for the collection".to_string()), None),

            RuntimeErrorType::NativeFunctionNotFound(name) =>
                (format!("native function not found: {}", name),
                 Some("ensure the native function is registered correctly".to_string()), None),

            RuntimeErrorType::PropertyNotFound(name) =>
                (format!("property '{}' not found", name),
                 Some("ensure the property name is spelled correctly or use the '.' accessor safely".to_string()), None),

            RuntimeErrorType::ReadInputFailed(err) =>
                (format!("failed to read input: {}", err),
                 Some("ensure the terminal supports standard input and is still open".to_string()), None),

            RuntimeErrorType::MissingArgument(func) =>
                (format!("'{}()' requires at least one argument", func),
                 Some(format!("call {}() with the required arguments", func)), None),

            RuntimeErrorType::WrongArgType { func, expected } =>
                (format!("'{}()' expects a {} argument", func, expected),
                 Some(format!("pass a {} value to {}()", expected, func)), None),

            RuntimeErrorType::WrongArgCount { func, expected } =>
                (format!("'{}()' expects {} argument(s)", func, expected),
                 Some(format!("ensure you are passing the correct number of arguments to {}()", func)), None),

            RuntimeErrorType::InvalidArgumentValue { func, message } =>
                (format!("'{}()' invalid argument: {}", func, message),
                 Some(format!("check the value passed to {}() matches documentation requirements", func)), None),

            RuntimeErrorType::ExpectedArrayOrIterator(func) =>
                (format!("'{}()' only supports arrays and iterators", func),
                 Some("pass an array or a valid iterator object".to_string()), None),

            RuntimeErrorType::ExpectedIterator(func) =>
                (format!("'{}()' expects an iterator", func),
                 Some("ensure you are passing a value returned by iter() or another iterator provider".to_string()), None),

            RuntimeErrorType::ValueIsNaN =>
                ("Not a Number (NaN) value detected".to_string(),
                 Some("NaN is not allowed in operations that require a finite value".to_string()),
                 Some("NaN is the product of an invalid numeric operation like: 0/0, sqrt(-1), Infinity/Infinity".to_string())),

            RuntimeErrorType::ValueIsInfinity =>
                ("Infinity value detected".to_string(),
                 Some("Infinity cannot be coerced to finite numeric types".to_string()), 
                 Some("Infinity examples: 1/0, log(0), very large numbers".to_string())),

            RuntimeErrorType::Panic { message, help, info } =>
                (message.clone(),
                 help.clone(),
                 info.clone()),
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RuntimeError {
    #[error("Type Mismatch")]
    TypeMismatch,
    #[error("Division by Zero")]
    DivisionByZero,
    #[error("Stack Underflow")]
    StackUnderflow,
    #[error("Stack Overflow: {0}")]
    StackOverflow(String),
    #[error("Type Error: {0}")]
    TypeError(String),
    #[error("Reference Error: {0}")]
    ReferenceError(String),
    #[error("Syntax Error: {0}")]
    SyntaxError(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
    #[error("Invalid Operation: {0}")]
    InvalidOperation(String),
    #[error("Import Error: {0}")]
    ImportError(String),
    #[error("Not Implemented: {0}")]
    NotImplemented(String),
    #[error("Unknown OpCode: 0x{0:02X}")]
    UnknownOpCode(u8),
    #[error("Undefined Variable: {0}")]
    UndefinedVariable(String),
    #[error("Index Error: {0}")]
    IndexError(String),
    #[error("Function Overridden: {0}")]
    FunctionOverridden(String),
    #[error("Multiple Main Functions: Only one 'main' function is allowed")]
    MultipleMainFunction,
    #[error("Runtime Error")]
    Typed(RuntimeErrorType),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl From<&str> for RuntimeError {
    fn from(s: &str) -> Self {
        RuntimeError::InternalError(s.to_string())
    }
}

impl From<String> for RuntimeError {
    fn from(s: String) -> Self {
        RuntimeError::InternalError(s)
    }
}

impl RuntimeError {
    pub fn get_info(&self) -> (String, Option<String>, Option<String>) {
        match self {
            RuntimeError::Typed(t) => t.get_info(),
            RuntimeError::TypeMismatch =>
                ("type mismatch between values".to_string(), Some("check that both values have compatible types".to_string()), None),
            RuntimeError::DivisionByZero =>
                ("division by zero".to_string(), Some("check divisor before performing division".to_string()), None),
            RuntimeError::StackUnderflow =>
                ("stack underflow: not enough values on stack".to_string(), Some("check stack operations for proper balance".to_string()), None),
            RuntimeError::StackOverflow(msg) =>
                (format!("stack overflow: {}", msg), Some("reduce recursion depth or avoid large nested structures".to_string()), None),
            RuntimeError::InvalidOperation(msg) =>
                (format!("invalid operation: {}", msg), None, None),
            RuntimeError::ReferenceError(msg) =>
                (format!("reference error: {}", msg), Some("check if the variable or object exists before using it".to_string()), None),
            RuntimeError::TypeError(msg) =>
                (format!("type error: {}", msg), Some("check value types before this operation".to_string()), None),
            RuntimeError::InternalError(msg) =>
                (format!("internal VM error: {}", msg), Some("this is likely a bug — please report it".to_string()), None),
            RuntimeError::NotImplemented(msg) =>
                (format!("not implemented: {}", msg), Some("this feature is not yet available".to_string()), None),
            RuntimeError::UnknownOpCode(op) =>
                (format!("unknown opcode: 0x{:02X}", op), Some("invalid bytecode instruction".to_string()), None),
            RuntimeError::UndefinedVariable(msg) =>
                (format!("undefined variable: {}", msg), Some("check variable spelling and scope".to_string()), None),
            RuntimeError::IndexError(msg) =>
                (format!("index error: {}", msg), Some("check array bounds and object keys".to_string()), None),
            RuntimeError::FunctionOverridden(msg) =>
                (format!("function overridden: {}", msg), Some("check for naming conflicts with function definitions".to_string()), None),
            RuntimeError::MultipleMainFunction =>
                ("only one 'main' function is allowed".to_string(), Some("remove the duplicate main function definition".to_string()), None),
            RuntimeError::SyntaxError(msg) =>
                (format!("syntax error: {}", msg), Some("check code syntax".to_string()), None),
            RuntimeError::ImportError(msg) =>
                (format!("import error: {}", msg), Some("check import paths and file existence".to_string()), None),
        }
    }

    pub fn format_pretty(&self) -> String {
        let (message, suggestion, details) = self.get_info();

        let error_label = "runtime-error".red().bold();
        let mut output = format!("\n{}: {}\n\n", error_label, message.bright_white());

        let label_padding = "     ";

        if let Some(suggestion) = suggestion {
            output += &format!("\n{}{}: {}\n", label_padding, "help".green().bold(), suggestion);
        }

        if let Some(details) = details {
            output += &format!("\n{}{}: {}\n", label_padding, "note".bright_blue().bold(), details);
        }

        output
    }
}
