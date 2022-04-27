use std::{
    convert::TryInto,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub use error::Error;

use crate::{
    file,
    store_impl::{
        file::{loose, path_to_name},
        packed,
    },
    FullName, PartialNameRef, Reference,
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
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        Error: From<E>,
    {
        let path = partial.try_into()?;
        let packed = self.assure_packed_refs_uptodate()?;
        self.find_one_with_verified_input(path.to_partial_path().as_ref(), packed.as_deref())
    }

    /// Similar to [`file::Store::find()`] but a non-existing ref is treated as error.
    ///
    /// Find only loose references, that is references that aren't in the packed-refs buffer.
    /// All symbolic references are loose references.
    /// `HEAD` is always a loose reference.
    pub fn try_find_loose<'a, Name, E>(&self, partial: Name) -> Result<Option<loose::Reference>, Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        Error: From<E>,
    {
        let path = partial.try_into()?;
        self.find_one_with_verified_input(path.to_partial_path().as_ref(), None)
            .map(|r| r.map(|r| r.try_into().expect("only loose refs are found without pack")))
    }

    /// Similar to [`file::Store::find()`], but allows to pass a snapshotted packed buffer instead.
    pub fn try_find_packed<'a, Name, E>(
        &self,
        partial: Name,
        packed: Option<&packed::Buffer>,
    ) -> Result<Option<Reference>, Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        Error: From<E>,
    {
        let path = partial.try_into()?;
        self.find_one_with_verified_input(path.to_partial_path().as_ref(), packed)
    }

    pub(crate) fn find_one_with_verified_input(
        &self,
        relative_path: &Path,
        packed: Option<&packed::Buffer>,
    ) -> Result<Option<Reference>, Error> {
        let is_all_uppercase = relative_path
            .to_string_lossy()
            .as_ref()
            .chars()
            .all(|c| c.is_ascii_uppercase());
        if relative_path.components().count() == 1 && is_all_uppercase {
            if let Some(r) = self.find_inner("", relative_path, None, Transform::None)? {
                return Ok(Some(r));
            }
        }

        for inbetween in &["", "tags", "heads", "remotes"] {
            match self.find_inner(*inbetween, relative_path, packed, Transform::EnforceRefsPrefix) {
                Ok(Some(r)) => return Ok(Some(r)),
                Ok(None) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }
        self.find_inner(
            "remotes",
            &relative_path.join("HEAD"),
            None,
            Transform::EnforceRefsPrefix,
        )
    }

    fn find_inner(
        &self,
        inbetween: &str,
        relative_path: &Path,
        packed: Option<&packed::Buffer>,
        transform: Transform,
    ) -> Result<Option<Reference>, Error> {
        let (base, is_definitely_full_path) = match transform {
            Transform::EnforceRefsPrefix => (
                if relative_path.starts_with("refs") {
                    PathBuf::new()
                } else {
                    PathBuf::from("refs")
                },
                true,
            ),
            Transform::None => (PathBuf::new(), false),
        };
        let relative_path = base.join(inbetween).join(relative_path);

        let path_to_open = git_path::to_windows_separators_on_windows_or_panic(
            git_path::into_bstr_or_panic_on_windows(&relative_path),
        );
        let contents = match self
            .ref_contents(&path_to_open)
            .map_err(|err| Error::ReadFileContents {
                err,
                path: path_to_open.into_owned(),
            })? {
            None => {
                if is_definitely_full_path {
                    if let Some(packed) = packed {
                        let full_name = path_to_name(match &self.namespace {
                            None => relative_path,
                            Some(namespace) => namespace.to_owned().into_namespaced_prefix(relative_path),
                        });
                        let full_name = PartialNameRef(full_name.into_owned().into());
                        if let Some(packed_ref) = packed.try_find(full_name)? {
                            let mut res: Reference = packed_ref.into();
                            if let Some(namespace) = &self.namespace {
                                res.strip_namespace(namespace);
                            }
                            return Ok(Some(res));
                        };
                    }
                }
                return Ok(None);
            }
            Some(c) => c,
        };
        Ok(Some({
            let full_name = path_to_name(&relative_path);
            loose::Reference::try_from_path(FullName(full_name.into_owned()), &contents)
                .map(Into::into)
                .map(|mut r: Reference| {
                    if let Some(namespace) = &self.namespace {
                        r.strip_namespace(namespace);
                    }
                    r
                })
                .map_err(|err| Error::ReferenceCreation { err, relative_path })?
        }))
    }
}

impl file::Store {
    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path(&self, name: &Path) -> PathBuf {
        match &self.namespace {
            None => self.base.join(name),
            Some(namespace) => self.base.join(namespace.to_path()).join(name),
        }
    }

    /// Read the file contents with a verified full reference path and return it in the given vector if possible.
    pub(crate) fn ref_contents(&self, relative_path: &Path) -> std::io::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();
        let ref_path = self.reference_path(relative_path);

        match std::fs::File::open(&ref_path) {
            Ok(mut file) => {
                if let Err(err) = file.read_to_end(&mut buf) {
                    return if ref_path.is_dir() { Ok(None) } else { Err(err) };
                }
                Ok(Some(buf))
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
        PartialNameRef, Reference,
    };

    impl file::Store {
        /// Similar to [`file::Store::try_find()`] but a non-existing ref is treated as error.
        pub fn find<'a, Name, E>(&self, partial: Name) -> Result<Reference, Error>
        where
            Name: TryInto<PartialNameRef<'a>, Error = E>,
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
            Name: TryInto<PartialNameRef<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            self.find_existing_inner(partial, packed)
        }

        /// Similar to [`file::Store::find()`] won't handle packed-refs.
        pub fn find_loose<'a, Name, E>(&self, partial: Name) -> Result<loose::Reference, Error>
        where
            Name: TryInto<PartialNameRef<'a>, Error = E>,
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
            Name: TryInto<PartialNameRef<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            let path = partial
                .try_into()
                .map_err(|err| Error::Find(find::Error::RefnameValidation(err.into())))?;
            match self.find_one_with_verified_input(path.to_partial_path().as_ref(), packed) {
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
