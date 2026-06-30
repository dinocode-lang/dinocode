// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/strict/number.rs
//  Desc:       Strict Number parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::parsers::numeric::{
    parse::{
        ParseNumeric,
        parse
    },
    error::{
        NumericParseError,
        error_i64
    },
    types::number::Number,
    utils::is_valid_int,
};

impl ParseNumeric for Number {
    #[inline(always)]
    fn parse(input: impl AsRef<[u8]>) -> Result<Self, NumericParseError> {
        let bytes = input.as_ref();
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
            parse::<f64>(bytes).map(Number::Float)
        } else {
            let val = parse::<i64>(bytes)?;
            if is_valid_int(val) {
                Ok(Number::Int(val))
            } else {
                Err(error_i64(bytes))
            }
        }
    }
}
