use git_object::bstr::BStr;
/// The crate powering file diffs.
pub use imara_diff;
pub use imara_diff::Algorithm;

/// Provide an iterator over the changes needed to turn `old` into `new` with `algorithm` into `sink`.
///
/// See [the `imara-diff` crate documentation][imara_diff] for information on how to implement a [`Sink`][imara_diff::Sink].
pub fn with<S>(old: &BStr, new: &BStr, algorithm: Algorithm, sink: S) -> S::Out
where
    S: imara_diff::Sink,
{
    let input = imara_diff::intern::InternedInput::new(old.as_ref(), new.as_ref());
    imara_diff::diff(algorithm, &input, sink)
}
