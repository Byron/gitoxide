use crate::parser::{parse_from_str, Event, ParsedSectionHeader, Parser};
use crate::values::Value;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, io::Read};

type SectionConfig<'a> = HashMap<&'a str, Value<'a>>;

/// This struct provides a high level wrapper to access `git-config` file. This
/// struct exists primarily for reading a config rather than modifying it, as
/// it discards comments and unnecessary whitespace.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize)]
pub struct GitConfig<'a>(HashMap<&'a str, HashMap<&'a str, SectionConfig<'a>>>);

const EMPTY_MARKER: &str = "@"; // Guaranteed to not be a {sub,}section or name.

impl<'a> GitConfig<'a> {
    /// Attempts to construct a instance given a [`Parser`] instance.
    ///
    /// This is _not_ a zero-copy operation. Due to how partial values may be
    /// provided, we necessarily need to copy and store these values until we
    /// are done.
    pub fn try_from_parser_with_options(
        parser: Parser<'a>,
        options: ConfigOptions,
    ) -> Result<Self, ()> {
        Self::try_from_event_iter_with_options(parser.into_iter(), options)
    }

    pub fn try_from_event_iter_with_options(
        iter: impl Iterator<Item = Event<'a>>,
        options: ConfigOptions,
    ) -> Result<Self, ()> {
        let mut sections: HashMap<&'a str, HashMap<&'a str, SectionConfig<'a>>> = HashMap::new();
        let mut current_section_name = EMPTY_MARKER;
        let mut current_subsection_name = EMPTY_MARKER;
        let mut ignore_until_next_section = false;
        let mut current_key = EMPTY_MARKER;
        let mut value_scratch = String::new();

        for event in iter {
            match event {
                Event::Comment(_) => (),
                Event::SectionHeader(ParsedSectionHeader {
                    name,
                    subsection_name,
                }) => {
                    current_section_name = name;
                    match (sections.get_mut(name), options.on_duplicate_section) {
                        (Some(_), OnDuplicateBehavior::Error) => todo!(),
                        (Some(section), OnDuplicateBehavior::Overwrite) => {
                            section.clear();
                        }
                        (Some(_), OnDuplicateBehavior::KeepExisting) => {
                            ignore_until_next_section = true;
                        }
                        (None, _) => {
                            sections.insert(name, HashMap::default());
                        }
                    }

                    match subsection_name {
                        Some(v) => current_subsection_name = v,
                        None => {
                            current_subsection_name = EMPTY_MARKER;
                            continue;
                        }
                    };

                    // subsection parsing

                    match (
                        sections
                            .get_mut(current_section_name)
                            .unwrap() // Guaranteed to exist at this point
                            .get_mut(current_subsection_name),
                        options.on_duplicate_section,
                    ) {
                        (Some(_), OnDuplicateBehavior::Error) => todo!(),
                        (Some(section), OnDuplicateBehavior::Overwrite) => section.clear(),
                        (Some(_), OnDuplicateBehavior::KeepExisting) => {
                            ignore_until_next_section = true;
                        }
                        (None, _) => (),
                    }
                }
                _ if ignore_until_next_section => (),
                Event::Key(key) => {
                    current_key = key;
                }
                Event::Value(v) => {
                    Self::insert_value(
                        &mut sections,
                        current_section_name,
                        current_subsection_name,
                        current_key,
                        v,
                        options.on_duplicate_name,
                    )?;
                }
                Event::Newline(_) => (),
                Event::ValueNotDone(v) => value_scratch.push_str(v),
                Event::ValueDone(v) => {
                    let mut completed_value = String::new();
                    value_scratch.push_str(v);
                    std::mem::swap(&mut completed_value, &mut value_scratch);
                    Self::insert_value(
                        &mut sections,
                        current_section_name,
                        current_subsection_name,
                        current_key,
                        Value::from_string(completed_value),
                        options.on_duplicate_name,
                    )?;
                }
            }
        }

        Ok(Self(sections))
    }

    fn insert_value(
        map: &mut HashMap<&'a str, HashMap<&'a str, SectionConfig<'a>>>,
        section: &str,
        subsection: &str,
        key: &'a str,
        value: Value<'a>,
        on_dup: OnDuplicateBehavior,
    ) -> Result<(), ()> {
        let config = map.get_mut(section).unwrap().get_mut(subsection).unwrap();

        if config.contains_key(key) {
            match on_dup {
                OnDuplicateBehavior::Error => return Err(()),
                OnDuplicateBehavior::Overwrite => {
                    config.insert(key, value);
                }
                OnDuplicateBehavior::KeepExisting => (),
            }
        } else {
            config.insert(key, value);
        }

        Ok(())
    }

    pub fn get_section(&self, section_name: &str) -> Option<&SectionConfig<'_>> {
        self.get_subsection(section_name, EMPTY_MARKER)
    }

    pub fn get_section_value(&self, section_name: &str, key: &str) -> Option<&Value<'_>> {
        self.get_section(section_name)
            .map(|section| section.get(key))
            .flatten()
    }

    pub fn get_subsection(
        &self,
        section_name: &str,
        subsection_name: &str,
    ) -> Option<&SectionConfig<'_>> {
        self.0
            .get(section_name)
            .map(|subsections| subsections.get(subsection_name))
            .flatten()
    }

    pub fn get_subsection_value(
        &self,
        section_name: &str,
        subsection_name: &str,
        key: &str,
    ) -> Option<&Value<'_>> {
        self.get_subsection(section_name, subsection_name)
            .map(|section| section.get(key))
            .flatten()
    }
}

impl<'a> TryFrom<Parser<'a>> for GitConfig<'a> {
    type Error = ();

    fn try_from(parser: Parser<'a>) -> Result<Self, Self::Error> {
        Self::try_from_parser_with_options(parser, ConfigOptions::default())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ConfigOptions {
    on_duplicate_section: OnDuplicateBehavior,
    on_duplicate_name: OnDuplicateBehavior,
}

impl ConfigOptions {
    pub fn on_duplicate_section(&mut self, behavior: OnDuplicateBehavior) -> &mut Self {
        self.on_duplicate_section = behavior;
        self
    }

    pub fn on_duplicate_name(&mut self, behavior: OnDuplicateBehavior) -> &mut Self {
        self.on_duplicate_name = behavior;
        self
    }
}

/// [`GitConfig`]'s valid possible actions when encountering a duplicate section
/// or key name within a section.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum OnDuplicateBehavior {
    /// Fail the operation, returning an error instead. This is the strictest
    /// behavior, and is the default.
    Error,
    /// Discard any data we had before on the
    Overwrite,
    KeepExisting,
}

impl Default for OnDuplicateBehavior {
    fn default() -> Self {
        Self::Error
    }
}
