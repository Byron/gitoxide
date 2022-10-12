use git_features::hash;

use crate::{write, File, Version};

/// The error produced by [`File::write()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl File {
    /// Write the index to `out` with `options`, to be readable by [`File::at()`], returning the version that was actually written
    /// to retain all information of this index.
    pub fn write_to(&self, mut out: impl std::io::Write, options: write::Options) -> std::io::Result<Version> {
        let mut hasher = hash::Write::new(&mut out, options.hash_kind);
        let version = self.state.write_to(&mut hasher, options)?;

        let hash = hasher.hash.digest();
        out.write_all(&hash)?;
        Ok(version)
    }

    /// Write ourselves to the path we were read from after acquiring a lock, using `options`.
    ///
    /// Note that the hash produced will be stored which is why we need to be mutable.
    pub fn write(&mut self, _options: write::Options) -> Result<(), Error> {
        todo!()
    }
}
