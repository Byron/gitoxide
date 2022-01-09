pub mod init {
    #![allow(unused)]

    use crate::file::decode;
    use crate::{File, State};
    use memmap2::Mmap;
    use std::path::{Path, PathBuf};

    mod error {
        use crate::file::decode;
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Io(err: std::io::Error) {
                    display("An IO error occurred while opening the index")
                    source(err)
                    from()
                }
                DecodeHeader(err: decode::header::Error) {
                    display("The header could not be understood")
                    source(err)
                    from()
                }
            }
        }
    }
    pub use error::Error;

    impl File {
        pub fn at(path: impl Into<PathBuf>, object_hash: git_hash::Kind) -> Result<Self, Error> {
            let path = path.into();
            let (data, mtime) = {
                // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
                let file = std::fs::File::open(&path)?;
                #[allow(unsafe_code)]
                let data = unsafe { Mmap::map(&file)? };
                (data, filetime::FileTime::from_last_modification_time(&file.metadata()?))
            };

            let (version, num_entries, post_header_data) = decode::header(&data, object_hash)?;
            let start_of_extensions = decode::extension::end_of_index_entry(&data, object_hash);

            Ok(File {
                state: State { timestamp: mtime },
                path,
            })
        }
    }
}

pub mod decode {
    use crate::Version;

    fn extension(data: &[u8]) -> ([u8; 4], u32, &[u8]) {
        let (signature, data) = data.split_at(4);
        let (size, data) = data.split_at(4);
        (signature.try_into().unwrap(), read_u32(size), data)
    }

    pub(crate) mod extension {
        use crate::extension::EndOfIndexEntry;
        use crate::file::decode;
        use crate::file::decode::read_u32;

        pub fn end_of_index_entry(data: &[u8], object_hash: git_hash::Kind) -> Option<EndOfIndexEntry> {
            let hash_len = object_hash.len_in_bytes();
            if data.len() < EndOfIndexEntry::SIZE_WITH_HEADER + hash_len {
                return None;
            }

            let start_of_eoie = data.len() - EndOfIndexEntry::SIZE_WITH_HEADER - hash_len;
            let data = &data[start_of_eoie..][..hash_len];

            let (signature, ext_size, data) = decode::extension(data);
            if &signature != EndOfIndexEntry::SIGNATURE || ext_size as usize != EndOfIndexEntry::SIZE {
                return None;
            }

            let (offset, hash) = data.split_at(4);
            let offset = read_u32(offset) as usize;
            if offset < decode::header::SIZE {
                return None;
            }
            todo!("eoie")
        }
    }

    pub mod header {
        pub(crate) const SIZE: usize = 4 /*signature*/ + 4 /*version*/ + 4 /* num entries */;

        mod error {
            use quick_error::quick_error;

            quick_error! {
                #[derive(Debug)]
                pub enum Error {
                    Corrupt(message: &'static str) {
                        display("{}", message)
                    }
                    UnsupportedVersion(version: u32) {
                        display("Index version {} is not supported", version)
                    }
                }
            }
        }
        pub use error::Error;
    }

    pub(crate) fn header(
        data: &[u8],
        object_hash: git_hash::Kind,
    ) -> Result<(crate::Version, u32, &[u8]), header::Error> {
        if data.len() < (3 * 4) + object_hash.len_in_bytes() {
            return Err(header::Error::Corrupt(
                "File is too small even for header with zero entries and smallest hash",
            ));
        }

        const SIGNATURE: &[u8] = b"DIRC";
        let (signature, data) = data.split_at(4);
        if signature != SIGNATURE {
            return Err(header::Error::Corrupt(
                "Signature mismatch - this doesn't claim to be a header file",
            ));
        }

        let (version, data) = data.split_at(4);
        let version = match read_u32(version) {
            2 => Version::V2,
            3 => Version::V3,
            4 => Version::V4,
            unknown => return Err(header::Error::UnsupportedVersion(unknown)),
        };
        let (entries, data) = data.split_at(4);
        let entries = read_u32(entries);

        Ok((version, entries, data))
    }

    #[inline]
    fn read_u32(b: &[u8]) -> u32 {
        u32::from_be_bytes(b.try_into().unwrap())
    }
}
