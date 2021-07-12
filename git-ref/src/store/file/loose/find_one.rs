use crate::{file, PartialName};
use std::{
    convert::TryInto,
    io::{self, Read},
    path::{Path, PathBuf},
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
    /// The lookup algorithm follows the one in [the git documentation][git-lookup-docs].
    ///
    /// [git-lookup-docs]: https://github.com/git/git/blob/5d5b1473453400224ebb126bf3947e0a3276bdf5/Documentation/revisions.txt#L34-L46
    pub fn find_one<'a, Name, E>(&self, partial: Name) -> Result<Option<file::Reference<'_>>, Error>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
        Error: From<E>,
    {
        let path = partial.try_into()?;
        self.find_one_with_verified_input(path.to_partial_path().as_ref())
    }

    pub(in crate::store::file) fn find_one_with_verified_input(
        &self,
        relative_path: &Path,
    ) -> Result<Option<file::Reference<'_>>, Error> {
        let is_all_uppercase = relative_path
            .to_string_lossy()
            .as_ref()
            .chars()
            .all(|c| c.is_ascii_uppercase());
        if relative_path.components().count() == 1 && is_all_uppercase {
            if let Some(r) = self.find_inner("", &relative_path, Transform::None)? {
                return Ok(Some(r));
            }
        }

        for inbetween in &["", "tags", "heads", "remotes"] {
            match self.find_inner(*inbetween, &relative_path, Transform::EnforceRefsPrefix) {
                Ok(Some(r)) => return Ok(Some(r)),
                Ok(None) => continue,
                Err(err) => return Err(err),
            }
        }
        self.find_inner("remotes", &relative_path.join("HEAD"), Transform::EnforceRefsPrefix)
    }

    fn find_inner(
        &self,
        inbetween: &str,
        relative_path: &Path,
        transform: Transform,
    ) -> Result<Option<file::Reference<'_>>, Error> {
        let relative_path = match transform {
            Transform::EnforceRefsPrefix => {
                if relative_path.starts_with("refs") {
                    PathBuf::new()
                } else {
                    PathBuf::from("refs")
                }
            }
            Transform::None => PathBuf::new(),
        }
        .join(inbetween)
        .join(relative_path);

        let contents = match self.ref_contents(&relative_path)? {
            None => return Ok(None),
            Some(c) => c,
        };
        Ok(Some(
            file::Reference::try_from_path(self, &relative_path, &contents, self.hash)
                .map_err(|err| Error::ReferenceCreation { err, relative_path })?,
        ))
    }
}

impl file::Store {
    /// Implements the logic required to transform a fully qualified refname into a filesystem path
    pub(crate) fn ref_path(&self, name: &Path) -> PathBuf {
        self.base.join(name)
    }

    /// Read the file contents with a verified full reference path and return it in the given vector if possible.
    pub(crate) fn ref_contents(&self, relative_path: &Path) -> std::io::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();
        let ref_path = self.ref_path(&relative_path);

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
    use crate::{file, file::find_one, PartialName};
    use std::convert::TryInto;

    impl file::Store {
        /// Similar to [`file::Store::find_one()`] but a non-existing ref is treated as error.
        pub fn find_one_existing<'a, Name, E>(&self, partial: Name) -> Result<file::Reference<'_>, Error>
        where
            Name: TryInto<PartialName<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            let path = partial
                .try_into()
                .map_err(|err| Error::Find(find_one::Error::RefnameValidation(err.into())))?;
            match self.find_one_with_verified_input(path.to_partial_path().as_ref()) {
                Ok(Some(r)) => Ok(r),
                Ok(None) => Err(Error::NotFound(path.to_partial_path().into_owned())),
                Err(err) => Err(err.into()),
            }
        }
    }

    mod error {
        use crate::file::find_one;
        use quick_error::quick_error;
        use std::path::PathBuf;

        quick_error! {
            /// The error returned by [file::Store::find_one_existing()][crate::file::Store::find_one_existing()].
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Find(err: find_one::Error) {
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
    pub use error::Error;
}

mod error {
    use crate::file;
    use quick_error::quick_error;
    use std::{
        convert::Infallible,
        io::{self},
        path::PathBuf,
    };

    quick_error! {
        /// The error returned by [file::Store::find_one()].
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
            ReferenceCreation{ err: file::reference::decode::Error, relative_path: PathBuf } {
                display("The reference at '{}' could not be instantiated", relative_path.display())
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
pub use error::Error;
