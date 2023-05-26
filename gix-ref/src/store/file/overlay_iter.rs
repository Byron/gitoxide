use std::{
    borrow::Cow,
    cmp::Ordering,
    io::Read,
    iter::Peekable,
    path::{Path, PathBuf},
};

use crate::{
    file::{loose, loose::iter::SortedLoosePaths, path_to_name},
    store_impl::{file, packed},
    BString, FullName, Namespace, Reference,
};

/// An iterator stepping through sorted input of loose references and packed references, preferring loose refs over otherwise
/// equivalent packed references.
///
/// All errors will be returned verbatim, while packed errors are depleted first if loose refs also error.
pub struct LooseThenPacked<'p, 's> {
    git_dir: &'s Path,
    common_dir: Option<&'s Path>,
    namespace: Option<&'s Namespace>,
    iter_packed: Option<Peekable<packed::Iter<'p>>>,
    iter_git_dir: Peekable<SortedLoosePaths>,
    #[allow(dead_code)]
    iter_common_dir: Option<Peekable<SortedLoosePaths>>,
    buf: Vec<u8>,
}

enum IterKind {
    Git,
    GitAndConsumeCommon,
    Common,
}

/// An intermediate structure to hold shared state alive long enough for iteration to happen.
#[must_use = "Iterators should be obtained from this platform"]
pub struct Platform<'s> {
    store: &'s file::Store,
    packed: Option<file::packed::SharedBufferSnapshot>,
}

impl<'p, 's> LooseThenPacked<'p, 's> {
    fn strip_namespace(&self, mut r: Reference) -> Reference {
        if let Some(namespace) = &self.namespace {
            r.strip_namespace(namespace);
        }
        r
    }

    fn loose_iter(&mut self, kind: IterKind) -> &mut Peekable<SortedLoosePaths> {
        match kind {
            IterKind::GitAndConsumeCommon => {
                drop(self.iter_common_dir.as_mut().map(|iter| iter.next()));
                &mut self.iter_git_dir
            }
            IterKind::Git => &mut self.iter_git_dir,
            IterKind::Common => self
                .iter_common_dir
                .as_mut()
                .expect("caller knows there is a common iter"),
        }
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
                source: err,
                path: refpath.clone(),
            })?;
        loose::Reference::try_from_path(name, &self.buf)
            .map_err(|err| {
                let relative_path = refpath
                    .strip_prefix(self.git_dir)
                    .ok()
                    .or_else(|| {
                        self.common_dir
                            .and_then(|common_dir| refpath.strip_prefix(common_dir).ok())
                    })
                    .expect("one of our bases contains the path");
                Error::ReferenceCreation {
                    source: err,
                    relative_path: relative_path.into(),
                }
            })
            .map(Into::into)
            .map(|r| self.strip_namespace(r))
    }
}

impl<'p, 's> Iterator for LooseThenPacked<'p, 's> {
    type Item = Result<Reference, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        fn advance_to_non_private(iter: &mut Peekable<SortedLoosePaths>) {
            while let Some(Ok((_path, name))) = iter.peek() {
                if name.category().map_or(false, |cat| cat.is_worktree_private()) {
                    iter.next();
                } else {
                    break;
                }
            }
        }

