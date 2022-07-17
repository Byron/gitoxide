use bstr::BStr;
use git_features::threading::OwnShared;
use std::borrow::Cow;

use crate::file::{MetadataFilter, SectionId};
use crate::parse::{Event, FrontMatterEvents};
use crate::{
    file::{self, rename_section, SectionMut},
    lookup,
    parse::section,
    File,
};

/// Mutating low-level access methods.
impl<'event> File<'event> {
    /// Returns an mutable section with a given `name` and optional `subsection_name`.
    pub fn section_mut<'a>(
        &'a mut self,
        name: impl AsRef<str>,
        subsection_name: Option<&str>,
    ) -> Result<SectionMut<'a, 'event>, lookup::existing::Error> {
        let id = self
            .section_ids_by_name_and_subname(name.as_ref(), subsection_name)?
            .rev()
            .next()
            .expect("BUG: Section lookup vec was empty");
        Ok(SectionMut::new(
            self.sections
                .get_mut(&id)
                .expect("BUG: Section did not have id from lookup"),
        ))
    }

    /// Returns the last found mutable section with a given `name` and optional `subsection_name`, that matches `filter`.
    ///
    /// If there are sections matching `section_name` and `subsection_name` but the `filter` rejects all of them, `Ok(None)`
    /// is returned.
    pub fn section_mut_filter<'a>(
        &'a mut self,
        name: impl AsRef<str>,
        subsection_name: Option<&str>,
        filter: &mut MetadataFilter,
    ) -> Result<Option<file::SectionMut<'a, 'event>>, lookup::existing::Error> {
        let id = self
            .section_ids_by_name_and_subname(name.as_ref(), subsection_name)?
            .rev()
            .find(|id| {
                let s = &self.sections[&id];
                filter(s.meta())
            });
        Ok(id.and_then(move |id| self.sections.get_mut(&id).map(|s| s.to_mut())))
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

    /// Removes the section with `name` and `subsection_name` , returning it if there was a matching section.
    /// If multiple sections have the same name, then the last one is returned. Note that
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
    /// let section = git_config.remove_section("hello", Some("world".into()));
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
    /// let section = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n    some-value = 4\n");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn remove_section<'a>(
        &mut self,
        name: &str,
        subsection_name: impl Into<Option<&'a str>>,
    ) -> Option<file::Section<'event>> {
        let id = self
            .section_ids_by_name_and_subname(name, subsection_name.into())
            .ok()?
            .rev()
            .next()?;
        self.section_order.remove(
            self.section_order
                .iter()
                .position(|v| *v == id)
                .expect("known section id"),
        );
        self.sections.remove(&id)
    }

    /// Removes the section with `name` and `subsection_name` that passed `filter`, returning the removed section
    /// if at least one section matched the `filter`.
    /// If multiple sections have the same name, then the last one is returned. Note that
    /// later sections with the same name have precedent over earlier ones.
    pub fn remove_section_filter<'a>(
        &mut self,
        name: &str,
        subsection_name: impl Into<Option<&'a str>>,
        filter: &mut MetadataFilter,
    ) -> Option<file::Section<'event>> {
        let id = self
            .section_ids_by_name_and_subname(name, subsection_name.into())
            .ok()?
            .rev()
            .find(|id| filter(self.sections.get(&id).expect("each id has a section").meta()))?;
        self.section_order.remove(
            self.section_order
                .iter()
                .position(|v| *v == id)
                .expect("known section id"),
        );
        self.sections.remove(&id)
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

    /// Renames the section with `name` and `subsection_name`, modifying the last matching section
    /// to use `new_name` and `new_subsection_name`.
    pub fn rename_section<'a>(
        &mut self,
        name: impl AsRef<str>,
        subsection_name: impl Into<Option<&'a str>>,
        new_name: impl Into<Cow<'event, str>>,
        new_subsection_name: impl Into<Option<Cow<'event, str>>>,
    ) -> Result<(), rename_section::Error> {
        let id = self
            .section_ids_by_name_and_subname(name.as_ref(), subsection_name.into())?
            .rev()
            .next()
            .expect("list of sections were empty, which violates invariant");
        let section = self.sections.get_mut(&id).expect("known section-id");
        section.header = section::Header::new(new_name, new_subsection_name)?;
        Ok(())
    }

    /// Renames the section with `name` and `subsection_name`, modifying the last matching section
    /// that also passes `filter` to use `new_name` and `new_subsection_name`.
    ///
    /// Note that the otherwise unused [`lookup::existing::Error::KeyMissing`] variant is used to indicate
    /// that the `filter` rejected all candidates, leading to no section being renamed after all.
    pub fn rename_section_filter<'a>(
        &mut self,
        name: impl AsRef<str>,
        subsection_name: impl Into<Option<&'a str>>,
        new_name: impl Into<Cow<'event, str>>,
        new_subsection_name: impl Into<Option<Cow<'event, str>>>,
        filter: &mut MetadataFilter,
    ) -> Result<(), rename_section::Error> {
        let id = self
            .section_ids_by_name_and_subname(name.as_ref(), subsection_name.into())?
            .rev()
            .find(|id| filter(self.sections.get(&id).expect("each id has a section").meta()))
            .ok_or(rename_section::Error::Lookup(lookup::existing::Error::KeyMissing))?;
        let section = self.sections.get_mut(&id).expect("known section-id");
        section.header = section::Header::new(new_name, new_subsection_name)?;
        Ok(())
    }

    /// Append another File to the end of ourselves, without loosing any information.
    pub fn append(&mut self, mut other: Self) -> &mut Self {
        let nl = self.detect_newline_style().to_owned();

        fn ends_with_newline<'a>(it: impl DoubleEndedIterator<Item = &'a Event<'a>>) -> bool {
            it.last().map_or(true, |e| e.to_bstr_lossy().last() == Some(&b'\n'))
        }
        fn starts_with_newline<'a>(mut it: impl Iterator<Item = &'a Event<'a>>) -> bool {
            it.next().map_or(true, |e| e.to_bstr_lossy().first() == Some(&b'\n'))
        }
        let newline_event = || Event::Newline(Cow::Owned(nl.clone()));

        fn assure_ends_with_newline_if<'a, 'b>(
            needs_nl: bool,
            events: &'b mut FrontMatterEvents<'a>,
            nl: &BStr,
        ) -> &'b mut FrontMatterEvents<'a> {
            if needs_nl && !ends_with_newline(events.iter()) {
                events.push(Event::Newline(nl.to_owned().into()));
            }
            events
        }

        let our_last_section_before_append =
            (self.section_id_counter != 0).then(|| SectionId(self.section_id_counter - 1));
        let mut last_added_section_id = None;

        for id in std::mem::take(&mut other.section_order) {
            let section = other.sections.remove(&id).expect("present");
            self.push_section_internal(section);

            let new_id = self.section_id_counter - 1;
            last_added_section_id = Some(SectionId(new_id));
            if let Some(post_matter) = other.frontmatter_post_section.remove(&id) {
                self.frontmatter_post_section.insert(SectionId(new_id), post_matter);
            }
        }

        if other.frontmatter_events.is_empty() {
            return self;
        }

        let mut needs_nl = !starts_with_newline(other.frontmatter_events.iter());
        if let Some(id) = last_added_section_id
            .or(our_last_section_before_append)
            .filter(|_| needs_nl)
        {
            if !ends_with_newline(self.sections[&id].body.0.iter()) {
                other.frontmatter_events.insert(0, newline_event());
                needs_nl = false;
            }
        }

        match our_last_section_before_append {
            Some(last_id) => assure_ends_with_newline_if(
                needs_nl,
                self.frontmatter_post_section.entry(last_id).or_default(),
                nl.as_ref(),
            )
            .extend(other.frontmatter_events),
            None => assure_ends_with_newline_if(needs_nl, &mut self.frontmatter_events, nl.as_ref())
                .extend(other.frontmatter_events),
        }
        self
    }

    fn detect_newline_style(&self) -> &BStr {
        fn extract_newline<'a, 'b>(e: &'a Event<'b>) -> Option<&'a BStr> {
            match e {
                Event::Newline(b) => b.as_ref().into(),
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
