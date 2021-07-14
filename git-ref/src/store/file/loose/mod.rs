///
pub mod find_one;

///
pub mod reflog;

///
pub mod iter {
    #![allow(dead_code, unused_variables, missing_docs)]
    use crate::store::file;
    use bstr::ByteSlice;
    use git_features::fs::walkdir::DirEntryIter;
    use os_str_bytes::OsStrBytes;
    use std::{
        io::Read,
        path::{Path, PathBuf},
    };

    /// An iterator over all valid loose reference paths as seen from a particular base directory.
    struct LoosePaths {
        base: PathBuf,
        file_walk: DirEntryIter,
    }

    impl LoosePaths {
        pub fn at_root(path: impl AsRef<Path>, base: impl Into<PathBuf>) -> Self {
            let file_walk = git_features::fs::WalkDir::new(path).into_iter();
            LoosePaths {
                base: base.into(),
                file_walk,
            }
        }
    }

    impl Iterator for LoosePaths {
        type Item = std::io::Result<PathBuf>;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(entry) = self.file_walk.next() {
                match entry {
                    Ok(entry) => {
                        if !entry.file_type().is_file() {
                            continue;
                        }
                        let full_path = entry.path().to_owned();
                        if git_validate::reference::name_partial(
                            full_path
                                .strip_prefix(&self.base)
                                .expect("prefix-stripping cannot fail as prefix is our root")
                                .to_raw_bytes()
                                .as_bstr(),
                        )
                        .is_ok()
                        {
                            return Some(Ok(full_path));
                        } else {
                            continue;
                        }
                    }
                    Err(err) => return Some(Err(err.into_io_error().expect("no symlink related errors"))),
                }
            }
            None
        }
    }

    /// An iterator over all loose references as seen from a particular base directory.
    pub struct Loose<'a> {
        parent: &'a file::Store,
        ref_paths: LoosePaths,
        buf: Vec<u8>,
    }

    impl<'a> Loose<'a> {
        pub fn at_root(store: &'a file::Store, root: impl AsRef<Path>, base: impl Into<PathBuf>) -> Self {
            Loose {
                parent: store,
                ref_paths: LoosePaths::at_root(root, base),
                buf: Vec::new(),
            }
        }
    }

    impl<'a> Iterator for Loose<'a> {
        type Item = Result<file::Reference<'a>, loose::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            self.ref_paths.next().map(|res| {
                res.map_err(loose::Error::Traversal).and_then(|validated_path| {
                    std::fs::File::open(&validated_path)
                        .and_then(|mut f| {
                            self.buf.clear();
                            f.read_to_end(&mut self.buf)
                        })
                        .map_err(loose::Error::ReadFileContents)
                        .and_then(|_| {
                            let relative_path = validated_path
                                .strip_prefix(&self.ref_paths.base)
                                .expect("root contains path");
                            file::Reference::try_from_path(self.parent, relative_path, &self.buf).map_err(|err| {
                                loose::Error::ReferenceCreation {
                                    err,
                                    relative_path: relative_path.into(),
                                }
                            })
                        })
                })
            })
        }
    }

    impl file::Store {
        /// Return an iterator over all loose references, notably not including any packed ones, in file system order.
        ///
        /// See [`Store::packed()`][file::Store::packed()] for interacting with packed references.
        pub fn loose_iter(&self) -> std::io::Result<Loose<'_>> {
            let refs = self.refs_dir();
            if !refs.is_dir() {
                return Err(std::io::ErrorKind::NotFound.into());
            }
            Ok(Loose::at_root(self, refs, self.base.clone()))
        }

        fn refs_dir(&self) -> PathBuf {
            self.base.join("refs")
        }
    }

    pub mod loose {
        mod error {
            use crate::file;
            use quick_error::quick_error;
            use std::{io, path::PathBuf};

            quick_error! {
                /// The error returned by [file::iter::Loose] iteration.
                #[derive(Debug)]
                #[allow(missing_docs)]
                pub enum Error {
                    Traversal(err: io::Error) {
                        display("The file system could not be traversed")
                        source(err)
                    }
                    ReadFileContents(err: io::Error) {
                        display("The ref file could not be read in full")
                        source(err)
                    }
                    ReferenceCreation{ err: file::reference::decode::Error, relative_path: PathBuf } {
                        display("The reference at '{}' could not be instantiated", relative_path.display())
                        source(err)
                    }
                }
            }
        }
        pub use error::Error;
    }
}

mod init {
    use crate::store::file;
    use std::path::PathBuf;

    impl file::Store {
        /// Create a new instance at the given `git_dir`, which commonly is a standard git repository with a
        /// `refs/` subdirectory.
        pub fn at(git_dir: impl Into<PathBuf>, write_reflog: crate::file::WriteReflog) -> Self {
            file::Store {
                base: git_dir.into(),
                write_reflog,
            }
        }
    }

    impl<P> From<P> for file::Store
    where
        P: Into<PathBuf>,
    {
        fn from(path: P) -> Self {
            file::Store::at(path, Default::default())
        }
    }
}
