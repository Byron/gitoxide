use crate::store_impl::packed;

impl AsRef<[u8]> for packed::Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.data.as_ref()[self.offset..]
    }
}

impl AsRef<[u8]> for packed::Backing {
    fn as_ref(&self) -> &[u8] {
        match self {
            packed::Backing::InMemory(data) => data,
            packed::Backing::Mapped(map) => map,
        }
    }
}

///
pub mod open {
    use std::path::PathBuf;

    use memmap2::Mmap;
    use winnow::prelude::*;
    use winnow::stream::Offset;

    use crate::store_impl::packed;

    /// Initialization
    impl packed::Buffer {
        /// Open the file at `path` and map it into memory if the file size is larger than `use_memory_map_if_larger_than_bytes`.
        ///
        /// In order to allow fast lookups and optimizations, the contents of the packed refs must be sorted.
        /// If that's not the case, they will be sorted on the fly with the data being written into a memory buffer.
        pub fn open(path: impl Into<PathBuf>, use_memory_map_if_larger_than_bytes: u64) -> Result<Self, Error> {
            let path = path.into();
            let (backing, offset) = {
                let backing = if std::fs::metadata(&path)?.len() <= use_memory_map_if_larger_than_bytes {
                    packed::Backing::InMemory(std::fs::read(&path)?)
                } else {
                    packed::Backing::Mapped(
                        // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
                        #[allow(unsafe_code)]
                        unsafe {
                            Mmap::map(&std::fs::File::open(&path)?)?
                        },
                    )
                };

                let (offset, sorted) = {
                    let mut input = backing.as_ref();
                    if *input.first().unwrap_or(&b' ') == b'#' {
                        let header = packed::decode::header::<()>
                            .parse_next(&mut input)
                            .map_err(|_| Error::HeaderParsing)?;
                        let offset = input.offset_from(&backing.as_ref());
                        (offset, header.sorted)
                    } else {
                        (0, false)
                    }
                };

                if !sorted {
                    // this implementation is likely slower than what git does, but it's less code, too.
                    let mut entries = packed::Iter::new(&backing.as_ref()[offset..])?.collect::<Result<Vec<_>, _>>()?;
                    entries.sort_by_key(|e| e.name.as_bstr());
                    let mut serialized = Vec::<u8>::new();
                    for entry in entries {
                        serialized.extend_from_slice(entry.target);
                        serialized.push(b' ');
                        serialized.extend_from_slice(entry.name.as_bstr());
                        serialized.push(b'\n');
                        if let Some(object) = entry.object {
                            serialized.push(b'^');
                            serialized.extend_from_slice(object);
                            serialized.push(b'\n');
                        }
                    }
                    (Backing::InMemory(serialized), 0)
                } else {
                    (backing, offset)
                }
            };
            Ok(packed::Buffer {
                offset,
                data: backing,
                path,
            })
        }
    }

    mod error {
        use crate::packed;

        /// The error returned by [`open()`][super::packed::Buffer::open()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("The packed-refs file did not have a header or wasn't sorted and could not be iterated")]
            Iter(#[from] packed::iter::Error),
            #[error("The header could not be parsed, even though first line started with '#'")]
            HeaderParsing,
            #[error("The buffer could not be opened or read")]
            Io(#[from] std::io::Error),
        }
    }
    pub use error::Error;

    use crate::packed::Backing;
}
