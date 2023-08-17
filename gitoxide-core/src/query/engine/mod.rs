pub enum Command {
    TracePath {
        /// The repo-relative path to the file to trace
        spec: gix::pathspec::Pattern,
    },
}

pub(crate) mod update;

pub use update::update;

mod command;
