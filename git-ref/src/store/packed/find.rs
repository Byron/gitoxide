use crate::{store::packed, PartialName};
use std::convert::TryInto;

/// packed-refs specific functionality
impl packed::Buffer {
    /// Find a reference with the given `name` and return it.
    pub fn find<'a, Name, E>(&self, name: Name) -> Result<Option<packed::Reference<'_>>, E>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
    {
        let _name = name.try_into()?;
        // (0..self.as_ref().len()).binary_search();
        todo!("actual signature and impl")
    }

    /// Find a reference with the given `name` and return it.
    pub fn find_existing<'a, Name, E>(&self, name: Name) -> Result<packed::Reference<'_>, existing::Error>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
        existing::Error: From<E>,
    {
        let _name = name.try_into()?;
        todo!("actual signature and impl")
    }
}

///
pub mod existing {
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [`find_existing()`][super::packed::Buffer::find_existing()]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            RefnameValidation(err: crate::name::Error) {
                display("The ref name or path is not a valid ref name")
                from()
                source(err)
            }
            NotFound {
                display("The reference did not exist even though that was expected")
            }
        }
    }
}
