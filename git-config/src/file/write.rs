use bstr::{BString, ByteSlice};

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
        let nl = self.detect_newline_style();

        let ends_with_newline = |e: &[crate::parse::Event<'_>]| -> bool {
            if e.is_empty() {
                return true;
            }
            e.iter()
                .rev()
                .take_while(|e| e.to_bstr_lossy().iter().all(|b| b.is_ascii_whitespace()))
                .find_map(|e| e.to_bstr_lossy().contains_str(nl).then(|| true))
                .unwrap_or(false)
        };

        {
            for event in self.frontmatter_events.as_ref() {
                event.write_to(&mut out)?;
            }

            if !ends_with_newline(self.frontmatter_events.as_ref()) && self.sections.iter().next().is_some() {
                out.write_all(&nl)?;
            }
        }

        let mut prev_section_ended_with_newline = true;
        for section_id in &self.section_order {
            if !prev_section_ended_with_newline {
                out.write_all(&nl)?;
            }
            let section = self.sections.get(section_id).expect("known section-id");
            section.write_to(&mut out)?;

            prev_section_ended_with_newline = ends_with_newline(section.body.0.as_ref());
            if let Some(post_matter) = self.frontmatter_post_section.get(section_id) {
                if !prev_section_ended_with_newline {
                    out.write_all(&nl)?;
                }
                for event in post_matter {
                    event.write_to(&mut out)?;
                }
                prev_section_ended_with_newline = ends_with_newline(post_matter);
            }
        }

        if !prev_section_ended_with_newline {
            out.write_all(&nl)?;
        }

        Ok(())
    }
}
