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

impl<'a> PartialOrd for BigInt<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for BigInt<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.sign, other.sign) {
            (0x01, 0x00) => Ordering::Less,
            (0x00, 0x01) => Ordering::Greater,
            (0x00, 0x00) => compare_unsigned(self.magnitude(), other.magnitude()),
                       _ => compare_unsigned(other.magnitude(), self.magnitude()),
        }
    }
}
