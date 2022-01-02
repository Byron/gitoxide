use crate::pack;
use crate::types::IndexAndPacks;
use git_features::progress::Progress;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

#[allow(missing_docs, unused)]

///
pub mod integrity {
    use crate::pack;

    /// Returned by [`Store::verify_integrity()`][crate::Store::verify_integrity()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        MultiIndexIntegrity(#[from] pack::index::traverse::Error<pack::multi_index::verify::integrity::Error>),
        #[error(transparent)]
        IndexIntegrity(#[from] pack::index::traverse::Error<pack::index::verify::integrity::Error>),
        #[error(transparent)]
        IndexOpen(#[from] pack::index::init::Error),
        #[error(transparent)]
        MultiIndexOpen(#[from] pack::multi_index::init::Error),
        #[error(transparent)]
        PackOpen(#[from] pack::data::init::Error),
        #[error(transparent)]
        InitializeODB(#[from] crate::store::load_index::Error),
        #[error("The disk on state changed while performing the operation, and we observed the change.")]
        NeedsRetryDueToChangeOnDisk,
    }

    /// Returned by [`Store::verify_integrity()`][crate::Store::verify_integrity()].
    pub struct Outcome<P> {
        /// Pack traversal statistics for each pack whose objects were checked.
        pub pack_traverse_statistics: Vec<pack::index::traverse::Statistics>,
        /// The provided progress instance.
        pub progress: P,
    }
}

impl super::Store {
    /// Check the integrity of all objects as per the given `options`.
    ///
    /// Note that this will not not force loading all indices or packs permanently, as we will only use the momentarily loaded disk state.
    /// This does, however, include all alternates.
    pub fn verify_integrity<C, P, F>(
        &self,
        mut progress: P,
        should_interrupt: &AtomicBool,
        options: pack::index::verify::integrity::Options<F>,
    ) -> Result<integrity::Outcome<P>, integrity::Error>
    where
        P: Progress,
        C: pack::cache::DecodeEntry,
        F: Fn() -> C + Send + Clone,
    {
        let mut index = self.index.load();
        if !index.is_initialized() {
            self.consolidate_with_disk_state(false)?;
            index = self.index.load();
            assert!(
                index.is_initialized(),
                "BUG: after consolidating successfully, we have an initialized index"
            )
        }

        progress.init(
            Some(index.slot_indices.len()),
            git_features::progress::count("pack indices"),
        );
        let mut statistics = Vec::new();
        for slot_index in &index.slot_indices {
            let slot = &self.files[*slot_index];
            if slot.generation.load(Ordering::SeqCst) != index.generation {
                return Err(integrity::Error::NeedsRetryDueToChangeOnDisk);
            }
            let files = slot.files.load();
            let files = Option::as_ref(&files).ok_or(integrity::Error::NeedsRetryDueToChangeOnDisk)?;

            match files {
                IndexAndPacks::Index(bundle) => {
                    let index;
                    let index = match bundle.index.loaded() {
                        Some(index) => index.deref(),
                        None => {
                            index = pack::index::File::at(bundle.index.path(), self.object_hash)?;
                            &index
                        }
                    };
                    let pack;
                    let data = match bundle.data.loaded() {
                        Some(pack) => pack.deref(),
                        None => {
                            pack = pack::data::File::at(bundle.data.path(), self.object_hash)?;
                            &pack
                        }
                    };
                    let outcome = index.verify_integrity(
                        Some(pack::index::verify::PackContext {
                            data,
                            options: options.clone(),
                        }),
                        progress.add_child("Checking integrity"),
                        should_interrupt,
                    )?;
                    statistics.push(
                        outcome
                            .pack_traverse_statistics
                            .expect("pack provided so there are stats"),
                    );
                }
                IndexAndPacks::MultiIndex(bundle) => {
                    let index;
                    let index = match bundle.multi_index.loaded() {
                        Some(index) => index.deref(),
                        None => {
                            index = pack::multi_index::File::at(bundle.multi_index.path())?;
                            &index
                        }
                    };
                    let outcome = index.verify_integrity(
                        progress.add_child("Checking integrity"),
                        should_interrupt,
                        options.clone(),
                    )?;
                    statistics.extend(outcome.pack_traverse_statistics);
                }
            }
            progress.inc();
        }

        for _loose_db in &*index.loose_dbs {
            // TODO: impl verify integrity for loose object databases
        }

        Ok(integrity::Outcome {
            pack_traverse_statistics: statistics,
            progress,
        })
    }
}