        fn peek_loose<'a>(
            git_dir: &'a mut Peekable<SortedLoosePaths>,
            common_dir: Option<&'a mut Peekable<SortedLoosePaths>>,
        ) -> Option<(&'a std::io::Result<(PathBuf, FullName)>, IterKind)> {
            match common_dir {
                Some(common_dir) => match (git_dir.peek(), {
                    advance_to_non_private(common_dir);
                    common_dir.peek()
                }) {
                    (None, None) => None,
                    (None, Some(res)) | (Some(_), Some(res @ Err(_))) => Some((res, IterKind::Common)),
                    (Some(res), None) | (Some(res @ Err(_)), Some(_)) => Some((res, IterKind::Git)),
                    (Some(r_gitdir @ Ok((_, git_dir_name))), Some(r_cd @ Ok((_, common_dir_name)))) => {
                        match git_dir_name.cmp(common_dir_name) {
                            Ordering::Less => Some((r_gitdir, IterKind::Git)),
                            Ordering::Equal => Some((r_gitdir, IterKind::GitAndConsumeCommon)),
                            Ordering::Greater => Some((r_cd, IterKind::Common)),
                        }
                    }
                },
                None => git_dir.peek().map(|r| (r, IterKind::Git)),
            }
        }
        match self.iter_packed.as_mut() {
            Some(packed_iter) => match (
                peek_loose(&mut self.iter_git_dir, self.iter_common_dir.as_mut()),
                packed_iter.peek(),
            ) {
                (None, None) => None,
                (None, Some(_)) | (Some(_), Some(Err(_))) => {
                    let res = packed_iter.next().expect("peeked value exists");
                    Some(self.convert_packed(res))
                }
                (Some((_, kind)), None) | (Some((Err(_), kind)), Some(_)) => {
                    let res = self.loose_iter(kind).next().expect("prior peek");
                    Some(self.convert_loose(res))
                }
                (Some((Ok((_, loose_name)), kind)), Some(Ok(packed))) => match loose_name.as_ref().cmp(packed.name) {
                    Ordering::Less => {
                        let res = self.loose_iter(kind).next().expect("prior peek");
                        Some(self.convert_loose(res))
                    }
                    Ordering::Equal => {
                        drop(packed_iter.next());
                        let res = self.loose_iter(kind).next().expect("prior peek");
                        Some(self.convert_loose(res))
                    }
                    Ordering::Greater => {
                        let res = packed_iter.next().expect("name retrieval configured");
                        Some(self.convert_packed(res))
                    }
                },
            },
            None => match peek_loose(&mut self.iter_git_dir, self.iter_common_dir.as_mut()) {
                None => None,
                Some((_, kind)) => self.loose_iter(kind).next().map(|res| self.convert_loose(res)),
            },
        }
    }
}

impl<'s> Platform<'s> {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themselves.
    pub fn all(&self) -> std::io::Result<LooseThenPacked<'_, '_>> {
        self.store.iter_packed(self.packed.as_ref().map(|b| &***b))
    }

    /// As [`iter(…)`][file::Store::iter()], but filters by `prefix`, i.e. "refs/heads".
    ///
    /// Please note that "refs/heads` or "refs\\heads" is equivalent to "refs/heads/"
    pub fn prefixed(&self, prefix: impl AsRef<Path>) -> std::io::Result<LooseThenPacked<'_, '_>> {
        self.store
            .iter_prefixed_packed(prefix, self.packed.as_ref().map(|b| &***b))
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

#[derive(Debug)]
pub(crate) enum IterInfo<'a> {
    Base {
        base: &'a Path,
    },
    BaseAndIterRoot {
        base: &'a Path,
        iter_root: PathBuf,
        prefix: Cow<'a, Path>,
    },
    PrefixAndBase {
        base: &'a Path,
        prefix: &'a Path,
    },
    ComputedIterationRoot {
        /// The root to iterate over
        iter_root: PathBuf,
        /// The top-level directory as boundary of all references, used to create their short-names after iteration
        base: &'a Path,
        /// The original prefix
        prefix: Cow<'a, Path>,
        /// The remainder of the prefix that wasn't a valid path
        remainder: Option<BString>,
    },
}

impl<'a> IterInfo<'a> {
    fn prefix(&self) -> Option<&Path> {
        match self {
            IterInfo::Base { .. } => None,
            IterInfo::PrefixAndBase { prefix, .. } => Some(*prefix),
            IterInfo::ComputedIterationRoot { prefix, .. } | IterInfo::BaseAndIterRoot { prefix, .. } => {
                prefix.as_ref().into()
            }
        }
    }

    fn into_iter(self) -> Peekable<SortedLoosePaths> {
        match self {
            IterInfo::Base { base } => SortedLoosePaths::at(base.join("refs"), base, None),
            IterInfo::BaseAndIterRoot {
                base,
                iter_root,
                prefix: _,
            } => SortedLoosePaths::at(iter_root, base, None),
            IterInfo::PrefixAndBase { base, prefix } => SortedLoosePaths::at(base.join(prefix), base, None),
            IterInfo::ComputedIterationRoot {
                iter_root,
                base,
                prefix: _,
                remainder,
            } => SortedLoosePaths::at(iter_root, base, remainder),
        }
        .peekable()
    }

