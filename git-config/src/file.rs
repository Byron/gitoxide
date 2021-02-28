use crate::parser::{parse_from_bytes, Error, Event, ParsedSectionHeader, Parser};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::{borrow::Cow, fmt::Display};

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
pub enum GitConfigError<'a> {
    /// The requested section does not exist.
    SectionDoesNotExist(&'a str),
    /// The requested subsection does not exist.
    SubSectionDoesNotExist(Option<&'a str>),
    /// The key does not exist in the requested section.
    KeyDoesNotExist(&'a str),
    FailedConversion,
}

impl Display for GitConfigError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Todo, try parse as utf8 first for better looking errors
            Self::SectionDoesNotExist(s) => write!(f, "Section '{}' does not exist.", s),
            Self::SubSectionDoesNotExist(s) => match s {
                Some(s) => write!(f, "Subsection '{}' does not exist.", s),
                None => write!(f, "Top level section does not exist."),
            },
            Self::KeyDoesNotExist(k) => write!(f, "Name '{}' does not exist.", k),
            Self::FailedConversion => write!(f, "Failed to convert to specified type."),
        }
    }
}

impl std::error::Error for GitConfigError<'_> {}

/// The section ID is a monotonically increasing ID used to refer to sections.
/// This value does not imply any ordering between sections, as new sections
/// with higher section IDs may be in between lower ID sections.
///
/// We need to use a section id because `git-config` permits sections with
/// identical names. As a result, we can't simply use the section name as a key
/// in a map.
///
/// This id guaranteed to be unique, but not guaranteed to be compact. In other
/// words, it's possible that a section may have an ID of 3 but the next section
/// has an ID of 5.
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
struct SectionId(usize);

#[derive(PartialEq, Eq, Clone, Debug)]
enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<Cow<'a, str>, Vec<SectionId>>),
}

/// High level `git-config` reader and writer.
///
/// Internally, this uses various acceleration data structures to improve
/// performance.
///
/// # Multivar behavior
///
/// `git` is flexible enough to allow users to set a key multiple times in
/// any number of identically named sections. When this is the case, the key
/// is known as a "multivar". In this case, `get_raw_value` follows the
/// "last one wins" approach that `git-config` internally uses for multivar
/// resolution.
///
/// Concretely, the following config has a multivar, `a`, with the values
/// of `b`, `c`, and `d`, while `e` is a single variable with the value
/// `f g h`.
///
/// ```text
/// [core]
///     a = b
///     a = c
/// [core]
///     a = d
///     e = f g h
/// ```
///
/// Calling methods that only one fetch value (such as [`get_raw_value`]) to
/// fetch `a` with the above config will return `d`, since the last valid config
/// value is `a = d`:
///
/// ```
/// # use git_config::file::GitConfig;
/// # use std::borrow::Cow;
/// # use std::convert::TryFrom;
/// # let git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
/// assert_eq!(git_config.get_raw_value("core", None, "a"), Ok(&Cow::Borrowed("d".as_bytes())));
/// ```
///
/// Consider the `multi` variants of the methods instead, if you want to work
/// with all values instead.
///
/// [`get_raw_value`]: Self::get_raw_value
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GitConfig<'a> {
    /// The list of events that occur before an actual section. Since a
    /// `git-config` file prohibits global values, this vec is limited to only
    /// comment, newline, and whitespace events.
    front_matter_events: Vec<Event<'a>>,
    section_lookup_tree: HashMap<Cow<'a, str>, Vec<LookupTreeNode<'a>>>,
    /// SectionId to section mapping. The value of this HashMap contains actual
    /// events
    sections: HashMap<SectionId, Vec<Event<'a>>>,
    section_headers: HashMap<SectionId, ParsedSectionHeader<'a>>,
    section_id_counter: usize,
    section_order: VecDeque<SectionId>,
}

