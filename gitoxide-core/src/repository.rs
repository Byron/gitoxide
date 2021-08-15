use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};

pub fn init(directory: Option<PathBuf>) -> Result<()> {
    git_repository::init::repository(directory.unwrap_or_default()).with_context(|| "Repository initialization failed")
}
