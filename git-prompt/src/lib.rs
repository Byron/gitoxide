//! Git style prompting with support for `GIT_ASKPASS` and `askpass` program configuration.
//!
//! ### Compatibility
//!
//! This is a unix-only crate which will return with an error when trying to obtain any prompt on other platforms.
//! On those platforms it is common to have helpers which perform this task so it shouldn't be a problem.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

mod types;
pub use types::{Error, Mode, Options};

///
pub mod unix;
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
            mode: Mode::Visible,
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
            mode: Mode::Hidden,
            askpass: None,
        },
    )
}
