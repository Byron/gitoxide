use crate::data::input;
use git_features::hash;
use std::io::Write;

/// An implementation of [`Iterator`] to write [encoded entries][output::Entry] to an inner implementation each time
/// `next()` is called.
pub struct FromEntriesIter<I, W> {
    /// An iterator for input [`output::Entry`] instances
    pub input: I,
    /// A way of writing encoded bytes.
    output: hash::Write<W>,
    /// Our trailing hash when done writing all input entries
    trailer: Option<git_hash::ObjectId>,
    /// The amount of objects in the iteration and the version of the packfile to be written.
    /// Will be `None` to signal the header was written already.
    header_info: Option<(crate::data::Version, u32)>,
    /// If we are done, no additional writes will occour
    is_done: bool,
}

impl<I, W> FromEntriesIter<I, W>
where
    I: Iterator<Item = Result<input::Entry, input::Error>>,
    W: std::io::Write,
{
    /// Create a new instance reading [entries][input::Entry] from an `input` iterator and write pack data bytes to
    /// `output` writer, resembling a pack of `version` with exactly `num_entries` amount of objects contained in it.
    /// `hash_kind` is the kind of hash to use for the pack checksum and maybe other places, depending on the version.
    ///
    /// # Panics
    ///
    /// Not all combinations of `hash_kind` and `version` are supported currently triggering assertion errors.
    pub fn new(
        input: I,
        output: W,
        num_entries: u32,
        version: crate::data::Version,
        hash_kind: git_hash::Kind,
    ) -> Self {
        assert!(
            matches!(version, crate::data::Version::V2),
            "currently only pack version 2 can be written",
        );
        assert!(
            matches!(hash_kind, git_hash::Kind::Sha1),
            "currently only Sha1 is supported, right now we don't know how other hashes are encoded",
        );
        FromEntriesIter {
            input,
            output: hash::Write::new(output, hash_kind),
            trailer: None,
            header_info: Some((version, num_entries)),
            is_done: false,
        }
    }

    /// Consume this instance and return the `output` implementation.
    ///
    /// _Note_ that the `input` iterator can be moved out of this instance beforehand.
    pub fn into_write(self) -> W {
        self.output.inner
    }

    /// Returns the trailing hash over all ~ entries once done.
    /// It's `None` if we are not yet done writing.
    pub fn digest(&self) -> Option<git_hash::ObjectId> {
        self.trailer
    }

    fn next_inner(&mut self, entry: input::Entry) -> Result<input::Entry, input::Error> {
        if let Some((version, num_entries)) = self.header_info.take() {
            let header_bytes = crate::data::header::encode(version, num_entries);
            self.output.write_all(&header_bytes[..])?;
        }
        entry
            .header
            .write_to(entry.decompressed_size as u64, &mut self.output)?;
        std::io::copy(
            &mut &*entry
                .compressed
                .as_deref()
                .expect("caller must configure generator to keep compressed bytes"),
            &mut self.output,
        )?;
        Ok(entry)
    }

    fn write_digest(&mut self) -> Result<(), input::Error> {
        let digest = self.output.hash.clone().digest();
        self.output.inner.write_all(&digest[..])?;
        self.output.inner.flush()?;
        self.is_done = true;
        self.trailer = Some(git_hash::ObjectId::from(digest));
        Ok(())
    }
}

impl<I, W> Iterator for FromEntriesIter<I, W>
where
    I: Iterator<Item = Result<input::Entry, input::Error>>,
    W: std::io::Write,
{
    /// The amount of bytes written to `out` if `Ok` or the error `E` received from the input.
    type Item = Result<input::Entry, input::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        match self.input.next() {
            Some(Ok(entry)) => Some(self.next_inner(entry)),
            Some(Err(err)) => {
                self.is_done = true;
                Some(Err(err))
            }
            None => self.write_digest().err().map(Err),
        }
    }
}
