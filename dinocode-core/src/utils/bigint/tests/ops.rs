use crate::utils::bigint::BigInt;

#[test]
fn add_positive_positive() {
    let a = BigInt::new(0x00, vec![1]);
    let b = BigInt::new(0x00, vec![2]);
    let result = &a + &b;
    assert_eq!(result.to_i64().unwrap(), 3);
}

#[test]
fn add_negative_negative() {
    let a = BigInt::new(0x01, vec![1]);
    let b = BigInt::new(0x01, vec![2]);
    let result = &a + &b;
    assert_eq!(result.to_i64().unwrap(), -3);
}

#[test]
fn add_positive_negative() {
    let a = BigInt::new(0x00, vec![5]);
    let b = BigInt::new(0x01, vec![3]);
    let result = &a + &b;
    assert_eq!(result.to_i64().unwrap(), 2);
}

#[test]
fn add_negative_positive() {
    let a = BigInt::new(0x01, vec![3]);
    let b = BigInt::new(0x00, vec![5]);
    let result = &a + &b;
    assert_eq!(result.to_i64().unwrap(), 2);
}

#[test]
fn sub_positive_positive() {
    let a = BigInt::new(0x00, vec![5]);
    let b = BigInt::new(0x00, vec![3]);
    let result = &a - &b;
    assert_eq!(result.to_i64().unwrap(), 2);
}

#[test]
fn sub_negative_negative() {
    let a = BigInt::new(0x01, vec![5]);
    let b = BigInt::new(0x01, vec![3]);
    let result = &a - &b;
    assert_eq!(result.to_i64().unwrap(), -2);
}

#[test]
fn mul_positive_positive() {
    let a = BigInt::new(0x00, vec![3]);
    let b = BigInt::new(0x00, vec![4]);
    let result = &a * &b;
    assert_eq!(result.to_i64().unwrap(), 12);
}

#[test]
fn mul_negative_positive() {
    let a = BigInt::new(0x01, vec![3]);
    let b = BigInt::new(0x00, vec![4]);
    let result = &a * &b;
    assert_eq!(result.to_i64().unwrap(), -12);
}

#[test]
fn mul_negative_negative() {
    let a = BigInt::new(0x01, vec![3]);
    let b = BigInt::new(0x01, vec![4]);
    let result = &a * &b;
    assert_eq!(result.to_i64().unwrap(), 12);
}

#[test]
fn div_positive_positive() {
    let a = BigInt::new(0x00, vec![12]);
    let b = BigInt::new(0x00, vec![3]);
    let result = &a / &b;
    assert_eq!(result.to_i64().unwrap(), 4);
}

#[test]
fn div_negative_positive() {
    let a = BigInt::new(0x01, vec![12]);
    let b = BigInt::new(0x00, vec![3]);
    let result = &a / &b;
    assert_eq!(result.to_i64().unwrap(), -4);
}

#[test]
fn rem_positive_positive() {
    let a = BigInt::new(0x00, vec![10]);
    let b = BigInt::new(0x00, vec![3]);
    let result = &a % &b;
    assert_eq!(result.to_i64().unwrap(), 1);
}

#[test]
fn rem_negative_positive() {
    let a = BigInt::new(0x01, vec![10]);
    let b = BigInt::new(0x00, vec![3]);
    let result = &a % &b;
    assert_eq!(result.to_i64().unwrap(), -1);
}

#[test]
fn pow_zero_exp() {
    let a = BigInt::new(0x00, vec![5]);
    let exp = BigInt::ZERO;
    let result = a.pow(&exp);
    assert_eq!(result.to_i64().unwrap(), 1);
}

#[test]
fn pow_positive_exp() {
    let a = BigInt::new(0x00, vec![2]);
    let exp = BigInt::new(0x00, vec![3]);
    let result = a.pow(&exp);
    assert_eq!(result.to_i64().unwrap(), 8);
}

#[test]
fn pow_zero_base() {
    let a = BigInt::ZERO;
    let exp = BigInt::new(0x00, vec![5]);
    let result = a.pow(&exp);
    assert!(result.is_zero());
}

#[test]
fn pow_negative_exp() {
    let a = BigInt::new(0x00, vec![2]);
    let exp = BigInt::new(0x01, vec![1]);
    let result = a.pow(&exp);
    assert!(result.is_zero());
}
