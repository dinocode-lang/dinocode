// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/utils/char_utils.rs
//  Desc:       Character utility functions for lexical analysis
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::types::{
    SPECIAL_CHARS,
    SPECIAL_CHARS_DOLLAR,
};
use crate::compiler::lexer::types::OPERATOR_INFO;

#[inline(always)]
pub fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || SPECIAL_CHARS_DOLLAR.contains(&b)
}

#[inline(always)]
pub fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || SPECIAL_CHARS_DOLLAR.contains(&b)
}

#[inline(always)]
pub fn is_interpolation_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || SPECIAL_CHARS.contains(&b)
}

#[inline(always)]
pub fn is_interpolation_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || SPECIAL_CHARS.contains(&b)
}

#[inline(always)]
pub fn is_op_start(ch: u8) -> Option<(&'static [u8], usize)> {
    let ch = ch as usize;
    if ch < 128 {
        let info = OPERATOR_INFO[ch];
        if info.1 > 0 {
            return Some(info);
        }
    }
    None
}

#[inline(always)]
pub fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

#[inline(always)]
pub fn is_hex_digit(b: u8) -> bool {
    match b {
        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => true,
        _ => false,
    }
}

#[inline(always)]
pub fn is_binary_digit(b: u8) -> bool {
    match b {
        b'0' | b'1' => true,
        _ => false,
    }
}

#[inline(always)]
pub fn is_octal_digit(b: u8) -> bool {
    match b {
        b'0'..=b'7' => true,
        _ => false,
    }
}

#[inline(always)]
pub fn is_sci_exp(b: u8) -> bool {
    match b {
        b'e' | b'E' => true,
        _ => false,
    }
}

#[inline(always)]
pub fn is_sci_digit(b: u8) -> bool {
    b.is_ascii_digit() || b == b'-' || b == b'+'
}

#[inline(always)]
pub fn is_bigint_posfix(b: u8) -> bool {
    b == b'n'
}