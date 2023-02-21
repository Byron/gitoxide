pub enum Command {
    TracePath {
        /// The repo-relative path to the file to trace
        spec: gix::path::Spec,
    },
}

pub(crate) mod update;
pub use update::update;

mod command;
