use crate::utils::bigint::BigInt;

#[test]
fn from_i64_zero() {
    let bi = BigInt::from_i64(0);
    assert!(bi.is_zero());
}

#[test]
fn from_i64_positive() {
    let bi = BigInt::from_i64(42);
    assert!(!bi.is_negative());
    assert_eq!(bi.to_i64().unwrap(), 42);
}

#[test]
fn from_i64_negative() {
    let bi = BigInt::from_i64(-42);
    assert!(bi.is_negative());
    assert_eq!(bi.to_i64().unwrap(), -42);
}

#[test]
fn from_i64_large() {
    let bi = BigInt::from_i64(i64::MAX);
    assert_eq!(bi.to_i64().unwrap(), i64::MAX);
}

#[test]
fn from_i64_min() {
    let bi = BigInt::from_i64(i64::MIN);
    assert_eq!(bi.to_i64().unwrap(), i64::MIN);
}

#[test]
fn from_f64_zero() {
    let bi = BigInt::from_f64(0.0);
    assert!(bi.is_zero());
}

#[test]
fn to_i64_zero() {
    let bi = BigInt::ZERO;
    assert_eq!(bi.to_i64().unwrap(), 0);
}

#[test]
fn to_i64_too_large() {
    let bi = BigInt::new(0x00, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(bi.to_i64().is_err());
}

#[test]
fn to_f64_zero() {
    let bi = BigInt::ZERO;
    assert_eq!(bi.to_f64().unwrap(), 0.0);
}
