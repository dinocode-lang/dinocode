// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/bigint/ops.rs
//  Desc:       ops trait implementations for BigInt
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::{
    cmp::Ordering,
    ops::{Add, Sub, Mul, Div, Rem}
};
use super::{
    bigint::BigInt,
    arithmetic::{
        add_unsigned,
        sub_unsigned,
        compare_unsigned,
        mul_unsigned,
        div_unsigned,
        mod_unsigned,
    },
};

impl<'a, 'b> Add<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn add(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let a = self.as_bytes();
        let b = rhs.as_bytes();

        if a.is_empty() || b.is_empty() {
            return BigInt::new(vec![0x00]);
        }

        let a_neg = a[0] == 0x01;
        let b_neg = b[0] == 0x01;
        let a_val = &a[1..];
        let b_val = &b[1..];

        match (a_neg, b_neg) {
            (false, false) => signed(0x00, add_unsigned(a_val, b_val)),
            (true, true)   => signed(0x01, add_unsigned(a_val, b_val)),
            (false, true)  => match compare_unsigned(a_val, b_val) {
                Ordering::Greater => signed(0x00, sub_unsigned(a_val, b_val)),
                Ordering::Less    => signed(0x01, sub_unsigned(b_val, a_val)),
                Ordering::Equal   => BigInt::new(vec![0x00, 0]),
            },
            (true, false) => match compare_unsigned(a_val, b_val) {
                Ordering::Greater => signed(0x01, sub_unsigned(a_val, b_val)),
                Ordering::Less    => signed(0x00, sub_unsigned(b_val, a_val)),
                Ordering::Equal   => BigInt::new(vec![0x00, 0]),
            },
        }
    }
}

impl<'a, 'b> Sub<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn sub(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let b = rhs.as_bytes();
        if b.is_empty() {
            return BigInt::new(self.as_bytes().to_vec());
        }
        let mut neg_b = b.to_vec();
        neg_b[0] ^= 0x01;
        self.add(&BigInt::new(neg_b))
    }
}

impl<'a, 'b> Mul<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn mul(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let a = self.as_bytes();
        let b = rhs.as_bytes();

        if a.is_empty() || b.is_empty() {
            return BigInt::new(vec![0x00]);
        }

        let a_neg = a[0] == 0x01;
        let b_neg = b[0] == 0x01;
        let a_val = &a[1..];
        let b_val = &b[1..];

        let result = mul_unsigned(a_val, b_val);
        let sign = if a_neg ^ b_neg { 0x01 } else { 0x00 };
        signed(sign, result)
    }
}

impl<'a, 'b> Div<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn div(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let a = self.as_bytes();
        let b = rhs.as_bytes();

        if b.is_empty() || a.is_empty() {
            return BigInt::new(vec![0x00]);
        }

        let b_val = &b[1..];
        if b_val.len() == 1 && b_val[0] == 0 {
            return BigInt::new(vec![0x00, 0]);
        }

        let a_neg = a[0] == 0x01;
        let b_neg = b[0] == 0x01;
        let a_val = &a[1..];

        let result = div_unsigned(a_val, b_val);
        let sign = if a_neg ^ b_neg { 0x01 } else { 0x00 };
        signed(sign, result)
    }
}

impl<'a, 'b> Rem<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn rem(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let a = self.as_bytes();
        let b = rhs.as_bytes();

        if b.is_empty() || a.is_empty() {
            return BigInt::new(vec![0x00]);
        }

        let b_val = &b[1..];
        if b_val.len() == 1 && b_val[0] == 0 {
            return BigInt::new(vec![0x00, 0]);
        }

        let a_neg = a[0] == 0x01;
        let a_val = &a[1..];

        let result = mod_unsigned(a_val, b_val);
        // Remainder takes the sign of the dividend
        signed(if a_neg { 0x01 } else { 0x00 }, result)
    }
}

impl<'a> BigInt<'a> {
    pub fn pow(&self, exp: &BigInt<'_>) -> BigInt<'static> {
        if exp.is_zero() {
            return BigInt::new(vec![0x00, 1]);
        }
        if self.is_zero() {
            return BigInt::new(vec![0x00, 0]);
        }

        let exp_bytes = exp.as_bytes();
        let exp_val = &exp_bytes[1..];

        // Convert exponent bytes to i64 (only for small exponents)
        let e: i64 = if exp_val.len() <= 8 {
            let mut result: i64 = 0;
            for &byte in exp_val {
                result = result * 256 + byte as i64;
            }
            if exp_bytes[0] == 0x01 {
                -result
            } else {
                result
            }
        } else {
            // Exponent too large
            return BigInt::new(vec![0x00, 0]);
        };

        if e < 0 {
            return BigInt::new(vec![0x00, 0]);
        }

        let mut result = BigInt::new(vec![0x00, 1]);
        let mut base = BigInt::new(self.as_bytes().to_vec());
        let mut e = e as u64;

        while e > 0 {
            if e % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            e /= 2;
        }

        result
    }
}

#[inline]
fn signed(sign: u8, mut magnitude: Vec<u8>) -> BigInt<'static> {
    magnitude.insert(0, sign);
    BigInt::new(magnitude)
}
