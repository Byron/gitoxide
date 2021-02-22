use crate::parser::{parse_from_str, Event, Parser, ParserError};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub enum GitConfigError<'a> {
    /// The requested section does not exist.
    SectionDoesNotExist(&'a str),
    /// The requested subsection does not exist.
    SubSectionDoesNotExist(Option<&'a str>),
    /// The key does not exist in the requested section.
    KeyDoesNotExist(&'a str),
}

/// High level `git-config` reader and writer.
pub struct GitConfig<'a> {
    /// The list of events that occur before an actual section. Since a
    /// `git-config` file prohibits global values, this vec is limited to only
    /// comment, newline, and whitespace events.
    front_matter_events: Vec<Event<'a>>,
    section_lookup_tree: HashMap<Cow<'a, str>, Vec<LookupTreeNode<'a>>>,
    /// SectionId to section mapping. The value of this HashMap contains actual
    /// events
    sections: HashMap<SectionId, Vec<Event<'a>>>,
    section_header_separators: HashMap<SectionId, Option<Cow<'a, str>>>,
    section_id_counter: usize,
    section_order: VecDeque<SectionId>,
}

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

#[derive(Debug, PartialEq, Eq)]
enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<Cow<'a, str>, Vec<SectionId>>),
}

impl<'a> GitConfig<'a> {
    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`GitConfig`].
    pub fn from_str(str: &'a str) -> Result<Self, ParserError> {
        Ok(Self::from_parser(parse_from_str(str)?))
    }

    pub fn from_parser(parser: Parser<'a>) -> Self {
        let mut new_self = Self {
            front_matter_events: vec![],
            sections: HashMap::new(),
            section_lookup_tree: HashMap::new(),
            section_header_separators: HashMap::new(),
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
                    let (name, subname) = (header.name, header.subsection_name);
                    maybe_section = Some(vec![]);
                    current_section_name = Some(name);
                    current_subsection_name = subname;
                    // We need to store the new, current id counter, so don't
                    // use new_section_id here and use the already incremented
                    // section id value.
                    new_self
                        .section_header_separators
                        .insert(SectionId(new_self.section_id_counter), header.separator);
                }
                e @ Event::Key(_)
                | e @ Event::Value(_)
                | e @ Event::ValueNotDone(_)
                | e @ Event::ValueDone(_) => maybe_section
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

    /// Returns an uninterpreted value given a section, an optional subsection
    /// and key.
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
    /// Calling this function to fetch `a` with the above config will return
    /// `d`, since the last valid config value is `a = d`:
    ///
    /// ```
    /// # use serde_git_config::config::GitConfig;
    /// # use std::borrow::Cow;
    /// # let git_config = GitConfig::from_str("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(git_config.get_raw_value("core", None, "a"), Ok(&Cow::from("d")));
    /// ```
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
    ) -> Result<&Cow<'a, str>, GitConfigError<'b>> {
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
    /// Calling this function to fetch `a` with the above config will return
    /// `d`, since the last valid config value is `a = d`:
    ///
    /// ```
    /// # use serde_git_config::config::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # let mut git_config = GitConfig::from_str("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let mut mut_value = git_config.get_raw_value_mut("core", None, "a")?;
    /// assert_eq!(mut_value, &mut Cow::from("d"));
    /// *mut_value = Cow::Borrowed("hello");
    /// assert_eq!(git_config.get_raw_value("core", None, "a"), Ok(&Cow::from("hello")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
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
    ) -> Result<&mut Cow<'a, str>, GitConfigError<'b>> {
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

    fn get_section_id_by_name_and_subname<'b>(
        &'a self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
    ) -> Result<SectionId, GitConfigError<'b>> {
        self.get_section_ids_by_name_and_subname(section_name, subsection_name)
            .map(|vec| {
                // get_section_ids_by_name_and_subname is guaranteed to return
                // a non-empty vec, so max can never return empty.
                *vec.into_iter().max().unwrap()
            })
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
    /// # use serde_git_config::config::GitConfig;
    /// # use std::borrow::Cow;
    /// # let git_config = GitConfig::from_str("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a"),
    ///     Ok(vec![&Cow::from("b"), &Cow::from("c"), &Cow::from("d")]),
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
        &'a self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<Vec<&Cow<'a, str>>, GitConfigError<'b>> {
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
    /// # use serde_git_config::config::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # let mut git_config = GitConfig::from_str("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(git_config.get_raw_multi_value("core", None, "a")?, vec!["b", "c", "d"]);
    /// for value in git_config.get_raw_multi_value_mut("core", None, "a")? {
    ///     *value = Cow::from("g");
    ///}
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![&Cow::from("g"), &Cow::from("g"), &Cow::from("g")],
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
    ) -> Result<Vec<&mut Cow<'a, str>>, GitConfigError<'b>> {
        let section_ids = self
            .get_section_ids_by_name_and_subname(section_name, subsection_name)?
            .to_vec();
        let mut found_key = false;
        let values: Vec<&mut Cow<'a, str>> = self
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
        &'a self,
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
        new_value: impl Into<Cow<'a, str>>,
    ) -> Result<(), GitConfigError<'b>> {
        let value = self.get_raw_value_mut(section_name, subsection_name, key)?;
        *value = new_value.into();
        Ok(())
    }
}

