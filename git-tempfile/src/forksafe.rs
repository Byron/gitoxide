use crate::{handle, AutoRemove};
use std::io::Write;
use tempfile::{NamedTempFile, TempPath};

enum TempfileOrTemppath {
    Tempfile(NamedTempFile),
    Temppath(TempPath),
}

pub(crate) struct ForksafeTempfile {
    inner: TempfileOrTemppath,
    cleanup: AutoRemove,
    pub owning_process_id: u32,
}

impl ForksafeTempfile {
    pub fn new(tempfile: NamedTempFile, cleanup: AutoRemove, mode: handle::Mode) -> Self {
        use handle::Mode::*;
        ForksafeTempfile {
            inner: match mode {
                Closed => TempfileOrTemppath::Temppath(tempfile.into_temp_path()),
                Writable => TempfileOrTemppath::Tempfile(tempfile),
            },
            cleanup,
            owning_process_id: std::process::id(),
        }
    }
}

impl ForksafeTempfile {
    pub fn as_mut_tempfile(&mut self) -> Option<&mut NamedTempFile> {
        match &mut self.inner {
            TempfileOrTemppath::Tempfile(file) => Some(file),
            TempfileOrTemppath::Temppath(_) => None,
        }
    }
    pub fn into_temppath(self) -> TempPath {
        match self.inner {
            TempfileOrTemppath::Tempfile(file) => file.into_temp_path(),
            TempfileOrTemppath::Temppath(path) => path,
        }
    }
    pub fn close(self) -> Self {
        if let TempfileOrTemppath::Tempfile(file) = self.inner {
            ForksafeTempfile {
                inner: TempfileOrTemppath::Temppath(file.into_temp_path()),
                cleanup: self.cleanup,
                owning_process_id: self.owning_process_id,
            }
        } else {
            self
        }
    }
    pub fn into_tempfile(self) -> Option<NamedTempFile> {
        match self.inner {
            TempfileOrTemppath::Tempfile(file) => Some(file),
            TempfileOrTemppath::Temppath(_) => None,
        }
    }
    pub fn drop_impl(self) {
        let file_path = match self.inner {
            TempfileOrTemppath::Tempfile(file) => file.path().to_owned(),
            TempfileOrTemppath::Temppath(path) => path.to_path_buf(),
        };
        let parent_directory = file_path.parent().expect("every tempfile has a parent directory");
        self.cleanup.execute_best_effort(&parent_directory);
    }

    pub fn drop_without_deallocation(self) {
        let temppath = match self.inner {
            TempfileOrTemppath::Tempfile(file) => {
                let (mut file, temppath) = file.into_parts();
                file.flush().ok();
                temppath
            }
            TempfileOrTemppath::Temppath(path) => path,
        };
        std::fs::remove_file(&temppath).ok();
        std::mem::forget(
            self.cleanup
                .execute_best_effort(temppath.parent().expect("every file has a directory")),
        );
        std::mem::forget(temppath); // leak memory to prevent deallocation
    }
}
