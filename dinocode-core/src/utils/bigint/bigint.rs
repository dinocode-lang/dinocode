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
pub struct BigInt<'a> {
    pub sign: u8,
    pub magnitude: Cow<'a, [u8]>,
}

impl<'a> BigInt<'a> {
    pub const ZERO: BigInt<'static> = BigInt { sign: 0x00, magnitude: Cow::Borrowed(&[0]) };
    pub const ONE:  BigInt<'static> = BigInt { sign: 0x00, magnitude: Cow::Borrowed(&[1]) };

    #[inline]
    pub fn new(sign: u8, magnitude: Vec<u8>) -> Self {
        BigInt { sign, magnitude: Cow::Owned(magnitude) }
    }

    #[inline]
    pub const fn new_const(sign: u8, magnitude: &'static [u8]) -> BigInt<'static> {
        BigInt { sign, magnitude: Cow::Borrowed(magnitude) }
    }

    #[inline]
    pub fn from_slice(sign: u8, magnitude: &'a [u8]) -> Self {
        BigInt { sign, magnitude: Cow::Borrowed(magnitude) }
    }

    #[inline]
    pub fn magnitude(&self) -> &[u8] {
        &self.magnitude
    }

    #[inline]
    pub fn into_owned(self) -> BigInt<'static> {
        BigInt { sign: self.sign, magnitude: Cow::Owned(self.magnitude.into_owned()) }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude.iter().all(|&x| x == 0)
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.sign == 0x01
    }
}

impl<'a> fmt::Display for BigInt<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mag = self.magnitude();
        if mag.is_empty() { return write!(f, "0"); }

        let mut digits = String::new();
        let mut working = mag.to_vec();

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
        if self.is_negative() { write!(f, "-")?; }
        for ch in digits.chars().rev() { write!(f, "{}", ch)?; }
        Ok(())
    }
}
