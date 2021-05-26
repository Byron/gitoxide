mod find {
    use crate::{loose, SafeName};
    use quick_error::quick_error;
    use std::convert::TryInto;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd
        }
    }

    impl loose::Store {
        pub fn find_one<'a>(&self, _path: impl TryInto<SafeName<'a>>) -> Result<loose::Reference<'_>, Error> {
            todo!("find one")
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
