// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/lax/bigint.rs
//  Desc:       Lax BigInt parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::{
    bigint::BigInt,
    parsers::numeric::{
        parse::ParseNumericLax,
        error::NumericParseError,
        utils::{
            clean_number,
            parse_bigint_digits_bytes,
        },
    },
};

impl ParseNumericLax for BigInt<'static> {
    fn parse_lax(input: impl AsRef<[u8]>, base: Option<u32>) -> Result<Self, NumericParseError> {
        let raw = input.as_ref();
        let bytes = clean_number(raw);

        if bytes.is_empty() {
            return Err(NumericParseError::new("cannot convert empty string to number".into(), None, None));
        }

        let (is_negative, content) = if bytes[0] == b'-' {
            (true, &bytes[1..])
        } else if bytes[0] == b'+' {
            (false, &bytes[1..])
        } else {
            (false, bytes.as_ref())
        };

        if content.is_empty() {
            return Err(NumericParseError::new("invalid bigint format".into(), None, None));
        }

        if let Some(explicit_base) = base {
            return parse_bigint_digits_bytes(content, explicit_base, is_negative)
                .map(BigInt::new);
        }

        let (resolved_base, digits) = if content.len() >= 2 && content[0] == b'0' {
            match content[1] {
                b'x' | b'X' => {
                    let d = &content[2..];
                    if d.is_empty() {
                        return Err(NumericParseError::new("invalid bigint format".into(), None, None));
                    }
                    (16u32, d)
                }
                b'b' | b'B' => {
                    let d = &content[2..];
                    if d.is_empty() {
                        return Err(NumericParseError::new("invalid bigint format".into(), None, None));
                    }
                    (2u32, d)
                }
                b'o' | b'O' => {
                    let d = &content[2..];
                    if d.is_empty() {
                        return Err(NumericParseError::new("invalid bigint format".into(), None, None));
                    }
                    (8u32, d)
                }
                _ => (10u32, content),
            }
        } else {
            (10u32, content)
        };

        parse_bigint_digits_bytes(digits, resolved_base, is_negative)
            .map(BigInt::new)
    }
}
