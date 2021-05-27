mod find {
    use crate::{loose, SafePartialName};
    use quick_error::quick_error;
    use std::path::Path;
    use std::{convert::TryInto, io, io::Read, path::PathBuf};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            RefnameValidation(err: crate::safe_name::Error) {
                display("The input name or path is not a valid ref name")
                source(err)
            }
            ReadFileContents(err: io::Error) {
                display("The ref file could not be read in full")
                from()
                source(err)
            }
            ReferenceCreation{ err: loose::reference::decode::Error, relative_path: PathBuf } {
                display("The reference at '{}' could not be instantiated", relative_path.display())
                source(err)
            }
        }
    }

    enum Transform {
        EnforceRefsPrefix,
        None,
    }

    impl loose::Store {
        /// As per [the git documentation][git-lookup-docs]
        ///
        /// [git-lookup-docs]: https://github.com/git/git/blob/5d5b1473453400224ebb126bf3947e0a3276bdf5/Documentation/revisions.txt#L34-L46
        pub fn find_one<'a, Name>(&self, path: Name) -> Result<Option<loose::Reference<'_>>, Error>
        where
            Name: TryInto<SafePartialName<'a>, Error = crate::safe_name::Error>,
        {
            let path = path.try_into().map_err(Error::RefnameValidation)?;

            let relative_path = path.to_path();
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
            todo!("check remotes HEAD")
        }

        fn find_inner(
            &self,
            inbetween: &str,
            relative_path: &Path,
            transform: Transform,
        ) -> Result<Option<loose::Reference<'_>>, Error> {
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
            let ref_path = self.base.join(&relative_path);
            let mut contents = Vec::new();
            match std::fs::File::open(ref_path) {
                Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(err) => return Err(err.into()),
                Ok(mut file) => file.read_to_end(&mut contents)?,
            };
            Ok(Some(
                loose::Reference::try_from_path(self, &relative_path, &contents)
                    .map_err(|err| Error::ReferenceCreation { err, relative_path })?,
            ))
        }
    }
}

mod init {
    use crate::loose;
    use std::path::PathBuf;

    impl loose::Store {
        pub fn new(path: impl Into<PathBuf>) -> Self {
            loose::Store { base: path.into() }
        }
    }

    impl<P> From<P> for loose::Store
    where
        P: Into<PathBuf>,
    {
        fn from(path: P) -> Self {
            loose::Store::new(path)
        }
    }
}
