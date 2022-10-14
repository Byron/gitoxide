use crate::clone::PrepareCheckout;
use crate::Repository;

///
pub mod main_worktree {
    use crate::clone::PrepareCheckout;
    use crate::Repository;
    use git_odb::FindExt;
    use std::path::PathBuf;
    use std::sync::atomic::AtomicBool;

    /// The error returned by [`PrepareCheckout::main_worktree()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Repository at \"{}\" is a bare repository and cannot have a main worktree checkout", git_dir.display())]
        BareRepository { git_dir: PathBuf },
        #[error("The object pointed to by HEAD is not a treeish")]
        NoHeadTree(#[from] crate::object::peel::to_kind::Error),
        #[error("Could not create index from tree at {id}")]
        IndexFromTree {
            id: git_hash::ObjectId,
            source: git_traverse::tree::breadthfirst::Error,
        },
        #[error(transparent)]
        WriteIndex(#[from] git_index::file::write::Error),
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
            let root_tree = repo
                .head_id()
                .expect("head points to an object")
                .object()
                .expect("downloaded from remote")
                .peel_to_tree()?
                .id;
            let index = git_index::State::from_tree(&root_tree, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())
                .map_err(|err| Error::IndexFromTree {
                    id: root_tree,
                    source: err,
                })?;
            let mut index = git_index::File::from_state(index, repo.index_path());
            index.write(Default::default())?;

            // git_worktree::index::checkout(
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
