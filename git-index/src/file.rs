pub mod init {
    #![allow(unused)]

    use std::path::{Path, PathBuf};

    use memmap2::Mmap;

    use crate::{extension, file::header, File, State};

    mod error {
        use quick_error::quick_error;

        use crate::file::header;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Io(err: std::io::Error) {
                    display("An IO error occurred while opening the index")
                    source(err)
                    from()
                }
                DecodeHeader(err: header::decode::Error) {
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

            let (version, num_entries, post_header_data) = header::decode(&data, object_hash)?;
            let start_of_extensions = extension::end_of_index_entry::decode(&data, object_hash);

            Ok(File {
                state: State { timestamp: mtime },
                path,
            })
        }
    }
}

pub mod header {
    pub(crate) const SIZE: usize = 4 /*signature*/ + 4 /*version*/ + 4 /* num entries */;

    pub mod decode {
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
    use crate::{util::read_u32, Version};

    pub(crate) fn decode(
        data: &[u8],
        object_hash: git_hash::Kind,
    ) -> Result<(crate::Version, u32, &[u8]), decode::Error> {
        if data.len() < (3 * 4) + object_hash.len_in_bytes() {
            return Err(decode::Error::Corrupt(
                "File is too small even for header with zero entries and smallest hash",
            ));
        }

        const SIGNATURE: &[u8] = b"DIRC";
        let (signature, data) = data.split_at(4);
        if signature != SIGNATURE {
            return Err(decode::Error::Corrupt(
                "Signature mismatch - this doesn't claim to be a header file",
            ));
        }

        let (version, data) = data.split_at(4);
        let version = match read_u32(version) {
            2 => Version::V2,
            3 => Version::V3,
            4 => Version::V4,
            unknown => return Err(decode::Error::UnsupportedVersion(unknown)),
        };
        let (entries, data) = data.split_at(4);
        let entries = read_u32(entries);

        Ok((version, entries, data))
    }
}
