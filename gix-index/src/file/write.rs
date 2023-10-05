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
        let _span = gix_features::trace::detail!("gix_index::File::write_to()", skip_hash = options.skip_hash);
        let (version, hash) = if options.skip_hash {
            let out: &mut dyn std::io::Write = &mut out;
            let version = self.state.write_to(out, options)?;
            (version, self.state.object_hash.null())
        } else {
            let mut hasher = hash::Write::new(&mut out, self.state.object_hash);
            let out: &mut dyn std::io::Write = &mut hasher;
            let version = self.state.write_to(out, options)?;
            (version, gix_hash::ObjectId::from(hasher.hash.digest()))
        };
        out.write_all(hash.as_slice())?;
        Ok((version, hash))
    }

    /// Write ourselves to the path we were read from after acquiring a lock, using `options`.
    ///
    /// Note that the hash produced will be stored which is why we need to be mutable.
    pub fn write(&mut self, options: write::Options) -> Result<(), Error> {
        let _span = gix_features::trace::detail!("gix_index::File::write()", path = ?self.path);
        let mut lock = std::io::BufWriter::with_capacity(
            64 * 1024,
            gix_lock::File::acquire_to_update_resource(&self.path, gix_lock::acquire::Fail::Immediately, None)?,
        );
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
