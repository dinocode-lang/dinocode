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
    utils::clean_number,
};

impl ParseNumericLax for f64 {
    fn parse_lax(input: impl AsRef<[u8]>, _base: Option<u32>) -> Result<Self, NumericParseError> {
        let raw = input.as_ref();
        let bytes = clean_number(raw);
        
        if bytes.is_empty() {
            return Err(error_f64(bytes.as_ref()));
        }

        std::str::from_utf8(bytes.as_ref())
            .map_err(|_| error_f64(bytes.as_ref()))
            .and_then(|s| s.parse::<f64>().map_err(|_| error_f64(bytes.as_ref())))
    }
}
