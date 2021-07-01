use crate::{File, Marker, DOT_SUFFIX};
use std::path::{Path, PathBuf};

fn strip_lock_suffix(lock_path: &Path) -> PathBuf {
    lock_path.with_extension(lock_path.extension().map_or("".to_string(), |ext| {
        let ext = ext.to_string_lossy();
        ext.split_at(ext.len().saturating_sub(DOT_SUFFIX.len())).0.to_string()
    }))
}

impl File {
    /// Obtain a mutable reference to the write handle and call `f(out)` with it.
    pub fn with_mut<T>(&mut self, f: impl FnOnce(&mut std::fs::File) -> std::io::Result<T>) -> std::io::Result<T> {
        self.inner.with_mut(|tf| f(tf.as_file_mut())).and_then(|res| res)
    }
    /// Commit the changes written to this lock file and overwrite the original resource atomically, returning the resource path
    /// on success.
    ///
    /// If a file is not committed, it will be deleted on drop or on signal.
    pub fn commit(self) -> std::io::Result<()> {
        let tf = self.inner.take().expect("tempfile is always present");
        let resource_path = strip_lock_suffix(tf.path());
        tf.persist(resource_path)?;
        Ok(())
    }
}

impl Marker {
    /// Commit the changes written to the previously open file and overwrite the original resource atomically, returning the resource path
    /// on success.
    ///
    /// This fails for markers which weren't created with [`File::close()`]
    pub fn commit(self) -> std::io::Result<()> {
        if !self.created_from_file {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "refusing to commit marker that was never opened",
            ));
        }
        let temppath = self.inner.take().expect("tempfile is always present");
        let resource_path = strip_lock_suffix(&temppath);
        temppath.persist(resource_path)?;
        Ok(())
    }
}
