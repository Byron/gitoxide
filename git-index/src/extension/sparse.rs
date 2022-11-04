use crate::extension::Signature;

/// The signature of the sparse index extension, nothing more than an indicator at this time.
pub const SIGNATURE: Signature = *b"sdir";

/// Serialize the sparse index extension to `out`
pub fn write_to(mut out: impl std::io::Write) -> Result<(), std::io::Error> {
    out.write_all(&SIGNATURE)?;
    out.write_all(&0_u32.to_be_bytes())?;
    Ok(())
}
