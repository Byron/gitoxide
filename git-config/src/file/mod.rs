//! This module provides a high level wrapper around a single `git-config` file.
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Add, AddAssign},
};

mod resolved;
pub use resolved::*;

mod section;
pub use section::*;

mod value;
pub use value::*;

/// Newtype to represent an index into some range. This is to differentiate
/// between raw usizes when multiple are present.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone, Copy)]
pub(crate) struct Index(pub(crate) usize);

impl Add<Size> for Index {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// Newtype to represent a size. This is to differentiate between raw usizes
/// when multiple are present.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone, Copy)]
pub(crate) struct Size(pub(crate) usize);

impl AddAssign<usize> for Size {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
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
pub(crate) struct SectionId(pub(crate) usize);

/// Internal data structure for the section id lookup tree used by
/// [`File`]. Note that order in Vec matters as it represents the order
/// of section ids with the matched section and name, and is used for precedence
/// management.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<Cow<'a, str>, Vec<SectionId>>),
}

pub mod from_env;

mod resolve_includes;
pub(crate) use resolve_includes::resolve_includes;

pub mod from_paths;

mod access;
mod impls;
mod init;
mod utils;

#[cfg(test)]
mod try_from_str_tests;
