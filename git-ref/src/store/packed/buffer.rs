use crate::store::packed;

impl AsRef<[u8]> for packed::Buffer {
    fn as_ref(&self) -> &[u8] {
        match &self.data {
            packed::Backing::InMemory(data) => &data[self.offset..],
            packed::Backing::Mapped(map) => &map[self.offset..],
        }
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
    use crate::store::{packed, packed::transaction::HEADER_LINE};
    use filebuffer::FileBuffer;
    use std::path::PathBuf;

    /// Initialization
    impl packed::Buffer {
        /// Open the file at `path` and map it into memory if the file size is larger than `use_memory_map_if_larger_than_bytes`.
        ///
        /// In order to allow fast lookups and optimizations, the contents of the packed refs must be sorted.
        /// If that's not the case, they will be sorted on the fly with the data being written into a memory buffer.
        pub fn open(path: impl Into<PathBuf>, use_memory_map_if_larger_than_bytes: u64) -> Result<Self, Error> {
            let path = path.into();
            let backing = if std::fs::metadata(&path)?.len() <= use_memory_map_if_larger_than_bytes {
                packed::Backing::InMemory(std::fs::read(&path)?)
            } else {
                packed::Backing::Mapped(FileBuffer::open(&path)?)
            };

            let (offset, sorted) = {
                let data = backing.as_ref();
                if *data.get(0).unwrap_or(&b' ') == b'#' {
                    let (records, header) = packed::decode::header::<()>(data).map_err(|_| Error::HeaderParsing)?;
                    let offset = records.as_ptr() as usize - data.as_ptr() as usize;
                    (offset, header.sorted)
                } else {
                    (0, false)
                }
            };
            if !sorted {
                return Err(Error::Unsorted);
            }
            Ok(packed::Buffer {
                offset,
                data: backing,
                path,
            })
        }
    }

    impl packed::Buffer {
        pub(crate) fn empty(path: impl Into<PathBuf>) -> Self {
            packed::Buffer {
                data: packed::Backing::InMemory(HEADER_LINE.into()),
                offset: HEADER_LINE.len(),
                path: path.into(),
            }
        }
    }

    mod error {
        use quick_error::quick_error;

        quick_error! {
            /// The error returned by [`open()`][super::packed::Buffer::open()].
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Unsorted {
                    display("The packed-refs file did not have a header or wasn't sorted.")
                }
                HeaderParsing {
                    display("The header could not be parsed, even though first line started with '#'")
                }
                Io(err: std::io::Error) {
                    display("The buffer could not be opened or read")
                    from()
                    source(err)
                }
            }
        }
    }
    pub use error::Error;
}
