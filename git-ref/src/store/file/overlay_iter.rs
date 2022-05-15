use std::{
    cmp::Ordering,
    io::Read,
    iter::Peekable,
    path::{Path, PathBuf},
};

use crate::{
    file::{loose, path_to_name},
    store_impl::{file, packed},
    BString, FullName, Namespace, Reference,
};

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references.
///
/// All errors will be returned verbatim, while packed errors are depleted first if loose refs also error.
pub struct LooseThenPacked<'p, 's> {
    base: &'s Path,
    namespace: Option<&'s Namespace>,
    packed: Option<Peekable<packed::Iter<'p>>>,
    loose: Peekable<loose::iter::SortedLoosePaths>,
    buf: Vec<u8>,
}

/// An intermediate structure to hold shared state alive long enough for iteration to happen.
#[must_use = "Iterators should be obtained from this platform"]
pub struct Platform<'s> {
    store: &'s file::Store,
    packed: Option<OwnShared<packed::Buffer>>,
}

impl<'p, 's> LooseThenPacked<'p, 's> {
    fn strip_namespace(&self, mut r: Reference) -> Reference {
        if let Some(namespace) = &self.namespace {
            r.strip_namespace(namespace);
        }
        r
    }

    fn convert_packed(
        &mut self,
        packed: Result<packed::Reference<'p>, packed::iter::Error>,
    ) -> Result<Reference, Error> {
        packed
            .map(Into::into)
            .map(|r| self.strip_namespace(r))
            .map_err(|err| match err {
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
            .map_err(|err| Error::ReadFileContents {
                err,
                path: refpath.to_owned(),
            })?;
        loose::Reference::try_from_path(name, &self.buf)
            .map_err(|err| Error::ReferenceCreation {
                err,
                relative_path: refpath.strip_prefix(&self.base).expect("base contains path").into(),
            })
            .map(Into::into)
            .map(|r| self.strip_namespace(r))
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

impl<'s> Platform<'s> {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themeselves.
    pub fn all(&self) -> std::io::Result<LooseThenPacked<'_, '_>> {
        self.store.iter_packed(self.packed.as_deref())
    }

    /// As [`iter(…)`][file::Store::iter()], but filters by `prefix`, i.e. "refs/heads".
    ///
    /// Please note that "refs/heads` or "refs\\heads" is equivalent to "refs/heads/"
    pub fn prefixed(&self, prefix: impl AsRef<Path>) -> std::io::Result<LooseThenPacked<'_, '_>> {
        self.store.iter_prefixed_packed(prefix, self.packed.as_deref())
    }
}

impl file::Store {
    /// Return a platform to obtain iterator over all references, or prefixed ones, loose or packed, sorted by their name.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themselves.
    pub fn iter(&self) -> Result<Platform<'_>, packed::buffer::open::Error> {
        Ok(Platform {
            store: self,
            packed: self.assure_packed_refs_uptodate()?,
        })
    }
}

impl file::Store {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themeselves.
    pub fn iter_packed<'s, 'p>(
        &'s self,
        packed: Option<&'p packed::Buffer>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        match self.namespace.as_ref() {
            Some(namespace) => self.iter_prefixed_unvalidated(namespace.to_path(), (None, None), packed),
            None => Ok(LooseThenPacked {
                base: &self.common_dir_resolved(),
                packed: match packed {
                    Some(packed) => Some(
                        packed
                            .iter()
                            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                            .peekable(),
                    ),
                    None => None,
                },
                loose: {
                    let (cd, refs) = self.common_and_refs_dir();
                    loose::iter::SortedLoosePaths::at_root_with_filename_prefix(refs, cd, None)
                }
                .peekable(),
                buf: Vec::new(),
                namespace: None,
            }),
        }
    }

    /// As [`iter(…)`][file::Store::iter()], but filters by `prefix`, i.e. "refs/heads".
    ///
    /// Please note that "refs/heads` or "refs\\heads" is equivalent to "refs/heads/"
    pub fn iter_prefixed_packed<'s, 'p>(
        &'s self,
        prefix: impl AsRef<Path>,
        packed: Option<&'p packed::Buffer>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        match self.namespace.as_ref() {
            None => {
                let (root, remainder) = self.validate_prefix(&self.common_dir_resolved(), prefix.as_ref())?;
                self.iter_prefixed_unvalidated(prefix, (root.into(), remainder), packed)
            }
            Some(namespace) => {
                let prefix = namespace.to_owned().into_namespaced_prefix(prefix);
                let (root, remainder) = self.validate_prefix(&self.common_dir_resolved(), &prefix)?;
                self.iter_prefixed_unvalidated(prefix, (root.into(), remainder), packed)
            }
        }
    }

    fn iter_prefixed_unvalidated<'s, 'p>(
        &'s self,
        prefix: impl AsRef<Path>,
        loose_root_and_filename_prefix: (Option<PathBuf>, Option<BString>),
        packed: Option<&'p packed::Buffer>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        let packed_prefix = path_to_name(prefix.as_ref());
        Ok(LooseThenPacked {
            base: self.common_dir_resolved(),
            packed: match packed {
                Some(packed) => Some(
                    packed
                        .iter_prefixed(packed_prefix.into_owned())
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                        .peekable(),
                ),
                None => None,
            },
            loose: loose::iter::SortedLoosePaths::at_root_with_filename_prefix(
                loose_root_and_filename_prefix
                    .0
                    .unwrap_or_else(|| self.common_dir_resolved().join(prefix)),
                self.common_dir_resolved().to_owned(),
                loose_root_and_filename_prefix.1,
            )
            .peekable(),
            buf: Vec::new(),
            namespace: self.namespace.as_ref(),
        })
    }
}

mod error {
    use std::{io, path::PathBuf};

    use git_object::bstr::BString;
    use quick_error::quick_error;

    use crate::store_impl::file;

    quick_error! {
        /// The error returned by the [`LooseThenPacked`][super::LooseThenPacked] iterator.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Traversal(err: io::Error) {
                display("The file system could not be traversed")
                source(err)
            }
            ReadFileContents{err: io::Error, path: PathBuf} {
                display("The ref file '{}' could not be read in full", path.display())
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
use git_features::threading::OwnShared;
