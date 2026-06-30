// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/parse.rs
//  Desc:       Numeric parsing trait
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use super::error::NumericParseError;

pub trait ParseNumeric: Sized {
    fn parse(input: impl AsRef<[u8]>) -> Result<Self, NumericParseError>;
}

pub trait ParseNumericLax: Sized {
    fn parse_lax(input: impl AsRef<[u8]>, base: Option<u32>) -> Result<Self, NumericParseError>;
}

#[inline(always)]
pub fn parse<T: ParseNumeric>(input: impl AsRef<[u8]>) -> Result<T, NumericParseError> {
    T::parse(input)
}

pub fn parse_lax<T: ParseNumericLax>(input: impl AsRef<[u8]>, base: Option<u32>) -> Result<T, NumericParseError> {
    T::parse_lax(input, base)
}
