use std::{
    convert::TryInto,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub use error::Error;

use crate::name::is_pseudo_ref;
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
        let mut buf2 = Vec::default();
        if is_pseudo_ref(partial_name.as_bstr()) {
            if let Some(r) = self.find_inner("", &partial_name, None, Transform::None, &mut buf, &mut buf2)? {
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
                &mut buf2,
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
            &mut buf2,
        )
    }

    fn find_inner(
        &self,
        inbetween: &str,
        partial_name: &PartialNameCow<'_>,
        packed: Option<&packed::Buffer>,
        transform: Transform,
        path_buf: &mut BString,
        content_buf: &mut Vec<u8>,
    ) -> Result<Option<Reference>, Error> {
        let add_refs_prefix = matches!(transform, Transform::EnforceRefsPrefix);
        let full_name = partial_name.construct_full_name_ref(add_refs_prefix, inbetween, path_buf);

        let did_read_file = self
            .ref_contents2(full_name, content_buf)
            .map_err(|err| Error::ReadFileContents {
                err,
                path: self.reference_path2(full_name),
            })?;

        if !did_read_file {
            if add_refs_prefix {
                if let Some(packed) = packed {
                    let full_name = match &self.namespace {
                        Some(namespace) => PartialNameCow(
                            namespace
                                .to_owned()
                                .into_namespaced_prefix_bstr(full_name.as_bstr())
                                .into(),
                        ),
                        None => full_name.into(),
                    };
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

        Ok(Some(
            loose::Reference::try_from_path(full_name.to_owned(), content_buf)
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
        ))
    }
}

impl file::Store {
    fn base_for_name(&self, name: FullNameRef<'_>) -> &Path {
        self.common_dir
            .as_deref()
            .and_then(|commondir| {
                name.category().map(|c| {
                    use crate::Category::*;
                    match c {
                        Tag | LocalBranch | RemoteBranch | Note | MainRef | MainPseudoRef | LinkedRef
                        | LinkedPseudoRef => commondir,
                        PseudoRef | Bisect | Rewritten | WorktreePrivate => self.git_dir.as_path(),
                    }
                })
            })
            .unwrap_or(self.git_dir.as_path())
    }

    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path2(&self, name: FullNameRef<'_>) -> PathBuf {
        let base = self.base_for_name(name);
        let relative_path = git_path::to_native_path_on_windows(name.as_bstr());
        match &self.namespace {
            None => base.join(relative_path),
            Some(namespace) => base.join(namespace.to_path()).join(relative_path),
        }
    }

    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path(&self, name: &Path) -> PathBuf {
        match &self.namespace {
            None => self.git_dir.join(name),
            Some(namespace) => self.git_dir.join(namespace.to_path()).join(name),
        }
    }
    /// Read the file contents with a verified full reference path and return it in the given vector if possible.
    pub(crate) fn ref_contents2(&self, name: FullNameRef<'_>, buf: &mut Vec<u8>) -> io::Result<bool> {
        let ref_path = self.reference_path2(name);
        buf.clear();

        match std::fs::File::open(&ref_path) {
            Ok(mut file) => {
                if let Err(err) = file.read_to_end(buf) {
                    return if ref_path.is_dir() { Ok(false) } else { Err(err) };
                }
                Ok(true)
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
            #[cfg(target_os = "windows")]
            Err(err) if err.kind() == std::io::ErrorKind::PermissionDenied => Ok(false),
            Err(err) => Err(err),
        }
    }

    /// Read the file contents with a verified full reference path and return it in the given vector if possible.
    pub(crate) fn ref_contents(&self, relative_path: &Path) -> io::Result<Option<Vec<u8>>> {
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
