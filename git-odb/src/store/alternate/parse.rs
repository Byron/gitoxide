use crate::store::alternate::unquote;
use git_object::bstr::ByteSlice;
use std::{borrow::Cow, path::PathBuf};

/// Returned as part of [`crate::alternate::Error::Parse`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not obtain an object path for the alternate directory '{}'", String::from_utf8_lossy(&.0))]
    PathConversion(Vec<u8>),
    #[error("Could not unquote alternate path")]
    Unquote(#[from] unquote::Error),
}

pub(crate) fn content(input: &[u8]) -> Result<Vec<PathBuf>, Error> {
    let mut out = Vec::new();
    for line in input.split(|b| *b == b'\n') {
        let line = line.as_bstr();
        if line.is_empty() || line.starts_with(b"#") {
            continue;
        }
        out.push(
            if line.starts_with(b"\"") {
                unquote::ansi_c(line)?
            } else {
                Cow::Borrowed(line)
            }
            .to_path()
            .map(ToOwned::to_owned)
            .map_err(|_| Error::PathConversion(line.to_vec()))?,
        )
    }
    Ok(out)
}
