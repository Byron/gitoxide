use bstr::{BStr, BString};
use git_features::threading::OwnShared;
use std::borrow::Cow;

use crate::file::SectionId;
use crate::parse::{Event, FrontMatterEvents};
use crate::{
    file::{self, rename_section, SectionMut},
    lookup,
    parse::section,
    File,
};

/// Mutating low-level access methods.
impl<'event> File<'event> {
    /// Returns an mutable section with a given name and optional subsection.
    pub fn section_mut<'a>(
        &'a mut self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&str>,
    ) -> Result<SectionMut<'a, 'event>, lookup::existing::Error> {
        let id = self
            .section_ids_by_name_and_subname(section_name.as_ref(), subsection_name)?
            .rev()
            .next()
            .expect("BUG: Section lookup vec was empty");
        Ok(SectionMut::new(
            self.sections
                .get_mut(&id)
                .expect("BUG: Section did not have id from lookup"),
        ))
    }

    /// Adds a new section. If a subsection name was provided, then
    /// the generated header will use the modern subsection syntax.
    /// Returns a reference to the new section for immediate editing.
    ///
    /// # Examples
    ///
    /// Creating a new empty section:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::convert::TryFrom;
    /// let mut git_config = git_config::File::default();
    /// let _section = git_config.new_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n");
    /// ```
    ///
    /// Creating a new empty section and adding values to it:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::convert::TryFrom;
    /// # use bstr::ByteSlice;
    /// # use git_config::parse::section;
    /// let mut git_config = git_config::File::default();
    /// let mut section = git_config.new_section("hello", Some("world".into()))?;
    /// section.push(section::Key::try_from("a")?, "b");
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n\ta = b\n");
    /// let _section = git_config.new_section("core", None);
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n\ta = b\n[core]\n");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new_section(
        &mut self,
        name: impl Into<Cow<'event, str>>,
        subsection: impl Into<Option<Cow<'event, str>>>,
    ) -> Result<SectionMut<'_, 'event>, section::header::Error> {
        let mut section =
            self.push_section_internal(file::Section::new(name, subsection, OwnShared::clone(&self.meta))?);
        section.push_newline();
        Ok(section)
    }

    /// Removes the section, returning the events it had, if any. If multiple
    /// sections have the same name, then the last one is returned. Note that
    /// later sections with the same name have precedent over earlier ones.
    ///
    /// # Examples
    ///
    /// Creating and removing a section:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::convert::TryFrom;
    /// let mut git_config = git_config::File::try_from(
    /// r#"[hello "world"]
    ///     some-value = 4
    /// "#)?;
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Precedence example for removing sections with the same name:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::convert::TryFrom;
    /// let mut git_config = git_config::File::try_from(
    /// r#"[hello "world"]
    ///     some-value = 4
    /// [hello "world"]
    ///     some-value = 5
    /// "#)?;
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n    some-value = 4\n");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn remove_section<'a>(
        &mut self,
        section_name: &str,
        subsection_name: impl Into<Option<&'a str>>,
    ) -> Option<file::section::Body<'event>> {
        let id = self
            .section_ids_by_name_and_subname(section_name, subsection_name.into())
            .ok()?
            .rev()
            .next()?;
        self.section_order.remove(
            self.section_order
                .iter()
                .position(|v| *v == id)
                .expect("known section id"),
        );
        self.sections.remove(&id).map(|s| s.body)
    }

    /// Adds the provided section to the config, returning a mutable reference
    /// to it for immediate editing.
    /// Note that its meta-data will remain as is.
    pub fn push_section(
        &mut self,
        section: file::Section<'event>,
    ) -> Result<SectionMut<'_, 'event>, section::header::Error> {
        Ok(self.push_section_internal(section))
    }

    /// Renames a section, modifying the last matching section.
    pub fn rename_section<'a>(
        &mut self,
        section_name: impl AsRef<str>,
        subsection_name: impl Into<Option<&'a str>>,
        new_section_name: impl Into<Cow<'event, str>>,
        new_subsection_name: impl Into<Option<Cow<'event, str>>>,
    ) -> Result<(), rename_section::Error> {
        let id = self
            .section_ids_by_name_and_subname(section_name.as_ref(), subsection_name.into())?
            .rev()
            .next()
            .expect("list of sections were empty, which violates invariant");
        let section = self.sections.get_mut(&id).expect("known section-id");
        section.header = section::Header::new(new_section_name, new_subsection_name)?;
        Ok(())
    }

    /// Append another File to the end of ourselves, without loosing any information.
    pub fn append(&mut self, mut other: Self) {
        let nl = self.detect_newline_style();

        fn ends_with_newline<'a>(it: impl DoubleEndedIterator<Item = &'a Event<'a>>) -> bool {
            it.last().map_or(true, |e| e.to_bstring().last() == Some(&b'\n'))
        }
        let newline_event = || Event::Newline(Cow::Owned(nl.clone()));

        fn assure_ends_with_newline<'a, 'b>(
            events: &'b mut FrontMatterEvents<'a>,
            nl: &BStr,
        ) -> &'b mut FrontMatterEvents<'a> {
            if !ends_with_newline(events.iter()) {
                events.push(Event::Newline(nl.to_owned().into()));
            }
            events
        }

        let last_section_id = (self.section_id_counter != 0).then(|| self.section_id_counter - 1);
        let mut last_added_section_id = None;
        for id in std::mem::take(&mut other.section_order) {
            let section = other.sections.remove(&id).expect("present");
            self.push_section_internal(section);

            let new_id = self.section_id_counter - 1;
            last_added_section_id = Some(new_id);
            if let Some(post_matter) = other.frontmatter_post_section.remove(&id) {
                self.frontmatter_post_section.insert(SectionId(new_id), post_matter);
            }
        }

        if !other.frontmatter_events.is_empty() {
            match last_added_section_id {
                Some(id) => {
                    if !ends_with_newline(self.sections[&SectionId(id)].body.0.iter()) {
                        other.frontmatter_events.insert(0, newline_event());
                    }
                }
                None => {
                    if !last_section_id.map_or(true, |id| {
                        ends_with_newline(self.sections[&SectionId(id)].body.0.iter())
                    }) {
                        other.frontmatter_events.insert(0, newline_event());
                    }
                }
            }

            match last_section_id {
                Some(last_id) => assure_ends_with_newline(
                    self.frontmatter_post_section.entry(SectionId(last_id)).or_default(),
                    nl.as_ref(),
                )
                .extend(other.frontmatter_events),
                None => {
                    assure_ends_with_newline(&mut self.frontmatter_events, nl.as_ref()).extend(other.frontmatter_events)
                }
            }
        }
    }

    fn detect_newline_style(&self) -> BString {
        fn extract_newline(e: &Event<'_>) -> Option<BString> {
            match e {
                Event::Newline(b) => b.as_ref().to_owned().into(),
                _ => None,
            }
        }

        self.frontmatter_events
            .iter()
            .find_map(extract_newline)
            .or_else(|| {
                self.sections()
                    .find_map(|s| s.body.as_ref().iter().find_map(extract_newline))
            })
            .unwrap_or_else(|| if cfg!(windows) { "\r\n" } else { "\n" }.into())
    }
}
