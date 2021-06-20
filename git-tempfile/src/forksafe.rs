use crate::{registration, AutoRemove};
use std::io::Write;
use tempfile::{NamedTempFile, TempPath};

pub(crate) struct ForksafeTempfile {
    pub inner: NamedTempFile,
    pub cleanup: AutoRemove,
    pub owning_process_id: u32,
}

impl ForksafeTempfile {
    pub fn new(inner: NamedTempFile, cleanup: AutoRemove, mode: registration::Mode) -> Self {
        match mode {
            registration::Mode::Closed => todo!("closed mode"),
            registration::Mode::Writable => ForksafeTempfile {
                inner,
                cleanup,
                owning_process_id: std::process::id(),
            },
        }
    }
}

impl ForksafeTempfile {
    pub fn into_temppath(self) -> Option<TempPath> {
        Some(self.inner.into_temp_path())
    }
    pub fn into_tempfile(self) -> Option<NamedTempFile> {
        Some(self.inner)
    }
    pub fn drop_impl(self) {
        let directory = self
            .inner
            .path()
            .parent()
            .expect("every tempfile has a parent directory")
            .to_owned();
        drop(self.inner);
        self.cleanup.execute_best_effort(&directory);
    }

    pub fn drop_without_deallocation(self) {
        let (mut file, temppath) = self.inner.into_parts();
        file.flush().ok();
        std::fs::remove_file(&temppath).ok();
        std::mem::forget(
            self.cleanup
                .execute_best_effort(temppath.parent().expect("every file has a directory")),
        );
        std::mem::forget(temppath); // leak memory to prevent deallocation
    }
}
