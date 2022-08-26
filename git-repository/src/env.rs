use crate::bstr::{BString, ByteVec};
use std::ffi::{OsStr, OsString};

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

/// Convert the given `input` into a `BString`, useful as `parse(try_from_os_str = <me>)` function.
pub fn os_str_to_bstring(input: &OsStr) -> Result<BString, String> {
    Vec::from_os_string(input.into())
        .map(Into::into)
        .map_err(|_| input.to_string_lossy().into_owned())
}
