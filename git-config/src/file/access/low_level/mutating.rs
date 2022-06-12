use std::borrow::Cow;

use crate::{
    file::{MutableSection, SectionBody},
    lookup,
    parser::{ParsedSectionHeader, SectionHeaderName},
    File,
};

/// Mutating low-level access methods.
impl<'a> File<'a> {
    /// Returns an mutable section reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section and optional
    /// subsection do not exist.
    pub fn section_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
    ) -> Result<MutableSection<'_, 'a>, lookup::existing::Error> {
        let section_ids = self.section_ids_by_name_and_subname(section_name, subsection_name)?;
        let id = section_ids.last().expect("BUG: Section lookup vec was empty");
        Ok(MutableSection::new(
            self.sections
                .get_mut(id)
                .expect("BUG: Section did not have id from lookup"),
        ))
    }

    /// Adds a new section to config. If a subsection name was provided, then
    /// the generated header will use the modern subsection syntax. Returns a
    /// reference to the new section for immediate editing.
    ///
    /// # Examples
    ///
    /// Creating a new empty section:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::convert::TryFrom;
    /// let mut git_config = git_config::File::new();
    /// let _section = git_config.new_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n");
    /// ```
    ///
    /// Creating a new empty section and adding values to it:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::convert::TryFrom;
    /// let mut git_config = git_config::File::new();
    /// let mut section = git_config.new_section("hello", Some("world".into()));
    /// section.push("a".into(), "b".as_bytes().into());
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n  a=b\n");
    /// let _section = git_config.new_section("core", None);
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n  a=b\n[core]\n");
    /// ```
    pub fn new_section(
        &mut self,
        section_name: impl Into<Cow<'a, str>>,
        subsection_name: impl Into<Option<Cow<'a, str>>>,
    ) -> MutableSection<'_, 'a> {
        let mut section = self.push_section(section_name, subsection_name, SectionBody::new());
        section.push_newline();
        section
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
    /// "#).unwrap();
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "");
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
    /// "#).unwrap();
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n    some-value = 4\n");
    /// ```
    pub fn remove_section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: impl Into<Option<&'lookup str>>,
    ) -> Option<SectionBody<'_>> {
        let id = self
            .section_ids_by_name_and_subname(section_name, subsection_name.into())
            .ok()?
            .pop()?;
        self.section_order.remove(
            self.section_order
                .iter()
                .position(|v| *v == id)
                .expect("Section order does not contain section that we were trying to remove"),
        );
        self.sections.remove(&id)
    }

    /// Adds the provided section to the config, returning a mutable reference
    /// to it.
    pub fn push_section(
        &mut self,
        section_name: impl Into<Cow<'a, str>>,
        subsection_name: impl Into<Option<Cow<'a, str>>>,
        section: SectionBody<'a>,
    ) -> MutableSection<'_, 'a> {
        let subsection_name = subsection_name.into();
        if subsection_name.is_some() {
            self.push_section_internal(
                ParsedSectionHeader {
                    name: SectionHeaderName(section_name.into()),
                    separator: Some(" ".into()),
                    subsection_name,
                },
                section,
            )
        } else {
            self.push_section_internal(
                ParsedSectionHeader {
                    name: SectionHeaderName(section_name.into()),
                    separator: None,
                    subsection_name: None,
                },
                section,
            )
        }
    }

    /// Renames a section, modifying the last matching section.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup doesn't exist
    pub fn rename_section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: impl Into<Option<&'lookup str>>,
        new_section_name: impl Into<SectionHeaderName<'a>>,
        new_subsection_name: impl Into<Option<Cow<'a, str>>>,
    ) -> Result<(), lookup::existing::Error> {
        let id = self.section_ids_by_name_and_subname(section_name, subsection_name.into())?;
        let id = id
            .last()
            .expect("list of sections were empty, which violates invariant");
        let header = self
            .section_headers
            .get_mut(id)
            .expect("sections does not have section id from section ids");
        header.name = new_section_name.into();
        header.subsection_name = new_subsection_name.into();

        Ok(())
    }
}
