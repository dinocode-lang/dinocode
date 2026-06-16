// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/bigint.rs
//  Desc:       BigInteger arithmetic operations.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::types::opcode;
use std::cmp::Ordering;
use std::cmp;
use std::char;

pub fn string_to_bigint_bits(value_str: &str) -> Result<Vec<u8>, String> {
    let (is_negative, digits) = if value_str.starts_with('-') {
        (true, &value_str[1..])
    } else {
        (false, value_str)
    };

    if digits.is_empty() {
        return Err("Empty BigInt value".to_string());
    }

    let (base, num_str) = if digits.starts_with("0x") || digits.starts_with("0X") {
        if digits.len() <= 2 { return Err("Missing digits after '0x' in BigInt".to_string()); }
        (16, &digits[2..])
    } else if digits.starts_with("0b") || digits.starts_with("0B") {
        if digits.len() <= 2 { return Err("Missing digits after '0b' in BigInt".to_string()); }
        (2, &digits[2..])
    } else {
        if digits.starts_with('0') && digits.len() > 1 {
            return Err(format!("Invalid leading zero in BigInt or malformed radix prefix: '{}'", digits));
        }
        (10, digits)
    };

    let mut bytes = Vec::new();
    let mut temp_str = num_str.to_string();
    
    while !temp_str.is_empty() && temp_str.chars().any(|c| c != '0') {
        let mut remainder = 0;
        let mut new_temp = String::new();
        
        for digit_char in temp_str.chars() {
            let digit = match digit_char.to_digit(base) {
                Some(d) => d as u32,
                None => return Err(format!("Invalid digit '{}' for base {} in BigInt", digit_char, base)),
            };
            
            let value = remainder * base + digit;
            let quotient = value / 256;
            remainder = value % 256;
            
            if !new_temp.is_empty() || quotient > 0 {
                let q_char = char::from_digit(quotient as u32, base).unwrap();
                new_temp.push(q_char);
            }
        }
        
        bytes.push(remainder as u8);
        temp_str = new_temp;
    }
    
    if bytes.is_empty() {
        bytes.push(0);
    }
    
    bytes.reverse();
    
    if is_negative {
        bytes.insert(0, 0x01);
    } else {
        bytes.insert(0, 0x00);
    }
    
    Ok(bytes)
}

pub fn bigint_bits_to_string(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "0".to_string();
    }

    let is_negative = bytes[0] == 0x01;
    let value_bytes = &bytes[1..];
    
    if value_bytes.is_empty() {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut working_bytes = value_bytes.to_vec();
    
    while !working_bytes.is_empty() && (working_bytes.len() > 1 || working_bytes[0] != 0) {
        let mut remainder = 0;
        let mut new_bytes = Vec::new();
        
        for &byte in &working_bytes {
            let value = (remainder << 8) | (byte as u32);
            let quotient = value / 10;
            remainder = value % 10;
            
            if !new_bytes.is_empty() || quotient > 0 {
                new_bytes.push(quotient as u8);
            }
        }
        
        result.push_str(&(remainder.to_string()));
        working_bytes = new_bytes;
    }
    
    let mut final_result: String = result.chars().rev().collect();
    if final_result.is_empty() {
        final_result = "0".to_string();
    }
    
    if is_negative {
        final_result.insert(0, '-');
    }
    
    final_result
}

pub fn bigint_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.is_empty() || b.is_empty() {
        return vec![0x00];
    }
    
    let a_negative = a[0] == 0x01;
    let b_negative = b[0] == 0x01;
    let a_val = &a[1..];
    let b_val = &b[1..];
    
    match (a_negative, b_negative) {
        (false, false) => {
            let result = add_unsigned(a_val, b_val);
            let mut final_result = vec![0x00];
            final_result.extend(result);
            final_result
        },
        (true, true) => {
            let result = add_unsigned(a_val, b_val);
            let mut final_result = vec![0x01];
            final_result.extend(result);
            final_result
        },
        (false, true) => {
            match compare_unsigned(a_val, b_val) {
                Ordering::Greater => {
                    let result = sub_unsigned(a_val, b_val);
                    let mut final_result = vec![0x00]; // Positive sign
                    final_result.extend(result);
                    final_result
                },
                Ordering::Less => {
                    let result = sub_unsigned(b_val, a_val);
                    let mut final_result = vec![0x01]; // Negative sign
                    final_result.extend(result);
                    final_result
                },
                Ordering::Equal => {
                    vec![0x00, 0] // Zero
                }
            }
        },
        (true, false) => {
            match compare_unsigned(a_val, b_val) {
                Ordering::Greater => {
                    let result = sub_unsigned(a_val, b_val);
                    let mut final_result = vec![0x01]; // Negative sign
                    final_result.extend(result);
                    final_result
                },
                Ordering::Less => {
                    let result = sub_unsigned(b_val, a_val);
                    let mut final_result = vec![0x00]; // Positive sign
                    final_result.extend(result);
                    final_result
                },
                Ordering::Equal => {
                    vec![0x00, 0] // Zero
                }
            }
        }
    }
}

