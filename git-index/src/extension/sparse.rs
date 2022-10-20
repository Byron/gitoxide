use crate::extension::Signature;
use std::convert::TryFrom;

/// The signature of the sparse index extension, nothing more than an indicator at this time.
pub const SIGNATURE: Signature = *b"sdir";

/// Serialize the sparse index extension to out
pub fn write_to(mut out: impl std::io::Write) -> Result<(), std::io::Error> {
    out.write_all(&SIGNATURE)?;
    out.write_all(&(u32::try_from(0).expect("0 is a u32")).to_be_bytes())?;
    Ok(())
}
