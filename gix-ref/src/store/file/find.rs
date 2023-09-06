use std::{
    borrow::Cow,
    convert::TryInto,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub use error::Error;

use crate::{
    file,
    store_impl::{file::loose, packed},
    BStr, BString, FullNameRef, PartialNameRef, Reference,
};

impl file::Store {
    /// Find a single reference by the given `path` which is required to be a valid reference name.
    ///
    /// Returns `Ok(None)` if no such ref exists.
    ///
    /// ### Note
    ///
    /// * The lookup algorithm follows the one in [the git documentation][git-lookup-docs].
    /// * The packed buffer is checked for modifications each time the method is called. See [`file::Store::try_find_packed()`]
    ///   for a version with more control.
    ///
    /// [git-lookup-docs]: https://github.com/git/git/blob/5d5b1473453400224ebb126bf3947e0a3276bdf5/Documentation/revisions.txt#L34-L46
    pub fn try_find<'a, Name, E>(&self, partial: Name) -> Result<Option<Reference>, Error>
    where
        Name: TryInto<&'a PartialNameRef, Error = E>,
        Error: From<E>,
    {
        let packed = self.assure_packed_refs_uptodate()?;
        self.find_one_with_verified_input(partial.try_into()?, packed.as_ref().map(|b| &***b))
    }

    /// Similar to [`file::Store::find()`] but a non-existing ref is treated as error.
    ///
    /// Find only loose references, that is references that aren't in the packed-refs buffer.
    /// All symbolic references are loose references.
    /// `HEAD` is always a loose reference.
    pub fn try_find_loose<'a, Name, E>(&self, partial: Name) -> Result<Option<loose::Reference>, Error>
    where
        Name: TryInto<&'a PartialNameRef, Error = E>,
        Error: From<E>,
    {
        self.find_one_with_verified_input(partial.try_into()?, None)
            .map(|r| r.map(|r| r.try_into().expect("only loose refs are found without pack")))
    }

    /// Similar to [`file::Store::find()`], but allows to pass a snapshotted packed buffer instead.
    pub fn try_find_packed<'a, Name, E>(
        &self,
        partial: Name,
        packed: Option<&packed::Buffer>,
    ) -> Result<Option<Reference>, Error>
    where
        Name: TryInto<&'a PartialNameRef, Error = E>,
        Error: From<E>,
    {
        self.find_one_with_verified_input(partial.try_into()?, packed)
    }

    pub(crate) fn find_one_with_verified_input(
        &self,
        partial_name: &PartialNameRef,
        packed: Option<&packed::Buffer>,
    ) -> Result<Option<Reference>, Error> {
        let mut buf = BString::default();
        for inbetween in &["", "tags", "heads", "remotes"] {
            match self.find_inner(inbetween, partial_name, packed, &mut buf) {
                Ok(Some(r)) => return Ok(Some(r)),
                Ok(None) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }
        if partial_name.as_bstr() != "HEAD" {
            self.find_inner(
                "remotes",
                partial_name
                    .to_owned()
                    .join("HEAD".into())
                    .expect("HEAD is valid name")
                    .as_ref(),
                None,
                &mut buf,
            )
        } else {
            Ok(None)
        }
    }

    fn find_inner(
        &self,
        inbetween: &str,
        partial_name: &PartialNameRef,
        packed: Option<&packed::Buffer>,
        path_buf: &mut BString,
    ) -> Result<Option<Reference>, Error> {
        let full_name = partial_name.construct_full_name_ref(inbetween, path_buf);
        let content_buf = self.ref_contents(full_name).map_err(|err| Error::ReadFileContents {
            source: err,
            path: self.reference_path(full_name),
        })?;

        match content_buf {
            None => {
                if let Some(packed) = packed {
                    if let Some(full_name) = packed::find::transform_full_name_for_lookup(full_name) {
                        let full_name_backing;
                        let full_name = match &self.namespace {
                            Some(namespace) => {
                                full_name_backing = namespace.to_owned().into_namespaced_name(full_name);
                                full_name_backing.as_ref()
                            }
                            None => full_name,
                        };
                        if let Some(packed_ref) = packed.try_find_full_name(full_name)? {
                            let mut res: Reference = packed_ref.into();
                            if let Some(namespace) = &self.namespace {
                                res.strip_namespace(namespace);
                            }
                            return Ok(Some(res));
                        };
                    }
                }
                Ok(None)
            }
            Some(content) => Ok(Some(
                loose::Reference::try_from_path(full_name.to_owned(), &content)
                    .map(Into::into)
                    .map(|mut r: Reference| {
                        if let Some(namespace) = &self.namespace {
                            r.strip_namespace(namespace);
                        }
                        r
                    })
                    .map_err(|err| Error::ReferenceCreation {
                        source: err,
                        relative_path: full_name.to_path().to_owned(),
                    })?,
            )),
        }
    }
}

impl file::Store {
    pub(crate) fn to_base_dir_and_relative_name<'a>(
        &self,
        name: &'a FullNameRef,
        is_reflog: bool,
    ) -> (Cow<'_, Path>, &'a FullNameRef) {
        let commondir = self.common_dir_resolved();
        let linked_git_dir =
            |worktree_name: &BStr| commondir.join("worktrees").join(gix_path::from_bstr(worktree_name));
        name.category_and_short_name()
            .map(|(c, sn)| {
                use crate::Category::*;
                let sn = FullNameRef::new_unchecked(sn);
                match c {
                    LinkedPseudoRef { name: worktree_name } => is_reflog
                        .then(|| (linked_git_dir(worktree_name).into(), sn))
                        .unwrap_or((commondir.into(), name)),
                    Tag | LocalBranch | RemoteBranch | Note => (commondir.into(), name),
                    MainRef | MainPseudoRef => (commondir.into(), sn),
                    LinkedRef { name: worktree_name } => sn
                        .category()
                        .map_or(false, |cat| cat.is_worktree_private())
                        .then(|| {
                            if is_reflog {
                                (linked_git_dir(worktree_name).into(), sn)
                            } else {
                                (commondir.into(), name)
                            }
                        })
                        .unwrap_or((commondir.into(), sn)),
                    PseudoRef | Bisect | Rewritten | WorktreePrivate => (self.git_dir.as_path().into(), name),
                }
            })
            .unwrap_or((commondir.into(), name))
    }

    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path_with_base<'b>(&self, name: &'b FullNameRef) -> (Cow<'_, Path>, Cow<'b, Path>) {
        let (base, name) = self.to_base_dir_and_relative_name(name, false);
        (
            base,
            match &self.namespace {
                None => gix_path::to_native_path_on_windows(name.as_bstr()),
                Some(namespace) => {
                    gix_path::to_native_path_on_windows(namespace.to_owned().into_namespaced_name(name).into_inner())
                }
            },
        )
    }

    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path(&self, name: &FullNameRef) -> PathBuf {
        let (base, relative_path) = self.reference_path_with_base(name);
        base.join(relative_path)
    }

    /// Read the file contents with a verified full reference path and return it in the given vector if possible.
    pub(crate) fn ref_contents(&self, name: &FullNameRef) -> io::Result<Option<Vec<u8>>> {
        let ref_path = self.reference_path(name);

        match std::fs::File::open(&ref_path) {
            Ok(mut file) => {
                let mut buf = Vec::with_capacity(128);
                if let Err(err) = file.read_to_end(&mut buf) {
                    return if ref_path.is_dir() { Ok(None) } else { Err(err) };
                }
                Ok(buf.into())
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            #[cfg(windows)]
            Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => Ok(None),
            Err(err) => Err(err),
        }
    }
}

