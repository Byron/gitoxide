use bstr::{BStr, ByteSlice};

///
pub mod name {
    /// The error used in [name()](super::name()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Submodule names cannot be empty")]
        Empty,
        #[error("Submodules names must not contains '..'")]
        ParentComponent,
    }
}

/// Return the original `name` if it is valid, or the respective error indicating what was wrong with it.
pub fn name(name: &BStr) -> Result<&BStr, name::Error> {
    if name.is_empty() {
        return Err(name::Error::Empty);
    }
    match name.find(b"..") {
        Some(pos) => {
            let &b = name.get(pos + 2).ok_or(name::Error::ParentComponent)?;
            if b == b'/' || b == b'\\' {
                Err(name::Error::ParentComponent)
            } else {
                Ok(name)
            }
        }
        None => Ok(name),
    }
}
