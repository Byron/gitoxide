use bstr::BString;

use crate::File;

impl File<'_> {
    /// Serialize this type into a `BString` for convenience.
    ///
    /// Note that `to_string()` can also be used, but might not be lossless.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        let mut buf = Vec::new();
        self.write_to(&mut buf).expect("io error impossible");
        buf.into()
    }

    /// Stream ourselves to the given `out`, in order to reproduce this file mostly losslessly
    /// as it was parsed.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        for event in self.frontmatter_events.as_ref() {
            event.write_to(&mut out)?;
        }

        for section_id in &self.section_order {
            self.sections
                .get(section_id)
                .expect("known section-id")
                .write_to(&mut out)?;
        }

        Ok(())
    }
}
