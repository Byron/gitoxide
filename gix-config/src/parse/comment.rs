use std::{borrow::Cow, fmt::Display};

use bstr::BString;

use crate::parse::Comment;

impl Comment<'_> {
    /// Turn this instance into a fully owned one with `'static` lifetime.
    #[must_use]
    pub fn to_owned(&self) -> Comment<'static> {
        Comment {
            tag: self.tag,
            text: Cow::Owned(self.text.as_ref().into()),
        }
    }

    /// Serialize this type into a `BString` for convenience.
    ///
    /// Note that `to_string()` can also be used, but might not be lossless.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        let mut buf = Vec::new();
        self.write_to(&mut buf).expect("io error impossible");
        buf.into()
    }

    /// Stream ourselves to the given `out`, in order to reproduce this comment losslessly.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        out.write_all(&[self.tag])?;
        out.write_all(self.text.as_ref())
    }
}

impl Display for Comment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_bstring(), f)
    }
}

impl From<Comment<'_>> for BString {
    fn from(c: Comment<'_>) -> Self {
        c.into()
    }
}

impl From<&Comment<'_>> for BString {
    fn from(c: &Comment<'_>) -> Self {
        c.to_bstring()
    }
}
