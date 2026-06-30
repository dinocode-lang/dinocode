// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/strict/bigint.rs
//  Desc:       Strict BigInt parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::{
    bigint::BigInt,
    parsers::numeric::{
        parse::ParseNumeric,
        error::NumericParseError,
        utils::parse_bigint_digits_bytes,
    },
};

impl ParseNumeric for BigInt<'static> {
    #[inline]
    fn parse(input: impl AsRef<[u8]>) -> Result<Self, NumericParseError> {
        let bytes = input.as_ref();
        if bytes.is_empty() {
            return Err(NumericParseError::new("cannot convert empty string to bigint".into(), None, None));
        }

        let (is_negative, digits) = if bytes[0] == b'-' {
            (true, &bytes[1..])
        } else if bytes[0] == b'+' {
            (false, &bytes[1..])
        } else {
            (false, bytes)
        };

        if digits.is_empty() {
            return Err(NumericParseError::new("invalid bigint format".into(), None, None));
        }

        parse_bigint_digits_bytes(digits, 10, is_negative)
            .map(BigInt::new)
    }
}
