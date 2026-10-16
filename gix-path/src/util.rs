use std::path::Path;

use bstr::{BStr, ByteSlice};

/// Return the leading Windows drive prefix, including `:`, if present, on any platform.
/// Virtual drives created with `subst` can use any Unicode character instead of an ASCII letter.
pub fn windows_drive_prefix(path: &BStr) -> Option<&BStr> {
    let (_, end, character) = path.char_indices().nth(1)?;
    (character == ':').then(|| path[..end].as_bstr())
}

/// return true if `path` is absolute, which depends on the platform but is always true if it starts with a `slash`, hence looks like
/// a linux path.
pub fn is_absolute(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    path.is_absolute() || path.to_str().and_then(|s| s.chars().next()) == Some('/')
}
