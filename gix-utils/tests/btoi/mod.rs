use gix_utils::btoi::{to_signed, to_signed_with_radix, to_unsigned, to_unsigned_with_radix};

#[test]
fn binary_to_unsigned() {
    assert_eq!(Ok(12345), to_unsigned(b"12345"));
    assert!(to_unsigned::<u8>(b"+1").is_err()); // only btoi allows signs
    assert!(to_unsigned::<u8>(b"256").is_err()); // overflow
}

#[test]
fn binary_to_unsigned_radix() {
    assert_eq!(Ok(255), to_unsigned_with_radix(b"ff", 16));
    assert_eq!(Ok(42), to_unsigned_with_radix(b"101010", 2));
}

#[test]
fn binary_to_integer_radix() {
    assert_eq!(Ok(10), to_signed_with_radix(b"a", 16));
    assert_eq!(Ok(10), to_signed_with_radix(b"+a", 16));
    assert_eq!(Ok(-42), to_signed_with_radix(b"-101010", 2));
}

#[test]
fn binary_to_integer() {
    assert_eq!(Ok(123), to_signed(b"123"));
    assert_eq!(Ok(123), to_signed(b"+123"));
    assert_eq!(Ok(-123), to_signed(b"-123"));

    assert!(to_signed::<u8>(b"123456789").is_err()); // overflow
    assert!(to_signed::<u64>(b"-1").is_err()); // underflow

    assert!(to_signed::<i32>(b" 42").is_err()); // leading space
}
