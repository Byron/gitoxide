use std::{borrow::Cow, convert::TryFrom};

use bstr::BStr;

use crate::{file::SectionBody, lookup, parse::section, File};

/// Read-only low-level access methods, as it requires generics for converting into
/// custom values defined in this crate like [`Integer`][crate::Integer] and
/// [`Color`][crate::Color].
impl<'event> File<'event> {
    /// Returns an interpreted value given a section, an optional subsection and
    /// key.
    ///
    /// It's recommended to use one of the value types provide dby this crate
    /// as they implement the conversion, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&BStr>`][std::convert::TryFrom].
    ///
    /// Consider [`Self::values`] if you want to get all values of a multivar instead.
    ///
    /// If a `string` is desired, use the [`string()`][Self::string()] method instead.
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
    pub fn value<'a, T: TryFrom<Cow<'a, BStr>>>(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Result<T, lookup::Error<T::Error>> {
        T::try_from(self.raw_value(section_name, subsection_name, key)?).map_err(lookup::Error::FailedConversion)
    }

    /// Like [`value()`][File::value()], but returning an `None` if the value wasn't found at `section[.subsection].key`
    pub fn try_value<'a, T: TryFrom<Cow<'a, BStr>>>(
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
    /// It's recommended to use one of the value types provide dby this crate
    /// as they implement the conversion, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&BStr>`][std::convert::TryFrom].
    ///
    /// Consider [`Self::value`] if you want to get a single value
    /// (following last-one-wins resolution) instead.
    ///
    /// To access plain strings, use the [`strings()`][Self::strings()] method instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::File;
    /// # use git_config::{Integer, Boolean};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # use bstr::ByteSlice;
    /// let config = r#"
    ///     [core]
    ///         a = true
    ///         c
    ///     [core]
    ///         a
    ///         a = false
    /// "#;
    /// let git_config = git_config::File::try_from(config).unwrap();
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.values::<Boolean>("core", None, "a")?;
    /// assert_eq!(
    ///     a_value,
    ///     vec![
    ///         Boolean(true),
    ///         Boolean(true),
    ///         Boolean(false),
    ///     ]
    /// );
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Vec<Boolean> = git_config.values("core", None, "c").unwrap();
    /// assert_eq!(c_value, vec![Boolean(true)]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [`value`]: crate::value
    /// [`TryFrom`]: std::convert::TryFrom
    pub fn values<'a, T: TryFrom<Cow<'a, BStr>>>(
        &'a self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Result<Vec<T>, lookup::Error<T::Error>> {
        self.raw_values(section_name, subsection_name, key)?
            .into_iter()
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(lookup::Error::FailedConversion)
    }

    /// Returns the last found immutable section with a given name and optional subsection name.
    pub fn section(
        &mut self,
        section_name: &str,
        subsection_name: Option<&str>,
    ) -> Result<&SectionBody<'event>, lookup::existing::Error> {
        let id = self
            .section_ids_by_name_and_subname(section_name, subsection_name)?
            .rev()
            .next()
            .expect("BUG: Section lookup vec was empty");
        Ok(self
            .sections
            .get(&id)
            .expect("BUG: Section did not have id from lookup"))
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
    /// let git_config = git_config::File::try_from(config)?;
    /// assert_eq!(git_config.sections_by_name("core").map_or(0, |s|s.count()), 3);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn sections_by_name<'a>(
        &'a self,
        section_name: &'a str,
    ) -> Option<impl Iterator<Item = &SectionBody<'event>> + '_> {
        self.section_ids_by_name(section_name).ok().map(move |ids| {
            ids.map(move |id| {
                self.sections
                    .get(&id)
                    .expect("section doesn't have id from from lookup")
            })
        })
    }

    /// Get all sections that match the `section_name`, returning all matching section header along with their body.
    ///
    /// `None` is returned if there is no section with `section_name`.
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
    /// let config = git_config::File::try_from(input)?;
    /// let url = config.sections_by_name_with_header("url");
    /// assert_eq!(url.map_or(0, |s| s.count()), 2);
    ///
    /// for (i, (header, body)) in config.sections_by_name_with_header("url").unwrap().enumerate() {
    ///     let url = header.subsection_name().unwrap();
    ///     let instead_of = body.value("insteadOf").unwrap();
    ///
    ///     if i == 0 {
    ///         assert_eq!(instead_of.as_ref(), "https://github.com/");
    ///         assert_eq!(url, "ssh://git@github.com/");
    ///     } else {
    ///         assert_eq!(instead_of.as_ref(), "https://bitbucket.org/");
    ///         assert_eq!(url, "ssh://git@bitbucket.org");
    ///     }
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn sections_by_name_with_header<'a>(
        &'a self,
        section_name: &'a str,
    ) -> Option<impl Iterator<Item = (&section::Header<'event>, &SectionBody<'event>)> + '_> {
        self.section_ids_by_name(section_name).ok().map(move |ids| {
            ids.map(move |id| {
                (
                    self.section_headers
                        .get(&id)
                        .expect("section doesn't have a section header??"),
                    self.sections
                        .get(&id)
                        .expect("section doesn't have id from from lookup"),
                )
            })
        })
    }

    /// Returns the number of values in the config, no matter in which section.
    ///
    /// For example, a config with multiple empty sections will return 0.
    /// This ignores any comments.
    #[must_use]
    pub fn num_values(&self) -> usize {
        self.sections.values().map(|section| section.num_values()).sum()
    }

    /// Returns if there are no entries in the config. This will return true
    /// if there are only empty sections or comments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sections.values().all(SectionBody::is_empty)
    }
}