pub fn bigint_sub(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.is_empty() || b.is_empty() {
        return vec![0x00];
    }
    
    let mut neg_b = b.to_vec();
    neg_b[0] = if b[0] == 0x01 { 0x00 } else { 0x01 };
    bigint_add(a, &neg_b)
}

fn add_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
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

fn sub_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut borrow = 0i16;
    
    let max_len = cmp::max(a.len(), b.len());
    
    for i in 0..max_len {
        let a_byte = if i < a.len() { a[a.len() - 1 - i] } else { 0 };
        let b_byte = if i < b.len() { b[b.len() - 1 - i] } else { 0 };
        
        let diff = (a_byte as i16) - (b_byte as i16) - borrow;
        result.push(((diff + 256) % 256) as u8);
        borrow = if diff < 0 { -1 } else { 0 };
    }
    
    result.reverse();
    
    while result.len() > 1 && result[0] == 0 {
        result.remove(0);
    }
    
    result
}

fn compare_unsigned(a: &[u8], b: &[u8]) -> Ordering {
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

pub fn bigint_is_zero(bits: &[u8]) -> bool {
    if bits.len() <= 1 {
        return true;
    }
    bits[1..].iter().all(|&b| b == 0)
}

pub fn bigint_mul(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.is_empty() || b.is_empty() {
        return vec![0x00]; // Return zero for corrupted data
    }
    
    let _a_negative = a[0] == 0x01;
    let _b_negative = b[0] == 0x01;
    let _a_val = &a[1..];
    let _b_val = &b[1..];
    
    let a_str = bigint_bits_to_string(a);
    let b_str = bigint_bits_to_string(b);
    
    let result = multiply_strings(&a_str, &b_str);
    string_to_bigint_bits(&result).unwrap()
}

fn multiply_strings(a_str: &str, b_str: &str) -> String {
    let a_clean = a_str.trim_start_matches('-');
    let b_clean = b_str.trim_start_matches('-');
    
    if a_clean == "0" || b_clean == "0" {
        return "0".to_string();
    }
    
    let a_digits: Vec<u32> = a_clean.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
    let b_digits: Vec<u32> = b_clean.chars().rev().map(|c| c.to_digit(10).unwrap()).collect();
    
    let mut result = vec![0u32; a_digits.len() + b_digits.len()];
    
    for (i, &a_digit) in a_digits.iter().enumerate() {
        let mut carry = 0u32;
        
        for (j, &b_digit) in b_digits.iter().enumerate() {
            let product = a_digit * b_digit + result[i + j] + carry;
            result[i + j] = product % 10;
            carry = product / 10;
        }
        
        if carry > 0 {
            result[i + b_digits.len()] += carry;
        }
    }
    
    while result.len() > 1 && result.last() == Some(&0) {
        result.pop();
    }
    
    let is_negative = a_str.starts_with('-') ^ b_str.starts_with('-');
    
    let mut result_str: String = result.iter().rev().map(|&d| d.to_string()).collect();
    if is_negative {
        result_str.insert(0, '-');
    }
    
    result_str
}

pub fn bigint_div(a: &[u8], b: &[u8]) -> Vec<u8> {
    if bigint_is_zero(b) {
        return vec![0x00, 0];
    }
    
    if a.is_empty() || b.is_empty() {
        return vec![0x00];
    }
    
    let a_negative = a[0] == 0x01;
    let b_negative = b[0] == 0x01;
    
    let a_str = bigint_bits_to_string(a);
    let b_str = bigint_bits_to_string(b);
    
    let a_clean = a_str.trim_start_matches('-');
    let b_clean = b_str.trim_start_matches('-');
    
    let result = divide_strings(a_clean, b_clean);
    
    let is_negative = a_negative ^ b_negative;
    let final_result = if is_negative && result != "0" {
        format!("-{}", result)
    } else {
        result
    };
    
    string_to_bigint_bits(&final_result).unwrap()
}

fn divide_strings(a_str: &str, b_str: &str) -> String {
    if b_str == "0" {
        return "0".to_string();
    }
    if a_str == "0" {
        return "0".to_string();
    }
    
    if a_str.len() < b_str.len() || (a_str.len() == b_str.len() && a_str < b_str) {
        return "0".to_string();
    }
    
    let mut result = String::new();
    let mut remainder = String::new();
    
    for digit_char in a_str.chars() {
        remainder.push(digit_char);
        
        while remainder.len() > 1 && remainder.starts_with('0') {
            remainder.remove(0);
        }
        
        let mut quotient_digit = 0;
        
        while compare_strings_numeric(&remainder, b_str) != Ordering::Less {
            remainder = subtract_strings(&remainder, b_str);
            quotient_digit += 1;
        }
        
        result.push_str(&(quotient_digit.to_string()));
    }
    
    while result.len() > 1 && result.starts_with('0') {
        result.remove(0);
    }
    
    result
}

fn compare_strings_numeric(a: &str, b: &str) -> Ordering {
    let a_clean = a.trim_start_matches('0');
    let b_clean = b.trim_start_matches('0');
    
    let a_final = if a_clean.is_empty() { "0" } else { a_clean };
    let b_final = if b_clean.is_empty() { "0" } else { b_clean };
    
    if a_final.len() != b_final.len() {
        return if a_final.len() > b_final.len() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
    
    a_final.cmp(b_final)
}

fn subtract_strings(a: &str, b: &str) -> String {
    let a_digits: Vec<u32> = a.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let b_digits: Vec<u32> = b.chars().map(|c| c.to_digit(10).unwrap()).collect();
    
    let mut result = Vec::new();
    let mut borrow = 0;
    
    for i in 0..a_digits.len() {
        let a_digit = a_digits[a_digits.len() - 1 - i];
        let b_digit = if i < b_digits.len() { b_digits[b_digits.len() - 1 - i] } else { 0 };
        
        let diff = a_digit as i32 - b_digit as i32 - borrow;
        if diff >= 0 {
            result.push(diff as u32);
            borrow = 0;
        } else {
            result.push((diff + 10) as u32);
            borrow = 1;
        }
    }
    
    result.reverse();
    
    while result.len() > 1 && result[0] == 0 {
        result.remove(0);
    }
    
    result.iter().map(|&d| d.to_string()).collect()
}

pub fn bigint_mod(a: &[u8], b: &[u8]) -> Vec<u8> {
    if bigint_is_zero(b) {
        return vec![0x00, 0];
    }
    
    let a_str = bigint_bits_to_string(a);
    let b_str = bigint_bits_to_string(b);
    
    let a_clean = a_str.trim_start_matches('-');
    let b_clean = b_str.trim_start_matches('-');
    
    let quotient = divide_strings(a_clean, b_clean);
    let product = multiply_strings(&quotient, b_clean);
    let remainder_str = subtract_strings(a_clean, &product);
    
    let final_result = if a_str.starts_with('-') && remainder_str != "0" {
        format!("-{}", remainder_str)
    } else {
        remainder_str
    };
    
    string_to_bigint_bits(&final_result).unwrap()
}

pub fn bigint_pow(a: &[u8], b: &[u8]) -> Vec<u8> {
    if bigint_is_zero(b) {
        return string_to_bigint_bits("1").unwrap();
    }
    
    if bigint_is_zero(a) {
        return string_to_bigint_bits("0").unwrap();
    }
    
    let b_str = bigint_bits_to_string(b);
    
    let exp_value = if let Ok(val) = b_str.parse::<i64>() {
        if val < 0 {
            return string_to_bigint_bits("0").unwrap();
        } else {
            val
        }
    } else {
        return string_to_bigint_bits("0").unwrap();
    };
    
    let mut result = string_to_bigint_bits("1").unwrap();
    let mut base = a.to_vec();
    let mut exponent = exp_value;
    
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = bigint_mul(&result, &base);
        }
        base = bigint_mul(&base, &base);
        exponent /= 2;
    }
    
    result
}

pub fn bigint_compare(a: &[u8], b: &[u8], op: u8) -> bool {
    match op {
        opcode::EQ => bigint_cmp_eq(a, b),
        opcode::NE => !bigint_cmp_eq(a, b),
        opcode::GT => bigint_cmp_gt(a, b),
        opcode::LT => bigint_cmp_lt(a, b),
        opcode::GE => bigint_cmp_ge(a, b),
        opcode::LE => bigint_cmp_le(a, b),
        _ => false,
    }
}

fn bigint_cmp_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

fn bigint_cmp_gt(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return a.len() > b.len(); }
    a.iter().zip(b.iter()).any(|(x, y)| x > y)
}

fn bigint_cmp_lt(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return a.len() < b.len(); }
    a.iter().zip(b.iter()).any(|(x, y)| x < y)
}

fn bigint_cmp_ge(a: &[u8], b: &[u8]) -> bool {
    bigint_cmp_eq(a, b) || bigint_cmp_gt(a, b)
}

fn bigint_cmp_le(a: &[u8], b: &[u8]) -> bool {
    bigint_cmp_eq(a, b) || bigint_cmp_lt(a, b)
}
