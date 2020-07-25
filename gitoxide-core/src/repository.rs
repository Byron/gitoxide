use anyhow::{Context as AnyhowContext, Result};

pub fn init() -> Result<()> {
    git_repository::init::repository().with_context(|| "Repository initialization failed")
}
