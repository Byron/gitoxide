// ported from https://github.com/niklasf/rust-btoi version 0.4.3
// see https://github.com/Byron/gitoxide/issues/729#issuecomment-1941515655

/// An error that can occur when parsing an integer.
///
/// * No digits
/// * Invalid digit
/// * Overflow
/// * Underflow
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIntegerError {
    kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ErrorKind {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
}

impl ParseIntegerError {
    fn desc(&self) -> &str {
        match self.kind {
            ErrorKind::Empty => "cannot parse integer without digits",
            ErrorKind::InvalidDigit => "invalid digit found in slice",
            ErrorKind::Overflow => "number too large to fit in target type",
            ErrorKind::Underflow => "number too small to fit in target type",
        }
    }
}

impl std::fmt::Display for ParseIntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.desc().fmt(f)
    }
}

impl std::error::Error for ParseIntegerError {
    fn description(&self) -> &str {
        self.desc()
    }
}
/// minimal subset of traits used by btoi_radix and btou_radix
pub trait MinNumTraits: Sized + Copy {
    ///
    fn from_u32(n: u32) -> Option<Self>;
    ///
    fn zero() -> Self;
    ///
    fn checked_mul(self, v: Self) -> Option<Self>;
    ///
    fn checked_add(self, v: Self) -> Option<Self>;
    ///
    fn checked_sub(self, v: Self) -> Option<Self>;
}

macro_rules! min_num_traits {
    ($t : ty, from_u32 => $from_u32 : expr) => {
        impl MinNumTraits for $t {
            fn from_u32(n: u32) -> Option<$t> {
                #[allow(clippy::redundant_closure_call)]
                $from_u32(n)
            }

            fn zero() -> Self {
                0
            }

            fn checked_mul(self, v: $t) -> Option<$t> {
                <$t>::checked_mul(self, v)
            }

            fn checked_add(self, v: $t) -> Option<$t> {
                <$t>::checked_add(self, v)
            }

            fn checked_sub(self, v: $t) -> Option<$t> {
                <$t>::checked_sub(self, v)
            }
        }
    };
}

min_num_traits!(i32, from_u32 => |n: u32| n.try_into().ok());
min_num_traits!(i64, from_u32 => |n: u32| Some(n.into()));
min_num_traits!(u64, from_u32 => |n: u32| Some(n.into()));
min_num_traits!(u8, from_u32 => |n: u32| n.try_into().ok());
min_num_traits!(usize, from_u32 => |n: u32| n.try_into().ok());

/// Converts a byte slice to an integer. Signs are not allowed.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for any of the following conditions:
///
/// * `bytes` is empty
/// * not all characters of `bytes` are `0-9`
/// * the number overflows `I`
///
/// # Panics
///
/// Panics in the pathological case that there is no representation of `10`
/// in `I`.
///
/// # Examples
///
/// ```
/// # use btoi::btou;
/// assert_eq!(Ok(12345), btou(b"12345"));
/// assert!(btou::<u8>(b"+1").is_err()); // only btoi allows signs
/// assert!(btou::<u8>(b"256").is_err()); // overflow
/// ```
///
/// [`ParseIntegerError`]: struct.ParseIntegerError.html
#[track_caller]
pub fn btou<I: MinNumTraits>(bytes: &[u8]) -> Result<I, ParseIntegerError> {
    btou_radix(bytes, 10)
}

#[test]
fn btou_assert() {
    assert_eq!(Ok(12345), btou(b"12345"));
    assert!(btou::<u8>(b"+1").is_err()); // only btoi allows signs
    assert!(btou::<u8>(b"256").is_err()); // overflow
}

/// Converts a byte slice in a given base to an integer. Signs are not allowed.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for any of the following conditions:
///
/// * `bytes` is empty
/// * not all characters of `bytes` are `0-9`, `a-z` or `A-Z`
/// * not all characters refer to digits in the given `radix`
/// * the number overflows `I`
///
/// # Panics
///
/// Panics if `radix` is not in the range `2..=36` (or in the pathological
/// case that there is no representation of `radix` in `I`).
///
/// # Examples
///
/// ```
/// # use btoi::btou_radix;
/// assert_eq!(Ok(255), btou_radix(b"ff", 16));
/// assert_eq!(Ok(42), btou_radix(b"101010", 2));
/// ```
///
/// [`ParseIntegerError`]: struct.ParseIntegerError.html
pub fn btou_radix<I: MinNumTraits>(bytes: &[u8], radix: u32) -> Result<I, ParseIntegerError> {
    assert!(
        (2..=36).contains(&radix),
        "radix must lie in the range 2..=36, found {radix}"
    );

    let base = I::from_u32(radix).expect("radix can be represented as integer");

    if bytes.is_empty() {
        return Err(ParseIntegerError { kind: ErrorKind::Empty });
    }

    let mut result = I::zero();

    for &digit in bytes {
        let x = match char::from(digit).to_digit(radix).and_then(I::from_u32) {
            Some(x) => x,
            None => {
                return Err(ParseIntegerError {
                    kind: ErrorKind::InvalidDigit,
                })
            }
        };
        result = match result.checked_mul(base) {
            Some(result) => result,
            None => {
                return Err(ParseIntegerError {
                    kind: ErrorKind::Overflow,
                })
            }
        };
        result = match result.checked_add(x) {
            Some(result) => result,
            None => {
                return Err(ParseIntegerError {
                    kind: ErrorKind::Overflow,
                })
            }
        };
    }

    Ok(result)
}

