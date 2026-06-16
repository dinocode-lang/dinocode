// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/types/token_type.rs
//  Desc:       Token type definitions..
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use super::operators::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Integer,
    BigInt,
    Float,
    Bool,
    String,
    Identifier,
    Op(Operator),
    Comma,
    End,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LString,
    RString,
    LBraceExpr,
    RBraceExpr,

    Redirect,
    DotAccess,
    NativeAccess,
    DollarCall,

    Indent,
    Dedent,
    If,
    Elif,
    Else,
    While,
    For,
    Break,
    Continue,

    Function,
    Return,
    Class,

    Is,
    IsMatch,
    In,
    InMatch,
}
