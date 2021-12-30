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
        #[error("Counted {actual} objects, but expected {expected} as per multi-index")]
        UnexpectedObjectCount { actual: usize, expected: usize },
        #[error("The object at multi-index entry {index} didn't match the expected oid sort-order or pack-offset")]
        OutOfOrder { index: usize },
        #[error("The fan at index {index} is out of order as it's larger then the following value.")]
        Fan { index: usize },
        #[error("The multi-index claims to have no objects")]
        Empty,
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

        if let Some(first_invalid) = crate::verify::fan(&self.fan) {
            return Err(crate::index::traverse::Error::Processor(integrity::Error::Fan {
                index: first_invalid,
            }));
        }

        if self.num_objects == 0 {
            return Err(crate::index::traverse::Error::Processor(integrity::Error::Empty));
        }

        let mut pack_traverse_outcomes = Vec::new();

        progress.set_name("Validating");
        let start = std::time::Instant::now();

        progress.init(
            Some(self.num_indices as usize),
            git_features::progress::count("indices"),
        );
        let mut order_progress = progress.add_child("obtain oid and offset");
        order_progress.init(
            Some(self.num_objects as usize),
            git_features::progress::count("objects"),
        );
        let mut oids_and_offsets = Vec::with_capacity(self.num_objects as usize);
        for (pack_id, index_file_name) in self.index_names.iter().enumerate() {
            progress.inc();
            let bundle = crate::Bundle::at(parent.join(index_file_name), self.object_hash)
                .map_err(integrity::Error::from)
                .map_err(crate::index::traverse::Error::Processor)?;

            let progress = progress.add_child(index_file_name.display().to_string());
            let crate::bundle::verify::integrity::Outcome {
                actual_index_checksum: _,
                pack_traverse_outcome,
                progress: _,
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

            oids_and_offsets.extend(
                bundle
                    .index
                    .iter()
                    .map(|e| (e.oid, e.pack_offset, pack_id as crate::multi_index::PackIndex)),
            );
            order_progress.inc_by(bundle.index.num_objects() as usize);
        }

        let order_start = std::time::Instant::now();
        order_progress.set_name("ordering by oid and deduplicating");
        order_progress.set(0);
        oids_and_offsets.sort_by(|l, r| l.0.cmp(&r.0));
        oids_and_offsets.dedup_by(|l, r| l.0.eq(&r.0));

        if oids_and_offsets.len() != self.num_objects as usize {
            return Err(crate::index::traverse::Error::Processor(
                integrity::Error::UnexpectedObjectCount {
                    actual: oids_and_offsets.len(),
                    expected: self.num_objects as usize,
                },
            ));
        }

        order_progress.set_name("comparing oid and pack offset");
        for (index, ((loid, lpack_offset, lpack_id), (roid, rpack_offset, rpack_id))) in oids_and_offsets
            .into_iter()
            .zip(self.iter().map(|e| (e.oid, e.pack_offset, e.pack_index)))
            .enumerate()
        {
            if loid != roid || lpack_offset != rpack_offset || lpack_id != rpack_id {
                if loid == roid && lpack_id != rpack_id {
                    // Right now we can't properly determine which pack would be chosen if objects exists in multiple packs, hence
                    // our comparison might be off here.
                    // TODO: check how git does the comparison or get into multi-index writing to be more true to the source.
                    continue;
                }
                return Err(crate::index::traverse::Error::Processor(integrity::Error::OutOfOrder {
                    index,
                }));
            }
        }
        order_progress.inc_by(self.num_objects as usize);
        order_progress.show_throughput(order_start);
        progress.show_throughput(start);

        Ok(integrity::Outcome {
            actual_index_checksum,
            pack_traverse_outcomes,
            progress,
        })
    }
}
