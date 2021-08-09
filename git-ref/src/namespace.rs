#![allow(missing_docs)]

use crate::PartialName;
use bstr::{BString, ByteSlice, ByteVec};
use std::convert::TryInto;

pub fn expand<'a, Name, E>(namespace: Name) -> Result<crate::mutable::FullName, expand::Error>
where
    Name: TryInto<PartialName<'a>, Error = E>,
    expand::Error: From<E>,
{
    let namespace = namespace.try_into()?.0;
    let mut out = BString::default();
    for component in namespace.split_str(b"/") {
        out.push_str("refs/namespaces/");
        out.push_str(component);
        out.push_str(b"/");
    }
    out.pop();
    debug_assert!(
        git_validate::reference::name(out.as_ref()).is_ok(),
        "we always produce valid ref names"
    );
    Ok(crate::mutable::FullName(out))
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
