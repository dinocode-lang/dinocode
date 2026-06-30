// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/bigint/conversions.rs
//  Desc:       BigInt conversions
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use super::BigInt;
use crate::utils::parsers::numeric::NumericParseError;

impl<'a> BigInt<'a> {
    pub fn from_i64(value: i64) -> Self {
        let sign: u8 = if value < 0 { 0x01 } else { 0x00 };
        let magnitude = value.unsigned_abs();

        if magnitude == 0 {
            return BigInt::new(vec![sign, 0]);
        }

        let be = magnitude.to_be_bytes();
        let first_nonzero = be.iter().position(|&b| b != 0).unwrap();
        let mut bytes = Vec::with_capacity(1 + 8 - first_nonzero);
        bytes.push(sign);
        bytes.extend_from_slice(&be[first_nonzero..]);
        BigInt::new(bytes)
    }

    pub fn from_f64(value: f64) -> Self {
        let sign = if value.is_sign_negative() { 0x01 } else { 0x00 };
        let mut magnitude = value.abs();

        if magnitude == 0.0 {
            return BigInt::new(vec![sign, 0]);
        }

        let mut bytes = Vec::new();
        while magnitude >= 1.0 {
            bytes.push((magnitude % 256.0) as u8);
            magnitude = (magnitude / 256.0).floor();
        }
        bytes.reverse();

        let mut result = Vec::with_capacity(1 + bytes.len());
        result.push(sign);
        result.extend(bytes);
        BigInt::new(result)
    }

    pub fn to_i64(&self) -> Result<i64, NumericParseError> {
        let bytes = self.as_bytes();
        if bytes.is_empty() || bytes.len() > 9 {
            return Err(NumericParseError::new("BigInt too large for integer".into(), None, None));
        }

        let is_negative = bytes[0] == 0x01;
        let value_bytes = &bytes[1..];

        let mut buf = [0u8; 8];
        buf[8 - value_bytes.len()..].copy_from_slice(value_bytes);
        let magnitude = u64::from_be_bytes(buf);

        if is_negative {
            if magnitude > i64::MIN.unsigned_abs() {
                return Err(NumericParseError::new("BigInt too large for integer".into(), None, None));
            }
            Ok((magnitude as i64).wrapping_neg())
        } else {
            if magnitude > i64::MAX as u64 {
                return Err(NumericParseError::new("BigInt too large for integer".into(), None, None));
            }
            Ok(magnitude as i64)
        }
    }

    pub fn to_f64(&self) -> Result<f64, NumericParseError> {
        let bytes = self.as_bytes();
        if bytes.is_empty() {
            return Err(NumericParseError::new("Invalid BigInt".into(), None, None));
        }

        let is_negative = bytes[0] == 0x01;
        let value_bytes = &bytes[1..];

        if value_bytes.is_empty() { return Ok(0.0); }

        // Values beyond 128 bytes can't be represented in f64 without loss of precision
        if value_bytes.len() > 128 {
            return Ok(if is_negative { f64::NEG_INFINITY } else { f64::INFINITY });
        }

        let mut result: f64 = 0.0;
        for &byte in value_bytes {
            result = result * 256.0 + (byte as f64);
        }

        Ok(if is_negative { -result } else { result })
    }
}
