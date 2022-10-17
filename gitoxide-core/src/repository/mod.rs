use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};
use git_repository as git;

pub fn init(directory: Option<PathBuf>) -> Result<git::discover::repository::Path> {
    git::create::into(
        directory.unwrap_or_default(),
        git::create::Kind::WithWorktree,
        git::create::Options::default(),
    )
    .with_context(|| "Repository initialization failed")
}

pub mod commit;
pub mod config;
mod credential;
pub use credential::function as credential;
#[cfg(feature = "blocking-client")]
pub mod clone;
pub mod exclude;
#[cfg(feature = "blocking-client")]
pub mod fetch;
#[cfg(feature = "blocking-client")]
pub use clone::function::clone;
#[cfg(feature = "blocking-client")]
pub use fetch::function::fetch;
pub mod index;
pub mod mailmap;
pub mod odb;
pub mod remote;
pub mod revision;
pub mod tree;
pub mod verify;
