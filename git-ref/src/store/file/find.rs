use std::borrow::Cow;
use std::{
    convert::TryInto,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub use error::Error;

use crate::{
    file,
    store_impl::{file::loose, packed},
    BString, FullNameRef, PartialNameCow, Reference,
};

enum Transform {
    EnforceRefsPrefix,
    None,
}

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
        Name: TryInto<PartialNameCow<'a>, Error = E>,
        Error: From<E>,
    {
        let packed = self.assure_packed_refs_uptodate()?;
        self.find_one_with_verified_input(partial.try_into()?, packed.as_deref())
    }

    /// Similar to [`file::Store::find()`] but a non-existing ref is treated as error.
    ///
    /// Find only loose references, that is references that aren't in the packed-refs buffer.
    /// All symbolic references are loose references.
    /// `HEAD` is always a loose reference.
    pub fn try_find_loose<'a, Name, E>(&self, partial: Name) -> Result<Option<loose::Reference>, Error>
    where
        Name: TryInto<PartialNameCow<'a>, Error = E>,
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
        Name: TryInto<PartialNameCow<'a>, Error = E>,
        Error: From<E>,
    {
        self.find_one_with_verified_input(partial.try_into()?, packed)
    }

    pub(crate) fn find_one_with_verified_input(
        &self,
        partial_name: PartialNameCow<'_>,
        packed: Option<&packed::Buffer>,
    ) -> Result<Option<Reference>, Error> {
        let mut buf = BString::default();
        if partial_name.looks_like_full_name() {
            if let Some(r) = self.find_inner("", &partial_name, None, Transform::None, &mut buf)? {
                return Ok(Some(r));
            }
        }

        for inbetween in &["", "tags", "heads", "remotes"] {
            match self.find_inner(
                *inbetween,
                &partial_name,
                packed,
                Transform::EnforceRefsPrefix,
                &mut buf,
            ) {
                Ok(Some(r)) => return Ok(Some(r)),
                Ok(None) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }
        self.find_inner(
            "remotes",
            &partial_name.join("HEAD").expect("HEAD is valid name"),
            None,
            Transform::EnforceRefsPrefix,
            &mut buf,
        )
    }

    fn find_inner(
        &self,
        inbetween: &str,
        partial_name: &PartialNameCow<'_>,
        packed: Option<&packed::Buffer>,
        transform: Transform,
        path_buf: &mut BString,
    ) -> Result<Option<Reference>, Error> {
        let add_refs_prefix = matches!(transform, Transform::EnforceRefsPrefix);
        let full_name = partial_name.construct_full_name_ref(add_refs_prefix, inbetween, path_buf);

        let content_buf = self.ref_contents(full_name).map_err(|err| Error::ReadFileContents {
            err,
            path: self.reference_path(full_name),
        })?;

        match content_buf {
            None => {
                if add_refs_prefix {
                    if let Some(packed) = packed {
                        if let Some(full_name) = packed::find::transform_full_name_for_lookup(full_name) {
                            let full_name_backing;
                            let full_name = match &self.namespace {
                                Some(namespace) => {
                                    full_name_backing = namespace.to_owned().into_namespaced_name(full_name);
                                    full_name_backing.to_ref()
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
                        err,
                        relative_path: full_name.to_path().to_owned(),
                    })?,
            )),
        }
    }
}

impl file::Store {
    pub(crate) fn base_dir_and_rela_path_for_name<'a>(&self, name: FullNameRef<'a>) -> (&Path, Cow<'a, Path>) {
        let (base, relative_path) = self
            .common_dir
            .as_deref()
            .and_then(|commondir| {
                name.category_and_short_name().and_then(|(c, sn)| {
                    use crate::Category::*;
                    Some(match c {
                        Tag | LocalBranch | RemoteBranch | Note => (commondir, name.as_bstr()),
                        MainRef | MainPseudoRef | LinkedRef | LinkedPseudoRef => (commondir, sn),
                        PseudoRef | Bisect | Rewritten | WorktreePrivate => return None,
                    })
                })
            })
            .unwrap_or((self.git_dir.as_path(), name.as_bstr()));
        let relative_path = git_path::to_native_path_on_windows(relative_path);
        (base, relative_path)
    }

    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path(&self, name: FullNameRef<'_>) -> PathBuf {
        let (base, relative_path) = self.base_dir_and_rela_path_for_name(name);
        match &self.namespace {
            None => base.join(relative_path),
            Some(namespace) => base.join(namespace.to_path()).join(relative_path),
        }
    }

    /// Read the file contents with a verified full reference path and return it in the given vector if possible.
    pub(crate) fn ref_contents(&self, name: FullNameRef<'_>) -> io::Result<Option<Vec<u8>>> {
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
            #[cfg(target_os = "windows")]
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
        PartialNameCow, Reference,
    };

    impl file::Store {
        /// Similar to [`file::Store::try_find()`] but a non-existing ref is treated as error.
        pub fn find<'a, Name, E>(&self, partial: Name) -> Result<Reference, Error>
        where
            Name: TryInto<PartialNameCow<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            let packed = self.assure_packed_refs_uptodate().map_err(find::Error::PackedOpen)?;
            self.find_existing_inner(partial, packed.as_deref())
        }

        /// Similar to [`file::Store::find()`], but supports a stable packed buffer.
        pub fn find_packed<'a, Name, E>(
            &self,
            partial: Name,
            packed: Option<&packed::Buffer>,
        ) -> Result<Reference, Error>
        where
            Name: TryInto<PartialNameCow<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            self.find_existing_inner(partial, packed)
        }

        /// Similar to [`file::Store::find()`] won't handle packed-refs.
        pub fn find_loose<'a, Name, E>(&self, partial: Name) -> Result<loose::Reference, Error>
        where
            Name: TryInto<PartialNameCow<'a>, Error = E>,
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
            Name: TryInto<PartialNameCow<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            let path = partial
                .try_into()
                .map_err(|err| Error::Find(find::Error::RefnameValidation(err.into())))?;
            match self.find_one_with_verified_input(path.clone(), packed) {
                Ok(Some(r)) => Ok(r),
                Ok(None) => Err(Error::NotFound(path.to_partial_path().to_owned())),
                Err(err) => Err(err.into()),
            }
        }
    }

    mod error {
        use std::path::PathBuf;

        use quick_error::quick_error;

        use crate::store_impl::file::find;

        quick_error! {
            /// The error returned by [file::Store::find_existing()][crate::file::Store::find_existing()].
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Find(err: find::Error) {
                    display("An error occured while trying to find a reference")
                    from()
                    source(err)
                }
                NotFound(name: PathBuf) {
                    display("The ref partially named '{}' could not be found", name.display())
                }
            }
        }
    }
}

mod error {
    use std::{convert::Infallible, io, path::PathBuf};

    use quick_error::quick_error;

    use crate::{file, store_impl::packed};

    quick_error! {
        /// The error returned by [file::Store::find()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            RefnameValidation(err: crate::name::Error) {
                display("The ref name or path is not a valid ref name")
                from()
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
            PackedRef(err: packed::find::Error) {
                display("A packed ref lookup failed")
                from()
                source(err)
            }
            PackedOpen(err: packed::buffer::open::Error) {
                display("Could not open the packed refs buffer when trying to find references.")
                from()
                source(err)
            }
        }
    }

    impl From<Infallible> for Error {
        fn from(_: Infallible) -> Self {
            unreachable!("this impl is needed to allow passing a known valid partial path as parameter")
        }
    }
}