///
pub mod existing {
    use std::convert::TryInto;

    pub use error::Error;

    use crate::{
        file::{self},
        store_impl::{
            file::{find, loose},
            packed,
        },
        PartialNameRef, Reference,
    };

    impl file::Store {
        /// Similar to [`file::Store::try_find()`] but a non-existing ref is treated as error.
        pub fn find<'a, Name, E>(&self, partial: Name) -> Result<Reference, Error>
        where
            Name: TryInto<&'a PartialNameRef, Error = E>,
            crate::name::Error: From<E>,
        {
            let packed = self.assure_packed_refs_uptodate().map_err(find::Error::PackedOpen)?;
            self.find_existing_inner(partial, packed.as_ref().map(|b| &***b))
        }

        /// Similar to [`file::Store::find()`], but supports a stable packed buffer.
        pub fn find_packed<'a, Name, E>(
            &self,
            partial: Name,
            packed: Option<&packed::Buffer>,
        ) -> Result<Reference, Error>
        where
            Name: TryInto<&'a PartialNameRef, Error = E>,
            crate::name::Error: From<E>,
        {
            self.find_existing_inner(partial, packed)
        }

        /// Similar to [`file::Store::find()`] won't handle packed-refs.
        pub fn find_loose<'a, Name, E>(&self, partial: Name) -> Result<loose::Reference, Error>
        where
            Name: TryInto<&'a PartialNameRef, Error = E>,
            crate::name::Error: From<E>,
        {
            self.find_existing_inner(partial, None)
                .map(|r| r.try_into().expect("always loose without packed"))
        }

        /// Similar to [`file::Store::find()`] but a non-existing ref is treated as error.
        pub(crate) fn find_existing_inner<'a, Name, E>(
            &self,
            partial: Name,
            packed: Option<&packed::Buffer>,
        ) -> Result<Reference, Error>
        where
            Name: TryInto<&'a PartialNameRef, Error = E>,
            crate::name::Error: From<E>,
        {
            let path = partial
                .try_into()
                .map_err(|err| Error::Find(find::Error::RefnameValidation(err.into())))?;
            match self.find_one_with_verified_input(path, packed) {
                Ok(Some(r)) => Ok(r),
                Ok(None) => Err(Error::NotFound {
                    name: path.to_partial_path().to_owned(),
                }),
                Err(err) => Err(err.into()),
            }
        }
    }

    mod error {
        use std::path::PathBuf;

        use crate::store_impl::file::find;

        /// The error returned by [file::Store::find_existing()][crate::file::Store::find()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("An error occurred while trying to find a reference")]
            Find(#[from] find::Error),
            #[error("The ref partially named {name:?} could not be found")]
            NotFound { name: PathBuf },
        }
    }
}

mod error {
    use std::{convert::Infallible, io, path::PathBuf};

    use crate::{file, store_impl::packed};

    /// The error returned by [file::Store::find()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The ref name or path is not a valid ref name")]
        RefnameValidation(#[from] crate::name::Error),
        #[error("The ref file {path:?} could not be read in full")]
        ReadFileContents { source: io::Error, path: PathBuf },
        #[error("The reference at \"{relative_path}\" could not be instantiated")]
        ReferenceCreation {
            source: file::loose::reference::decode::Error,
            relative_path: PathBuf,
        },
        #[error("A packed ref lookup failed")]
        PackedRef(#[from] packed::find::Error),
        #[error("Could not open the packed refs buffer when trying to find references.")]
        PackedOpen(#[from] packed::buffer::open::Error),
    }

    impl From<Infallible> for Error {
        fn from(_: Infallible) -> Self {
            unreachable!("this impl is needed to allow passing a known valid partial path as parameter")
        }
    }
}
