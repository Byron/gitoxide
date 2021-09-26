#![deny(rust_2018_idioms)]

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChangeLog {
    pub sections: Vec<changelog::Section>,
}

pub mod changelog;
pub mod command;
pub mod commit;

mod context;
pub use context::Context;

pub mod bat;
pub mod git;
pub mod traverse;
mod utils;
