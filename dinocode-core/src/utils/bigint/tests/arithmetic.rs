use crate::utils::bigint::arithmetic::*;

fn h(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn add_carry() {
    assert_eq!(add_unsigned(&[0xFF, 0xFF], &[1]), h("010000"));
}

#[test]
fn sub_borrow() {
    assert_eq!(sub_unsigned(&h("0100"), &[1]), vec![0xFF]);
}

#[test]
fn mul_large() {
    assert_eq!(mul_unsigned(&h("00010000"), &h("00010000")), h("0100000000"));
}

#[test]
fn mul_zero() {
    assert_eq!(mul_unsigned(&[0], &[1]), vec![0]);
    assert_eq!(mul_unsigned(&[1], &[0]), vec![0]);
}

#[test]
fn div_multi_digit() {
    assert_eq!(div_unsigned(&h("010000"), &h("0101")), vec![0xFF]);
    assert_eq!(mod_unsigned(&h("010000"), &h("0101")), vec![1]);
}

#[test]
fn divmod_power_of_two() {
    assert_eq!(div_unsigned(&h("0100000000"), &h("00010000")), vec![1, 0, 0]);
    assert_eq!(mod_unsigned(&h("0100000000"), &h("00010000")), vec![0]);
}

#[test]
fn div_less_than_divisor() {
    assert_eq!(div_unsigned(&[5], &[10]), vec![0]);
    assert_eq!(mod_unsigned(&[5], &[10]), vec![5]);
}

#[test]
fn div_equal() {
    assert_eq!(div_unsigned(&[10], &[10]), vec![1]);
    assert_eq!(mod_unsigned(&[10], &[10]), vec![0]);
}
