//! A high level wrapper around a single or multiple `git-config` file, for reading and mutation.
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Add, AddAssign},
};

use bstr::BStr;

mod mutable;

pub use mutable::{
    multi_value::MultiValueMut,
    section::{SectionBody, SectionBodyIter, SectionMut},
    value::ValueMut,
};

mod init;
pub use init::{from_env, from_paths};

///
pub mod rename_section {
    /// The error returned by [`File::rename_section(…)`][crate::File::rename_section()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Lookup(#[from] crate::lookup::existing::Error),
        #[error(transparent)]
        Section(#[from] crate::parse::section::header::Error),
    }
}

mod access;
mod impls;
mod utils;

/// A strongly typed index into some range.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone, Copy)]
pub(crate) struct Index(pub(crate) usize);

impl Add<Size> for Index {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// A stronlgy typed a size.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone, Copy)]
pub(crate) struct Size(pub(crate) usize);

impl AddAssign<usize> for Size {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

/// The section ID is a monotonically increasing ID used to refer to section bodies.
/// This value does not imply any ordering between sections, as new sections
/// with higher section IDs may be in between lower ID sections after `File` mutation.
///
/// We need to use a section id because `git-config` permits sections with
/// identical names, making it ambiguous when used in maps, for instance.
///
/// This id guaranteed to be unique, but not guaranteed to be compact. In other
/// words, it's possible that a section may have an ID of 3 but the next section
/// has an ID of 5 as 4 was deleted.
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
pub(crate) struct SectionBodyId(pub(crate) usize);

/// All section body ids referred to by a section name.
///
/// Note that order in Vec matters as it represents the order
/// of section ids with the matched section and name, and is used for precedence
/// management.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum SectionBodyIds<'a> {
    /// The list of section ids to use for obtaining the section body.
    Terminal(Vec<SectionBodyId>),
    /// A hashmap from sub-section names to section ids.
    NonTerminal(HashMap<Cow<'a, BStr>, Vec<SectionBodyId>>),
}
#[cfg(test)]
mod tests;
