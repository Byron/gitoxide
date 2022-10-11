use crate::clone::PrepareCheckout;
use crate::Repository;

///
pub mod main_worktree {
    use crate::clone::PrepareCheckout;
    use crate::Repository;
    use std::sync::atomic::AtomicBool;

    /// The error returned by [`PrepareCheckout::main_worktree()`].
    #[derive(Debug, thiserror::Error)]
    #[error("TBD")]
    pub struct Error {}

    /// Modification
    impl PrepareCheckout {
        /// Checkout the main worktree
        pub fn main_worktree(
            &mut self,
            _progress: impl crate::Progress,
            _should_interrupt: &AtomicBool,
        ) -> Result<Repository, Error> {
            todo!()
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
