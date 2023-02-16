use gix_features::hash;

use crate::{write, File, Version};

/// The error produced by [`File::write()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not acquire lock for index file")]
    AcquireLock(#[from] gix_lock::acquire::Error),
    #[error("Could not commit lock for index file")]
    CommitLock(#[from] gix_lock::commit::Error<gix_lock::File>),
}

impl File {
    /// Write the index to `out` with `options`, to be readable by [`File::at()`], returning the version that was actually written
    /// to retain all information of this index.
    pub fn write_to(
        &self,
        mut out: impl std::io::Write,
        options: write::Options,
    ) -> std::io::Result<(Version, gix_hash::ObjectId)> {
        let mut hasher = hash::Write::new(&mut out, self.state.object_hash);
        let version = self.state.write_to(&mut hasher, options)?;

        let hash = hasher.hash.digest();
        out.write_all(&hash)?;
        Ok((version, gix_hash::ObjectId::from(hash)))
    }

    /// Write ourselves to the path we were read from after acquiring a lock, using `options`.
    ///
    /// Note that the hash produced will be stored which is why we need to be mutable.
    pub fn write(&mut self, options: write::Options) -> Result<(), Error> {
        let mut lock = std::io::BufWriter::new(gix_lock::File::acquire_to_update_resource(
            &self.path,
            gix_lock::acquire::Fail::Immediately,
            None,
        )?);
        let (version, digest) = self.write_to(&mut lock, options)?;
        match lock.into_inner() {
            Ok(lock) => lock.commit()?,
            Err(err) => return Err(err.into_error().into()),
        };
        self.state.version = version;
        self.checksum = Some(digest);
        Ok(())
    }
}
