#![allow(missing_docs)]

use crate::PartialName;
use std::convert::TryInto;

pub fn expand<'a, Name, E>(namespace: Name) -> Result<crate::mutable::FullName, expand::Error>
where
    Name: TryInto<PartialName<'a>, Error = E>,
    expand::Error: From<E>,
{
    let _namespace = namespace.try_into()?;
    todo!("impl")
}

pub mod expand {
    use quick_error::quick_error;
    use std::convert::Infallible;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
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
