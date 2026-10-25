use crate::utils::bigint::BigInt;
use std::cmp::Ordering;

#[test]
fn cmp_positive_equal() {
    let a = BigInt::new(0x00, vec![1, 2, 3]);
    let b = BigInt::new(0x00, vec![1, 2, 3]);
    assert_eq!(a.cmp(&b), Ordering::Equal);
}

#[test]
fn cmp_positive_greater() {
    let a = BigInt::new(0x00, vec![2]);
    let b = BigInt::new(0x00, vec![1]);
    assert_eq!(a.cmp(&b), Ordering::Greater);
}

#[test]
fn cmp_positive_less() {
    let a = BigInt::new(0x00, vec![1]);
    let b = BigInt::new(0x00, vec![2]);
    assert_eq!(a.cmp(&b), Ordering::Less);
}

#[test]
fn cmp_negative_equal() {
    let a = BigInt::new(0x01, vec![1, 2, 3]);
    let b = BigInt::new(0x01, vec![1, 2, 3]);
    assert_eq!(a.cmp(&b), Ordering::Equal);
}

#[test]
fn cmp_negative_greater() {
    let a = BigInt::new(0x01, vec![2]);
    let b = BigInt::new(0x01, vec![1]);
    assert_eq!(a.cmp(&b), Ordering::Less);
}

#[test]
fn cmp_negative_less() {
    let a = BigInt::new(0x01, vec![1]);
    let b = BigInt::new(0x01, vec![2]);
    assert_eq!(a.cmp(&b), Ordering::Greater);
}

#[test]
fn cmp_positive_vs_negative() {
    let pos = BigInt::new(0x00, vec![1]);
    let neg = BigInt::new(0x01, vec![1]);
    assert_eq!(pos.cmp(&neg), Ordering::Greater);
    assert_eq!(neg.cmp(&pos), Ordering::Less);
}

#[test]
fn cmp_zero_positive() {
    let zero = BigInt::ZERO;
    let pos = BigInt::new(0x00, vec![1]);
    assert_eq!(zero.cmp(&pos), Ordering::Less);
}

#[test]
fn cmp_zero_negative() {
    let zero = BigInt::ZERO;
    let neg = BigInt::new(0x01, vec![1]);
    assert_eq!(zero.cmp(&neg), Ordering::Greater);
}
