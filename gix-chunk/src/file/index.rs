use std::ops::Range;

use crate::file::Index;

///
pub mod offset_by_kind {
    use std::fmt::{Display, Formatter};

    /// The error returned by [`Index::offset_by_id()`][super::Index::offset_by_id()].
    #[allow(missing_docs)]
    #[derive(Debug)]
    pub struct Error {
        pub kind: crate::Id,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Chunk named {:?} was not found in chunk file index",
                std::str::from_utf8(&self.kind).unwrap_or("<non-ascii>")
            )
        }
    }

    impl std::error::Error for Error {}
}

///
pub mod data_by_kind {
    /// The error returned by [`Index::data_by_id()`][super::Index::data_by_id()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The chunk wasn't found in the file index")]
        NotFound(#[from] super::offset_by_kind::Error),
        #[error("The offsets into the file couldn't be represented by usize")]
        FileTooLarge,
    }
}

/// An entry of a chunk file index
pub struct Entry {
    /// The kind of the chunk file
    pub kind: crate::Id,
    /// The offset, relative to the beginning of the file, at which to find the chunk and its end.
    pub offset: Range<crate::file::Offset>,
}

impl Index {
    /// The size of a single index entry in bytes
    pub const ENTRY_SIZE: usize = std::mem::size_of::<u32>() + std::mem::size_of::<u64>();
    /// The smallest possible size of an index, consisting only of the sentinel value pointing past itself.
    pub const EMPTY_SIZE: usize = Index::ENTRY_SIZE;

    /// Returns the size in bytes an index with `num_entries` would take.
    pub const fn size_for_entries(num_entries: usize) -> usize {
        Self::ENTRY_SIZE * (num_entries + 1/*sentinel*/)
    }

    /// Find a chunk of `kind` and return its offset into the data if found
    pub fn offset_by_id(&self, kind: crate::Id) -> Result<Range<crate::file::Offset>, offset_by_kind::Error> {
        self.chunks
            .iter()
            .find_map(|c| (c.kind == kind).then(|| c.offset.clone()))
            .ok_or(offset_by_kind::Error { kind })
    }

    /// Find a chunk of `kind` and return its offset as usize range into the data if found.
    ///
    ///
    /// # Panics
    ///
    /// - if the usize conversion fails, which isn't expected as memory maps can't be created if files are too large
    ///   to require such offsets.
    pub fn usize_offset_by_id(&self, kind: crate::Id) -> Result<Range<usize>, offset_by_kind::Error> {
        self.chunks
            .iter()
            .find_map(|c| (c.kind == kind).then(|| crate::range::into_usize_or_panic(c.offset.clone())))
            .ok_or(offset_by_kind::Error { kind })
    }

    /// Like [`Index::usize_offset_by_id()`] but with support for validation and transformation using a function.
    pub fn validated_usize_offset_by_id<T>(
        &self,
        kind: crate::Id,
        validate: impl FnOnce(Range<usize>) -> T,
    ) -> Result<T, offset_by_kind::Error> {
        self.chunks
            .iter()
            .find_map(|c| (c.kind == kind).then(|| crate::range::into_usize_or_panic(c.offset.clone())))
            .map(validate)
            .ok_or(offset_by_kind::Error { kind })
    }

    /// Find a chunk of `kind` and return its data slice based on its offset.
    pub fn data_by_id<'a>(&self, data: &'a [u8], kind: crate::Id) -> Result<&'a [u8], data_by_kind::Error> {
        let offset = self.offset_by_id(kind)?;
        Ok(&data[crate::range::into_usize(offset).ok_or(data_by_kind::Error::FileTooLarge)?])
    }

    /// Return the end offset lf the last chunk, which is the highest offset as well.
    /// It's definitely available as we have one or more chunks.
    pub fn highest_offset(&self) -> crate::file::Offset {
        self.chunks.last().expect("at least one chunk").offset.end
    }
}
