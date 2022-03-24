use std::{borrow::Cow, path::PathBuf};

use git_object::bstr::ByteSlice;

/// Returned as part of [`crate::alternate::Error::Parse`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not obtain an object path for the alternate directory '{}'", String::from_utf8_lossy(.0))]
    PathConversion(Vec<u8>),
    #[error("Could not unquote alternate path")]
    Unquote(#[from] git_quote::ansi_c::undo::Error),
}

pub(crate) fn content(input: &[u8]) -> Result<Vec<PathBuf>, Error> {
    let mut out = Vec::new();
    for line in input.split(|b| *b == b'\n') {
        let line = line.as_bstr();
        if line.is_empty() || line.starts_with(b"#") {
            continue;
        }
        out.push(
            git_features::path::from_bstr(if line.starts_with(b"\"") {
                git_quote::ansi_c::undo(line)?
            } else {
                Cow::Borrowed(line)
            })
            .map_err(|_| Error::PathConversion(line.to_vec()))?
            .into_owned(),
        )
    }
    Ok(out)
}
