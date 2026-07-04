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
use crate::{
    utils::parsers::numeric::NumericParseError,
    formatter::{DinoError, FormatterColor},
};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RuntimeError {
    #[error("Invalid binary operation: {op} cannot be applied to {left} and {right}")]
    InvalidBinaryOperation { left: &'static str, op: &'static str, right: &'static str },
    #[error("Missing argument for '{0}'")]
    MissingArgument(&'static str),
    #[error("Wrong argument type for '{func}': expected {expected}")]
    WrongArgType { func: &'static str, expected: &'static str },
    #[error("Wrong argument count for '{func}': expected {expected}")]
    WrongArgCount { func: &'static str, expected: &'static str },
    #[error("Invalid argument value for '{func}': {message}")]
    InvalidArgumentValue { func: &'static str, message: &'static str },

    #[error("Value is NaN")]
    ValueIsNaN,
    #[error("Value is Infinity")]
    ValueIsInfinity,
    #[error("Numeric parse error")]
    NumericParse(NumericParseError),
    #[error("Expected numeric value, got '{0}'")]
    ExpectedNumeric(&'static str),
    #[error("Cannot convert '{from}' to {to}")]
    CannotConvert { from: &'static str, to: &'static str, help: Option<&'static str>, info: Option<&'static str> },
    
    #[error("Index out of bounds")]
    IndexOutOfBounds,
    #[error("Array index must be numeric")]
    ArrayIndexNotNumeric,
    #[error("Member access requires an object or array")]
    MemberAccessNotObject,
    #[error("Value is not iterable: {0:?}")]
    NotIterable(Option<&'static str>),
    #[error("Attempted to call '{0}' as a function")]
    CallNotFunction(&'static str),
    #[error("Expected instance of '{0}'")]
    ExpectedInstance(&'static str),
    #[error("Expected iterable for '{0}'")]
    ExpectedIterable(&'static str),
    #[error("Property '{0}' not found")]
    PropertyNotFound(String),

    #[error("Native function not found: {0}")]
    NativeFunctionNotFound(&'static str),
    #[error("Failed to read input: {0}")]
    ReadInputFailed(String),
    
    #[error("Panic: {message}")]
    Panic { message: String, help: Option<String>, info: Option<String> },

    // Generic
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Stack underflow")]
    StackUnderflow,
    #[error("Recursion depth limit exceeded")]
    RecursionLimitExceeded,
    #[error("Function not found")]
    FunctionNotFound,
    #[error("Constant index out of bounds")]
    ConstantIndexOutOfBounds,
    #[error("Invalid type index")]
    InvalidTypeIndex,
    #[error("Internal error: {0}")]
    InternalError(&'static str),
    #[error("Unknown opcode: 0x{0:02X}")]
    UnknownOpCode(u8),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl RuntimeError {
    pub fn to_dino_error(&self, line: u32, column: u32) -> DinoError<'static> {
        match self {
            RuntimeError::NotIterable(t) => {
                match t {
                    Some(type_name) => {
                        DinoError::new(line, column)
                            .add_message("'", FormatterColor::Default)
                            .add_message(type_name, FormatterColor::WhiteBold)
                            .add_message("' is not iterable", FormatterColor::Default)
                    }
                    None => {
                        DinoError::new(line, column)
                            .add_message("cannot iterate a plain object", FormatterColor::Default)
                    }
                }
            }
            RuntimeError::ArrayIndexNotNumeric => {
                DinoError::new(line, column)
                    .add_message("array index must be a numeric value", FormatterColor::Default)
                    .add_note("pass a numerically representable value", FormatterColor::Green)
            }
            RuntimeError::MemberAccessNotObject => {
                DinoError::new(line, column)
                    .add_message("member access requires an object or array", FormatterColor::Default)
            }
            RuntimeError::CallNotFunction(t) => {
                DinoError::new(line, column)
                    .add_message("attempted to call a '", FormatterColor::Default)
                    .add_message(t, FormatterColor::WhiteBold)
                    .add_message("' value as a function", FormatterColor::Default)
                    .add_note("check that the value is a function before calling it", FormatterColor::Green)
            }
            RuntimeError::InvalidBinaryOperation { left, op, right } => {
                DinoError::new(line, column)
                    .add_message("operator '", FormatterColor::Default)
                    .add_message(op, FormatterColor::WhiteBold)
                    .add_message("' cannot be applied to '", FormatterColor::Default)
                    .add_message(left, FormatterColor::WhiteBold)
                    .add_message("' and '", FormatterColor::Default)
                    .add_message(right, FormatterColor::WhiteBold)
                    .add_message("'", FormatterColor::Default)
                    .add_note("check that both operands have compatible types for this operation", FormatterColor::Green)
            }
            RuntimeError::NumericParse(err) => {
                let mut dino = DinoError::new(line, column).add_message_owned(err.message.clone(), FormatterColor::Default);
                if let Some(help) = &err.help {
                    dino = dino.add_note_owned(help.clone(), FormatterColor::Green);
                }
                if let Some(info) = &err.info {
                    dino = dino.add_info_owned(info.clone(), FormatterColor::BrightBlueBold);
                }
                dino
            }
            RuntimeError::CannotConvert { from, to, help, info } => {
                let mut dino = DinoError::new(line, column)
                    .add_message("cannot convert '", FormatterColor::Default)
                    .add_message(from, FormatterColor::WhiteBold)
                    .add_message("' to ", FormatterColor::Default)
                    .add_message(to, FormatterColor::WhiteBold);
                if let Some(h) = help {
                    dino = dino.add_note(h, FormatterColor::Green);
                }
                if let Some(i) = info {
                    dino = dino.add_info(i, FormatterColor::BrightBlueBold);
                }
                dino
            }
            RuntimeError::ExpectedNumeric(t) => {
                DinoError::new(line, column)
                    .add_message("expected numeric argument, got '", FormatterColor::Default)
                    .add_message(t, FormatterColor::WhiteBold)
                    .add_message("'", FormatterColor::Default)
                    .add_note("pass a numerically representable value", FormatterColor::Green)
            }
            RuntimeError::ExpectedInstance(type_name) => {
                DinoError::new(line, column)
                    .add_message("expected a '", FormatterColor::Default)
                    .add_message(type_name, FormatterColor::WhiteBold)
                    .add_message("' instance", FormatterColor::Default)
                    .add_info("this method can only be called on '", FormatterColor::Green)
                    .add_info(type_name, FormatterColor::WhiteBold)
                    .add_info("' objects", FormatterColor::Green)
            }
            RuntimeError::IndexOutOfBounds => {
                DinoError::new(line, column)
                    .add_message("index out of bounds", FormatterColor::Default)
                    .add_note("check that the index is within the valid range for the collection", FormatterColor::Green)
            }
            RuntimeError::NativeFunctionNotFound(name) => {
                DinoError::new(line, column)
                    .add_message("native function not found: ", FormatterColor::Default)
                    .add_message(name, FormatterColor::WhiteBold)
            }
            RuntimeError::PropertyNotFound(name) => {
                DinoError::new(line, column)
                    .add_message("property '", FormatterColor::Default)
                    .add_message_owned(name.clone(), FormatterColor::WhiteBold)
                    .add_message("' not found", FormatterColor::Default)
                    .add_note("ensure the property name is spelled correctly", FormatterColor::Green)
            }
            RuntimeError::ReadInputFailed(err) => {
                DinoError::new(line, column)
                    .add_message("failed to read input", FormatterColor::Default)
                    .add_info_owned(err.clone(), FormatterColor::Green)
            }
            RuntimeError::MissingArgument(func) => {
                DinoError::new(line, column)
                    .add_message("'", FormatterColor::Default)
                    .add_message(func, FormatterColor::WhiteBold)
                    .add_info("()' requires at least one argument", FormatterColor::Default)
            }
            RuntimeError::WrongArgType { func, expected } => {
                DinoError::new(line, column)
                    .add_message("'", FormatterColor::Default)
                    .add_message(func, FormatterColor::WhiteBold)
                    .add_message("()' expects a ", FormatterColor::Default)
                    .add_message(expected, FormatterColor::WhiteBold)
                    .add_message(" argument", FormatterColor::Default)
            }
            RuntimeError::WrongArgCount { func, expected } => {
                DinoError::new(line, column)
                    .add_message("'", FormatterColor::Default)
                    .add_message(func, FormatterColor::WhiteBold)
                    .add_message("()' expects ", FormatterColor::Default)
                    .add_message(expected, FormatterColor::WhiteBold)
                    .add_message(" argument(s)", FormatterColor::Default)
            }
            RuntimeError::InvalidArgumentValue { func, message } => {
                DinoError::new(line, column)
                    .add_message("'", FormatterColor::Default)
                    .add_message(func, FormatterColor::WhiteBold)
                    .add_message("()' invalid argument", FormatterColor::Default)
                    .add_info(message, FormatterColor::Green)
            }
            RuntimeError::ExpectedIterable(func) => {
                DinoError::new(line, column)
                    .add_message("'", FormatterColor::Default)
                    .add_message(func, FormatterColor::WhiteBold)
                    .add_message("()' expects an iterable object", FormatterColor::Default)
            }
            RuntimeError::ValueIsNaN => {
                DinoError::new(line, column)
                    .add_message("Not a Number (NaN) value detected", FormatterColor::Default)
                    .add_note("NaN is not allowed in operations that require a finite value", FormatterColor::Green)
                    .add_info("NaN examples: 0/0, infi/infi", FormatterColor::BrightBlueBold)
            }
            RuntimeError::ValueIsInfinity => {
                DinoError::new(line, column)
                    .add_message("Infinity value detected", FormatterColor::Default)
                    .add_note("Infinity is not allowed in operations that require a finite value", FormatterColor::Green)
                    .add_info("Infinity examples: 1/0, very large numbers", FormatterColor::BrightBlueBold)
            }
            RuntimeError::Panic { message, help, info } => {
                let mut dino = DinoError::new(line, column).add_message_owned(message.clone(), FormatterColor::Default);
                if let Some(h) = help {
                    dino = dino.add_note_owned(h.clone(), FormatterColor::Green);
                }
                if let Some(i) = info {
                    dino = dino.add_info_owned(i.clone(), FormatterColor::BrightBlueBold);
                }
                dino
            }

            // Generic
            RuntimeError::DivisionByZero => {
                DinoError::new(line, column)
                    .add_message("division by zero", FormatterColor::Default)
                    .add_note("check divisor before performing division", FormatterColor::Green)
            }
            RuntimeError::StackUnderflow => {
                DinoError::new(line, column)
                    .add_message("stack underflow", FormatterColor::Default)
                    .add_note("check stack operations for proper balance", FormatterColor::Green)
            }
            RuntimeError::RecursionLimitExceeded => {
                DinoError::new(line, column)
                    .add_message("recursion depth limit exceeded", FormatterColor::Default)
            }
            RuntimeError::FunctionNotFound => {
                DinoError::new(line, column)
                    .add_message("function not found", FormatterColor::Default)
            }
            RuntimeError::ConstantIndexOutOfBounds => {
                DinoError::new(line, column)
                    .add_message("constant index out of bounds", FormatterColor::Default)
            }
            RuntimeError::InvalidTypeIndex => {
                DinoError::new(line, column)
                    .add_message("invalid type index", FormatterColor::Default)
            }
            RuntimeError::InternalError(msg) => {
                DinoError::new(line, column)
                    .add_message(msg, FormatterColor::Default)
                    .add_info("this is likely a bug, please report it", FormatterColor::Green)
            }
            RuntimeError::UnknownOpCode(op) => {
                let opcode_str = format!("{:02X}", op);
                DinoError::new(line, column)
                    .add_message("unknown opcode: 0x", FormatterColor::Default)
                    .add_message_owned(opcode_str, FormatterColor::WhiteBold)
            }
        }
    }
}
