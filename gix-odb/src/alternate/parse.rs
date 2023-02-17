use std::{borrow::Cow, path::PathBuf};

use gix_object::bstr::ByteSlice;

/// Returned as part of [`crate::alternate::Error::Parse`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not obtain an object path for the alternate directory '{}'", String::from_utf8_lossy(.0))]
    PathConversion(Vec<u8>),
    #[error("Could not unquote alternate path")]
    Unquote(#[from] gix_quote::ansi_c::undo::Error),
}

pub(crate) fn content(input: &[u8]) -> Result<Vec<PathBuf>, Error> {
    let mut out = Vec::new();
    for line in input.split(|b| *b == b'\n') {
        let line = line.as_bstr();
        if line.is_empty() || line.starts_with(b"#") {
            continue;
        }
        out.push(
            gix_path::try_from_bstr(if line.starts_with(b"\"") {
                gix_quote::ansi_c::undo(line)?.0
            } else {
                Cow::Borrowed(line)
            })
            .map_err(|_| Error::PathConversion(line.to_vec()))?
            .into_owned(),
        )
    }
    Ok(out)
}
