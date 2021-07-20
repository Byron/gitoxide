use super::{LooseThenPacked, Reference};
use crate::{
    file::path_to_name,
    store::{file, packed},
};
use std::{
    cmp::Ordering,
    io::Read,
    path::{Path, PathBuf},
};

impl<'p, 's> LooseThenPacked<'p, 's> {
    fn convert_packed(
        &mut self,
        packed: Result<packed::Reference<'p>, packed::iter::Error>,
    ) -> Result<Reference<'p, 's>, Error> {
        packed.map(Reference::Packed).map_err(|err| match err {
            packed::iter::Error::Reference {
                invalid_line,
                line_number,
            } => Error::PackedReference {
                invalid_line,
                line_number,
            },
            packed::iter::Error::Header { .. } => unreachable!("this one only happens on iteration creation"),
        })
    }

    fn convert_loose(&mut self, res: std::io::Result<(PathBuf, FullName)>) -> Result<Reference<'p, 's>, Error> {
        let (refpath, name) = res.map_err(Error::Traversal)?;
        std::fs::File::open(&refpath)
            .and_then(|mut f| {
                self.buf.clear();
                f.read_to_end(&mut self.buf)
            })
            .map_err(Error::ReadFileContents)?;
        file::Reference::try_from_path(self.parent, name, &self.buf)
            .map_err(|err| Error::ReferenceCreation {
                err,
                relative_path: refpath
                    .strip_prefix(&self.parent.base)
                    .expect("base contains path")
                    .into(),
            })
            .map(Reference::Loose)
    }
}

impl<'p, 's> Iterator for LooseThenPacked<'p, 's> {
    type Item = Result<Reference<'p, 's>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.loose.peek(), self.packed.peek()) {
            (None, None) => None,
            (None, Some(_)) | (Some(_), Some(Err(_))) => {
                let res = self.packed.next().expect("peeked value exists");
                Some(self.convert_packed(res))
            }
            (Some(_), None) | (Some(Err(_)), Some(_)) => {
                let res = self.loose.next().expect("peeked value exists");
                Some(self.convert_loose(res))
            }
            (Some(Ok(loose)), Some(Ok(packed))) => {
                let loose_name = loose.1.as_bstr();
                match loose_name.cmp(packed.name.as_bstr()) {
                    Ordering::Less => {
                        let res = self.loose.next().expect("name retrieval configured");
                        Some(self.convert_loose(res))
                    }
                    Ordering::Equal => {
                        drop(self.packed.next());
                        let res = self.loose.next().expect("peeked value exists");
                        Some(self.convert_loose(res))
                    }
                    Ordering::Greater => {
                        let res = self.packed.next().expect("name retrieval configured");
                        Some(self.convert_packed(res))
                    }
                }
            }
        }
    }
}

impl file::Store {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Note that the caller is responsible for the freshness of the `packed` references buffer.
    /// If a reference cannot be parsed or read, the error will be visible to the caller and the iteration
    /// continues.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themeselves.
    pub fn iter<'p, 's>(&'s self, packed: &'p packed::Buffer) -> std::io::Result<LooseThenPacked<'p, 's>> {
        Ok(LooseThenPacked {
            parent: self,
            packed: packed
                .iter()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                .peekable(),
            loose: file::loose::iter::SortedLoosePaths::at_root_with_names(self.refs_dir(), self.base.clone())
                .peekable(),
            buf: Vec::new(),
        })
    }

    /// As [`iter(…)`][file::Store::iter()], but filters by `prefix`, i.e. "refs/heads".
    ///
    /// Please note that "refs/heads` or "refs\\heads" is equivalent to "refs/heads/"
    pub fn iter_prefixed<'p, 's>(
        &'s self,
        packed: &'p packed::Buffer,
        prefix: impl AsRef<Path>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        let packed_prefix = path_to_name(self.validate_prefix(prefix.as_ref())?);
        Ok(LooseThenPacked {
            parent: self,
            packed: packed
                .iter_prefixed(packed_prefix)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                .peekable(),
            loose: file::loose::iter::SortedLoosePaths::at_root_with_names(self.base.join(prefix), self.base.clone())
                .peekable(),
            buf: Vec::new(),
        })
    }
}

mod error {
    use crate::store::file;
    use bstr::BString;
    use quick_error::quick_error;
    use std::{io, path::PathBuf};

    quick_error! {
        /// The error returned by the [`LooseThenPacked`][super::LooseThenPacked] iterator.
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
            ReferenceCreation{ err: file::loose::reference::decode::Error, relative_path: PathBuf } {
                display("The reference at '{}' could not be instantiated", relative_path.display())
                source(err)
            }
            PackedReference { invalid_line: BString, line_number: usize } {
                display("Invalid reference in line {}: '{}'", line_number, invalid_line)
            }
        }
    }
}

use crate::mutable::FullName;
pub use error::Error;
