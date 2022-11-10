//! Utilities to handle program arguments and other values of interest.
use std::ffi::{OsStr, OsString};

use crate::bstr::{BString, ByteVec};

/// Returns the name of the agent for identification towards a remote server as statically known when compiling the crate.
/// Suitable for both `git` servers and HTTP servers, and used unless configured otherwise.
///
/// Note that it's meant to be used in conjunction with [`protocol::fetch::agent()`][crate::protocol::fetch::agent()] which
/// prepends `git/`.
pub fn agent() -> &'static str {
    concat!("oxide-", env!("CARGO_PKG_VERSION"))
}

/// Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms.
#[cfg(not(target_vendor = "apple"))]
pub fn args_os() -> impl Iterator<Item = OsString> {
    std::env::args_os()
}

/// Equivalent to `std::env::args_os()`, but with precomposed unicode on MacOS and other apple platforms.
///
/// Note that this ignores `core.precomposeUnicode` as git-config isn't available yet. It's default enabled in modern git though.
#[cfg(target_vendor = "apple")]
pub fn args_os() -> impl Iterator<Item = OsString> {
    use unicode_normalization::UnicodeNormalization;
    std::env::args_os().map(|arg| match arg.to_str() {
        Some(arg) => arg.nfc().collect::<String>().into(),
        None => arg,
    })
}

/// Convert the given `input` into a `BString`, useful as `#[clap(parse(try_from_os_str = git::env::os_str_to_bstring))]` function.
pub fn os_str_to_bstring(input: &OsStr) -> Result<BString, String> {
    Vec::from_os_string(input.into())
        .map(Into::into)
        .map_err(|_| input.to_string_lossy().into_owned())
}
