#[derive(PartialEq, Eq, Debug)]
pub struct ChangeLog {
    pub sections: Vec<changelog::Section>,
}

pub mod changelog;
pub mod command;
pub mod commit;

mod context;
pub use context::Context;

pub mod git;
pub mod traverse;
mod utils;
