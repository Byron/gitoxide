use crate::store::packed;

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
    use crate::store::packed;
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
            let (backing, offset) = {
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
        use quick_error::quick_error;

        quick_error! {
            /// The error returned by [`open()`][super::packed::Buffer::open()].
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Iter(err: packed::iter::Error) {
                    display("The packed-refs file did not have a header or wasn't sorted and could not be iterated")
                    from()
                    source(err)
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
    use crate::packed::Backing;
    pub use error::Error;
}
