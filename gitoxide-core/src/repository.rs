use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};

pub fn init(directory: Option<PathBuf>) -> Result<git_repository::Path> {
    git_repository::init::into(directory.unwrap_or_default()).with_context(|| "Repository initialization failed")
}
