pub(crate) mod driver;
pub(crate) mod eol;
mod ident;
mod pipeline;
mod worktree;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
