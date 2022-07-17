use crate::file::{Metadata, Section, SectionMut};
use crate::parse::section;
use crate::{file, parse};
use bstr::BString;
use std::borrow::Cow;
use std::ops::Deref;

pub(crate) mod body;
pub use body::{Body, BodyIter};
use git_features::threading::OwnShared;

impl<'a> Deref for Section<'a> {
    type Target = Body<'a>;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

/// Instantiation and conversion
impl<'a> Section<'a> {
    /// Create a new section with the given `name` and optional, `subsection`, `meta`-data and an empty body.
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        subsection: impl Into<Option<Cow<'a, str>>>,
        meta: impl Into<OwnShared<file::Metadata>>,
    ) -> Result<Self, parse::section::header::Error> {
        Ok(Section {
            header: parse::section::Header::new(name, subsection)?,
            body: Default::default(),
            meta: meta.into(),
        })
    }
}

/// Access
impl<'a> Section<'a> {
    /// Return our header.
    pub fn header(&self) -> &section::Header<'a> {
        &self.header
    }

    /// Return our body, containing all keys and values.
    pub fn body(&self) -> &Body<'a> {
        &self.body
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

    /// Stream ourselves to the given `out`, in order to reproduce this section mostly losslessly
    /// as it was parsed.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        self.header.write_to(&mut out)?;
        for event in self.body.as_ref() {
            event.write_to(&mut out)?;
        }
        Ok(())
    }

    /// Return additional information about this sections origin.
    pub fn meta(&self) -> &Metadata {
        &self.meta
    }

    /// Returns a mutable version of this section for adjustment of values.
    pub fn to_mut(&mut self) -> SectionMut<'_, 'a> {
        SectionMut::new(self)
    }
}
