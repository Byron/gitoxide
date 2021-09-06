use std::{
    cmp::Ordering,
    io::Read,
    iter::Peekable,
    path::{Path, PathBuf},
};

use crate::{
    file::{loose, path_to_name},
    store::{file, packed},
    FullName, Reference,
};

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references.
///
/// All errors will be returned verbatim, while packed errors are depleted first if loose refs also error.
pub struct LooseThenPacked<'p, 's> {
    base: &'s Path,
    packed: Option<Peekable<packed::Iter<'p>>>,
    loose: Peekable<loose::iter::SortedLoosePaths>,
    buf: Vec<u8>,
}

impl<'p, 's> LooseThenPacked<'p, 's> {
    fn convert_packed(
        &mut self,
        packed: Result<packed::Reference<'p>, packed::iter::Error>,
    ) -> Result<Reference, Error> {
        packed.map(Into::into).map_err(|err| match err {
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

    fn convert_loose(&mut self, res: std::io::Result<(PathBuf, FullName)>) -> Result<Reference, Error> {
        let (refpath, name) = res.map_err(Error::Traversal)?;
        std::fs::File::open(&refpath)
            .and_then(|mut f| {
                self.buf.clear();
                f.read_to_end(&mut self.buf)
            })
            .map_err(Error::ReadFileContents)?;
        loose::Reference::try_from_path(name, &self.buf)
            .map_err(|err| Error::ReferenceCreation {
                err,
                relative_path: refpath.strip_prefix(&self.base).expect("base contains path").into(),
            })
            .map(Into::into)
    }
}

impl<'p, 's> Iterator for LooseThenPacked<'p, 's> {
    type Item = Result<Reference, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.packed.as_mut() {
            Some(packed_iter) => match (self.loose.peek(), packed_iter.peek()) {
                (None, None) => None,
                (None, Some(_)) | (Some(_), Some(Err(_))) => {
                    let res = packed_iter.next().expect("peeked value exists");
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
                            drop(packed_iter.next());
                            let res = self.loose.next().expect("peeked value exists");
                            Some(self.convert_loose(res))
                        }
                        Ordering::Greater => {
                            let res = packed_iter.next().expect("name retrieval configured");
                            Some(self.convert_packed(res))
                        }
                    }
                }
            },
            None => self.loose.next().map(|res| self.convert_loose(res)),
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
    pub fn iter<'p, 's>(&'s self, packed: Option<&'p packed::Buffer>) -> std::io::Result<LooseThenPacked<'p, 's>> {
        match &self.namespace {
            Some(namespace) => self.iter_prefixed_unvalidated(packed, namespace.to_path()),
            None => Ok(LooseThenPacked {
                base: &self.base,
                packed: match packed {
                    Some(packed) => Some(
                        packed
                            .iter()
                            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                            .peekable(),
                    ),
                    None => None,
                },
                loose: loose::iter::SortedLoosePaths::at_root_with_names(self.refs_dir(), self.base.to_owned())
                    .peekable(),
                buf: Vec::new(),
            }),
        }
    }

    /// As [`iter(…)`][file::Store::iter()], but filters by `prefix`, i.e. "refs/heads".
    ///
    /// Please note that "refs/heads` or "refs\\heads" is equivalent to "refs/heads/"
    pub fn iter_prefixed<'p, 's>(
        &'s self,
        packed: Option<&'p packed::Buffer>,
        prefix: impl AsRef<Path>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        self.validate_prefix(prefix.as_ref())?;
        match &self.namespace {
            None => self.iter_prefixed_unvalidated(packed, prefix),
            Some(namespace) => {
                self.iter_prefixed_unvalidated(packed, namespace.to_owned().into_namespaced_prefix(prefix))
            }
        }
    }

    fn iter_prefixed_unvalidated<'p, 's>(
        &'s self,
        packed: Option<&'p packed::Buffer>,
        prefix: impl AsRef<Path>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        let packed_prefix = path_to_name(prefix.as_ref());
        Ok(LooseThenPacked {
            base: &self.base,
            packed: match packed {
                Some(packed) => Some(
                    packed
                        .iter_prefixed(packed_prefix)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                        .peekable(),
                ),
                None => None,
            },
            loose: loose::iter::SortedLoosePaths::at_root_with_names(self.base.join(prefix), self.base.to_owned())
                .peekable(),
            buf: Vec::new(),
        })
    }
}

mod error {
    use std::{io, path::PathBuf};

    use git_object::bstr::BString;
    use quick_error::quick_error;

    use crate::store::file;

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

pub use error::Error;
