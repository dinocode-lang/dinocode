// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/bigint/arithmetic.rs
//  Desc:       Raw unsigned arithmetic for BigInt
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::cmp::{self, Ordering};

pub fn add_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut carry = 0u16;
    let max_len = cmp::max(a.len(), b.len());

    for i in 0..max_len {
        let a_byte = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
        let b_byte = if i < b.len() { b[b.len() - 1 - i] } else { 0 };

        let sum = (a_byte as u16) + (b_byte as u16) + carry;
        result.push((sum % 256) as u8);
        carry = sum / 256;
    }

    if carry > 0 {
        result.push(carry as u8);
    }

    result.reverse();
    result
}

pub fn sub_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut borrow = 0i16;
    let max_len = cmp::max(a.len(), b.len());

    for i in 0..max_len {
        let a_byte = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
        let b_byte = if i < b.len() { b[b.len() - 1 - i] } else { 0 };

        let diff = (a_byte as i16) - (b_byte as i16) - borrow;
        // Handle negative diff without wrapping issues
        result.push(((diff + 256) % 256) as u8);
        borrow = if diff < 0 { -1 } else { 0 };
    }

    result.reverse();

    while result.len() > 1 && result[0] == 0 {
        result.remove(0);
    }

    result
}

pub fn compare_unsigned(a: &[u8], b: &[u8]) -> Ordering {
    let mut a_trimmed = a;
    while a_trimmed.len() > 1 && a_trimmed[0] == 0 {
        a_trimmed = &a_trimmed[1..];
    }

    let mut b_trimmed = b;
    while b_trimmed.len() > 1 && b_trimmed[0] == 0 {
        b_trimmed = &b_trimmed[1..];
    }

    if a_trimmed.len() != b_trimmed.len() {
        return if a_trimmed.len() > b_trimmed.len() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }

    for (a_byte, b_byte) in a_trimmed.iter().zip(b_trimmed.iter()) {
        match a_byte.cmp(b_byte) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    Ordering::Equal
}

pub fn mul_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.is_empty() || b.is_empty() {
        return vec![0];
    }

    let a_trimmed = trim_leading_zeros(a);
    let b_trimmed = trim_leading_zeros(b);

    if a_trimmed.len() == 1 && a_trimmed[0] == 0 || b_trimmed.len() == 1 && b_trimmed[0] == 0 {
        return vec![0];
    }

    let a_len = a_trimmed.len();
    let b_len = b_trimmed.len();
    let mut result = vec![0u16; a_len + b_len];

    for (i, &a_byte) in a_trimmed.iter().rev().enumerate() {
        let mut carry = 0u16;
        for (j, &b_byte) in b_trimmed.iter().rev().enumerate() {
            let idx = a_len + b_len - i - j - 2;
            let product = (a_byte as u16) * (b_byte as u16) + result[idx] + carry;
            result[idx] = product;
            carry = product >> 8;
        }
        let idx = a_len - i - 1;
        result[idx] += carry;
    }

    // Convert u16 to u8 with carry propagation
    let mut final_result = Vec::with_capacity(result.len());
    let mut carry = 0u16;
    for &val in &result {
        let sum = val + carry;
        final_result.push((sum & 0xFF) as u8);
        carry = sum >> 8;
    }

    trim_leading_zeros_vec(final_result)
}

pub fn div_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    if b.is_empty() {
        return vec![0];
    }

    let b_trimmed = trim_leading_zeros(b);
    if b_trimmed.len() == 1 && b_trimmed[0] == 0 {
        return vec![0];
    }

    let a_trimmed = trim_leading_zeros(a);
    if a_trimmed.len() == 1 && a_trimmed[0] == 0 {
        return vec![0];
    }

    if compare_unsigned(a_trimmed, b_trimmed) == Ordering::Less {
        return vec![0];
    }

    // Binary long division
    let mut dividend = a_trimmed.to_vec();
    let divisor = b_trimmed.to_vec();
    let mut quotient = vec![0u8; dividend.len()];
    let dividend_len = dividend.len();

    for _i in 0..(dividend_len * 8) {
        // Shift dividend left by 1
        let mut carry = false;
        for byte in dividend.iter_mut().rev() {
            let new_carry = (*byte & 0x80) != 0;
            *byte <<= 1;
            if carry {
                *byte |= 1;
            }
            carry = new_carry;
        }

        // Shift quotient left by 1
        let mut q_carry = false;
        for byte in quotient.iter_mut().rev() {
            let new_carry = (*byte & 0x80) != 0;
            *byte <<= 1;
            if q_carry {
                *byte |= 1;
            }
            q_carry = new_carry;
        }

        // if dividend >= divisor, subtract and set quotient
        {
            let cmp = compare_unsigned(&dividend, &divisor);
            if cmp != Ordering::Less {
                dividend = sub_unsigned(&dividend, &divisor);
                let last_idx = quotient.len() - 1;
                quotient[last_idx] |= 1;
            }
        }
    }

    trim_leading_zeros_vec(quotient)
}

pub fn mod_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    if b.is_empty() {
        return vec![0];
    }

    let b_trimmed = trim_leading_zeros(b);
    if b_trimmed.len() == 1 && b_trimmed[0] == 0 {
        return vec![0];
    }

    let a_trimmed = trim_leading_zeros(a);
    if a_trimmed.len() == 1 && a_trimmed[0] == 0 {
        return vec![0];
    }

    if compare_unsigned(a_trimmed, b_trimmed) == Ordering::Less {
        return a_trimmed.to_vec();
    }

    // Binary long division
    let mut dividend = a_trimmed.to_vec();
    let divisor = b_trimmed.to_vec();
    let dividend_len = dividend.len();

    for _ in 0..(dividend_len * 8) {
        // Shift dividend left by 1
        let mut carry = false;
        for byte in dividend.iter_mut().rev() {
            let new_carry = (*byte & 0x80) != 0;
            *byte <<= 1;
            if carry {
                *byte |= 1;
            }
            carry = new_carry;
        }

        // if dividend >= divisor, subtract
        {
            let cmp = compare_unsigned(&dividend, &divisor);
            if cmp != Ordering::Less {
                dividend = sub_unsigned(&dividend, &divisor);
            }
        }
    }

    trim_leading_zeros_vec(dividend)
}

fn trim_leading_zeros(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len() - 1);
    &bytes[start..]
}

fn trim_leading_zeros_vec(mut bytes: Vec<u8>) -> Vec<u8> {
    let zeros = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len() - 1);
    if zeros > 0 {
        bytes.drain(0..zeros);
    }
    bytes
}
