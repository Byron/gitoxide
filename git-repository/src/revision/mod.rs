//! Revisions is the generalized notion of a commit.
//!
//! This module provides utilities to walk graphs of revisions and specify revisions and ranges of revisions.

pub use git_revision as plumbing;

///
pub mod walk;
pub use walk::iter::Walk;
