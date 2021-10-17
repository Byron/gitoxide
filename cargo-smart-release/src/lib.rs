#![deny(rust_2018_idioms)]

pub use context::Context;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChangeLog {
    pub sections: Vec<changelog::Section>,
}

pub mod changelog;
pub mod command;
pub mod commit;

pub mod bat;
mod context;
pub mod git;
pub mod traverse;
mod utils;
pub mod version;
