// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/lax/i64.rs
//  Desc:       Lax i64 parsing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::utils::parsers::numeric::{
    parse::ParseNumericLax,
    error::{
        NumericParseError,
        error_i64,
        error_hex,
        error_bin,
        error_octal,
    },
    utils::{
        clean_number,
        is_valid_int,
        parse_i64_hex,
        parse_i64_bin,
        parse_i64_decimal,
        parse_i64_octal,
    },
};

impl ParseNumericLax for i64 {
    fn parse_lax(input: impl AsRef<[u8]>, base: Option<u32>) -> Result<Self, NumericParseError> {
        let raw = input.as_ref();
        let bytes = clean_number(raw);
        
        if bytes.is_empty() {
            return Err(error_i64(bytes.as_ref()));
        }

        if let Some(explicit_base) = base {
            return parse_i64_with_base(&bytes, explicit_base);
        }

        let (is_negative, content, is_hex, is_bin, is_oct) = {
            let is_neg = bytes[0] == b'-';
            let cont = if is_neg || bytes[0] == b'+' { &bytes[1..] } else { bytes.as_ref() };
            
            let prefix_check = &cont[..cont.len().min(2)];
            let hex = prefix_check.len() == 2 && prefix_check[0] == b'0' && prefix_check[1] == b'x';
            let bin = !hex && prefix_check.len() == 2 && prefix_check[0] == b'0' && prefix_check[1] == b'b';
            let oct = !hex && !bin && prefix_check.len() == 2 && prefix_check[0] == b'0' && prefix_check[1] == b'o';
            
            (is_neg, cont, hex, bin, oct)
        };

        let (val, is_valid) = if is_hex {
            let v = parse_i64_hex(&content[2..])?;
            let fv = if is_negative { -v } else { v };
            (fv, is_valid_int(fv))
        } else if is_bin {
            let v = parse_i64_bin(&content[2..])?;
            let fv = if is_negative { -v } else { v };
            (fv, is_valid_int(fv))
        } else if is_oct {
            let v = parse_i64_octal(&content[2..])?;
            let fv = if is_negative { -v } else { v };
            (fv, is_valid_int(fv))
        } else {
            let v = parse_i64_decimal(content)?;
            let fv = if is_negative { -v } else { v };
            (fv, is_valid_int(fv))
        };

        if is_valid {
            Ok(val)
        } else if is_hex {
            Err(error_hex(bytes.as_ref()))
        } else if is_bin {
            Err(error_bin(bytes.as_ref()))
        } else if is_oct {
            Err(error_octal(bytes.as_ref()))
        } else {
            Err(error_i64(bytes.as_ref()))
        }
    }
}

fn parse_i64_with_base(bytes: &[u8], base: u32) -> Result<i64, NumericParseError> {
    let (is_negative, content) = {
        let is_neg = bytes[0] == b'-';
        let cont = if is_neg || bytes[0] == b'+' { &bytes[1..] } else { bytes };
        (is_neg, cont)
    };

    let v = match base {
        2 => parse_i64_bin(content)?,
        8 => parse_i64_octal(content)?,
        10 => parse_i64_decimal(content)?,
        16 => parse_i64_hex(content)?,
        _ => return Err(NumericParseError::new(
            format!("invalid base '{}'", base),
            None,
            Some("must be 2, 8, 10, or 16".to_string()),
        )),
    };

    let fv = if is_negative { -v } else { v };
    
    if is_valid_int(fv) {
        Ok(fv)
    } else {
        Err(error_i64(&bytes))
    }
}
