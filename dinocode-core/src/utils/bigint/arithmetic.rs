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
    let a_len = a.len();
    let b_len = b.len();
    let max_len = cmp::max(a_len, b_len);
    let mut result = vec![0u8; max_len + 1];
    let mut carry = 0u16;

    for i in 0..max_len {
        let a_byte = if i < a_len { a[a_len - 1 - i] as u16 } else { 0 };
        let b_byte = if i < b_len { b[b_len - 1 - i] as u16 } else { 0 };
        let sum = a_byte + b_byte + carry;
        result[max_len - i] = (sum & 0xFF) as u8;
        carry = sum >> 8;
    }
    result[0] = carry as u8;

    if result[0] == 0 { result[1..].to_vec() } else { result }
}

pub fn sub_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    let a_len = a.len();
    let b_len = b.len();
    let mut result = vec![0u8; a_len];
    let mut borrow = 0i16;

    for i in 0..a_len {
        let a_byte = a[a_len - 1 - i] as i16;
        let b_byte = if i < b_len { b[b_len - 1 - i] as i16 } else { 0 };
        let diff = a_byte - b_byte - borrow;
        result[a_len - 1 - i] = ((diff + 256) & 0xFF) as u8;
        borrow = if diff < 0 { 1 } else { 0 };
    }

    trim_leading_zeros_vec(result)
}

pub fn compare_unsigned(a: &[u8], b: &[u8]) -> Ordering {
    let a = trim_leading_zeros(a);
    let b = trim_leading_zeros(b);
    match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(b),
        other           => other,
    }
}

pub fn mul_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    let a = trim_leading_zeros(a);
    let b = trim_leading_zeros(b);

    if a == [0] || b == [0] {
        return vec![0];
    }

    let a_len = a.len();
    let b_len = b.len();
    let result_len = a_len + b_len;
    let mut acc = vec![0u32; result_len];

    for (i, &ai) in a.iter().rev().enumerate() {
        for (j, &bj) in b.iter().rev().enumerate() {
            acc[result_len - 1 - i - j] += (ai as u32) * (bj as u32);
        }
    }

    let mut carry = 0u32;
    for slot in acc.iter_mut().rev() {
        let total = *slot + carry;
        *slot = total & 0xFF;
        carry = total >> 8;
    }

    let bytes: Vec<u8> = acc.iter().map(|&x| x as u8).collect();
    trim_leading_zeros_vec(bytes)
}

// Knuth Algorithm (base 256)
fn divmod_unsigned(u: &[u8], v: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let u = trim_leading_zeros(u);
    let v = trim_leading_zeros(v);

    match compare_unsigned(u, v) {
        Ordering::Less  => return (vec![0], u.to_vec()),
        Ordering::Equal => return (vec![1], vec![0]),
        _ => {}
    }

    if v.len() == 1 {
        let d = v[0] as u32;
        let mut q = Vec::with_capacity(u.len());
        let mut rem: u32 = 0;
        for &byte in u {
            let cur = (rem << 8) | (byte as u32);
            q.push((cur / d) as u8);
            rem = cur % d;
        }
        return (trim_leading_zeros_vec(q), vec![rem as u8]);
    }

    let shift = v[0].leading_zeros() as usize;
    let vn = shl_bytes(v, shift);
    let n = vn.len();
    let m = u.len() - v.len();

    let un_raw = shl_bytes(u, shift);
    let need = m + n + 1;
    let mut un = vec![0u8; need];
    let offset = need.saturating_sub(un_raw.len());
    un[offset..].copy_from_slice(&un_raw[..need - offset]);

    let mut q = vec![0u8; m + 1];
    let vn1 = vn[0] as u64;
    let vn2 = if n >= 2 { vn[1] as u64 } else { 0 };

    for j in 0..=m {
        let u0 = un[j] as u64;
        let u1 = un[j + 1] as u64;
        let u2 = if j + 2 < need { un[j + 2] as u64 } else { 0 };
        let uj01 = (u0 << 8) | u1;

        let mut qhat = if u0 >= vn1 { 0xFF_u64 } else { uj01 / vn1 };
        let mut rhat = uj01 - qhat * vn1;

        while qhat >= 0x100 || qhat * vn2 > (rhat << 8) | u2 {
            qhat -= 1;
            rhat += vn1;
            if rhat >= 0x100 { break; }
        }

        let mut borrow: u64 = 0;
        for k in (0..n).rev() {
            let idx = j + k + 1;
            let prod = qhat * (vn[k] as u64) + borrow;
            let diff = (un[idx] as u64).wrapping_sub(prod & 0xFF);
            un[idx] = (diff & 0xFF) as u8;
            borrow = (prod >> 8) + if (diff >> 63) != 0 { 1 } else { 0 };
        }
        let lead = (un[j] as u64).wrapping_sub(borrow);
        un[j] = (lead & 0xFF) as u8;
        q[j] = qhat as u8;

        if (lead >> 63) != 0 {
            q[j] -= 1;
            let mut carry: u16 = 0;
            for k in (0..n).rev() {
                let idx = j + k + 1;
                let s = (un[idx] as u16) + (vn[k] as u16) + carry;
                un[idx] = (s & 0xFF) as u8;
                carry = s >> 8;
            }
            un[j] = ((un[j] as u16 + carry) & 0xFF) as u8;
        }
    }

    let rem = shr_bytes(&un[m + 1..], shift);
    (trim_leading_zeros_vec(q), trim_leading_zeros_vec(rem))
}

pub fn div_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    divmod_unsigned(a, b).0
}

pub fn mod_unsigned(a: &[u8], b: &[u8]) -> Vec<u8> {
    divmod_unsigned(a, b).1
}

fn shl_bytes(src: &[u8], bits: usize) -> Vec<u8> {
    if bits == 0 { return src.to_vec(); }
    let mut out = vec![0u8; src.len() + 1];
    let mut carry: u16 = 0;
    for (i, &b) in src.iter().rev().enumerate() {
        let shifted = ((b as u16) << bits) | carry;
        out[src.len() - i] = (shifted & 0xFF) as u8;
        carry = shifted >> 8;
    }
    out[0] = carry as u8;
    trim_leading_zeros_vec(out)
}

fn shr_bytes(src: &[u8], bits: usize) -> Vec<u8> {
    if bits == 0 || src.is_empty() { return src.to_vec(); }
    let mut out = vec![0u8; src.len()];
    let mut carry: u8 = 0;
    let mask = (1u8 << bits) - 1;
    for (i, &b) in src.iter().enumerate() {
        out[i] = (b >> bits) | (carry << (8 - bits));
        carry  = b & mask;
    }
    trim_leading_zeros_vec(out)
}

#[inline]
fn trim_leading_zeros(bytes: &[u8]) -> &[u8] {
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len().saturating_sub(1));
    &bytes[start..]
}

#[inline]
fn trim_leading_zeros_vec(mut bytes: Vec<u8>) -> Vec<u8> {
    let zeros = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len().saturating_sub(1));
    if zeros > 0 { bytes.drain(..zeros); }
    bytes
}
