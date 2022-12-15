use std::iter::Peekable;

use git_features::hash;

use crate::data::input;

/// An implementation of [`Iterator`] to write [encoded entries][input::Entry] to an inner implementation each time
/// `next()` is called.
///
/// It is able to deal with an unknown amount of objects as it will rewrite the pack header once the entries iterator
/// is depleted and compute the hash in one go by re-reading the whole file.
pub struct EntriesToBytesIter<I: Iterator, W> {
    /// An iterator for input [`input::Entry`] instances
    pub input: Peekable<I>,
    /// A way of writing encoded bytes.
    output: W,
    /// Our trailing hash when done writing all input entries
    trailer: Option<git_hash::ObjectId>,
    /// The amount of objects in the iteration and the version of the packfile to be written.
    /// Will be `None` to signal the header was written already.
    data_version: crate::data::Version,
    /// The amount of entries seen so far
    num_entries: u32,
    /// If we are done, no additional writes will occur
    is_done: bool,
    /// The kind of hash to use for the digest
    object_hash: git_hash::Kind,
}

impl<I, W> EntriesToBytesIter<I, W>
where
    I: Iterator<Item = Result<input::Entry, input::Error>>,
    W: std::io::Read + std::io::Write + std::io::Seek,
{
    /// Create a new instance reading [entries][input::Entry] from an `input` iterator and write pack data bytes to
    /// `output` writer, resembling a pack of `version`. The amount of entries will be dynaimcally determined and
    /// the pack is completed once the last entry was written.
    /// `object_hash` is the kind of hash to use for the pack checksum and maybe other places, depending on the version.
    ///
    /// # Panics
    ///
    /// Not all combinations of `object_hash` and `version` are supported currently triggering assertion errors.
    pub fn new(input: I, output: W, version: crate::data::Version, object_hash: git_hash::Kind) -> Self {
        assert!(
            matches!(version, crate::data::Version::V2),
            "currently only pack version 2 can be written",
        );
        assert!(
            matches!(object_hash, git_hash::Kind::Sha1),
            "currently only Sha1 is supported, right now we don't know how other hashes are encoded",
        );
        EntriesToBytesIter {
            input: input.peekable(),
            output,
            object_hash,
            num_entries: 0,
            trailer: None,
            data_version: version,
            is_done: false,
        }
    }

    /// Returns the trailing hash over all ~ entries once done.
    /// It's `None` if we are not yet done writing.
    pub fn digest(&self) -> Option<git_hash::ObjectId> {
        self.trailer
    }

    fn next_inner(&mut self, entry: input::Entry) -> Result<input::Entry, input::Error> {
        if self.num_entries == 0 {
            let header_bytes = crate::data::header::encode(self.data_version, 0);
            self.output.write_all(&header_bytes[..])?;
        }
        self.num_entries += 1;
        entry.header.write_to(entry.decompressed_size, &mut self.output)?;
        std::io::copy(
            &mut entry
                .compressed
                .as_deref()
                .expect("caller must configure generator to keep compressed bytes"),
            &mut self.output,
        )?;
        Ok(entry)
    }

    fn write_header_and_digest(&mut self, last_entry: Option<&mut input::Entry>) -> Result<(), input::Error> {
        let header_bytes = crate::data::header::encode(self.data_version, self.num_entries);
        let num_bytes_written = if last_entry.is_some() {
            self.output.stream_position()?
        } else {
            header_bytes.len() as u64
        };
        self.output.seek(std::io::SeekFrom::Start(0))?;
        self.output.write_all(&header_bytes[..])?;
        self.output.flush()?;

        self.output.seek(std::io::SeekFrom::Start(0))?;
        let interrupt_never = std::sync::atomic::AtomicBool::new(false);
        let digest = hash::bytes(
            &mut self.output,
            num_bytes_written as usize,
            self.object_hash,
            &mut git_features::progress::Discard,
            &interrupt_never,
        )?;
        self.output.write_all(digest.as_slice())?;
        self.output.flush()?;

        self.is_done = true;
        if let Some(last_entry) = last_entry {
            last_entry.trailer = Some(digest);
        }
        self.trailer = Some(digest);
        Ok(())
    }
}

impl<I, W> Iterator for EntriesToBytesIter<I, W>
where
    I: Iterator<Item = Result<input::Entry, input::Error>>,
    W: std::io::Read + std::io::Write + std::io::Seek,
{
    /// The amount of bytes written to `out` if `Ok` or the error `E` received from the input.
    type Item = Result<input::Entry, input::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        match self.input.next() {
            Some(res) => Some(match res {
                Ok(entry) => self.next_inner(entry).and_then(|mut entry| {
                    if self.input.peek().is_none() {
                        self.write_header_and_digest(Some(&mut entry)).map(|_| entry)
                    } else {
                        Ok(entry)
                    }
                }),
                Err(err) => {
                    self.is_done = true;
                    Err(err)
                }
            }),
            None => match self.write_header_and_digest(None) {
                Ok(_) => None,
                Err(err) => Some(Err(err)),
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}
