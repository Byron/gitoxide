use crate::parse::section::{into_cow_bstr, Header, Name};
use crate::parse::Event;
use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::borrow::Cow;
use std::fmt::Display;

/// The error returned by [`Header::new(…)`][super::Header::new()].
#[derive(Debug, PartialOrd, PartialEq, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("section names can only be ascii, '-'")]
    InvalidName,
}

impl<'a> Header<'a> {
    /// Instantiate a new header either with a section `name`, e.g. "core" serializing to `["core"]`
    /// or `[remote "origin"]` for `subsection` being "origin" and `name` being "remote".
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        subsection: impl Into<Option<Cow<'a, str>>>,
    ) -> Result<Header<'a>, Error> {
        let name = Name(validated_name(into_cow_bstr(name.into()))?);
        if let Some(subsection_name) = subsection.into() {
            Ok(Header {
                name,
                separator: Some(Cow::Borrowed(" ".into())),
                subsection_name: Some(into_cow_bstr(subsection_name)),
            })
        } else {
            Ok(Header {
                name,
                separator: None,
                subsection_name: None,
            })
        }
    }
}

fn validated_name(name: Cow<'_, BStr>) -> Result<Cow<'_, BStr>, Error> {
    name.iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'-')
        .then(|| name)
        .ok_or(Error::InvalidName)
}

impl Header<'_> {
    ///Return true if this is a header like `[legacy.subsection]`, or false otherwise.
    pub fn is_legacy(&self) -> bool {
        self.separator.as_deref().map_or(false, |n| n == ".")
    }

    /// Return the subsection name, if present, i.e. "origin" in `[remote "origin"]`.
    pub fn subsection_name(&self) -> Option<&BStr> {
        self.subsection_name.as_deref()
    }

    /// Return the name of the header, like "remote" in `[remote "origin"]`.
    pub fn name(&self) -> &BStr {
        self.name.as_ref().as_bstr()
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

    /// Stream ourselves to the given `out`, in order to reproduce this header mostly losslessly
    /// as it was parsed.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        out.write_all(b"[")?;
        out.write_all(self.name.as_ref())?;

        if let (Some(sep), Some(subsection)) = (&self.separator, &self.subsection_name) {
            let sep = sep.as_ref();
            out.write_all(sep)?;
            if sep == "." {
                out.write_all(subsection.as_ref())?;
            } else {
                out.write_all(&[b'"'])?;
                out.write_all(escape_subsection(subsection.as_ref()).as_ref())?;
                out.write_all(&[b'"'])?;
            }
        }

        out.write_all(b"]")
    }

    /// Turn this instance into a fully owned one with `'static` lifetime.
    #[must_use]
    pub fn to_owned(&self) -> Header<'static> {
        Header {
            name: self.name.to_owned(),
            separator: self.separator.clone().map(|v| Cow::Owned(v.into_owned())),
            subsection_name: self.subsection_name.clone().map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

fn escape_subsection(name: &BStr) -> Cow<'_, BStr> {
    if name.find_byteset(b"\\\"").is_none() {
        return name.into();
    }
    let mut buf = Vec::with_capacity(name.len());
    for b in name.iter().copied() {
        match b {
            b'\\' => buf.push_str(br#"\\"#),
            b'"' => buf.push_str(br#"\""#),
            _ => buf.push(b),
        }
    }
    BString::from(buf).into()
}

impl Display for Header<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_bstring(), f)
    }
}

impl From<Header<'_>> for BString {
    fn from(header: Header<'_>) -> Self {
        header.into()
    }
}

impl From<&Header<'_>> for BString {
    fn from(header: &Header<'_>) -> Self {
        header.to_bstring()
    }
}

impl<'a> From<Header<'a>> for Event<'a> {
    fn from(header: Header<'_>) -> Event<'_> {
        Event::SectionHeader(header)
    }
}
