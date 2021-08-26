use std::{
    convert::TryInto,
    io::{self, Read},
    path::{Path, PathBuf},
};

use bstr::ByteSlice;
pub use error::Error;

use crate::{
    file,
    store::{
        file::{loose, path_to_name},
        packed,
    },
    FullName, PartialNameRef,
};

enum Transform {
    EnforceRefsPrefix,
    None,
}

impl file::Store {
    /// Find a single reference by the given `path` which is required to be a valid reference name.
    ///
    /// If `packed` is provided, the reference search will extend to the packed buffer created with [`packed()`][file::Store::packed_buffer()].
    /// Note that the caller is responsible for its freshness, i.e. assuring it wasn't modified since it was read.
    ///
    /// Returns `Ok(None)` if no such ref exists.
    ///
    /// ### Note
    ///
    /// The lookup algorithm follows the one in [the git documentation][git-lookup-docs].
    ///
    /// [git-lookup-docs]: https://github.com/git/git/blob/5d5b1473453400224ebb126bf3947e0a3276bdf5/Documentation/revisions.txt#L34-L46
    pub fn try_find<'a, 'p, 's, Name, E>(
        &'s self,
        partial: Name,
        packed: Option<&'p packed::Buffer>,
    ) -> Result<Option<file::Reference<'p>>, Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        Error: From<E>,
    {
        let path = partial.try_into()?;
        self.find_one_with_verified_input(path.to_partial_path().as_ref(), packed)
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
        self.try_find(partial, None)
            .map(|r| r.map(|r| r.try_into().expect("only loose refs are found without pack")))
    }

    pub(in crate::store::file) fn find_one_with_verified_input<'p>(
        &self,
        relative_path: &Path,
        packed: Option<&'p packed::Buffer>,
    ) -> Result<Option<file::Reference<'p>>, Error> {
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

    fn find_inner<'p>(
        &self,
        inbetween: &str,
        relative_path: &Path,
        packed: Option<&'p packed::Buffer>,
        transform: Transform,
    ) -> Result<Option<file::Reference<'p>>, Error> {
        let (base, is_definitely_absolute) = match transform {
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

        let contents = match self.ref_contents(&relative_path)? {
            None => {
                if is_definitely_absolute {
                    if let Some(packed) = packed {
                        let full_name = path_to_name(relative_path);
                        let full_name = PartialNameRef((*full_name).as_bstr());
                        if let Some(packed_ref) = packed.try_find(full_name)? {
                            return Ok(Some(file::Reference::Packed(packed_ref)));
                        };
                    }
                }
                return Ok(None);
            }
            Some(c) => c,
        };
        Ok(Some({
            let full_name = path_to_name(&relative_path);
            loose::Reference::try_from_path(FullName(full_name), &contents)
                .map(file::Reference::Loose)
                .map_err(|err| Error::ReferenceCreation { err, relative_path })?
        }))
    }
}

impl file::Store {
    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn reference_path(&self, name: &Path) -> PathBuf {
        self.base.join(name)
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
        store::{
            file::{find, loose},
            packed,
        },
        PartialNameRef,
    };

    impl file::Store {
        /// Similar to [`file::Store::find()`] but a non-existing ref is treated as error.
        pub fn find<'a, 'p, 's, Name, E>(
            &'s self,
            partial: Name,
            packed: Option<&'p packed::Buffer>,
        ) -> Result<file::Reference<'p>, Error>
        where
            Name: TryInto<PartialNameRef<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            let path = partial
                .try_into()
                .map_err(|err| Error::Find(find::Error::RefnameValidation(err.into())))?;
            match self.find_one_with_verified_input(path.to_partial_path().as_ref(), packed) {
                Ok(Some(r)) => Ok(r),
                Ok(None) => Err(Error::NotFound(path.to_partial_path().into_owned())),
                Err(err) => Err(err.into()),
            }
        }

        /// Similar to [`file::Store::find()`] won't handle packed-refs.
        pub fn find_loose<'a, Name, E>(&self, partial: Name) -> Result<loose::Reference, Error>
        where
            Name: TryInto<PartialNameRef<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            self.find(partial, None)
                .map(|r| r.try_into().expect("always loose without packed"))
        }
    }

    mod error {
        use std::path::PathBuf;

        use quick_error::quick_error;

        use crate::store::file::find;

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

    use crate::{file, store::packed};

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
            ReadFileContents(err: io::Error) {
                display("The ref file could not be read in full")
                from()
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
        }
    }

    impl From<Infallible> for Error {
        fn from(_: Infallible) -> Self {
            unreachable!("this impl is needed to allow passing a known valid partial path as parameter")
        }
    }
}
