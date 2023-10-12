use crate::{Blob, ObjectDetached};

///
#[cfg(feature = "blob-diff")]
pub mod diff {
    use std::ops::Range;

    use crate::{bstr::ByteSlice, object::blob::diff::line::Change};

    /// A platform to keep temporary information to perform line diffs on modified blobs.
    ///
    pub struct Platform<'old, 'new> {
        /// The previous version of the blob.
        pub old: crate::Object<'old>,
        /// The new version of the blob.
        pub new: crate::Object<'new>,
        /// The algorithm to use when calling [imara_diff::diff()][gix_diff::blob::diff()].
        /// This value is determined by the `diff.algorithm` configuration.
        pub algo: gix_diff::blob::Algorithm,
    }

    ///
    pub mod init {
        /// The error returned by [`Platform::from_ids()`][super::Platform::from_ids()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Could not find the previous blob or the new blob to diff against")]
            FindExisting(#[from] crate::object::find::existing::Error),
            #[error("Could not obtain diff algorithm from configuration")]
            DiffAlgorithm(#[from] crate::config::diff::algorithm::Error),
        }
    }

    impl<'old, 'new> Platform<'old, 'new> {
        /// Produce a platform for performing various diffs after obtaining the object data of `previous_id` and `new_id`.
        ///
        /// Note that these objects are treated as raw data and are assumed to be blobs.
        pub fn from_ids(
            previous_id: &crate::Id<'old>,
            new_id: &crate::Id<'new>,
        ) -> Result<Platform<'old, 'new>, init::Error> {
            match previous_id
                .object()
                .and_then(|old| new_id.object().map(|new| (old, new)))
            {
                Ok((old, new)) => {
                    let algo = match new_id.repo.config.diff_algorithm() {
                        Ok(algo) => algo,
                        Err(err) => return Err(err.into()),
                    };
                    Ok(Platform { old, new, algo })
                }
                Err(err) => Err(err.into()),
            }
        }
    }

    ///
    pub mod line {
        use crate::bstr::BStr;

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

    impl<'old, 'new> Platform<'old, 'new> {
        /// Perform a diff on lines between the old and the new version of a blob, passing each hunk of lines to `process_hunk`.
        /// The diffing algorithm is determined by the `diff.algorithm` configuration.
        ///
        /// Note that you can invoke the diff more flexibly as well.
        // TODO: more tests (only tested insertion right now)
        pub fn lines<FnH, E>(&self, mut process_hunk: FnH) -> Result<(), E>
        where
            FnH: FnMut(line::Change<'_, '_>) -> Result<(), E>,
            E: std::error::Error,
        {
            let input = self.line_tokens();
            let mut err = None;
            let mut lines = Vec::new();
            gix_diff::blob::diff(self.algo, &input, |before: Range<u32>, after: Range<u32>| {
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

            match err {
                Some(err) => Err(err),
                None => Ok(()),
            }
        }

        /// Count the amount of removed and inserted lines efficiently.
        pub fn line_counts(&self) -> gix_diff::blob::sink::Counter<()> {
            let tokens = self.line_tokens();
            gix_diff::blob::diff(self.algo, &tokens, gix_diff::blob::sink::Counter::default())
        }

        /// Return a tokenizer which treats lines as smallest unit for use in a [diff operation][gix_diff::blob::diff()].
        ///
        /// The line separator is determined according to normal git rules and filters.
        pub fn line_tokens(&self) -> gix_diff::blob::intern::InternedInput<&[u8]> {
            // TODO: make use of `core.eol` and/or filters to do line-counting correctly. It's probably
            //       OK to just know how these objects are saved to know what constitutes a line.
            gix_diff::blob::intern::InternedInput::new(self.old.data.as_bytes(), self.new.data.as_bytes())
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