    fn from_prefix(base: &'a Path, prefix: Cow<'a, Path>) -> std::io::Result<Self> {
        if prefix.is_absolute() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "prefix must be a relative path, like 'refs/heads'",
            ));
        }
        use std::path::Component::*;
        if prefix.components().any(|c| matches!(c, CurDir | ParentDir)) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Refusing to handle prefixes with relative path components",
            ));
        }
        let iter_root = base.join(prefix.as_ref());
        if iter_root.is_dir() {
            Ok(IterInfo::BaseAndIterRoot {
                base,
                iter_root,
                prefix,
            })
        } else {
            let filename_prefix = iter_root
                .file_name()
                .map(ToOwned::to_owned)
                .map(|p| {
                    gix_path::try_into_bstr(PathBuf::from(p))
                        .map(|p| p.into_owned())
                        .map_err(|_| {
                            std::io::Error::new(std::io::ErrorKind::InvalidInput, "prefix contains ill-formed UTF-8")
                        })
                })
                .transpose()?;
            let iter_root = iter_root
                .parent()
                .expect("a parent is always there unless empty")
                .to_owned();
            Ok(IterInfo::ComputedIterationRoot {
                base,
                prefix,
                iter_root,
                remainder: filename_prefix,
            })
        }
    }
}

impl file::Store {
    /// Return an iterator over all references, loose or `packed`, sorted by their name.
    ///
    /// Errors are returned similarly to what would happen when loose and packed refs where iterated by themselves.
    pub fn iter_packed<'s, 'p>(
        &'s self,
        packed: Option<&'p packed::Buffer>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        match self.namespace.as_ref() {
            Some(namespace) => self.iter_from_info(
                IterInfo::PrefixAndBase {
                    base: self.git_dir(),
                    prefix: namespace.to_path(),
                },
                self.common_dir().map(|base| IterInfo::PrefixAndBase {
                    base,
                    prefix: namespace.to_path(),
                }),
                packed,
            ),
            None => self.iter_from_info(
                IterInfo::Base { base: self.git_dir() },
                self.common_dir().map(|base| IterInfo::Base { base }),
                packed,
            ),
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
                let prefix = prefix.as_ref();
                let git_dir_info = IterInfo::from_prefix(self.git_dir(), prefix.into())?;
                let common_dir_info = self
                    .common_dir()
                    .map(|base| IterInfo::from_prefix(base, prefix.into()))
                    .transpose()?;
                self.iter_from_info(git_dir_info, common_dir_info, packed)
            }
            Some(namespace) => {
                let prefix = namespace.clone().into_namespaced_prefix(prefix);
                let git_dir_info = IterInfo::from_prefix(self.git_dir(), prefix.clone().into())?;
                let common_dir_info = self
                    .common_dir()
                    .map(|base| IterInfo::from_prefix(base, prefix.into()))
                    .transpose()?;
                self.iter_from_info(git_dir_info, common_dir_info, packed)
            }
        }
    }

    fn iter_from_info<'s, 'p>(
        &'s self,
        git_dir_info: IterInfo<'_>,
        common_dir_info: Option<IterInfo<'_>>,
        packed: Option<&'p packed::Buffer>,
    ) -> std::io::Result<LooseThenPacked<'p, 's>> {
        Ok(LooseThenPacked {
            git_dir: self.git_dir(),
            common_dir: self.common_dir(),
            iter_packed: match packed {
                Some(packed) => Some(
                    match git_dir_info.prefix() {
                        Some(prefix) => packed.iter_prefixed(path_to_name(prefix).into_owned()),
                        None => packed.iter(),
                    }
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                    .peekable(),
                ),
                None => None,
            },
            iter_git_dir: git_dir_info.into_iter(),
            iter_common_dir: common_dir_info.map(IterInfo::into_iter),
            buf: Vec::new(),
            namespace: self.namespace.as_ref(),
        })
    }
}

mod error {
    use std::{io, path::PathBuf};

    use gix_object::bstr::BString;

    use crate::store_impl::file;

    /// The error returned by the [`LooseThenPacked`][super::LooseThenPacked] iterator.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The file system could not be traversed")]
        Traversal(#[source] io::Error),
        #[error("The ref file {path:?} could not be read in full")]
        ReadFileContents { source: io::Error, path: PathBuf },
        #[error("The reference at \"{relative_path}\" could not be instantiated")]
        ReferenceCreation {
            source: file::loose::reference::decode::Error,
            relative_path: PathBuf,
        },
        #[error("Invalid reference in line {line_number}: {invalid_line:?}")]
        PackedReference { invalid_line: BString, line_number: usize },
    }
}
pub use error::Error;
