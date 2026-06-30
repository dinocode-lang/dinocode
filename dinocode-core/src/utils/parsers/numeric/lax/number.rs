// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/lax/number.rs
//  Desc:       Lax Number parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::parsers::numeric::{
    parse::{
        ParseNumericLax,
        parse_lax
    },
    error::{
        NumericParseError,
        error_i64
    },
    types::number::Number,
    utils::{
        trim_whitespace,
        is_valid_int
    },
};

impl ParseNumericLax for Number {
    fn parse_lax(input: impl AsRef<[u8]>, base: Option<u32>) -> Result<Self, NumericParseError> {
        let bytes = input.as_ref();
        let bytes = trim_whitespace(bytes);
        
        if bytes.is_empty() {
            return Err(error_i64(bytes));
        }

        let limit = bytes.len().min(40);
        let numeric = &bytes[..limit];
        
        let has_sign = {
            let b = numeric[0];
            b == b'+' || b == b'-'
        };
        let digits_len = bytes.len() - if has_sign { 1 } else { 0 };

        let is_float = digits_len > 14 || numeric.iter().any(|&b| b == b'.' || b == b'e' || b == b'E');

        if is_float {
            parse_lax::<f64>(bytes, base).map(Number::Float)
        } else {
            let val = parse_lax::<i64>(bytes, base)?;
            if is_valid_int(val) {
                Ok(Number::Int(val))
            } else {
                Err(error_i64(bytes))
            }
        }
    }
}
