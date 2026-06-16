// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/utils/string.rs
//  Desc:       Helper functions for string processing in lexer.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::types::{
    Token,
    TokenType,
};
use crate::compiler::lexer::types::{
    TokenList,
    LexerContext,
    LexerContextInfo,
    ParseMode,
};
use super::char_utils::{
    is_interpolation_ident_start,
    is_interpolation_ident,
};
use std::str;

#[inline(always)]
pub fn push_utf8_char(string_buf: &mut String, bytes: &[u8], start_idx: usize) -> usize {
    let bytes_len = bytes.len();
    
    if start_idx >= bytes_len {
        return 0;
    }
    
    let first_byte = bytes[start_idx];
    
    let seq_len = if first_byte & 0x80 == 0 {
        1 // ASCII
    // UTF-8
    } else if first_byte & 0xE0 == 0xC0 {
        2
    } else if first_byte & 0xF0 == 0xE0 {
        3
    } else if first_byte & 0xF8 == 0xF0 {
        4
    } else {
        1 // Invalid UTF-8, treat as single byte
    };
    
    if start_idx + seq_len > bytes_len {
        // Not enough bytes, treat as single byte
        string_buf.push(first_byte as char);
        return 1;
    }
    
    let utf8_slice = &bytes[start_idx..start_idx + seq_len];
    match str::from_utf8(utf8_slice) {
        Ok(s) => {
            string_buf.push_str(s);
            seq_len
        }
        Err(_) => {
            // Invalid UTF-8, treat as single byte
            string_buf.push(first_byte as char);
            1
        }
    }
}

#[inline(always)]
pub fn handle_escape(b: u8, string_buf: &mut String) {
    match b {
        b'n' => string_buf.push('\n'),
        b'r' => string_buf.push('\r'),
        b't' => string_buf.push('\t'),
        b'\\' => string_buf.push('\\'),
        b'"' => string_buf.push('"'),
        b'$' => string_buf.push('$'),
        c => {
            string_buf.push('\\');
            string_buf.push(c as char);
        }
    }
}

#[inline(always)]
pub fn handle_interpolation(
    bytes: &[u8],
    source: &str,
    i: &mut usize,
    line: u32,
    column: &mut u32,
    info: &LexerContextInfo,
    string_buf: &mut String,
    tokens: &mut TokenList,
    ctx: &mut LexerContext
) {
    if *i + 1 < bytes.len() {
        match bytes[*i + 1] {
            b'{' => {
                if !string_buf.is_empty() {
                    tokens.push(Token::string(string_buf.clone(), Some((info.from_line, info.from_column, string_buf.len()))), ctx);
                    string_buf.clear();
                }
                tokens.push(Token::delim(TokenType::LBraceExpr, Some((line, *column, 1))), ctx);
                ctx.push_mode(ParseMode::InFStringExpr(0));
                *i += 1;
                *column += 1;
                ctx.depth += 1;
            }
            c if is_interpolation_ident_start(c) => {
                let start = *i + 1;
                let mut end = start;
                while end < bytes.len() && is_interpolation_ident(bytes[end]) {
                    end += 1;
                }
                let var_name = &source[start..end];
                if !string_buf.is_empty() {
                    tokens.push(Token::string(string_buf.clone(), Some((info.from_line, info.from_column, string_buf.len()))), ctx);
                    string_buf.clear();
                }
                tokens.push(Token::identifier(var_name, Some((line, *column + 1, var_name.len()))), ctx);
                *i = end - 1;
                *column += (end - start) as u32;
            }
            _ => {
                string_buf.push('$');
            }
        }
    } else {
        string_buf.push('$');
    }
}
