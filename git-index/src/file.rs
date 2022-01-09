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

            let (version, num_entries, data) = decode::header(&data)?;

            Ok(File {
                state: State { timestamp: mtime },
                path,
            })
        }
    }
}

pub mod decode {
    use crate::Version;

    pub mod header {
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

    pub(crate) fn header(data: &[u8]) -> Result<(crate::Version, u32, &[u8]), header::Error> {
        if data.len() < 3 * 4 {
            return Err(header::Error::Corrupt("The header is truncated"));
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
