#![forbid(unsafe_code)]

use bstr::{BStr, BString, ByteSlice};

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

pub mod borrowed;
pub mod owned;

mod types;
pub use types::*;

pub mod commit {
    use crate::borrowed;
    use bstr::{BStr, ByteSlice};

    pub struct ExtraHeaders<I> {
        inner: I,
    }

    impl<'a, I> ExtraHeaders<I>
    where
        I: Iterator<Item = (&'a BStr, &'a BStr)>,
    {
        pub fn new(iter: I) -> Self {
            ExtraHeaders { inner: iter }
        }
        pub fn find(mut self, name: &str) -> Option<&'a BStr> {
            self.inner
                .find_map(move |(k, v)| if k == name.as_bytes().as_bstr() { Some(v) } else { None })
        }
        pub fn find_all(self, name: &'a str) -> impl Iterator<Item = &'a BStr> {
            self.inner
                .filter_map(move |(k, v)| if k == name.as_bytes().as_bstr() { Some(v) } else { None })
        }
        pub fn mergetags(self) -> impl Iterator<Item = Result<borrowed::Tag<'a>, borrowed::Error>> {
            self.find_all("mergetag").map(|b| borrowed::Tag::from_bytes(b))
        }
        pub fn pgp_signature(self) -> Option<&'a BStr> {
            self.find("gpgsig")
        }
    }
}
