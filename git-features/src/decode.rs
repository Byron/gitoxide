use std::io::Read;

/// Decode variable int numbers from a `Read` implementation.
///
/// Note: currently overflow checks are only done in debug mode.
#[inline]
pub fn leb64_from_read(mut r: impl Read) -> Result<(u64, usize), std::io::Error> {
    let mut b = [0u8; 1];
    let mut i = 0;
    r.read_exact(&mut b)?;
    i += 1;
    let mut value = b[0] as u64 & 0x7f;
    while b[0] & 0x80 != 0 {
        r.read_exact(&mut b)?;
        i += 1;
        debug_assert!(i <= 10, "Would overflow value at 11th iteration");
        value += 1;
        value = (value << 7) + (b[0] as u64 & 0x7f)
    }
    Ok((value, i))
}

/// Decode variable int numbers.
#[inline]
pub fn leb64(d: &[u8]) -> (u64, usize) {
    let mut i = 0;
    let mut c = d[i];
    i += 1;
    let mut value = c as u64 & 0x7f;
    while c & 0x80 != 0 {
        c = d[i];
        i += 1;
        debug_assert!(i <= 10, "Would overflow value at 11th iteration");
        value += 1;
        value = (value << 7) + (c as u64 & 0x7f)
    }
    (value, i)
}