#[test]
fn btou_radix_assert() {
    assert_eq!(Ok(255), btou_radix(b"ff", 16));
    assert_eq!(Ok(42), btou_radix(b"101010", 2));
}

/// Converts a byte slice to an integer.
///
/// Like [`btou`], but numbers may optionally start with a sign (`-` or `+`).
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for any of the following conditions:
///
/// * `bytes` has no digits
/// * not all characters of `bytes` are `0-9`, excluding an optional leading
///   sign
/// * the number overflows or underflows `I`
///
/// # Panics
///
/// Panics in the pathological case that there is no representation of `10`
/// in `I`.
///
/// # Examples
///
/// ```
/// # use btoi::btoi;
/// assert_eq!(Ok(123), btoi(b"123"));
/// assert_eq!(Ok(123), btoi(b"+123"));
/// assert_eq!(Ok(-123), btoi(b"-123"));
///
/// assert!(btoi::<i16>(b"123456789").is_err()); // overflow
/// assert!(btoi::<u32>(b"-1").is_err()); // underflow
///
/// assert!(btoi::<i32>(b" 42").is_err()); // leading space
/// ```
///
/// [`btou`]: fn.btou.html
/// [`ParseIntegerError`]: struct.ParseIntegerError.html
pub fn btoi<I: MinNumTraits>(bytes: &[u8]) -> Result<I, ParseIntegerError> {
    btoi_radix(bytes, 10)
}

#[test]
fn btoi_assert() {
    assert_eq!(Ok(123), btoi(b"123"));
    assert_eq!(Ok(123), btoi(b"+123"));
    assert_eq!(Ok(-123), btoi(b"-123"));

    assert!(btoi::<u8>(b"123456789").is_err()); // overflow
    assert!(btoi::<u64>(b"-1").is_err()); // underflow

    assert!(btoi::<i32>(b" 42").is_err()); // leading space
}

/// Converts a byte slice in a given base to an integer.
///
/// Like [`btou_radix`], but numbers may optionally start with a sign
/// (`-` or `+`).
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for any of the following conditions:
///
/// * `bytes` has no digits
/// * not all characters of `bytes` are `0-9`, `a-z`, `A-Z`, exluding an
///   optional leading sign
/// * not all characters refer to digits in the given `radix`, exluding an
///   optional leading sign
/// * the number overflows or underflows `I`
///
/// # Panics
///
/// Panics if `radix` is not in the range `2..=36` (or in the pathological
/// case that there is no representation of `radix` in `I`).
///
/// # Examples
///
/// ```
/// # use btoi::btoi_radix;
/// assert_eq!(Ok(10), btoi_radix(b"a", 16));
/// assert_eq!(Ok(10), btoi_radix(b"+a", 16));
/// assert_eq!(Ok(-42), btoi_radix(b"-101010", 2));
/// ```
///
/// [`btou_radix`]: fn.btou_radix.html
/// [`ParseIntegerError`]: struct.ParseIntegerError.html
fn btoi_radix<I: MinNumTraits>(bytes: &[u8], radix: u32) -> Result<I, ParseIntegerError> {
    assert!(
        (2..=36).contains(&radix),
        "radix must lie in the range 2..=36, found {radix}"
    );

    let base = I::from_u32(radix).expect("radix can be represented as integer");

    if bytes.is_empty() {
        return Err(ParseIntegerError { kind: ErrorKind::Empty });
    }

    let digits = match bytes[0] {
        b'+' => return btou_radix(&bytes[1..], radix),
        b'-' => &bytes[1..],
        _ => return btou_radix(bytes, radix),
    };

    if digits.is_empty() {
        return Err(ParseIntegerError { kind: ErrorKind::Empty });
    }

    let mut result = I::zero();

    for &digit in digits {
        let x = match char::from(digit).to_digit(radix).and_then(I::from_u32) {
            Some(x) => x,
            None => {
                return Err(ParseIntegerError {
                    kind: ErrorKind::InvalidDigit,
                })
            }
        };
        result = match result.checked_mul(base) {
            Some(result) => result,
            None => {
                return Err(ParseIntegerError {
                    kind: ErrorKind::Underflow,
                })
            }
        };
        result = match result.checked_sub(x) {
            Some(result) => result,
            None => {
                return Err(ParseIntegerError {
                    kind: ErrorKind::Underflow,
                })
            }
        };
    }

    Ok(result)
}

#[test]
fn btoi_radix_assert() {
    assert_eq!(Ok(10), btoi_radix(b"a", 16));
    assert_eq!(Ok(10), btoi_radix(b"+a", 16));
    assert_eq!(Ok(-42), btoi_radix(b"-101010", 2));
}
