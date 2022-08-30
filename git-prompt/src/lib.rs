//! Git style prompting with support for `GIT_ASKPASS` and `askpass` program configuration.
//!
//! ### Compatibility
//!
//! This is a unix-only crate which will return with an error when trying to obtain any prompt on other platforms.
//! On those platforms it is common to have helpers which perform this task so it shouldn't be a problem.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

/// The error returned by [ask()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The current platform has no implementation for prompting in the terminal")]
    UnsupportedPlatform,
    #[error("Failed to open terminal at {:?} for writing prompt, or to write it", unix::TTY_PATH)]
    TtyIo(#[from] std::io::Error),
    #[cfg(unix)]
    #[error("Failed to obtain or set terminal configuration")]
    TerminalConfiguration(#[from] nix::errno::Errno),
}

/// The options used in `[ask()]`.
#[derive(Default, Copy, Clone)]
pub struct Options<'a> {
    /// The path or name (for lookup in `PATH`) to the askpass program to call before prompting the user.
    ///
    /// It's called like this `askpass <prompt>`, but note that it won't know if the input should be hidden or not.
    pub askpass: Option<&'a Path>,
    /// If true, what's prompted is a secret and thus should be hidden.
    pub secret: bool,
}

///
pub mod unix;

use std::path::Path;
#[cfg(unix)]
use unix::imp;

#[cfg(not(unix))]
mod imp {
    use crate::{Error, Options};

    pub(crate) fn ask(_prompt: &str, _opts: Options<'_>) -> Result<String, Error> {
        Err(Error::UnsupportedPlatform)
    }
}

/// Ask the user given a `prompt`, returning the result.
pub fn ask(prompt: &str, opts: Options<'_>) -> Result<String, Error> {
    imp::ask(prompt, opts)
}

/// Ask for information typed by the user into the terminal after showing the prompt`, like `"Username: `.
///
/// Use [`ask()`] for more control.
pub fn openly(prompt: impl AsRef<str>) -> Result<String, Error> {
    imp::ask(
        prompt.as_ref(),
        Options {
            secret: false,
            askpass: None,
        },
    )
}

/// Ask for information _securely_ after showing the `prompt` (like `"password: "`) by not showing what's typed.
///
/// Use [`ask()`] for more control.
pub fn securely(prompt: impl AsRef<str>) -> Result<String, Error> {
    imp::ask(
        prompt.as_ref(),
        Options {
            secret: true,
            askpass: None,
        },
    )
}
