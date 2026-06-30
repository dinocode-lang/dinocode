// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/bigint/bigint.rs
//  Desc:       BigInt type definition
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::{fmt, borrow::Cow};

// [sign][magnitude]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt<'a>(pub Cow<'a, [u8]>);

impl<'a> BigInt<'a> {
    pub const ZERO: BigInt<'static> = BigInt::new_const(&[0x00, 0]);
    pub const ONE: BigInt<'static> = BigInt::new_const(&[0x00, 1]);

    #[inline]
    pub fn new(bytes: Vec<u8>) -> Self {
        BigInt(Cow::Owned(bytes))
    }

    #[inline]
    pub const fn new_const(bytes: &'static [u8]) -> BigInt<'static> {
        BigInt(Cow::Borrowed(bytes))
    }

    #[inline]
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        BigInt(Cow::Borrowed(bytes))
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn into_owned(self) -> BigInt<'static> {
        BigInt(Cow::Owned(self.0.into_owned()))
    }

    pub fn is_zero(&self) -> bool {
        let b = self.as_bytes();
        b.len() <= 1 || b[1..].iter().all(|&x| x == 0)
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.as_bytes().first() == Some(&0x01)
    }
}

impl<'a> fmt::Display for BigInt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        if bytes.is_empty() { return write!(f, "0"); }

        let is_negative = bytes[0] == 0x01;
        let value_bytes = &bytes[1..];
        if value_bytes.is_empty() { return write!(f, "0"); }

        let mut digits = String::new();
        let mut working = value_bytes.to_vec();

        while !working.is_empty() && (working.len() > 1 || working[0] != 0) {
            let mut remainder = 0u32;
            let mut next = Vec::new();

            for &byte in &working {
                let val = (remainder << 8) | (byte as u32);
                let q   = val / 10;
                remainder = val % 10;
                if !next.is_empty() || q > 0 {
                    next.push(q as u8);
                }
            }

            digits.push(char::from_digit(remainder, 10).unwrap());
            working = next;
        }

        if digits.is_empty() { digits.push('0'); }
        if is_negative { write!(f, "-")?; }
        for ch in digits.chars().rev() { write!(f, "{}", ch)?; }
        Ok(())
    }
}
