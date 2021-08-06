//! This module provides a high level wrapper around a single `git-config` file.

mod error;
mod git_config;
mod section;
mod value;

pub use error::GitConfigError;
pub use git_config::GitConfig;
