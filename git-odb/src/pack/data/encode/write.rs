use crate::{hash, pack, pack::data::encode};
use std::io::Write;

/// The error returned by `next()` in the [`Entries`] iterator.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error<E>
where
    E: std::error::Error + 'static,
{
    #[error(transparent)]
    Io(std::io::Error),
    #[error(transparent)]
    Input(encode::entries::Error<E>),
}

/// An implementation of [`Iterator`] to write [encoded entries][encode::Entry] to an inner implementation each time
/// `next()` is called.
pub struct Entries<I, W> {
    /// An iterator for input [`encode::Entry`] instances
    pub input: I,
    /// A way of writing encoded bytes.
    output: hash::Write<W>,
    /// The amount of objects in the iteration and the version of the packfile to be written.
    /// Will be `None` to signal the header was written already.
    header_info: Option<(pack::data::Version, u32)>,
    /// If we are done, no additional writes will occour
    is_done: bool,
}

impl<I, W, E> Entries<I, W>
where
    I: Iterator<Item = Result<Vec<encode::Entry>, encode::entries::Error<E>>>,
    W: std::io::Write,
    E: std::error::Error + 'static,
{
    /// Create a new instance reading [entries][encode::Entry] from an `input` iterator and write pack data bytes to
    /// `output` writer, resembling a pack of `version` with exactly `num_entries` amount of objects contained in it.
    /// `hash_kind` is the kind of hash to use for the pack checksum and maybe other places, depending on the version.
    ///
    /// # Panics
    ///
    /// Not all combinations of `hash_kind` and `version` are supported currently triggering assertion errors.
    pub fn new(input: I, output: W, num_entries: u32, version: pack::data::Version, hash_kind: git_hash::Kind) -> Self {
        assert!(
            matches!(version, pack::data::Version::V2),
            "currently only pack version 2 can be written",
        );
        assert!(
            matches!(hash_kind, git_hash::Kind::Sha1),
            "currently only Sha1 is supported",
        );
        Entries {
            input,
            output: hash::Write::new(output, hash_kind),
            header_info: Some((version, num_entries)),
            is_done: false,
        }
    }

    /// Consume this instance and return the `output` implementation.
    ///
    /// _Note_ that the input can be moved out of this instance beforehand.
    pub fn into_write(self) -> W {
        self.output.inner
    }
}

impl<I, W, E> Iterator for Entries<I, W>
where
    I: Iterator<Item = Result<Vec<encode::Entry>, encode::entries::Error<E>>>,
    W: std::io::Write,
    E: std::error::Error + 'static,
{
    /// The amount of bytes written to `out` if `Ok` or the error `E` received from the input.
    type Item = Result<u64, Error<encode::entries::Error<E>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        let mut written = 0;
        if let Some((version, num_entries)) = self.header_info.take() {
            let header_bytes = pack::data::header::encode(version, num_entries);
            if let Err(err) = self.output.write_all(&header_bytes[..]) {
                self.is_done = true;
                return Some(Err(Error::Io(err)));
            }
            written += header_bytes.len();
        }
        match self.input.next() {
            Some(_entries) => todo!("serialize entries"),
            None => todo!("write footer and set is_done = false"),
        }
    }
}
