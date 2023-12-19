#![allow(clippy::result_large_err)]
mod util;
use util::*;

mod clone;
mod commit;
mod config;
#[cfg(feature = "blob-diff")]
mod diff;
mod head;
mod id;
mod init;
mod object;
mod reference;
mod remote;
mod repository;
#[cfg(feature = "revision")]
mod revision;
#[cfg(feature = "attributes")]
mod submodule;
