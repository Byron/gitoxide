use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};
use git_repository as git;

pub fn init(directory: Option<PathBuf>) -> Result<git::discover::repository::Path> {
    git_repository::create::into(
        directory.unwrap_or_default(),
        git::create::Options {
            bare: false,
            fs_capabilities: None,
        },
    )
    .with_context(|| "Repository initialization failed")
}

pub mod commit;
pub mod config;
pub mod exclude;
pub mod mailmap;
pub mod odb;
pub mod remote;
pub mod revision;
pub mod tree;
pub mod verify;
