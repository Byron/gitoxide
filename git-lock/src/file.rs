use crate::{File, Marker, DOT_LOCK_SUFFIX};
use std::path::{Path, PathBuf};

fn strip_lock_suffix(lock_path: &Path) -> PathBuf {
    lock_path.with_extension(lock_path.extension().map_or("".to_string(), |ext| {
        let ext = ext.to_string_lossy();
        ext.split_at(ext.len().saturating_sub(DOT_LOCK_SUFFIX.len()))
            .0
            .to_string()
    }))
}

impl File {
    /// Obtain a mutable reference to the write handle and call `f(out)` with it.
    pub fn with_mut<T>(&mut self, f: impl FnOnce(&mut std::fs::File) -> std::io::Result<T>) -> std::io::Result<T> {
        self.inner.with_mut(|tf| f(tf.as_file_mut())).and_then(|res| res)
    }
    /// Close the lock file to prevent further writes and to save system resources.
    /// A call to [Marker::commit()] is allowed on the [`Marker`] to write changes back to the resource.
    pub fn close(self) -> std::io::Result<Marker> {
        Ok(Marker {
            inner: self.inner.close()?,
            created_from_file: true,
            lock_path: self.lock_path,
        })
    }

    /// Return the path at which the lock file resides
    pub fn lock_path(&self) -> &Path {
        &self.lock_path
    }

    /// Return the path at which the locked resource resides
    pub fn resource_path(&self) -> PathBuf {
        strip_lock_suffix(&self.lock_path)
    }
}

impl Marker {
    /// Return the path at which the lock file resides
    pub fn lock_path(&self) -> &Path {
        &self.lock_path
    }

    /// Return the path at which the locked resource resides
    pub fn resource_path(&self) -> PathBuf {
        strip_lock_suffix(&self.lock_path)
    }
}
