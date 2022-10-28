use git_object::bstr::BStr;
/// The crate powering file diffs.
pub use imara_diff as imara;
pub use imara_diff::Algorithm;

/// Create a diff yielding the changes to turn `old` into `new` with `algorithm`. `make_input` obtains the `old` and `new`
/// byte buffers and produces an interner, which is then passed to `make_sink` for creating a processor over the changes.
///
/// See [the `imara-diff` crate documentation][imara] for information on how to implement a [`Sink`][imara::Sink].
pub fn with<'a, FnI, FnS, S>(
    old: &'a BStr,
    new: &'a BStr,
    algorithm: Algorithm,
    make_input: FnI,
    make_sink: FnS,
) -> (imara::intern::InternedInput<&'a [u8]>, S::Out)
where
    FnI: FnOnce(&'a [u8], &'a [u8]) -> imara::intern::InternedInput<&'a [u8]>,
    FnS: FnOnce(&imara::intern::InternedInput<&'a [u8]>) -> S,
    S: imara_diff::Sink,
{
    let input = make_input(old.as_ref(), new.as_ref());
    let sink = make_sink(&input);
    let out = imara::diff(algorithm, &input, sink);
    (input, out)
}
