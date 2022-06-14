use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};
use git_repository as git;

pub fn init(directory: Option<PathBuf>) -> Result<git::discover::repository::Path> {
    git_repository::create::into(directory.unwrap_or_default(), git::create::Options { bare: false })
        .with_context(|| "Repository initialization failed")
}

pub mod tree;

pub mod commit;

pub mod revision;

pub mod verify;

pub mod odb;

pub mod mailmap;

pub mod exclude;
