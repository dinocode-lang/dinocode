// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/utils.rs
//  Desc:       Utility functions for numeric parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::borrow::Cow;
use crate::types::DinoRef;
use super::error::{
    NumericParseError,
    error_i64, 
    error_hex, 
    error_bin, 
    error_octal
};

#[inline(always)]
pub const fn is_valid_int(v: i64) -> bool {
    v >= DinoRef::INT_MIN && v <= DinoRef::INT_MAX
}

#[inline]
fn is_whitespace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r')
}

#[inline]
pub fn trim_whitespace(bytes: &[u8]) -> &[u8] {
    let start = match bytes.iter().position(|&c| !is_whitespace(c)) {
        Some(pos) => pos,
        None => return &[],
    };
    let end = bytes[start..].iter().rposition(|&c| !is_whitespace(c)).unwrap() + 1;

    &bytes[start..start + end]
}

#[inline]
pub fn filter(bytes: &[u8], ch: u8) -> Cow<'_, [u8]> {
    if let Some(pos) = bytes.iter().position(|&c| c == ch) {
        let mut filtered = Vec::with_capacity(bytes.len() - 1);
        filtered.extend_from_slice(&bytes[..pos]);
        filtered.extend(bytes[pos..].iter().filter(|&&c| c != ch));
        Cow::Owned(filtered)
    } else {
        Cow::Borrowed(bytes)
    }
}

#[inline]
pub fn clean_number(bytes: &[u8]) -> Cow<'_, [u8]> {
    let trimmed = trim_whitespace(bytes);
    filter(trimmed, b'_')
}

#[inline(always)]
pub fn parse_i64_hex(bytes: &[u8]) -> Result<i64, NumericParseError> {
    if bytes.is_empty() {
        return Err(error_hex(bytes));
    }

    let mut result: i64 = 0;
    for &byte in bytes {
        let digit = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => return Err(error_hex(bytes)),
        };

        if result > (i64::MAX - digit as i64) / 16 {
            return Err(error_hex(bytes));
        }

        result = result * 16 + digit as i64;
    }

    Ok(result)
}

#[inline(always)]
pub fn parse_i64_bin(bytes: &[u8]) -> Result<i64, NumericParseError> {
    if bytes.is_empty() {
        return Err(error_bin(bytes));
    }

    let mut result: i64 = 0;
    for &byte in bytes {
        let digit = match byte {
            b'0' => 0,
            b'1' => 1,
            _ => return Err(error_bin(bytes)),
        };

        if result > (i64::MAX - digit as i64) / 2 {
            return Err(error_bin(bytes));
        }

        result = result * 2 + digit as i64;
    }

    Ok(result)
}

#[inline(always)]
pub fn parse_i64_decimal(bytes: &[u8]) -> Result<i64, NumericParseError> {
    if bytes.is_empty() {
        return Err(error_i64(bytes));
    }

    let mut result: i64 = 0;

    for &byte in bytes {
        if byte < b'0' || byte > b'9' {
            return Err(error_i64(bytes));
        }
        let digit = (byte - b'0') as i64;

        if result > (i64::MAX - digit) / 10 {
            return Err(error_i64(bytes));
        }

        result = result * 10 + digit;
    }

    Ok(result)
}

pub fn parse_i64_octal(bytes: &[u8]) -> Result<i64, NumericParseError> {
    if bytes.is_empty() {
        return Err(error_octal(bytes));
    }

    let mut result: i64 = 0;
    for &byte in bytes {
        let digit = match byte {
            b'0'..=b'7' => byte - b'0',
            _ => return Err(error_octal(bytes)),
        };

        if result > (i64::MAX - digit as i64) / 8 {
            return Err(error_octal(bytes));
        }

        result = result * 8 + digit as i64;
    }

    Ok(result)
}

pub fn parse_bigint_digits_bytes(digits: &[u8], base: u32, is_negative: bool) -> Result<Vec<u8>, NumericParseError> {
    let mut working: Vec<u8> = digits.to_vec();
    let mut result: Vec<u8> = Vec::new();

    // Validate all digits first
    for &b in &working {
        let valid = match base {
            2  => b == b'0' || b == b'1',
            8  => b >= b'0' && b <= b'7',
            10 => b.is_ascii_digit(),
            16 => b.is_ascii_hexdigit(),
            _  => return Err(NumericParseError::new(
                format!("invalid base '{}'", base),
                None,
                Some("must be 2, 8, 10, or 16".to_string()),
            )),
        };
        if !valid {
            return Err(NumericParseError::new("invalid integer format".to_string(), None, None));
        }
    }

    while !working.is_empty() && working.iter().any(|&b| b != b'0') {
        let mut remainder: u32 = 0;
        let mut next: Vec<u8> = Vec::new();

        for &b in &working {
            let digit = if b.is_ascii_digit() {
                (b - b'0') as u32
            } else {
                (b.to_ascii_lowercase() - b'a') as u32 + 10
            };
            let value = remainder * base + digit;
            let q = value / 256;
            remainder = value % 256;
            if !next.is_empty() || q > 0 {
                // Rebuild quotient digits in the same base
                let ch = if q < 10 { b'0' + q as u8 } else { b'a' + q as u8 - 10 };
                next.push(ch);
            }
        }

        result.push(remainder as u8);
        working = next;
    }

    if result.is_empty() { result.push(0); }
    result.reverse();
    result.insert(0, if is_negative { 0x01 } else { 0x00 });
    Ok(result)
}
