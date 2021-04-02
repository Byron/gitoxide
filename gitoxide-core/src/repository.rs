use anyhow::{Context as AnyhowContext, Result};
use std::path::PathBuf;

pub fn init(directory: Option<PathBuf>) -> Result<()> {
    git_repository::init::repository(directory).with_context(|| "Repository initialization failed")
}
