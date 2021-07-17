use crate::store::packed;

impl AsRef<[u8]> for packed::Buffer {
    fn as_ref(&self) -> &[u8] {
        match self {
            packed::Buffer::InMemory { data, offset } => &data[*offset..],
            packed::Buffer::Mapped { map, offset } => &map[*offset..],
        }
    }
}

///
pub mod open {
    use crate::store::packed;
    use filebuffer::FileBuffer;
    use std::path::Path;

    /// Initialization
    impl packed::Buffer {
        /// Open the file at `path` and map it into memory if the file size is larger than `use_memory_map_if_larger_than_bytes`.
        ///
        /// In order to allow fast lookups and optimizations, the contents of the packed refs must be sorted.
        /// If that's not the case, they will be sorted on the fly with the data being written into a memory buffer.
        pub fn open(path: impl AsRef<Path>, use_memory_map_if_larger_than_bytes: u64) -> Result<Self, Error> {
            let path = path.as_ref();
            let buf = if std::fs::metadata(path)?.len() <= use_memory_map_if_larger_than_bytes {
                packed::Buffer::InMemory {
                    data: std::fs::read(path)?,
                    offset: 0,
                }
            } else {
                packed::Buffer::Mapped {
                    map: FileBuffer::open(path)?,
                    offset: 0,
                }
            };

            let (buf, sorted) = {
                let data = buf.as_ref();
                if *data.get(0).unwrap_or(&b' ') == b'#' {
                    let (records, header) = packed::decode::header::<()>(data).map_err(|_| Error::HeaderParsing)?;
                    let offset = records.as_ptr() as usize - data.as_ptr() as usize;
                    (
                        match buf {
                            packed::Buffer::Mapped { map, .. } => packed::Buffer::Mapped { map, offset },
                            packed::Buffer::InMemory { data, .. } => packed::Buffer::InMemory { data, offset },
                        },
                        header.sorted,
                    )
                } else {
                    (buf, false)
                }
            };
            if !sorted {
                return Err(Error::Unsorted);
            }
            Ok(buf)
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
