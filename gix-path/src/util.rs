use std::path::Path;

/// return true if `path` is absolute, which depends on the platform but is always true if it starts with a `slash`, hence looks like
/// a linux path.
pub fn is_absolute(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    path.is_absolute() || path.to_str().and_then(|s| s.chars().next()) == Some('/')
}
