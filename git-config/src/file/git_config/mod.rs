use std::{borrow::Cow, collections::HashMap};

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
pub(crate) struct SectionId(pub(crate) usize);

/// Internal data structure for the section id lookup tree used by
/// [`GitConfig`]. Note that order in Vec matters as it represents the order
/// of section ids with the matched section and name, and is used for precedence
/// management.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<Cow<'a, str>, Vec<SectionId>>),
}

pub mod from_env;
pub use from_env::functions::{from_env, from_env_paths};

mod resolve_includes;
pub(crate) use resolve_includes::resolve_includes;

pub mod from_paths;

mod access;
mod impls;
mod init;
mod utils;

#[cfg(test)]
mod try_from_str_tests;
