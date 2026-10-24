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
            return BigInt::new(sign, vec![0]);
        }

        let be = magnitude.to_be_bytes();
        let first_nonzero = be.iter().position(|&b| b != 0).unwrap();
        BigInt::new(sign, be[first_nonzero..].to_vec())
    }

    pub fn from_f64(value: f64) -> Self {
        let sign = if value.is_sign_negative() { 0x01 } else { 0x00 };
        let mut magnitude = value.abs();

        if magnitude == 0.0 {
            return BigInt::new(sign, vec![0]);
        }

        let mut bytes = Vec::new();
        while magnitude >= 1.0 {
            bytes.push((magnitude % 256.0) as u8);
            magnitude = (magnitude / 256.0).floor();
        }
        bytes.reverse();
        BigInt::new(sign, bytes)
    }

    pub fn to_i64(&self) -> Result<i64, NumericParseError> {
        let mag = self.magnitude();
        if mag.len() > 8 {
            return Err(NumericParseError::new("BigInt too large for integer".into(), None, None));
        }

        let mut buf = [0u8; 8];
        buf[8 - mag.len()..].copy_from_slice(mag);
        let magnitude = u64::from_be_bytes(buf);

        if self.is_negative() {
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
        let mag = self.magnitude();
        if mag.is_empty() { return Ok(0.0); }

        if mag.len() > 128 {
            return Ok(if self.is_negative() { f64::NEG_INFINITY } else { f64::INFINITY });
        }

        let mut result: f64 = 0.0;
        for &byte in mag {
            result = result * 256.0 + (byte as f64);
        }

        Ok(if self.is_negative() { -result } else { result })
    }
}
