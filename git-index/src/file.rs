mod impls {
    use std::ops::{Deref, DerefMut};

    use crate::{File, State};

    impl Deref for File {
        type Target = State;

        fn deref(&self) -> &Self::Target {
            &self.state
        }
    }

    impl DerefMut for File {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.state
        }
    }
}

pub mod init {
    #![allow(unused)]

    use std::path::{Path, PathBuf};

    use memmap2::Mmap;

    use crate::{decode, extension, File, State};

    mod error {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Io(err: std::io::Error) {
                    display("An IO error occurred while opening the index")
                    source(err)
                    from()
                }
                Decode(err: crate::decode::Error) {
                    display("The file could not be decoded")
                    source(err)
                    from()
                }
            }
        }
    }
    pub use error::Error;

    impl File {
        pub fn at(path: impl Into<PathBuf>, options: decode::Options) -> Result<Self, Error> {
            let path = path.into();
            let (data, mtime) = {
                // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
                let file = std::fs::File::open(&path)?;
                #[allow(unsafe_code)]
                let data = unsafe { Mmap::map(&file)? };
                (data, filetime::FileTime::from_last_modification_time(&file.metadata()?))
            };

            let (state, checksum) = State::from_bytes(&data, mtime, options)?;
            Ok(File { state, path, checksum })
        }
    }
}

mod verify {
    use crate::File;
    use std::sync::atomic::AtomicBool;

    pub mod error {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Io(err: std::io::Error) {
                    display("Could not read index file to generate hash")
                    source(err)
                    from()
                }
                ChecksumMismatch { actual: git_hash::ObjectId, expected: git_hash::ObjectId }{
                    display("Index checksum should have been {}, but was {}", expected, actual)
                }
            }
        }
    }
    pub use error::Error;

    impl File {
        pub fn verify_integrity(&self) -> Result<(), Error> {
            let num_bytes_to_hash = self.path.metadata()?.len() - self.checksum.as_bytes().len() as u64;
            let should_interrupt = AtomicBool::new(false);
            let actual = git_features::hash::bytes_of_file(
                &self.path,
                num_bytes_to_hash as usize,
                self.checksum.kind(),
                &mut git_features::progress::Discard,
                &should_interrupt,
            )?;
            (actual == self.checksum).then(|| ()).ok_or(Error::ChecksumMismatch {
                actual,
                expected: self.checksum,
            })
        }
    }
}
