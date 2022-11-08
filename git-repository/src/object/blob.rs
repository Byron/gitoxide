///
pub mod diff {
    use crate::bstr::ByteSlice;

    /// A platform to keep temporary information to perform line diffs on modified blobs.
    ///
    pub struct Platform<'old, 'new> {
        /// The previous version of the blob.
        pub old: crate::Object<'old>,
        /// The new version of the blob.
        pub new: crate::Object<'new>,
        /// The algorithm to use when calling [imara_diff::diff()][git_diff::blob::diff()].
        /// This value is determined by the `diff.algorithm` configuration.
        pub algo: git_diff::blob::Algorithm,
    }

    ///
    pub mod init {
        /// The error returned by [`Platform::new()`][super::Platform::new()].
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

    impl<'old, 'new> Platform<'old, 'new> {
        /// Perform a diff on lines between the old and the new version of a blob, passing each hunk of lines to `process_hunk`.
        /// The diffing algorithm is determined by the `diff.algorithm` configuration.
        ///
        /// Note that you can invoke the diff more flexibly as well.
        pub fn lines<FnH, S>(&self, _process_hunk: FnH)
        where
            FnH: for<'a> FnOnce(&git_diff::blob::intern::InternedInput<&'a [u8]>) -> S,
        {
            let _intern = self.line_tokens();
            // git_diff::blob::diff(self.algo, &intern);
            todo!()
        }

        /// Count the amount of removed and inserted lines efficiently.
        pub fn line_counts(&self) -> git_diff::blob::sink::Counter<()> {
            let tokens = self.line_tokens();
            git_diff::blob::diff(self.algo, &tokens, git_diff::blob::sink::Counter::default())
        }

        /// Return a tokenizer which treats lines as smallest unit.
        ///
        /// The line separator is determined according to normal git rules and filters.
        pub fn line_tokens(&self) -> git_diff::blob::intern::InternedInput<&[u8]> {
            // TODO: make use of `core.eol` and/or filters to do line-counting correctly. It's probably
            //       OK to just know how these objects are saved to know what constitutes a line.
            git_diff::blob::intern::InternedInput::new(self.old.data.as_bytes(), self.new.data.as_bytes())
        }
    }
}
