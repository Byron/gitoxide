use anyhow::{Context as AnyhowContext, Result};
use std::path::PathBuf;

pub fn init(directory: Option<PathBuf>) -> Result<()> {
    git_repository::init::repository(directory.unwrap_or_default()).with_context(|| "Repository initialization failed")
}
