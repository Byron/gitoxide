use std::collections::{HashMap, VecDeque};

use crate::{
    file::{SectionBody, SectionBodyId, SectionBodyIds},
    parse::section,
};

/// High level `git-config` reader and writer.
///
/// This is the full-featured implementation that can deserialize, serialize,
/// and edit `git-config` files without loss of whitespace or comments.
///
/// # 'multivar' behavior
///
/// `git` is flexible enough to allow users to set a key multiple times in
/// any number of identically named sections. When this is the case, the key
/// is known as a _"multivar"_. In this case, [`raw_value()`] follows the
/// "last one wins".
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
/// Calling methods that fetch or set only one value (such as [`raw_value()`])
/// key `a` with the above config will fetch `d` or replace `d`, since the last
/// valid config key/value pair is `a = d`:
///
/// ```
/// # use std::borrow::Cow;
/// # use std::convert::TryFrom;
/// # let git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
/// assert_eq!(git_config.raw_value("core", None, "a").unwrap().as_ref(), "d");
/// ```
///
/// Consider the `multi` variants of the methods instead, if you want to work
/// with all values.
///
/// [`raw_value()`]: Self::raw_value
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct File<'event> {
    /// The list of events that occur before an actual section. Since a
    /// `git-config` file prohibits global values, this vec is limited to only
    /// comment, newline, and whitespace events.
    pub(crate) frontmatter_events: crate::parse::FrontMatterEvents<'event>,
    /// Section name and subsection name to section id lookup tree.
    pub(crate) section_lookup_tree: HashMap<section::Name<'event>, Vec<SectionBodyIds<'event>>>,
    /// This indirection with the SectionId as the key is critical to flexibly
    /// supporting `git-config` sections, as duplicated keys are permitted.
    pub(crate) sections: HashMap<SectionBodyId, SectionBody<'event>>,
    /// A way to reconstruct the complete section being a header and a body.
    pub(crate) section_headers: HashMap<SectionBodyId, section::Header<'event>>,
    /// Internal monotonically increasing counter for section ids.
    pub(crate) section_id_counter: usize,
    /// Section order for output ordering.
    pub(crate) section_order: VecDeque<SectionBodyId>,
}
