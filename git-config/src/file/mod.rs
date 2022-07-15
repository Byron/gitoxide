//! A high level wrapper around a single or multiple `git-config` file, for reading and mutation.
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Add, AddAssign},
};

use bstr::BStr;

mod mutable;

pub use mutable::{multi_value::MultiValueMut, section::SectionMut, value::ValueMut};

mod init;
pub use init::{from_env, from_paths};

mod access;
mod impls;
mod utils;

///
pub mod section;

///
pub mod resolve_includes {
    /// Options to handle includes, like `include.path` or `includeIf.<condition>.path`,
    #[derive(Clone, Copy)]
    pub struct Options<'a> {
        /// The maximum allowed length of the file include chain built by following nested resolve_includes where base level is depth = 0.
        pub max_depth: u8,
        /// When max depth is exceeded while following nested includes,
        /// return an error if true or silently stop following resolve_includes.
        ///
        /// Setting this value to false allows to read configuration with cycles,
        /// which otherwise always results in an error.
        pub error_on_max_depth_exceeded: bool,

        /// Used during path interpolation, both for include paths before trying to read the file, and for
        /// paths used in conditional `gitdir` includes.
        pub interpolate: crate::path::interpolate::Context<'a>,

        /// Additional context for conditional includes to work.
        pub conditional: conditional::Context<'a>,
    }

    impl Options<'_> {
        /// Provide options to never follow include directives at all.
        pub fn no_follow() -> Self {
            Options {
                max_depth: 0,
                error_on_max_depth_exceeded: false,
                interpolate: Default::default(),
                conditional: Default::default(),
            }
        }
    }

    impl<'a> Options<'a> {
        /// Provide options to follow includes like git does, provided the required `conditional` and `interpolate` contexts
        /// to support `gitdir` and `onbranch` based `includeIf` directives as well as standard `include.path` resolution.
        /// Note that the follow-mode is `git`-style, following at most 10 indirections while
        /// producing an error if the depth is exceeded.
        pub fn follow(
            interpolate: crate::path::interpolate::Context<'a>,
            conditional: conditional::Context<'a>,
        ) -> Self {
            Options {
                max_depth: 10,
                error_on_max_depth_exceeded: true,
                interpolate,
                conditional,
            }
        }

        /// Set the context used for interpolation when interpolating paths to include as well as the paths
        /// in `gitdir` conditional includes.
        pub fn interpolate_with(mut self, context: crate::path::interpolate::Context<'a>) -> Self {
            self.interpolate = context;
            self
        }
    }

    impl Default for Options<'_> {
        fn default() -> Self {
            Self::no_follow()
        }
    }

    ///
    pub mod conditional {
        /// Options to handle conditional includes like `includeIf.<condition>.path`.
        #[derive(Clone, Copy, Default)]
        pub struct Context<'a> {
            /// The location of the .git directory. If `None`, `gitdir` conditions cause an error.
            ///
            /// Used for conditional includes, e.g. `includeIf.gitdir:…` or `includeIf:gitdir/i…`.
            pub git_dir: Option<&'a std::path::Path>,
            /// The name of the branch that is currently checked out. If `None`, `onbranch` conditions cause an error.
            ///
            /// Used for conditional includes, e.g. `includeIf.onbranch:main.…`
            pub branch_name: Option<&'a git_ref::FullNameRef>,
        }
    }
}

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

/// A section in a git-config file, like `[core]` or `[remote "origin"]`, along with all of its keys.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Section<'a> {
    header: crate::parse::section::Header<'a>,
    body: section::Body<'a>,
}

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
pub(crate) struct SectionId(pub(crate) usize);

/// All section body ids referred to by a section name.
///
/// Note that order in Vec matters as it represents the order
/// of section ids with the matched section and name, and is used for precedence
/// management.
#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum SectionBodyIds<'a> {
    /// The list of section ids to use for obtaining the section body.
    Terminal(Vec<SectionId>),
    /// A hashmap from sub-section names to section ids.
    NonTerminal(HashMap<Cow<'a, BStr>, Vec<SectionId>>),
}
#[cfg(test)]
mod tests;
