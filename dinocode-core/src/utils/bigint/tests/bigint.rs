use crate::utils::bigint::BigInt;

#[test]
fn test_new() {
    let bi = BigInt::new(0x00, vec![1, 2, 3]);
    assert_eq!(bi.sign, 0x00);
    assert_eq!(bi.magnitude(), &[1, 2, 3]);
}

#[test]
fn test_into_owned() {
    let data = vec![1, 2, 3];
    let bi = BigInt::from_slice(0x00, &data);
    let owned = bi.into_owned();
    assert_eq!(owned.magnitude(), &[1, 2, 3]);
}

#[test]
fn test_display_zero() {
    assert_eq!(format!("{}", BigInt::ZERO), "0");
}

#[test]
fn test_display_positive() {
    let bi = BigInt::new(0x00, vec![1]);
    assert_eq!(format!("{}", bi), "1");
}

#[test]
fn test_display_negative() {
    let bi = BigInt::new(0x01, vec![1]);
    assert_eq!(format!("{}", bi), "-1");
}

#[test]
fn test_display_large() {
    let bi = BigInt::new(0x00, vec![0x01, 0x00]);
    assert_eq!(format!("{}", bi), "256");
}
