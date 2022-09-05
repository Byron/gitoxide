use std::borrow::Cow;

use bstr::{BStr, ByteSlice};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn b(s: &str) -> &bstr::BStr {
    s.into()
}

pub fn cow_str(s: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(s.as_bytes().as_bstr())
}

mod boolean;
mod color;
mod integer;
mod path;
