// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/strict/f64.rs
//  Desc:       Strict f64 parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::parsers::numeric::{
    parse::ParseNumeric,
    error::{
        NumericParseError,
        error_f64,
    },
};

impl ParseNumeric for f64 {
    #[inline(always)]
    fn parse(input: impl AsRef<[u8]>) -> Result<Self, NumericParseError> {
        let bytes = input.as_ref();
        std::str::from_utf8(bytes)
            .map_err(|_| error_f64(bytes))
            .and_then(|s| s.parse::<f64>().map_err(|_| error_f64(bytes)))
    }
}
