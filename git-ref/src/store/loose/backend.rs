mod find {
    use crate::{loose, SafeName};
    use quick_error::quick_error;
    use std::io::Read;
    use std::{convert::TryInto, io};

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
        }
    }

    impl loose::Store {
        pub fn find_one<'a, Name>(&self, path: Name) -> Result<Option<loose::Reference<'_>>, Error>
        where
            Name: TryInto<SafeName<'a>, Error = crate::safe_name::Error>,
        {
            let path = path.try_into().map_err(|err| Error::RefnameValidation(err))?;

            let relative_path = path.to_path();
            let ref_path = self.base.join(relative_path);
            let mut contents = Vec::new();
            match std::fs::File::open(ref_path) {
                Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(err) => return Err(err.into()),
                Ok(mut file) => file.read_to_end(&mut contents)?,
            };
            todo!("impl")
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
