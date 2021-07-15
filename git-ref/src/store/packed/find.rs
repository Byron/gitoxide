use crate::{store::packed, PartialName};
use std::convert::TryInto;

/// packed-refs specific functionality
impl packed::Buffer {
    /// Find a reference with the given `name` and return it.
    pub fn find<'a, Name, E>(&self, name: Name) -> Result<Option<packed::Reference<'_>>, Error>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
        Error: From<E>,
    {
        let name = name.try_into()?;
        match self.binary_search_by(name) {
            Ok(line_start) => Ok(Some(
                packed::decode::reference::<()>(&self.as_ref()[line_start..])
                    .map_err(|_| Error::Parse)?
                    .1,
            )),
            Err(_line_to_insert) => Ok(None),
        }
    }

    /// Find a reference with the given `name` and return it.
    pub fn find_existing<'a, Name, E>(&self, name: Name) -> Result<packed::Reference<'_>, existing::Error>
    where
        Name: TryInto<PartialName<'a>, Error = E>,
        Error: From<E>,
    {
        match self.find(name) {
            Ok(Some(r)) => Ok(r),
            Ok(None) => Err(existing::Error::NotFound),
            Err(err) => Err(existing::Error::Find(err)),
        }
    }

    /// Perform a binary search where `Ok(pos)` is the beginning of the line that matches `name` perfectly and `Err(pos)`
    /// is the beginning of the line at which `name` could be inserted to still be in sort order.
    fn binary_search_by(&self, name: PartialName<'_>) -> Result<usize, usize> {
        let a = self.as_ref();
        let search_start_of_record = |ofs: usize| {
            a[..ofs]
                .rfind(b"\n")
                .and_then(|pos| {
                    let candidate = pos + 1;
                    if a[candidate] == b'^' {
                        a[..pos].rfind(b"\n").map(|pos| pos + 1)
                    } else {
                        Some(candidate)
                    }
                })
                .unwrap_or(0)
        };
        a.binary_search_by_key(&name.0.as_ref(), |b: &u8| {
            let ofs = b as *const u8 as usize - a.as_ptr() as usize;
            packed::decode::reference::<()>(&a[search_start_of_record(ofs)..])
                .map(|(_rest, r)| r.full_name.as_ref())
                .unwrap_or(&[])
        })
        .map(search_start_of_record)
        .map_err(search_start_of_record)
    }
}

mod error {
    use quick_error::quick_error;
    quick_error! {
        /// The error returned by [`find()`][super::packed::Buffer::find()]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            RefnameValidation(err: crate::name::Error) {
                display("The ref name or path is not a valid ref name")
                from()
                source(err)
            }
            Parse {
                display("The reference could not be parsed")
            }
        }
    }
}
use bstr::ByteSlice;
pub use error::Error;

///
pub mod existing {
    use quick_error::quick_error;

    quick_error! {
        /// The error returned by [`find_existing()`][super::packed::Buffer::find_existing()]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Find(err: super::Error) {
                display("The find operation failed")
                from()
                source(err)
            }
            NotFound {
                display("The reference did not exist even though that was expected")
            }
        }
    }
}
