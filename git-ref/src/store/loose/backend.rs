mod find {
    use crate::{loose, SafeName};
    use quick_error::quick_error;
    use std::convert::TryInto;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            RefnameValidation(err: crate::safe_name::Error) {
                display("The input name or path is not a valid ref name")
                source(err)
            }
        }
    }

    impl loose::Store {
        pub fn find_one<'a, Name>(&self, path: Name) -> Result<loose::Reference<'_>, Error>
        where
            Name: TryInto<SafeName<'a>, Error = crate::safe_name::Error>,
        {
            let path = path.try_into().map_err(|err| Error::RefnameValidation(err))?;
            let ref_path = self.base.join(path.to_path());
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
