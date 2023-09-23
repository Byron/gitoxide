use std::path::PathBuf;

use anyhow::{Context as AnyhowContext, Result};
use gix::bstr::BString;

pub fn init(directory: Option<PathBuf>) -> Result<gix::discover::repository::Path> {
    gix::create::into(
        directory.unwrap_or_default(),
        gix::create::Kind::WithWorktree,
        gix::create::Options::default(),
    )
    .with_context(|| "Repository initialization failed")
}

pub enum PathsOrPatterns {
    Paths(Box<dyn std::iter::Iterator<Item = BString>>),
    Patterns(Vec<BString>),
}

#[cfg(feature = "archive")]
pub mod archive;
pub mod commit;
pub mod config;
mod credential;
pub use credential::function as credential;
pub mod attributes;
#[cfg(feature = "blocking-client")]
pub mod clone;
pub mod exclude;
#[cfg(feature = "blocking-client")]
pub mod fetch;
#[cfg(feature = "blocking-client")]
pub use clone::function::clone;
#[cfg(feature = "blocking-client")]
pub use fetch::function::fetch;

pub mod commitgraph;
pub mod index;
pub mod mailmap;
pub mod odb;
pub mod remote;
pub mod revision;
pub mod status;
pub mod submodule;
pub mod tree;
pub mod verify;
