// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/utils/numeric.rs
//  Desc:       Lexer numeric parsers
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::utils::parsers::numeric::parse_lax;
use dinocode_core::utils::parsers::numeric::NumericParseError;

#[inline(always)]
pub fn parse_i64_lex(s: &str, is_negative: bool) -> Result<i64, NumericParseError> {
    parse_lax::<i64>(s.as_bytes(), None)
        .map(|v| if is_negative { -v } else { v })
}

#[inline(always)]
pub fn parse_f64_lex(s: &str, is_negative: bool) -> Result<f64, NumericParseError> {
    parse_lax::<f64>(s.as_bytes(), None)
        .map(|v| if is_negative { -v } else { v })
}
