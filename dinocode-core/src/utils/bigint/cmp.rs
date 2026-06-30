// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/bigint/cmp.rs
//  Desc:       BigInt comparisons
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::cmp::Ordering;
use super::{
    bigint::BigInt,
    arithmetic::compare_unsigned,
};
use crate::types::opcode;

impl<'a> PartialOrd for BigInt<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for BigInt<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.as_bytes();
        let b = other.as_bytes();

        if a.is_empty() || b.is_empty() {
            return Ordering::Equal;
        }

        let a_neg = a[0] == 0x01;
        let b_neg = b[0] == 0x01;

        match (a_neg, b_neg) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => compare_unsigned(&a[1..], &b[1..]),
            // reversed magnitude ordering
            (true, true) => compare_unsigned(&b[1..], &a[1..]),
        }
    }
}

impl<'a> BigInt<'a> {
    pub fn compare_op(&self, rhs: &BigInt<'_>, op: u8) -> bool {
        match op {
            opcode::EQ => self == rhs,
            opcode::NE => self != rhs,
            opcode::GT => self > rhs,
            opcode::LT => self < rhs,
            opcode::GE => self >= rhs,
            opcode::LE => self <= rhs,
            _ => false,
        }
    }
}
