pub mod init {
    #![allow(unused)]
    use crate::{File, State};
    use memmap2::Mmap;
    use std::path::{Path, PathBuf};

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
                state: State { timestamp: mtime },
                path,
            })
        }
    }
}

pub mod decode {
    pub mod header {
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
                }
            }
        }
        pub use error::Error;
    }

    fn header(data: &[u8]) -> Result<(crate::Version, &[u8]), header::Error> {
        todo!("header parsing")
    }
}
