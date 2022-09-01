use std::borrow::Cow;

use git_features::threading::OwnShared;

use crate::{
    file::{self, rename_section, write::ends_with_newline, MetadataFilter, SectionId, SectionMut},
    lookup,
    parse::{section, Event, FrontMatterEvents},
    File,
};

/// Mutating low-level access methods.
impl<'event> File<'event> {
    /// Returns an mutable section with a given `name` and optional `subsection_name`, _if it exists_.
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
        let nl = self.detect_newline_style_smallvec();
        Ok(self
            .sections
            .get_mut(&id)
            .expect("BUG: Section did not have id from lookup")
            .to_mut(nl))
    }
    /// Returns an mutable section with a given `name` and optional `subsection_name`, _if it exists_, or create a new section.
    pub fn section_mut_or_create_new<'a>(
        &'a mut self,
        name: impl AsRef<str>,
        subsection_name: Option<&str>,
    ) -> Result<SectionMut<'a, 'event>, section::header::Error> {
        self.section_mut_or_create_new_filter(name, subsection_name, &mut |_| true)
    }

    /// Returns an mutable section with a given `name` and optional `subsection_name`, _if it exists_ **and** passes `filter`, or create
    /// a new section.
    pub fn section_mut_or_create_new_filter<'a>(
        &'a mut self,
        name: impl AsRef<str>,
        subsection_name: Option<&str>,
        filter: &mut MetadataFilter,
    ) -> Result<SectionMut<'a, 'event>, section::header::Error> {
        let name = name.as_ref();
        match self
            .section_ids_by_name_and_subname(name.as_ref(), subsection_name)
            .ok()
            .and_then(|it| {
                it.rev().find(|id| {
                    let s = &self.sections[id];
                    filter(s.meta())
                })
            }) {
            Some(id) => {
                let nl = self.detect_newline_style_smallvec();
                Ok(self
                    .sections
                    .get_mut(&id)
                    .expect("BUG: Section did not have id from lookup")
                    .to_mut(nl))
            }
            None => self.new_section(name.to_owned(), subsection_name.map(|n| Cow::Owned(n.to_owned()))),
        }
    }

    /// Returns the last found mutable section with a given `name` and optional `subsection_name`, that matches `filter`, _if it exists_.
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
                let s = &self.sections[id];
                filter(s.meta())
            });
        let nl = self.detect_newline_style_smallvec();
        Ok(id.and_then(move |id| self.sections.get_mut(&id).map(move |s| s.to_mut(nl))))
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
    /// let section = git_config.new_section("hello", Some("world".into()))?;
    /// let nl = section.newline().to_owned();
    /// assert_eq!(git_config.to_string(), format!("[hello \"world\"]{nl}"));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// section.push(section::Key::try_from("a")?, Some("b".into()));
    /// let nl = section.newline().to_owned();
    /// assert_eq!(git_config.to_string(), format!("[hello \"world\"]{nl}\ta = b{nl}"));
    /// let _section = git_config.new_section("core", None);
    /// assert_eq!(git_config.to_string(), format!("[hello \"world\"]{nl}\ta = b{nl}[core]{nl}"));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new_section(
        &mut self,
        name: impl Into<Cow<'event, str>>,
        subsection: impl Into<Option<Cow<'event, str>>>,
    ) -> Result<SectionMut<'_, 'event>, section::header::Error> {
        let id = self.push_section_internal(file::Section::new(name, subsection, OwnShared::clone(&self.meta))?);
        let nl = self.detect_newline_style_smallvec();
        let mut section = self.sections.get_mut(&id).expect("each id yields a section").to_mut(nl);
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
            .find(|id| filter(self.sections.get(id).expect("each id has a section").meta()))?;
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
        let id = self.push_section_internal(section);
        let nl = self.detect_newline_style_smallvec();
        let section = self.sections.get_mut(&id).expect("each id yields a section").to_mut(nl);
        Ok(section)
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
            .find(|id| filter(self.sections.get(id).expect("each id has a section").meta()))
            .ok_or(rename_section::Error::Lookup(lookup::existing::Error::KeyMissing))?;
        let section = self.sections.get_mut(&id).expect("known section-id");
        section.header = section::Header::new(new_name, new_subsection_name)?;
        Ok(())
    }

    /// Append another File to the end of ourselves, without losing any information.
    pub fn append(&mut self, other: Self) -> &mut Self {
        self.append_or_insert(other, None)
    }

    /// Append another File to the end of ourselves, without losing any information.
    pub(crate) fn append_or_insert(&mut self, mut other: Self, mut insert_after: Option<SectionId>) -> &mut Self {
        let nl = self.detect_newline_style_smallvec();
        fn extend_and_assure_newline<'a>(
            lhs: &mut FrontMatterEvents<'a>,
            rhs: FrontMatterEvents<'a>,
            nl: &impl AsRef<[u8]>,
        ) {
            if !ends_with_newline(lhs.as_ref(), nl, true)
                && !rhs.first().map_or(true, |e| e.to_bstr_lossy().starts_with(nl.as_ref()))
            {
                lhs.push(Event::Newline(Cow::Owned(nl.as_ref().into())))
            }
            lhs.extend(rhs);
        }
        let our_last_section_before_append =
            insert_after.or_else(|| (self.section_id_counter != 0).then(|| SectionId(self.section_id_counter - 1)));

        for id in std::mem::take(&mut other.section_order) {
            let section = other.sections.remove(&id).expect("present");

            let new_id = match insert_after {
                Some(id) => {
                    let new_id = self.insert_section_after(section, id);
                    insert_after = Some(new_id);
                    new_id
                }
                None => self.push_section_internal(section),
            };

            if let Some(post_matter) = other.frontmatter_post_section.remove(&id) {
                self.frontmatter_post_section.insert(new_id, post_matter);
            }
        }

        if other.frontmatter_events.is_empty() {
            return self;
        }

        match our_last_section_before_append {
            Some(last_id) => extend_and_assure_newline(
                self.frontmatter_post_section.entry(last_id).or_default(),
                other.frontmatter_events,
                &nl,
            ),
            None => extend_and_assure_newline(&mut self.frontmatter_events, other.frontmatter_events, &nl),
        }
        self
    }
}
