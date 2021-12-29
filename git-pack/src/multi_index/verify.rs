use std::sync::atomic::AtomicBool;

use git_features::progress::Progress;

use crate::multi_index::File;

///
pub mod integrity {
    /// Returned by [`multi_index::File::verify_integrity()`][crate::multi_index::File::verify_integrity()].
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        MultiIndexChecksum(#[from] crate::multi_index::verify::checksum::Error),
        #[error(transparent)]
        IndexIntegrity(#[from] crate::index::verify::integrity::Error),
        #[error(transparent)]
        BundleInit(#[from] crate::bundle::init::Error),
    }

    /// Returned by [`multi_index::File::verify_integrity()`][crate::multi_index::File::verify_integrity()].
    pub struct Outcome<P> {
        /// The computed checksum of the multi-index which matched the stored one.
        pub actual_index_checksum: git_hash::ObjectId,
        /// The for each entry in [`index_names()`][super::File::index_names()] provide the corresponding pack traversal outcome.
        pub pack_traverse_outcomes: Vec<crate::index::traverse::Outcome>,
        /// The provided progress instance.
        pub progress: P,
    }
}

///
pub mod checksum {
    /// Returned by [`multi_index::File::verify_checksum()`][crate::multi_index::File::verify_checksum()].
    pub type Error = crate::verify::checksum::Error;
}

impl File {
    /// Validate that our [`checksum()`][File::checksum()] matches the actual contents
    /// of this index file, and return it if it does.
    pub fn verify_checksum(
        &self,
        progress: impl Progress,
        should_interrupt: &AtomicBool,
    ) -> Result<git_hash::ObjectId, checksum::Error> {
        crate::verify::checksum_on_disk_or_mmap(
            self.path(),
            &self.data,
            self.checksum(),
            self.object_hash,
            progress,
            should_interrupt,
        )
    }

    /// Similar to [`crate::Bundle::verify_integrity()`] but checks all contained indices and their packs.
    ///
    /// Note that it's considered a failure if an index doesn't have a corresponding pack.
    #[allow(unused)]
    pub fn verify_integrity<C, P>(
        &self,
        verify_mode: crate::index::verify::Mode,
        traversal: crate::index::traverse::Algorithm,
        make_pack_lookup_cache: impl Fn() -> C + Send + Clone,
        thread_limit: Option<usize>,
        mut progress: P,
        should_interrupt: &AtomicBool,
    ) -> Result<integrity::Outcome<P>, crate::index::traverse::Error<integrity::Error>>
    where
        P: Progress,
        C: crate::cache::DecodeEntry,
    {
        let parent = self.path.parent().expect("must be in a directory");

        let actual_index_checksum = self
            .verify_checksum(
                progress.add_child(format!("checksum of '{}'", self.path.display())),
                should_interrupt,
            )
            .map_err(integrity::Error::from)
            .map_err(crate::index::traverse::Error::Processor)?;

        let mut pack_traverse_outcomes = Vec::new();
        for index_file_name in &self.index_names {
            let bundle = crate::Bundle::at(parent.join(index_file_name), self.object_hash)
                .map_err(integrity::Error::from)
                .map_err(crate::index::traverse::Error::Processor)?;

            progress.set_name(index_file_name.display().to_string());
            let crate::bundle::verify::integrity::Outcome {
                actual_index_checksum: _,
                pack_traverse_outcome,
                progress: used_progress,
            } = bundle
                .verify_integrity(
                    verify_mode,
                    traversal,
                    make_pack_lookup_cache.clone(),
                    thread_limit,
                    progress,
                    should_interrupt,
                )
                .map_err(|err| {
                    use crate::index::traverse::Error::*;
                    match err {
                        Processor(err) => Processor(integrity::Error::IndexIntegrity(err)),
                        VerifyChecksum(err) => VerifyChecksum(err),
                        Tree(err) => Tree(err),
                        TreeTraversal(err) => TreeTraversal(err),
                        PackDecode { id, offset, source } => PackDecode { id, offset, source },
                        PackMismatch { expected, actual } => PackMismatch { expected, actual },
                        PackObjectMismatch {
                            expected,
                            actual,
                            offset,
                            kind,
                        } => PackObjectMismatch {
                            expected,
                            actual,
                            offset,
                            kind,
                        },
                        Crc32Mismatch {
                            expected,
                            actual,
                            offset,
                            kind,
                        } => Crc32Mismatch {
                            expected,
                            actual,
                            offset,
                            kind,
                        },
                        Interrupted => Interrupted,
                    }
                })?;
            pack_traverse_outcomes.push(pack_traverse_outcome);
            progress = used_progress;
        }

        Ok(integrity::Outcome {
            actual_index_checksum,
            pack_traverse_outcomes,
            progress,
        })
    }
}
