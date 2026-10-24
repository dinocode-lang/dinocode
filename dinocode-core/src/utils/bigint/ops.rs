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
        let a_mag = self.magnitude();
        let b_mag = rhs.magnitude();

        match (self.is_negative(), rhs.is_negative()) {
            (false, false) => BigInt::new(0x00, add_unsigned(a_mag, b_mag)),
            (true,  true)  => BigInt::new(0x01, add_unsigned(a_mag, b_mag)),
            (false, true)  => match compare_unsigned(a_mag, b_mag) {
                Ordering::Greater => BigInt::new(0x00, sub_unsigned(a_mag, b_mag)),
                Ordering::Less    => BigInt::new(0x01, sub_unsigned(b_mag, a_mag)),
                Ordering::Equal   => BigInt::ZERO.into_owned(),
            },
            (true, false) => match compare_unsigned(a_mag, b_mag) {
                Ordering::Greater => BigInt::new(0x01, sub_unsigned(a_mag, b_mag)),
                Ordering::Less    => BigInt::new(0x00, sub_unsigned(b_mag, a_mag)),
                Ordering::Equal   => BigInt::ZERO.into_owned(),
            },
        }
    }
}

impl<'a, 'b> Sub<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn sub(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let flipped_sign = rhs.sign ^ 0x01;
        let neg_rhs = BigInt { sign: flipped_sign, magnitude: rhs.magnitude.clone() };
        self.add(&neg_rhs)
    }
}

impl<'a, 'b> Mul<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn mul(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let sign = self.sign ^ rhs.sign;
        BigInt::new(sign, mul_unsigned(self.magnitude(), rhs.magnitude()))
    }
}

impl<'a, 'b> Div<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn div(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        let sign = self.sign ^ rhs.sign;
        BigInt::new(sign, div_unsigned(self.magnitude(), rhs.magnitude()))
    }
}

impl<'a, 'b> Rem<&'b BigInt<'b>> for &'a BigInt<'a> {
    type Output = BigInt<'static>;

    fn rem(self, rhs: &'b BigInt<'b>) -> BigInt<'static> {
        BigInt::new(self.sign, mod_unsigned(self.magnitude(), rhs.magnitude()))
    }
}

impl<'a> BigInt<'a> {
    pub fn pow(&self, exp: &BigInt<'_>) -> BigInt<'static> {
        if exp.is_zero() {
            return BigInt::ONE.into_owned();
        }
        if self.is_zero() {
            return BigInt::ZERO.into_owned();
        }

        let exp_mag = exp.magnitude();
        let e: i64 = if exp_mag.len() <= 8 {
            let mut result: i64 = 0;
            for &byte in exp_mag {
                result = result * 256 + byte as i64;
            }
            if exp.is_negative() { -result } else { result }
        } else {
            return BigInt::ZERO.into_owned();
        };

        if e < 0 {
            return BigInt::ZERO.into_owned();
        }

        let mut result = BigInt::ONE.into_owned();
        let mut base   = self.clone().into_owned();
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
