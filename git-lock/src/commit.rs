use crate::{File, Marker};
use std::path::PathBuf;

pub mod marker {
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Persist { err: std::io::Error, marker: super::Marker } {
                display("Failed to persist marker")
                source(err)
            }
            NotCreatedFromFile {
                display("refusing to commit marker that was never opened")
            }
        }
    }
}

impl Marker {
    /// Commit the changes written to the previously open file and overwrite the original file atomically, returning the resource path
    /// on success. It will return the written resource path.
    ///
    /// This fails for markers which weren't created with [`File::close()`]
    pub fn commit(mut self) -> Result<PathBuf, marker::Error> {
        if !self.created_from_file {
            return Err(marker::Error::NotCreatedFromFile);
        }
        let resource_path = self.resource_path();
        let temppath = self.inner.take().expect("tempfile is always present");
        if let Err(err) = temppath.persist(&resource_path) {
            self.inner = git_tempfile::Handle::<git_tempfile::handle::Closed>::from(err.path);
            Err(marker::Error::Persist {
                err: err.error,
                marker: self,
            })
        } else {
            Ok(resource_path)
        }
    }
}

impl File {
    /// Commit the changes written to this lock file and overwrite the original file atomically, returning the resource path
    /// on success. It returns the written resource path.
    ///
    /// If a file is not committed, it will be deleted on drop or on signal.
    pub fn commit(self) -> std::io::Result<PathBuf> {
        let resource_path = self.resource_path();
        let tf = self.inner.take().expect("tempfile is always present");
        tf.persist(&resource_path)?;
        Ok(resource_path)
    }
}
