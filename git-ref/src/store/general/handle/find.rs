use std::convert::TryInto;

use crate::{store, PartialNameRef, Reference};

mod error {
    use std::convert::Infallible;

    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [crate::Store::find()].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Loose(err: crate::file::find::Error) {
                display("An error occurred while finding a reference in the loose file database")
                from()
                source(err)
            }
            RefnameValidation(err: crate::name::Error) {
                display("The ref name or path is not a valid ref name")
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

pub use error::Error;

use crate::store::handle;

impl store::Handle {
    /// TODO: actually implement this with handling of the packed buffer.
    pub fn try_find<'a, Name, E>(&self, partial: Name) -> Result<Option<Reference>, Error>
    where
        Name: TryInto<PartialNameRef<'a>, Error = E>,
        Error: From<E>,
    {
        let _name = partial.try_into()?;
        match &self.state {
            handle::State::Loose { store: _, .. } => {
                todo!()
            }
        }
    }
}

mod existing {
    mod error {
        use std::path::PathBuf;

        use quick_error::quick_error;

        quick_error! {
            /// The error returned by [file::Store::find_existing()][crate::file::Store::find_existing()].
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Find(err: crate::store::find::Error) {
                    display("An error occurred while finding a reference in the database")
                    from()
                    source(err)
                }
                NotFound(name: PathBuf) {
                    display("The ref partially named '{}' could not be found", name.display())
                }
            }
        }
    }

    use std::convert::TryInto;

    pub use error::Error;

    use crate::{store, PartialNameRef, Reference};

    impl store::Handle {
        /// Similar to [`crate::file::Store::find()`] but a non-existing ref is treated as error.
        pub fn find<'a, Name, E>(&self, _partial: Name) -> Result<Reference, Error>
        where
            Name: TryInto<PartialNameRef<'a>, Error = E>,
            crate::name::Error: From<E>,
        {
            todo!()
            // match self.try_find(partial) {}
            // match self.find_one_with_verified_input(path.to_partial_path().as_ref(), packed) {
            //     Ok(Some(r)) => Ok(r),
            //     Ok(None) => Err(Error::NotFound(path.to_partial_path().into_owned())),
            //     Err(err) => Err(err.into()),
            // }
        }
    }
}
