use bstr::BStr;
use std::{borrow::Cow, convert::TryFrom};

use crate::{file::SectionBody, lookup, parse::section, File};

/// Read-only low-level access methods.
impl<'a> File<'a> {
    /// Returns an interpreted value given a section, an optional subsection and
    /// key.
    ///
    /// It's recommended to use one of the values in the [`value`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::multi_value`] if you want to get all values of a
    /// multivar instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::File;
    /// # use git_config::{Integer, Boolean};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = 10k
    ///         c
    /// "#;
    /// let git_config = git_config::File::try_from(config)?;
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.value::<Integer>("core", None, "a")?;
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Boolean = git_config.value("core", None, "c")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`value`]: crate::value
    /// [`TryFrom`]: std::convert::TryFrom
    pub fn value<T: TryFrom<Cow<'a, BStr>>>(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Result<T, lookup::Error<T::Error>> {
        T::try_from(self.raw_value(section_name, subsection_name, key)?).map_err(lookup::Error::FailedConversion)
    }

    /// Like [`value()`][File::value()], but returning an `Option` if the value wasn't found.
    pub fn try_value<T: TryFrom<Cow<'a, BStr>>>(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Option<Result<T, T::Error>> {
        self.raw_value(section_name, subsection_name, key).ok().map(T::try_from)
    }

    /// Returns all interpreted values given a section, an optional subsection
    /// and key.
    ///
    /// It's recommended to use one of the values in the [`value`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::value`] if you want to get a single value
    /// (following last-one-wins resolution) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::File;
    /// # use git_config::{Integer, String, Boolean, boolean::True};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = true
    ///         c = g
    ///     [core]
    ///         a
    ///         a = false
    /// "#;
    /// let git_config = git_config::File::try_from(config).unwrap();
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.multi_value::<Boolean>("core", None, "a")?;
    /// assert_eq!(
    ///     a_value,
    ///     vec![
    ///         Boolean::True(True::Explicit(Cow::Borrowed("true".into()))),
    ///         Boolean::True(True::Implicit),
    ///         Boolean::False(Cow::Borrowed("false".into())),
    ///     ]
    /// );
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Vec<String> = git_config.multi_value("core", None, "c")?;
    /// assert_eq!(c_value, vec![String { value: Cow::Borrowed("g".into()) }]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`value`]: crate::value
    /// [`TryFrom`]: std::convert::TryFrom
    pub fn multi_value<'lookup, T: TryFrom<Cow<'a, BStr>>>(
        &'a self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Vec<T>, lookup::Error<T::Error>> {
        self.raw_multi_value(section_name, subsection_name, key)?
            .into_iter()
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(lookup::Error::FailedConversion)
    }

    /// Returns an immutable section reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section and optional
    /// subsection do not exist.
    pub fn section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
    ) -> Result<&SectionBody<'a>, lookup::existing::Error> {
        let section_ids = self.section_ids_by_name_and_subname(section_name, subsection_name)?;
        let id = section_ids.last().expect("BUG: Section lookup vec was empty");
        Ok(self.sections.get(id).expect("BUG: Section did not have id from lookup"))
    }

    /// Gets all sections that match the provided name, ignoring any subsections.
    ///
    /// # Examples
    ///
    /// Provided the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core ""]
    ///     c = d
    /// [core "apple"]
    ///     e = f
    /// ```
    ///
    /// Calling this method will yield all sections:
    ///
    /// ```
    /// # use git_config::File;
    /// # use git_config::{Integer, Boolean};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = b
    ///     [core ""]
    ///         c = d
    ///     [core "apple"]
    ///         e = f
    /// "#;
    /// let git_config = git_config::File::try_from(config).unwrap();
    /// assert_eq!(git_config.sections_by_name("core").len(), 3);
    /// ```
    #[must_use]
    pub fn sections_by_name<'lookup>(&self, section_name: &'lookup str) -> Vec<&SectionBody<'a>> {
        self.section_ids_by_name(section_name)
            .unwrap_or_default()
            .into_iter()
            .map(|id| {
                self.sections
                    .get(&id)
                    .expect("section doesn't have id from from lookup")
            })
            .collect()
    }

    /// Get all sections that match the `section_name`, returning all matching section header along with their body.
    ///
    /// An empty `Vec` is returned if there is no section with `section_name`.
    ///
    /// # Example
    ///
    /// Provided the following config:
    /// ```plain
    /// [url "ssh://git@github.com/"]
    ///     insteadOf = https://github.com/
    /// [url "ssh://git@bitbucket.org"]
    ///     insteadOf = https://bitbucket.org/
    /// ```
    /// Calling this method will yield all section bodies and their header:
    ///
    /// ```rust
    /// use git_config::File;
    /// use git_config::parse::section;
    /// use std::borrow::Cow;
    /// use std::convert::TryFrom;
    /// use nom::AsBytes;
    ///
    /// let input = r#"
    /// [url "ssh://git@github.com/"]
    ///    insteadOf = https://github.com/
    /// [url "ssh://git@bitbucket.org"]
    ///    insteadOf = https://bitbucket.org/
    /// "#;
    /// let config = git_config::File::try_from(input).unwrap();
    /// let url = config.sections_by_name_with_header("url");
    /// assert_eq!(url.len(), 2);
    ///
    /// for (i, (header, body)) in url.iter().enumerate() {
    ///     let url = header.subsection_name.as_ref();
    ///     let instead_of = body.value(&section::Key::from("insteadOf"));
    ///
    ///     // todo(unstable-order): the order is not always the same, so `i` cannot be used here
    ///     if instead_of.as_ref().unwrap().as_ref().as_bytes().eq("https://github.com/".as_bytes()) {
    ///         assert_eq!(instead_of.unwrap().as_ref(), "https://github.com/".as_bytes());
    ///         assert_eq!(url.unwrap().as_ref(), "ssh://git@github.com/");
    ///     } else {
    ///         assert_eq!(instead_of.unwrap().as_ref(), "https://bitbucket.org/".as_bytes());
    ///         assert_eq!(url.unwrap().as_ref(), "ssh://git@bitbucket.org");
    ///     }
    /// }
    /// ```
    pub fn sections_by_name_with_header<'lookup>(
        &self,
        section_name: &'lookup str,
    ) -> Vec<(&section::Header<'a>, &SectionBody<'a>)> {
        self.section_ids_by_name(section_name)
            .unwrap_or_default()
            .into_iter()
            .map(|id| {
                (
                    self.section_headers
                        .get(&id)
                        .expect("section doesn't have a section header??"),
                    self.sections
                        .get(&id)
                        .expect("section doesn't have id from from lookup"),
                )
            })
            .collect()
    }

    /// Returns the number of values in the config, no matter in which section.
    ///
    /// For example, a config with multiple empty sections will return 0.
    /// This ignores any comments.
    #[must_use]
    pub fn num_values(&self) -> usize {
        self.sections
            .values()
            .fold(0, |acc, section| acc + section.num_values())
    }

    /// Returns if there are no entries in the config. This will return true
    /// if there are only empty sections or comments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.values().all(SectionBody::is_empty)
    }
}
