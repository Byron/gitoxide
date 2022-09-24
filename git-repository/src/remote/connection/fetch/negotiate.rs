#[derive(Copy, Clone)]
pub(crate) enum Algorithm {
    /// Our very own implementation that probably should be replaced by one of the known algorithms soon.
    Naive,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("We were unable to figure out what objects the server should send after {rounds} round(s)")]
    NegotiationFailed { rounds: usize },
}

/// Negotiate one round with `algo` by looking at `ref_map` and adjust `arguments` to contain the haves and wants.
/// If this is not the first round, the `previous_response` is set with the last recorded server response.
pub(crate) fn one_round(
    _algo: Algorithm,
    _round: usize,
    _repo: &crate::Repository,
    _ref_map: &crate::remote::fetch::RefMap<'_>,
    _arguments: &mut git_protocol::fetch::Arguments,
    _previous_response: Option<&git_protocol::fetch::Response>,
) -> Result<bool, Error> {
    todo!()
}
