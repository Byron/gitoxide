mod driver;
mod eol;
mod ident;
mod worktree;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