#[cfg(test)]
mod from_parser {
    use super::*;

    #[test]
    fn parse_empty() {
        let config = GitConfig::from_str("").unwrap();
        assert!(config.section_header_separators.is_empty());
        assert_eq!(config.section_id_counter, 0);
        assert!(config.section_lookup_tree.is_empty());
        assert!(config.sections.is_empty());
        assert!(config.section_order.is_empty());
    }

    #[test]
    fn parse_single_section() {
        let mut config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), None);
            map
        };
        assert_eq!(config.section_header_separators, expected_separators);
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
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("a")),
                    Event::Value(Cow::Borrowed("b")),
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("c")),
                    Event::Value(Cow::Borrowed("d")),
                ],
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_single_subsection() {
        let mut config = GitConfig::from_str("[core.subsec]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), Some(Cow::Borrowed(".")));
            map
        };
        assert_eq!(config.section_header_separators, expected_separators);
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
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("a")),
                    Event::Value(Cow::Borrowed("b")),
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("c")),
                    Event::Value(Cow::Borrowed("d")),
                ],
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_multiple_sections() {
        let mut config = GitConfig::from_str("[core]\na=b\nc=d\n[other]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), None);
            map.insert(SectionId(1), None);
            map
        };
        assert_eq!(config.section_header_separators, expected_separators);
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
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("a")),
                    Event::Value(Cow::Borrowed("b")),
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("c")),
                    Event::Value(Cow::Borrowed("d")),
                    Event::Newline(Cow::Borrowed("\n")),
                ],
            );
            sections.insert(
                SectionId(1),
                vec![
                    Event::Key(Cow::Borrowed("e")),
                    Event::Value(Cow::Borrowed("f")),
                ],
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
        let mut config = GitConfig::from_str("[core]\na=b\nc=d\n[core]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), None);
            map.insert(SectionId(1), None);
            map
        };
        assert_eq!(config.section_header_separators, expected_separators);
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
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("a")),
                    Event::Value(Cow::Borrowed("b")),
                    Event::Newline(Cow::Borrowed("\n")),
                    Event::Key(Cow::Borrowed("c")),
                    Event::Value(Cow::Borrowed("d")),
                    Event::Newline(Cow::Borrowed("\n")),
                ],
            );
            sections.insert(
                SectionId(1),
                vec![
                    Event::Key(Cow::Borrowed("e")),
                    Event::Value(Cow::Borrowed("f")),
                ],
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
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::Borrowed("b"))
        );
        assert_eq!(
            config.get_raw_value("core", None, "c"),
            Ok(&Cow::Borrowed("d"))
        );
    }

    #[test]
    fn last_one_wins_respected_in_section() {
        let config = GitConfig::from_str("[core]\na=b\na=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::Borrowed("d"))
        );
    }

    #[test]
    fn last_one_wins_respected_across_section() {
        let config = GitConfig::from_str("[core]\na=b\n[core]\na=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::Borrowed("d"))
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist("foo"))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist("aaaaaa"))
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::from_str("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "a"),
            Ok(&Cow::Borrowed("b"))
        );
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Ok(&Cow::Borrowed("c"))
        );
    }
}

#[cfg(test)]
mod get_raw_multi_value {
    use super::*;

    #[test]
    fn single_value_is_identical_to_single_value_query() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            vec![config.get_raw_value("core", None, "a").unwrap()],
            config.get_raw_multi_value("core", None, "a").unwrap()
        );
    }

    #[test]
    fn multi_value_in_section() {
        let config = GitConfig::from_str("[core]\na=b\na=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec!["b", "c"]
        );
    }

    #[test]
    fn multi_value_across_sections() {
        let config = GitConfig::from_str("[core]\na=b\na=c\n[core]a=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec!["b", "c", "d"]
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist("foo"))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist("aaaaaa"))
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::from_str("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec!["b"]
        );
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a").unwrap(),
            vec!["c"]
        );
    }

    #[test]
    fn non_relevant_subsection_is_ignored() {
        let config = GitConfig::from_str("[core]\na=b\na=c\n[core]a=d\n[core]g=g").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec!["b", "c", "d"]
        );
    }
}
