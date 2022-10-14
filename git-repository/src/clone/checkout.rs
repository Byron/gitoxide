use crate::clone::PrepareCheckout;
use crate::Repository;

///
pub mod main_worktree {
    use crate::clone::PrepareCheckout;
    use crate::Repository;
    use std::path::PathBuf;
    use std::sync::atomic::AtomicBool;

    /// The error returned by [`PrepareCheckout::main_worktree()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Repository at \"{}\" is a bare repository and cannot have a main worktree checkout", git_dir.display())]
        BareRepository { git_dir: PathBuf },
    }

    /// Modification
    impl PrepareCheckout {
        /// Checkout the main worktree
        pub fn main_worktree(
            &mut self,
            _progress: impl crate::Progress,
            _should_interrupt: &AtomicBool,
        ) -> Result<Repository, Error> {
            let repo = self
                .repo
                .as_ref()
                .expect("still present as we never succeeded the worktree checkout yet");
            let _workdir = repo.work_dir().ok_or_else(|| Error::BareRepository {
                git_dir: repo.git_dir().to_owned(),
            })?;
            let _index_path = repo.index_path();
            //     git_worktree::index::checkout(
            //     &mut index,
            //     workdir,
            //     {
            //         let objects = repo.objects.into_arc()?;
            //         move |oid, buf| {
            //             objects.find_blob(oid, buf).ok();
            //             if empty_files {
            //                 // We always want to query the ODB here…
            //                 objects.find_blob(oid, buf)?;
            //                 buf.clear();
            //                 // …but write nothing
            //                 Ok(git::objs::BlobRef { data: buf })
            //             } else {
            //                 objects.find_blob(oid, buf)
            //             }
            //         }
            //     },
            //     &mut files,
            //     &mut bytes,
            //     should_interrupt,
            //     opts,
            // );
            todo!("which branch to use?")
        }
    }
}

/// Consumption
impl PrepareCheckout {
    /// Persist the contained repository as is even if an error may have occurred when checking out the main working tree.
    pub fn persist(mut self) -> Repository {
        self.repo.take().expect("present and consumed once")
    }
}

impl Drop for PrepareCheckout {
    fn drop(&mut self) {
        if let Some(repo) = self.repo.take() {
            std::fs::remove_dir_all(repo.work_dir().unwrap_or_else(|| repo.path())).ok();
        }
    }
}

impl From<PrepareCheckout> for Repository {
    fn from(prep: PrepareCheckout) -> Self {
        prep.persist()
    }
}