impl<'a> GitConfig<'a> {
    fn push_section(
        &mut self,
        current_section_name: Option<Cow<'a, str>>,
        current_subsection_name: Option<Cow<'a, str>>,
        maybe_section: &mut Option<Vec<Event<'a>>>,
    ) {
        if let Some(section) = maybe_section.take() {
            let new_section_id = SectionId(self.section_id_counter);
            self.sections.insert(new_section_id, section);
            let lookup = self
                .section_lookup_tree
                .entry(current_section_name.unwrap())
                .or_default();

            let mut found_node = false;
            if let Some(subsection_name) = current_subsection_name {
                for node in lookup.iter_mut() {
                    if let LookupTreeNode::NonTerminal(subsection) = node {
                        found_node = true;
                        subsection
                            // Despite the clone `push_section` is always called
                            // with a Cow::Borrowed, so this is effectively a
                            // copy.
                            .entry(subsection_name.clone())
                            .or_default()
                            .push(new_section_id);
                        break;
                    }
                }
                if !found_node {
                    let mut map = HashMap::new();
                    map.insert(subsection_name, vec![new_section_id]);
                    lookup.push(LookupTreeNode::NonTerminal(map));
                }
            } else {
                for node in lookup.iter_mut() {
                    if let LookupTreeNode::Terminal(vec) = node {
                        found_node = true;
                        vec.push(new_section_id);
                        break;
                    }
                }
                if !found_node {
                    lookup.push(LookupTreeNode::Terminal(vec![new_section_id]))
                }
            }
            self.section_order.push_back(new_section_id);
            self.section_id_counter += 1;
        }
    }

