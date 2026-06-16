// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/types/token.rs
//  Desc:       Token structure and related methods.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::types::{TokenType, Operator};
use super::token_value::TokenValue;

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub value: TokenValue,
    pub is_unary: bool,
    pub is_breaker: bool,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub length: Option<usize>,
}

impl Token {
    pub fn new(typ: TokenType, value: TokenValue, pos: Option<(u32, u32, usize)>) -> Self {
        let (line, column, length) = pos.unwrap_or((0, 0, 0));
        Self {
            typ,
            value,
            is_unary: false,
            is_breaker: false,
            line: Some(line),
            column: Some(column),
            length: Some(length),
        }
    }
    
    pub fn integer(val: i64, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::Integer, TokenValue::Integer(val), pos)
    }

    pub fn bigint(val: String, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::BigInt, TokenValue::BigInt(val), pos)
    }

    pub fn float(val: f64, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::Float, TokenValue::Float(val), pos)
    }

    pub fn bool(val: bool, pos: Option<(u32, u32, usize)>) -> Self {
        let length = if val { 4 } else { 5 }; // "true" o "false"
        Self::new(TokenType::Bool, TokenValue::Bool(val), pos.map(|(l, c, _)| (l, c, length)))
    }

    pub fn none(pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::Bool, TokenValue::None, pos.map(|(l, c, len)| (l, c, if len > 0 { len } else { 4 })))
    }

    pub fn nan(pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::Float, TokenValue::Float(f64::NAN), pos.map(|(l, c, _)| (l, c, 3)))
    }

    pub fn infi(pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::Float, TokenValue::Float(f64::INFINITY), pos.map(|(l, c, _)| (l, c, 8)))
    }

    pub fn string<T: Into<String>>(val: T, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(
            TokenType::String,
            TokenValue::String(val.into()),
            pos,
        )
    }

    pub fn identifier<T: Into<String>>(val: T, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(
            TokenType::Identifier,
            TokenValue::String(val.into()),
            pos,
        )
    }

    pub fn dot_access<T: Into<String>>(val: T, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(
            TokenType::DotAccess,
            TokenValue::String(val.into()),
            pos,
        )
    }

    pub fn native_access<T: Into<String>>(val: T, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(
            TokenType::NativeAccess,
            TokenValue::String(val.into()),
            pos,
        )
    }

    pub fn dollar_call<T: Into<String>>(val: T, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(
            TokenType::DollarCall,
            TokenValue::String(val.into()),
            pos,
        )
    }
    
    pub fn redirect(pos: Option<(u32, u32, usize)>) -> Self {
        Self::delim(TokenType::Redirect, pos)
    }

    pub fn class(pos: Option<(u32, u32, usize)>) -> Self {
        Self::delim(TokenType::Class, pos)
    }

    pub fn function(pos: Option<(u32, u32, usize)>) -> Self {
        Self::delim(TokenType::Function, pos)
    }

    pub fn op(operator: Operator, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(TokenType::Op(operator), TokenValue::None, pos)
    }

    pub fn delim(typ: TokenType, pos: Option<(u32, u32, usize)>) -> Self {
        Self::new(typ, TokenValue::None, pos)
    }

    pub fn end(pos: Option<(u32, u32, usize)>) -> Self {
        Self::delim(TokenType::End, pos)
    }

    pub fn comma() -> Self {
        Self::delim(TokenType::Comma, None)
    }

    pub fn with_unary(mut self, is_unary: bool) -> Self {
        self.is_unary = is_unary;
        self
    }

    pub fn with_breaker(mut self, is_breaker: bool) -> Self {
        self.is_breaker = is_breaker;
        self
    }
    
    pub fn position(&self) -> Option<(u32, u32)> {
        match (self.line, self.column) {
            (Some(line), Some(col)) => Some((line, col)),
            _ => None,
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.typ, TokenType::Op(_))
    }

    pub fn is_continuous(&self) -> bool {
        matches!(
            self.typ,
            TokenType::LParen | TokenType::LBrace | TokenType::LBracket |
            TokenType::LString | TokenType::LBraceExpr | TokenType::Comma | TokenType::Redirect |
            TokenType::Indent | TokenType::Function | TokenType::Class | 
            TokenType::For
        ) || (self.is_operator() && !self.is_breaker)
    }
    
    pub fn is_delimiter(&self) -> bool {
        matches!(
            self.typ,
            TokenType::RParen | TokenType::RBrace | TokenType::RBracket | 
            TokenType::RString | TokenType::RBraceExpr | TokenType::Comma | TokenType::End | 
            TokenType::Dedent
        ) || (self.is_operator() && !self.is_unary)
    }
}
