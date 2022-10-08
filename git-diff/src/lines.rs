use git_object::bstr::BStr;
/// The crate powering file diffs.
pub use similar;
pub use similar::Algorithm;
use similar::TextDiff;

/// Provide an iterator over the changes needed to turn `old` into `new` with `algorithm`.
///
/// See [the `similar` crate documentation][similar::TextDiffConfig::diff_lines()] for information on how to use the iterator.
pub fn with<'old, 'new, 'bufs>(
    old: &'old BStr,
    new: &'new BStr,
    algorithm: Algorithm,
) -> TextDiff<'old, 'new, 'bufs, [u8]> {
    similar::TextDiffConfig::default()
        .algorithm(algorithm)
        .diff_lines(old.as_ref(), new.as_ref())
}

/// Provide an iterator over the changes needed to turn `old` into `new` with Myers algorithm, the default for `git`.
///
/// See [the `similar` crate documentation][similar::TextDiffConfig::diff_lines()] for information on how to use the iterator.
pub fn myers<'old, 'new, 'bufs>(old: &'old BStr, new: &'new BStr) -> TextDiff<'old, 'new, 'bufs, [u8]> {
    with(old, new, Algorithm::Myers)
}
