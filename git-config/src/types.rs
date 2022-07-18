use git_features::threading::OwnShared;
use std::collections::{HashMap, VecDeque};

use crate::file::Metadata;
use crate::{
    color, file,
    file::{SectionBodyIds, SectionId},
    integer,
    parse::section,
};

/// A list of known sources for configuration files, with the first one being overridden
/// by the second one, and so forth.
/// Note that included files via `include.path` and `includeIf.<condition>.path` inherit
/// their source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Source {
    /// System-wide configuration path. This is defined as
    /// `$(prefix)/etc/gitconfig` (where prefix is the git-installation directory).
    System,
    /// Also known as the user configuration path. This is usually `~/.gitconfig`.
    Global,
    /// Second user-specific configuration path; if `$XDG_CONFIG_HOME` is not
    /// set or empty, `$HOME/.config/git/config` will be used.
    User,
    /// The configuration of the repository itself, located in `.git/config`.
    Local,
    /// Configuration specific to a worktree as created with `git worktree` and
    /// typically located in `$GIT_DIR/config.worktree` if `extensions.worktreeConfig`
    /// is enabled.
    Worktree,
    /// values parsed from the environment.
    Env,
    /// Values set from the command-line.
    Cli,
    /// Entirely internal from a programmatic source
    Api,
}

impl Source {
    /// Return true if the source indicates a location within a file of a repository.
    pub fn is_in_repository(self) -> bool {
        matches!(self, Source::Local | Source::Worktree)
    }
}

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
/// # Filtering
///
/// All methods exist in a `*_filter(…, filter)` version to allow skipping sections by
/// their metadata. That way it's possible to select values based on their `git_sec::Trust`
/// for example, or by their location.
///
/// Note that the filter may be executed even on sections that don't contain the key in question,
/// even though the section will have matched the `name` and `subsection_name` respectively.
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
    /// The list of events that occur before any section. Since a
    /// `git-config` file prohibits global values, this vec is limited to only
    /// comment, newline, and whitespace events.
    pub(crate) frontmatter_events: crate::parse::FrontMatterEvents<'event>,
    /// Frontmatter events to be placed after the given section.
    pub(crate) frontmatter_post_section: HashMap<SectionId, crate::parse::FrontMatterEvents<'event>>,
    /// Section name to section id lookup tree, with section bodies for subsections being in a non-terminal
    /// variant of `SectionBodyIds`.
    pub(crate) section_lookup_tree: HashMap<section::Name<'event>, Vec<SectionBodyIds<'event>>>,
    /// This indirection with the SectionId as the key is critical to flexibly
    /// supporting `git-config` sections, as duplicated keys are permitted.
    pub(crate) sections: HashMap<SectionId, file::Section<'event>>,
    /// Internal monotonically increasing counter for section ids.
    pub(crate) section_id_counter: usize,
    /// Section order for output ordering.
    pub(crate) section_order: VecDeque<SectionId>,
    /// The source of the File itself, which is attached to new sections automatically.
    pub(crate) meta: OwnShared<Metadata>,
}

/// Any value that may contain a foreground color, background color, a
/// collection of color (text) modifiers, or a combination of any of the
/// aforementioned values, like `red` or `brightgreen`.
///
/// Note that `git-config` allows color values to simply be a collection of
/// [`color::Attribute`]s, and does not require a [`color::Name`] for either the
/// foreground or background color.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Color {
    /// A provided foreground color
    pub foreground: Option<color::Name>,
    /// A provided background color
    pub background: Option<color::Name>,
    /// A potentially empty set of text attributes
    pub attributes: color::Attribute,
}

/// Any value that can be interpreted as an integer.
///
/// This supports any numeric value that can fit in a [`i64`], excluding the
/// suffix. The suffix is parsed separately from the value itself, so if you
/// wish to obtain the true value of the integer, you must account for the
/// suffix after fetching the value. [`integer::Suffix`] provides
/// [`bitwise_offset()`][integer::Suffix::bitwise_offset] to help with the
/// math, or [to_decimal()][Integer::to_decimal()] for obtaining a usable value in one step.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Integer {
    /// The value, without any suffix modification
    pub value: i64,
    /// A provided suffix, if any.
    pub suffix: Option<integer::Suffix>,
}

/// Any value that can be interpreted as a boolean.
///
/// Note that while values can effectively be any byte string, the `git-config`
/// documentation has a strict subset of values that may be interpreted as a
/// boolean value, all of which are ASCII and thus UTF-8 representable.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
pub struct Boolean(pub bool);

/// Any value that can be interpreted as a path to a resource on disk.
///
/// Git represents file paths as byte arrays, modeled here as owned or borrowed byte sequences.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Path<'a> {
    /// The path string, un-interpolated
    pub value: std::borrow::Cow<'a, bstr::BStr>,
}
