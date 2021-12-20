use crate::file::Index;
use std::ops::Range;

///
pub mod offset_by_kind {
    use std::fmt::{Display, Formatter};

    /// The error returned by [Index::offset_by_kind()][super::Index::offset_by_kind()].
    #[allow(missing_docs)]
    #[derive(Debug)]
    pub struct Error {
        pub kind: crate::Kind,
        pub name: &'static str,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Chunk named {:?} (id = {}) was not found in chunk file index",
                self.name, self.kind
            )
        }
    }

    impl std::error::Error for Error {}
}

///
pub mod data_by_kind {
    use quick_error::quick_error;
    quick_error! {
        /// The error returned by [Index::data_by_kind()][super::Index::data_by_kind()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            NotFound(err: super::offset_by_kind::Error) {
                display("The chunk wasn't found in the file index")
                from()
                source(err)
            }
            FileTooLarge {
                display("The offsets into the file couldn't be represented by usize")
            }
        }
    }
}

/// An entry of a chunk file index
pub struct Entry {
    /// The kind of the chunk file
    pub kind: crate::Kind,
    /// The offset, relative to the beginning of the file, at which to find the chunk and its end.
    pub offset: Range<crate::file::Offset>,
}

impl Index {
    /// The size of a single index entry in bytes
    pub const ENTRY_SIZE: usize = std::mem::size_of::<u32>() + std::mem::size_of::<u64>();
    /// The smallest possible size of an index, consisting only of the sentinel value pointing past itself.
    pub const EMPTY_SIZE: usize = Index::ENTRY_SIZE;

    /// Find a chunk of `kind` and return its offset into the data if found
    pub fn offset_by_kind(
        &self,
        kind: crate::Kind,
        name: &'static str,
    ) -> Result<Range<crate::file::Offset>, offset_by_kind::Error> {
        self.chunks
            .iter()
            .find_map(|c| (c.kind == kind).then(|| c.offset.clone()))
            .ok_or(offset_by_kind::Error { kind, name })
    }

    /// Find a chunk of `kind` and return its data slice based on its offset.
    pub fn data_by_kind<'a>(
        &self,
        data: &'a [u8],
        kind: crate::Kind,
        name: &'static str,
    ) -> Result<&'a [u8], data_by_kind::Error> {
        let offset = self.offset_by_kind(kind, name)?;
        Ok(&data[crate::into_usize_range(offset).ok_or(data_by_kind::Error::FileTooLarge)?])
    }
}
