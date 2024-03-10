use crate::{Blob, ObjectDetached};

///
#[cfg(feature = "blob-diff")]
pub mod diff {
    use std::ops::Range;

    use gix_diff::blob::{platform::prepare_diff::Operation, ResourceKind};

    use crate::{
        bstr::ByteSlice,
        object::{blob::diff::lines::Change, tree::diff::change::Event},
    };

    /// A platform to keep temporary information to perform line diffs on modified blobs.
    ///
    pub struct Platform<'a> {
        /// The cache holding diffable data related to our blobs.
        pub resource_cache: &'a mut gix_diff::blob::Platform,
    }

    ///
    #[allow(clippy::empty_docs)]
    pub mod init {
        /// The error returned by [`Platform::from_tree_change()`][super::Platform::from_tree_change()].
        pub type Error = gix_diff::blob::platform::set_resource::Error;
    }

    impl<'a> Platform<'a> {
        /// Produce a platform for performing various diffs after obtaining the data from a single `tree_change`.
        pub fn from_tree_change(
            tree_change: &crate::object::tree::diff::Change<'_, '_, '_>,
            resource_cache: &'a mut gix_diff::blob::Platform,
        ) -> Result<Platform<'a>, init::Error> {
            match tree_change.event {
                Event::Addition { entry_mode, id } => {
                    resource_cache.set_resource(
                        id.repo.object_hash().null(),
                        entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::OldOrSource,
                        &id.repo.objects,
                    )?;
                    resource_cache.set_resource(
                        id.inner,
                        entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::NewOrDestination,
                        &id.repo.objects,
                    )?;
                }
                Event::Deletion { entry_mode, id } => {
                    resource_cache.set_resource(
                        id.inner,
                        entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::OldOrSource,
                        &id.repo.objects,
                    )?;
                    resource_cache.set_resource(
                        id.repo.object_hash().null(),
                        entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::NewOrDestination,
                        &id.repo.objects,
                    )?;
                }
                Event::Modification {
                    previous_entry_mode,
                    previous_id,
                    entry_mode,
                    id,
                } => {
                    resource_cache.set_resource(
                        previous_id.inner,
                        previous_entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::OldOrSource,
                        &previous_id.repo.objects,
                    )?;
                    resource_cache.set_resource(
                        id.inner,
                        entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::NewOrDestination,
                        &id.repo.objects,
                    )?;
                }
                Event::Rewrite {
                    source_location,
                    source_entry_mode,
                    source_id,
                    entry_mode,
                    id,
                    diff: _,
                    copy: _,
                } => {
                    resource_cache.set_resource(
                        source_id.inner,
                        source_entry_mode.kind(),
                        source_location,
                        ResourceKind::OldOrSource,
                        &source_id.repo.objects,
                    )?;
                    resource_cache.set_resource(
                        id.inner,
                        entry_mode.kind(),
                        tree_change.location,
                        ResourceKind::NewOrDestination,
                        &id.repo.objects,
                    )?;
                }
            }
            Ok(Self { resource_cache })
        }
    }

    ///
    #[allow(clippy::empty_docs)]
    pub mod lines {
        use crate::bstr::BStr;

        /// The error returned by [Platform::lines()](super::Platform::lines()).
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error<E>
        where
            E: std::error::Error + Send + Sync + 'static,
        {
            #[error(transparent)]
            ProcessHunk(E),
            #[error(transparent)]
            PrepareDiff(#[from] gix_diff::blob::platform::prepare_diff::Error),
        }

        /// A change to a hunk of lines.
        pub enum Change<'a, 'data> {
            /// Lines were added.
            Addition {
                /// The lines themselves without terminator.
                lines: &'a [&'data BStr],
            },
            /// Lines were removed.
            Deletion {
                /// The lines themselves without terminator.
                lines: &'a [&'data BStr],
            },
            /// Lines have been replaced.
            Modification {
                /// The replaced lines without terminator.
                lines_before: &'a [&'data BStr],
                /// The new lines without terminator.
                lines_after: &'a [&'data BStr],
            },
        }
    }

    impl<'a> Platform<'a> {
        /// Perform a diff on lines between the old and the new version of a blob, passing each hunk of lines to `process_hunk`.
        /// The diffing algorithm is determined by the `diff.algorithm` configuration, or individual diff drivers.
        /// Note that `process_hunk` is not called if one of the involved resources are binary, but that can be determined
        /// by introspecting the outcome.
        // TODO: more tests (only tested insertion right now)
        pub fn lines<FnH, E>(
            &mut self,
            mut process_hunk: FnH,
        ) -> Result<gix_diff::blob::platform::prepare_diff::Outcome<'_>, lines::Error<E>>
        where
            FnH: FnMut(lines::Change<'_, '_>) -> Result<(), E>,
            E: std::error::Error + Send + Sync + 'static,
        {
            self.resource_cache.options.skip_internal_diff_if_external_is_configured = false;

            let prep = self.resource_cache.prepare_diff()?;
            match prep.operation {
                Operation::InternalDiff { algorithm } => {
                    let input = prep.interned_input();
                    let mut err = None;
                    let mut lines = Vec::new();

                    gix_diff::blob::diff(algorithm, &input, |before: Range<u32>, after: Range<u32>| {
                        if err.is_some() {
                            return;
                        }
                        lines.clear();
                        lines.extend(
                            input.before[before.start as usize..before.end as usize]
                                .iter()
                                .map(|&line| input.interner[line].as_bstr()),
                        );
                        let end_of_before = lines.len();
                        lines.extend(
                            input.after[after.start as usize..after.end as usize]
                                .iter()
                                .map(|&line| input.interner[line].as_bstr()),
                        );
                        let hunk_before = &lines[..end_of_before];
                        let hunk_after = &lines[end_of_before..];
                        if hunk_after.is_empty() {
                            err = process_hunk(Change::Deletion { lines: hunk_before }).err();
                        } else if hunk_before.is_empty() {
                            err = process_hunk(Change::Addition { lines: hunk_after }).err();
                        } else {
                            err = process_hunk(Change::Modification {
                                lines_before: hunk_before,
                                lines_after: hunk_after,
                            })
                            .err();
                        }
                    });

                    if let Some(err) = err {
                        return Err(lines::Error::ProcessHunk(err));
                    }
                }
                Operation::ExternalCommand { .. } => {
                    unreachable!("we disabled that")
                }
                Operation::SourceOrDestinationIsBinary => {}
            };
            Ok(prep)
        }

        /// Count the amount of removed and inserted lines efficiently.
        /// Note that nothing will happen if one of the inputs is binary, and `None` will be returned.
        pub fn line_counts(
            &mut self,
        ) -> Result<Option<gix_diff::blob::sink::Counter<()>>, gix_diff::blob::platform::prepare_diff::Error> {
            self.resource_cache.options.skip_internal_diff_if_external_is_configured = false;

            let prep = self.resource_cache.prepare_diff()?;
            match prep.operation {
                Operation::InternalDiff { algorithm } => {
                    let tokens = prep.interned_input();
                    let counter = gix_diff::blob::diff(algorithm, &tokens, gix_diff::blob::sink::Counter::default());
                    Ok(Some(counter))
                }
                Operation::ExternalCommand { .. } => {
                    unreachable!("we disabled that")
                }
                Operation::SourceOrDestinationIsBinary => Ok(None),
            }
        }
    }
}

/// Remove Lifetime
impl Blob<'_> {
    /// Create an owned instance of this object, copying our data in the process.
    pub fn detached(&self) -> ObjectDetached {
        ObjectDetached {
            id: self.id,
            kind: gix_object::Kind::Blob,
            data: self.data.clone(),
        }
    }

    /// Sever the connection to the `Repository` and turn this instance into a standalone object.
    pub fn detach(self) -> ObjectDetached {
        self.into()
    }

    /// Retrieve this instance's data, leaving its own data empty.
    ///
    /// This method works around the immovability of members of this type.
    pub fn take_data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.data)
    }
}
