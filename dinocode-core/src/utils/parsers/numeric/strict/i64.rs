// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/strict/i64.rs
//  Desc:       Strict i64 parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::parsers::numeric::{
    parse::ParseNumeric,
    error::{
        NumericParseError,
        error_i64,
    },
    utils::{
        parse_i64_decimal,
        is_valid_int,
    },
};

impl ParseNumeric for i64 {
    #[inline(always)]
    fn parse(input: impl AsRef<[u8]>) -> Result<Self, NumericParseError> {
        let bytes = input.as_ref();
        if bytes.is_empty() {
            return Err(error_i64(bytes));
        }

        let (is_negative, content) = {
            let is_neg = bytes[0] == b'-';
            let cont = if is_neg || bytes[0] == b'+' { &bytes[1..] } else { bytes };
            (is_neg, cont)
        };

        let v = parse_i64_decimal(content)?;
        let fv = if is_negative { -v } else { v };

        if is_valid_int(fv) {
            Ok(fv)
        } else {
            Err(error_i64(bytes))
        }
    }
}
