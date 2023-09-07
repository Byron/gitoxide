#[cfg(not(feature = "revparse-regex"))]
mod util;
#[cfg(not(feature = "revparse-regex"))]
use util::*;

#[cfg(not(feature = "revparse-regex"))]
mod clone;
#[cfg(not(feature = "revparse-regex"))]
mod commit;
#[cfg(not(feature = "revparse-regex"))]
mod config;
#[cfg(not(feature = "revparse-regex"))]
mod head;
#[cfg(not(feature = "revparse-regex"))]
mod id;
#[cfg(not(feature = "revparse-regex"))]
mod init;
#[cfg(not(feature = "revparse-regex"))]
mod object;
#[cfg(not(feature = "revparse-regex"))]
mod reference;
#[cfg(not(feature = "revparse-regex"))]
mod remote;
#[cfg(not(feature = "revparse-regex"))]
mod repository;
#[cfg(not(feature = "revparse-regex"))]
#[cfg(feature = "revision")]
mod revision;
#[cfg(not(feature = "revparse-regex"))]
mod submodule;
