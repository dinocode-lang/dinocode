// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/lax/f64.rs
//  Desc:       Lax f64 parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::parsers::numeric::{
    parse::ParseNumericLax,
    error::{
        NumericParseError,
        error_f64,
    },
    utils::trim_whitespace,
};

impl ParseNumericLax for f64 {
    fn parse_lax(input: impl AsRef<[u8]>, _base: Option<u32>) -> Result<Self, NumericParseError> {
        let bytes = input.as_ref();
        let bytes = trim_whitespace(bytes);
        
        if bytes.is_empty() {
            return Err(error_f64(bytes));
        }

        std::str::from_utf8(bytes)
            .map_err(|_| error_f64(bytes))
            .and_then(|s| s.parse::<f64>().map_err(|_| error_f64(bytes)))
    }
}
