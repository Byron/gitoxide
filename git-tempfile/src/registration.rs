use crate::{AutoRemove, ContainingDirectory, ForksafeTempfile, Registration, NEXT_MAP_INDEX, REGISTER};
use std::{io, path::Path};
use tempfile::NamedTempFile;

/// Creation and ownership transfer
impl Registration {
    /// Create a registered tempfile at the given `path`, where `path` includes the desired filename.
    ///
    /// Depending on the `directory` configuration, intermediate directories will be created, and depending on `cleanup` empty
    /// intermediate directories will be removed.
    pub fn at_path_writable(
        path: impl AsRef<Path>,
        directory: ContainingDirectory,
        cleanup: AutoRemove,
    ) -> io::Result<Registration> {
        let path = path.as_ref();
        let tempfile = {
            let mut builder = tempfile::Builder::new();
            let dot_ext_storage;
            match path.file_stem() {
                Some(stem) => builder.prefix(stem),
                None => builder.prefix(""),
            };
            if let Some(ext) = path.extension() {
                dot_ext_storage = format!(".{}", ext.to_string_lossy());
                builder.suffix(&dot_ext_storage);
            }
            let parent_dir = path.parent().expect("parent directory is present");
            let parent_dir = directory.resolve(parent_dir)?;
            ForksafeTempfile::new(builder.rand_bytes(0).tempfile_in(parent_dir)?, cleanup)
        };
        let id = NEXT_MAP_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        expect_none(REGISTER.insert(id, Some(tempfile)));
        Ok(Registration { id })
    }

    /// Create a registered tempfile within `containing_directory` with a name that won't clash, and clean it up as specified with `cleanup`.
    /// Control how to deal with intermediate directories with `directory`.
    /// The temporary file is opened and can be written to using the [`map()`][Registration::map()] method.
    pub fn new_writable(
        containing_directory: impl AsRef<Path>,
        directory: ContainingDirectory,
        cleanup: AutoRemove,
    ) -> io::Result<Registration> {
        let containing_directory = directory.resolve(containing_directory.as_ref())?;
        let id = NEXT_MAP_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        expect_none(REGISTER.insert(
            id,
            Some(ForksafeTempfile::new(
                NamedTempFile::new_in(containing_directory)?,
                cleanup,
            )),
        ));
        Ok(Registration { id })
    }

    /// Take ownership of the temporary file.
    ///
    /// It's a theoretical possibility that the file isn't present anymore if signals interfere, hence the `Option`
    pub fn take(self) -> Option<NamedTempFile> {
        let res = REGISTER.remove(&self.id);
        std::mem::forget(self);
        res.and_then(|(_k, v)| v.map(|v| v.inner))
    }
}

/// Mutation
impl Registration {
    /// Obtain a mutable handler to the underlying named tempfile and call `f(&mut named_tempfile)` on it.
    ///
    /// Note that for the duration of the call, a signal interrupting the operation will cause the tempfile not to be cleaned up.
    /// Also note that it might theoretically be possible that due to signal interference the underlying tempfile isn't present
    /// anymore which may cause the function `f` not to be called and an io error kind `Interrupted` is returned, consuming the
    /// handle in the process.
    pub fn map<T>(self, once: impl FnOnce(&mut NamedTempFile) -> T) -> std::io::Result<(Self, T)> {
        match REGISTER.remove(&self.id) {
            Some((id, Some(mut t))) => {
                let res = once(&mut t.inner);
                expect_none(REGISTER.insert(id, Some(t)));
                Ok((self, res))
            }
            None | Some((_, None)) => Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                format!("The tempfile with id {} wasn't available anymore", self.id),
            )),
        }
    }
}

impl ContainingDirectory {
    fn resolve(self, dir: &Path) -> std::io::Result<&Path> {
        match self {
            ContainingDirectory::Exists => Ok(dir),
            ContainingDirectory::CreateAllRaceProof(retries) => crate::create_dir::all(dir, retries),
        }
    }
}

fn expect_none<T>(v: Option<T>) {
    assert!(
        v.is_none(),
        "there should never be conflicts or old values as ids are never reused."
    );
}

impl Drop for Registration {
    fn drop(&mut self) {
        if let Some((_id, Some(tempfile))) = REGISTER.remove(&self.id) {
            tempfile.drop_impl();
        }
    }
}