    /// Returns an interpreted value given a section, an optional subsection and
    /// key.
    ///
    /// It's recommended to use one of the values in the [`values`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::get_multi_value`] if you want to get all values of a
    /// multivar instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Value, Boolean};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = 10k
    ///         c
    /// "#;
    /// let git_config = GitConfig::try_from(config).unwrap();
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.get_value::<Integer>("core", None, "a")?;
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Boolean = git_config.get_value("core", None, "c")?;
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`values`]: crate::values
    /// [`TryFrom`]: std::convert::TryFrom
    pub fn get_value<'b, 'c, T: TryFrom<&'c [u8]>>(
        &'c self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<T, GitConfigError<'b>> {
        T::try_from(self.get_raw_value(section_name, subsection_name, key)?)
            .map_err(|_| GitConfigError::FailedConversion)
    }

    fn get_section_id_by_name_and_subname<'b>(
        &'a self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
    ) -> Result<SectionId, GitConfigError<'b>> {
        self.get_section_ids_by_name_and_subname(section_name, subsection_name)
            .map(|vec| {
                // get_section_ids_by_name_and_subname is guaranteed to return
                // a non-empty vec, so max can never return empty.
                *vec.iter().max().unwrap()
            })
    }

    /// Returns an uninterpreted value given a section, an optional subsection
    /// and key.
    ///
    /// Consider [`Self::get_raw_multi_value`] if you want to get all values of
    /// a multivar instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, or if the section and subsection do not exist.
    pub fn get_raw_value<'b>(
        &self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<&Cow<'a, [u8]>, GitConfigError<'b>> {
        let key = key;
        // Note: cannot wrap around the raw_multi_value method because we need
        // to guarantee that the highest section id is used (so that we follow
        // the "last one wins" resolution strategy by `git-config`).
        let section_id = self.get_section_id_by_name_and_subname(section_name, subsection_name)?;

        // section_id is guaranteed to exist in self.sections, else we have a
        // violated invariant.
        let events = self.sections.get(&section_id).unwrap();
        let mut found_key = false;
        let mut latest_value = None;
        for event in events {
            match event {
                Event::Key(event_key) if *event_key == key => found_key = true,
                Event::Value(v) if found_key => {
                    found_key = false;
                    latest_value = Some(v);
                }
                _ => (),
            }
        }

        latest_value.ok_or(GitConfigError::KeyDoesNotExist(key))
    }

    /// Returns a mutable reference to an uninterpreted value given a section,
    /// an optional subsection and key.
    ///
    /// Consider [`Self::get_raw_multi_value_mut`] if you want to get mutable
    /// references to all values of a multivar instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, or if the section and subsection do not exist.
    pub fn get_raw_value_mut<'b>(
        &mut self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<&mut Cow<'a, [u8]>, GitConfigError<'b>> {
        let key = key;
        // Note: cannot wrap around the raw_multi_value method because we need
        // to guarantee that the highest section id is used (so that we follow
        // the "last one wins" resolution strategy by `git-config`).
        let section_id = self.get_section_id_by_name_and_subname(section_name, subsection_name)?;

        // section_id is guaranteed to exist in self.sections, else we have a
        // violated invariant.
        let events = self.sections.get_mut(&section_id).unwrap();
        let mut found_key = false;
        let mut latest_value = None;
        for event in events {
            match event {
                Event::Key(event_key) if *event_key == key => found_key = true,
                Event::Value(v) if found_key => {
                    found_key = false;
                    latest_value = Some(v);
                }
                _ => (),
            }
        }

        latest_value.ok_or(GitConfigError::KeyDoesNotExist(key))
    }

    /// Returns all uninterpreted values given a section, an optional subsection
    /// and key. If you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Attempting to get all values of `a` yields the following:
    ///
    /// ```
    /// # use git_config::file::GitConfig;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a"),
    ///     Ok(vec![
    ///         &Cow::<[u8]>::Borrowed(b"b"),
    ///         &Cow::<[u8]>::Borrowed(b"c"),
    ///         &Cow::<[u8]>::Borrowed(b"d"),
    ///     ]),
    /// );
    /// ```
    ///
    /// Consider [`Self::get_raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in any requested
    /// section and subsection, or if no instance of the section and subsections
    /// exist.
    pub fn get_raw_multi_value<'b>(
        &self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<Vec<&Cow<'_, [u8]>>, GitConfigError<'b>> {
        let key = key;
        let mut values = vec![];
        for section_id in self.get_section_ids_by_name_and_subname(section_name, subsection_name)? {
            let mut found_key = false;
            // section_id is guaranteed to exist in self.sections, else we
            // have a violated invariant.
            for event in self.sections.get(section_id).unwrap() {
                match event {
                    Event::Key(event_key) if *event_key == key => found_key = true,
                    Event::Value(v) if found_key => {
                        values.push(v);
                        found_key = false;
                    }
                    _ => (),
                }
            }
        }

        if values.is_empty() {
            Err(GitConfigError::KeyDoesNotExist(key))
        } else {
            Ok(values)
        }
    }

    /// Returns mutable references to all uninterpreted values given a section,
    /// an optional subsection and key. If you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Attempting to get all values of `a` yields the following:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![
    ///         &Cow::Borrowed(b"b"),
    ///         &Cow::Borrowed(b"c"),
    ///         &Cow::Borrowed(b"d")
    ///     ]
    /// );
    /// for value in git_config.get_raw_multi_value_mut("core", None, "a")? {
    ///     *value = Cow::Borrowed(b"g");
    ///}
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![
    ///         &Cow::Borrowed(b"g"),
    ///         &Cow::Borrowed(b"g"),
    ///         &Cow::Borrowed(b"g")
    ///     ],
    /// );
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Consider [`Self::get_raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// Note that this operation is relatively expensive, requiring a full
    /// traversal of the config.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in any requested
    /// section and subsection, or if no instance of the section and subsections
    /// exist.
    pub fn get_raw_multi_value_mut<'b>(
        &mut self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<Vec<&mut Cow<'a, [u8]>>, GitConfigError<'b>> {
        let key = key;
        let section_ids = self
            .get_section_ids_by_name_and_subname(section_name, subsection_name)?
            .to_vec();
        let mut found_key = false;
        let values: Vec<&mut Cow<'a, [u8]>> = self
            .sections
            .iter_mut()
            .filter_map(|(k, v)| {
                if section_ids.contains(k) {
                    let mut values = vec![];
                    for event in v {
                        match event {
                            Event::Key(event_key) if *event_key == key => found_key = true,
                            Event::Value(v) if found_key => {
                                values.push(v);
                                found_key = false;
                            }
                            _ => (),
                        }
                    }
                    Some(values)
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        if values.is_empty() {
            Err(GitConfigError::KeyDoesNotExist(key))
        } else {
            Ok(values)
        }
    }

    fn get_section_ids_by_name_and_subname<'b>(
        &self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
    ) -> Result<&[SectionId], GitConfigError<'b>> {
        let section_ids = self
            .section_lookup_tree
            .get(section_name)
            .ok_or(GitConfigError::SectionDoesNotExist(section_name))?;
        let mut maybe_ids = None;
        // Don't simplify if and matches here -- the for loop currently needs
        // `n + 1` checks, while the if and matches will result in the for loop
        // needing `2n` checks.
        if let Some(subsect_name) = subsection_name {
            for node in section_ids {
                if let LookupTreeNode::NonTerminal(subsection_lookup) = node {
                    maybe_ids = subsection_lookup.get(subsect_name);
                    break;
                }
            }
        } else {
            for node in section_ids {
                if let LookupTreeNode::Terminal(subsection_lookup) = node {
                    maybe_ids = Some(subsection_lookup);
                    break;
                }
            }
        }
        maybe_ids
            .map(Vec::as_slice)
            .ok_or(GitConfigError::SubSectionDoesNotExist(subsection_name))
    }

    pub fn set_raw_value<'b>(
        &mut self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
        new_value: impl Into<Cow<'a, [u8]>>,
    ) -> Result<(), GitConfigError<'b>> {
        let value = self.get_raw_value_mut(section_name, subsection_name, key)?;
        *value = new_value.into();
        Ok(())
    }

    /// Sets a multivar in a given section, optional subsection, and key value.
    ///
    /// This internally zips together the new values and the existing values.
    /// As a result, if more new values are provided than the current amount of
    /// multivars, then the latter values are not applied. If there are less
    /// new values than old ones then the remaining old values are unmodified.
    ///
    /// If you need finer control over which values of the multivar are set,
    /// consider using [`get_raw_multi_value_mut`].
    ///
    /// todo: examples and errors
    ///
    /// [`get_raw_multi_value_mut`]: Self::get_raw_multi_value_mut
    pub fn set_raw_multi_value<'b>(
        &mut self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
        new_values: Vec<Cow<'a, [u8]>>,
    ) -> Result<(), GitConfigError<'b>> {
        let values = self.get_raw_multi_value_mut(section_name, subsection_name, key)?;
        for (old, new) in values.into_iter().zip(new_values) {
            *old = new;
        }
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`GitConfig`]. See [`parse_from_str`] for more information.
    ///
    /// [`parse_from_str`]: crate::parser::parse_from_str
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        parse_from_bytes(s.as_bytes()).map(Self::from)
    }
}

impl<'a> TryFrom<&'a [u8]> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`GitConfig`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        parse_from_bytes(value).map(Self::from)
    }
}

impl<'a> From<Parser<'a>> for GitConfig<'a> {
    fn from(parser: Parser<'a>) -> Self {
        let mut new_self = Self {
            front_matter_events: vec![],
            sections: HashMap::new(),
            section_lookup_tree: HashMap::new(),
            section_headers: HashMap::new(),
            section_id_counter: 0,
            section_order: VecDeque::new(),
        };

        // Current section that we're building
        let mut current_section_name: Option<Cow<'a, str>> = None;
        let mut current_subsection_name: Option<Cow<'a, str>> = None;
        let mut maybe_section: Option<Vec<Event<'a>>> = None;

        for event in parser.into_iter() {
            match event {
                Event::SectionHeader(header) => {
                    new_self.push_section(
                        current_section_name,
                        current_subsection_name,
                        &mut maybe_section,
                    );

                    // Initialize new section
                    // We need to store the new, current id counter, so don't
                    // use new_section_id here and use the already incremented
                    // section id value.
                    new_self
                        .section_headers
                        .insert(SectionId(new_self.section_id_counter), header.clone());
                    let (name, subname) = (header.name, header.subsection_name);
                    maybe_section = Some(vec![]);
                    current_section_name = Some(name);
                    current_subsection_name = subname;
                }
                e @ Event::Key(_)
                | e @ Event::Value(_)
                | e @ Event::ValueNotDone(_)
                | e @ Event::ValueDone(_)
                | e @ Event::KeyValueSeparator => maybe_section
                    .as_mut()
                    .expect("Got a section-only event before a section")
                    .push(e),
                e @ Event::Comment(_) | e @ Event::Newline(_) | e @ Event::Whitespace(_) => {
                    match maybe_section {
                        Some(ref mut section) => section.push(e),
                        None => new_self.front_matter_events.push(e),
                    }
                }
            }
        }

        // The last section doesn't get pushed since we only push if there's a
        // new section header, so we need to call push one more time.
        new_self.push_section(
            current_section_name,
            current_subsection_name,
            &mut maybe_section,
        );

        new_self
    }
}

impl Display for GitConfig<'_> {
    /// Note that this is a best-effort attempt at printing a `GitConfig`. If
    /// there are non UTF-8 values in your config, this will _NOT_ render as
    /// read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for front_matter in &self.front_matter_events {
            front_matter.fmt(f)?;
        }

        for section_id in &self.section_order {
            self.section_headers.get(section_id).unwrap().fmt(f)?;
            for event in self.sections.get(section_id).unwrap() {
                event.fmt(f)?;
            }
        }

        Ok(())
    }
}

// todo impl serialize

#[cfg(test)]
mod from_parser {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn parse_empty() {
        let config = GitConfig::try_from("").unwrap();
        assert!(config.section_headers.is_empty());
        assert_eq!(config.section_id_counter, 0);
        assert!(config.section_lookup_tree.is_empty());
        assert!(config.sections.is_empty());
        assert!(config.section_order.is_empty());
    }

    #[test]
    fn parse_single_section() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                Cow::Borrowed("core"),
                vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                ],
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_single_subsection() {
        let mut config = GitConfig::try_from("[core.subsec]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", (".", "subsec")));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            let mut inner_tree = HashMap::new();
            inner_tree.insert(Cow::Borrowed("subsec"), vec![SectionId(0)]);
            tree.insert(
                Cow::Borrowed("core"),
                vec![LookupTreeNode::NonTerminal(inner_tree)],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                ],
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_multiple_sections() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d\n[other]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map.insert(SectionId(1), section_header("other", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                Cow::Borrowed("core"),
                vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
            );
            tree.insert(
                Cow::Borrowed("other"),
                vec![LookupTreeNode::Terminal(vec![SectionId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                    newline_event(),
                ],
            );
            sections.insert(
                SectionId(1),
                vec![name_event("e"), Event::KeyValueSeparator, value_event("f")],
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(
            config.section_order.make_contiguous(),
            &[SectionId(0), SectionId(1)]
        );
    }

    #[test]
    fn parse_multiple_duplicate_sections() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d\n[core]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map.insert(SectionId(1), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                Cow::Borrowed("core"),
                vec![LookupTreeNode::Terminal(vec![SectionId(0), SectionId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                    newline_event(),
                ],
            );
            sections.insert(
                SectionId(1),
                vec![name_event("e"), Event::KeyValueSeparator, value_event("f")],
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(
            config.section_order.make_contiguous(),
            &[SectionId(0), SectionId(1)]
        );
    }
}

#[cfg(test)]
mod get_raw_value {
    use super::*;

    #[test]
    fn single_section() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::<[u8]>::Borrowed(b"b"))
        );
        assert_eq!(
            config.get_raw_value("core", None, "c"),
            Ok(&Cow::<[u8]>::Borrowed(b"d"))
        );
    }

    #[test]
    fn last_one_wins_respected_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::<[u8]>::Borrowed(b"d"))
        );
    }

    #[test]
    fn last_one_wins_respected_across_section() {
        let config = GitConfig::try_from("[core]\na=b\n[core]\na=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::<[u8]>::Borrowed(b"d"))
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist("foo"))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist("aaaaaa"))
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::<[u8]>::Borrowed(b"b"))
        );
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Ok(&Cow::<[u8]>::Borrowed(b"c"))
        );
    }
}

#[cfg(test)]
mod get_value {
    use super::*;
    use crate::values::{Boolean, TrueVariant, Value};
    use std::error::Error;

    #[test]
    fn single_section() -> Result<(), Box<dyn Error>> {
        let config = GitConfig::try_from("[core]\na=b\nc").unwrap();
        let first_value: Value = config.get_value("core", None, "a")?;
        let second_value: Boolean = config.get_value("core", None, "c")?;

        assert_eq!(first_value, Value::Other(Cow::Borrowed(b"b")));
        assert_eq!(second_value, Boolean::True(TrueVariant::Implicit));

        Ok(())
    }
}

#[cfg(test)]
mod get_raw_multi_value {
    use super::*;

    #[test]
    fn single_value_is_identical_to_single_value_query() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            vec![config.get_raw_value("core", None, "a").unwrap()],
            config.get_raw_multi_value("core", None, "a").unwrap()
        );
    }

    #[test]
    fn multi_value_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![&Cow::Borrowed(b"b"), &Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn multi_value_across_sections() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![
                &Cow::Borrowed(b"b"),
                &Cow::Borrowed(b"c"),
                &Cow::Borrowed(b"d")
            ]
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist("foo"))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist("aaaaaa"))
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![&Cow::Borrowed(b"b")]
        );
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a").unwrap(),
            vec![&Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn non_relevant_subsection_is_ignored() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d\n[core]g=g").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![
                &Cow::Borrowed(b"b"),
                &Cow::Borrowed(b"c"),
                &Cow::Borrowed(b"d")
            ]
        );
    }
}

#[cfg(test)]
mod display {
    use super::*;

    #[test]
    fn can_reconstruct_empty_config() {
        let config = r#"

        "#;
        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_non_empty_config() {
        let config = r#"[user]
        email = code@eddie.sh
[core]
        autocrlf = input
[push]
        default = simple
[commit]
        gpgsign = true
[gpg]
        program = gpg
[url "ssh://git@github.com/"]
        insteadOf = "github://"
[url "ssh://git@git.eddie.sh/edward/"]
        insteadOf = "gitea://"
[pull]
        ff = only
[init]
        defaultBranch = master"#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_with_implicits() {
        let config = r#"[user]
        email
        name
[core]
        autocrlf
[push]
        default
[commit]
        gpgsign"#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_without_whitespace_in_middle() {
        let config = r#"[core]
        autocrlf=input
[push]
        default=simple
[commit]
        gpgsign=true
[pull]
        ff = only
[init]
        defaultBranch = master"#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }
}
