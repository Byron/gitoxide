use std::collections::HashMap;

use crate::parser::{parse_from_str, Event, Parser, ParserError};

#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
struct SectionId(usize);

#[derive(Debug, PartialEq, Eq)]
enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<&'a str, Vec<SectionId>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GitConfigError<'a> {
    SectionDoesNotExist(&'a str),
    SubSectionDoesNotExist(Option<&'a str>),
    KeyDoesNotExist(&'a str),
}

/// High level `git-config` reader and writer.
pub struct GitConfig<'a> {
    front_matter_events: Vec<Event<'a>>,
    section_lookup_tree: HashMap<&'a str, Vec<LookupTreeNode<'a>>>,
    sections: HashMap<SectionId, Vec<Event<'a>>>,
    section_header_separators: HashMap<SectionId, Option<&'a str>>,
    section_id_counter: usize,
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
        };

        // Current section that we're building
        let mut current_section_name: Option<&str> = None;
        let mut current_subsection_name: Option<&str> = None;
        let mut maybe_section: Option<Vec<Event<'a>>> = None;

        for event in parser.into_iter() {
            match event {
                e @ Event::Comment(_) => match maybe_section {
                    Some(ref mut section) => section.push(e),
                    None => new_self.front_matter_events.push(e),
                },
                Event::SectionHeader(header) => {
                    new_self.push_section(
                        &mut current_section_name,
                        &mut current_subsection_name,
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
                e @ Event::Key(_) => maybe_section
                    .as_mut()
                    .expect("Got a section-only event before a section")
                    .push(e),
                e @ Event::Value(_) => maybe_section
                    .as_mut()
                    .expect("Got a section-only event before a section")
                    .push(e),
                e @ Event::Newline(_) => match maybe_section {
                    Some(ref mut section) => section.push(e),
                    None => new_self.front_matter_events.push(e),
                },
                e @ Event::ValueNotDone(_) => maybe_section
                    .as_mut()
                    .expect("Got a section-only event before a section")
                    .push(e),
                e @ Event::ValueDone(_) => maybe_section
                    .as_mut()
                    .expect("Got a section-only event before a section")
                    .push(e),
                e @ Event::Whitespace(_) => match maybe_section {
                    Some(ref mut section) => section.push(e),
                    None => new_self.front_matter_events.push(e),
                },
            }
        }

        new_self.push_section(
            &mut current_section_name,
            &mut current_subsection_name,
            &mut maybe_section,
        );

        new_self
    }

    fn push_section(
        &mut self,
        current_section_name: &mut Option<&'a str>,
        current_subsection_name: &mut Option<&'a str>,
        maybe_section: &mut Option<Vec<Event<'a>>>,
    ) {
        let new_section_id = SectionId(self.section_id_counter);
        if let Some(section) = maybe_section.take() {
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
                            .entry(subsection_name)
                            .or_default()
                            .push(new_section_id);
                        break;
                    }
                }
                if !found_node {
                    let mut map = HashMap::new();
                    map.insert(*subsection_name, vec![new_section_id]);
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
            self.section_id_counter += 1;
        }
    }

    /// Returns an uninterpreted value given a section and optional subsection
    /// and key.
    ///
    /// Note that `git-config` follows a "last-one-wins" rule for single values.
    /// If multiple sections contain the same key, then the last section's last
    /// key's value will be returned.
    ///
    /// Concretely, if you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Then this function will return `d`:
    ///
    /// ```
    /// # use serde_git_config::config::GitConfig;
    /// # let git_config = GitConfig::from_str("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(git_config.get_raw_single_value("core", None, "a"), Ok("d"));
    /// ```
    ///
    /// The the resolution is as follows
    pub fn get_raw_single_value<'b>(
        &self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<&'a str, GitConfigError<'b>> {
        // Note: cannot wrap around the raw_multi_value method because we need
        // to guarantee that the highest section id is used (so that we follow
        // the "last one wins" resolution strategy by `git-config`).
        let section_id = self
            .get_section_id_by_name_and_subname(section_name, subsection_name)
            .ok_or(GitConfigError::SubSectionDoesNotExist(subsection_name))?;

        // section_id is guaranteed to exist in self.sections, else we have a
        // violated invariant.
        let events = self.sections.get(&section_id).unwrap();
        let mut found_key = false;
        let mut latest_value = None;
        // logic needs fixing for last one wins rule
        for event in events {
            match event {
                Event::Key(event_key) if *event_key == key => found_key = true,
                Event::Value(v) if found_key => {
                    found_key = false;
                    latest_value = Some(*v);
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
    ) -> Option<SectionId> {
        self.get_section_ids_by_name_and_subname(section_name, subsection_name)
            .map(|vec| vec.into_iter().max())
            .flatten()
    }

    pub fn get_raw_multi_value<'b>(
        &'a self,
        section_name: &'b str,
        subsection_name: Option<&'b str>,
        key: &'b str,
    ) -> Result<Vec<&'a str>, GitConfigError<'b>> {
        let values = self
            .get_section_ids_by_name_and_subname(section_name, subsection_name)
            .ok_or(GitConfigError::SubSectionDoesNotExist(subsection_name))?
            .iter()
            .map(|section_id| {
                let mut found_key = false;
                // section_id is guaranteed to exist in self.sections, else we have a
                // violated invariant.
                for event in self.sections.get(section_id).unwrap() {
                    match event {
                        Event::Key(event_key) if *event_key == key => found_key = true,
                        Event::Value(v) if found_key => return Ok(*v),
                        _ => (),
                    }
                }

                Err(GitConfigError::KeyDoesNotExist(key))
            })
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

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
    ) -> Option<Vec<SectionId>> {
        let section_ids = self.section_lookup_tree.get(section_name)?;
        if let Some(subsect_name) = subsection_name {
            let mut maybe_ids = None;
            for node in section_ids {
                if let LookupTreeNode::NonTerminal(subsection_lookup) = node {
                    maybe_ids = subsection_lookup.get(subsect_name);
                    break;
                }
            }
            maybe_ids.map(|vec| vec.clone())
        } else {
            let mut maybe_ids = None;
            for node in section_ids {
                if let LookupTreeNode::Terminal(subsection_lookup) = node {
                    maybe_ids = subsection_lookup.iter().max();
                    break;
                }
            }
            maybe_ids.map(|v| vec![*v])
        }
    }
}

#[cfg(test)]
mod git_config {
    mod from_parser {
        use super::super::*;

        #[test]
        fn parse_empty() {
            let config = GitConfig::from_str("").unwrap();
            assert!(config.section_header_separators.is_empty());
            assert_eq!(config.section_id_counter, 0);
            assert!(config.section_lookup_tree.is_empty());
            assert!(config.sections.is_empty());
        }

        #[test]
        fn parse_single_section() {
            let config = GitConfig::from_str("[core]\na=b\nc=d").unwrap();
            let expected_separators = {
                let mut map = HashMap::new();
                map.insert(SectionId(0), None);
                map
            };
            assert_eq!(config.section_header_separators, expected_separators);
            assert_eq!(config.section_id_counter, 1);
            let expected_lookup_tree = {
                let mut tree = HashMap::new();
                tree.insert("core", vec![LookupTreeNode::Terminal(vec![SectionId(0)])]);
                tree
            };
            assert_eq!(config.section_lookup_tree, expected_lookup_tree);
            let expected_sections = {
                let mut sections = HashMap::new();
                sections.insert(
                    SectionId(0),
                    vec![
                        Event::Newline("\n"),
                        Event::Key("a"),
                        Event::Value("b"),
                        Event::Newline("\n"),
                        Event::Key("c"),
                        Event::Value("d"),
                    ],
                );
                sections
            };
            assert_eq!(config.sections, expected_sections);
        }

        #[test]
        fn parse_single_subsection() {}

        #[test]
        fn parse_multiple_sections() {}

        #[test]
        fn parse_multiple_duplicate_sections() {}
    }
}
