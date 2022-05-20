//! This module provides a high level wrapper around a single `git-config` file.

pub mod git_config;
mod resolved;
mod section;
mod value;

use std::ops::{Add, AddAssign};

pub use resolved::*;
pub use section::*;
pub use value::*;

pub use self::git_config::*;

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
