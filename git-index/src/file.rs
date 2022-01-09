pub mod init {
    #![allow(unused)]
    use crate::File;
    use memmap2::Mmap;
    use std::path::Path;

    mod error {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Io(err: std::io::Error) {
                    display("An IO error occurred while reading the index")
                    source(err)
                    from()
                }
            }
        }
    }
    pub use error::Error;

    impl File {
        pub fn at(path: impl AsRef<Path>, object_hash: git_hash::Kind) -> Result<Self, Error> {
            // SAFETY: we have to take the risk of somebody changing the file underneath. Git never writes into the same file.
            #[allow(unsafe_code)]
            let data = unsafe { Mmap::map(&std::fs::File::open(path)?)? };

            todo!("read file")
        }
    }
}
