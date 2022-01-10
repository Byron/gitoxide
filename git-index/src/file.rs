mod impls {
    use crate::{File, State};
    use std::ops::{Deref, DerefMut};

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

    use crate::{extension, File, State};

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
        pub fn at(path: impl Into<PathBuf>, object_hash: git_hash::Kind) -> Result<Self, Error> {
            let path = path.into();
            let (data, mtime) = {
                // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
                let file = std::fs::File::open(&path)?;
                #[allow(unsafe_code)]
                let data = unsafe { Mmap::map(&file)? };
                (data, filetime::FileTime::from_last_modification_time(&file.metadata()?))
            };

            Ok(File {
                state: State::from_bytes(&data, mtime, object_hash)?,
                path,
            })
        }
    }
}
