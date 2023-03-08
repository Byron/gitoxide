use gix_refspec::RefSpec;

/// The way the negotiation is performed
#[derive(Copy, Clone)]
pub(crate) enum Algorithm {
    /// Our very own implementation that probably should be replaced by one of the known algorithms soon.
    Naive,
}

/// The error returned during negotiation.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("We were unable to figure out what objects the server should send after {rounds} round(s)")]
    NegotiationFailed { rounds: usize },
}

/// Negotiate one round with `algo` by looking at `ref_map` and adjust `arguments` to contain the haves and wants.
/// If this is not the first round, the `previous_response` is set with the last recorded server response.
/// Returns `true` if the negotiation is done from our side so the server won't keep asking.
#[allow(clippy::too_many_arguments)]
pub(crate) fn one_round(
    algo: Algorithm,
    round: usize,
    repo: &crate::Repository,
    ref_map: &crate::remote::fetch::RefMap,
    fetch_tags: crate::remote::fetch::Tags,
    arguments: &mut gix_protocol::fetch::Arguments,
    _previous_response: Option<&gix_protocol::fetch::Response>,
    wants_shallow_change: Option<&[RefSpec]>,
) -> Result<bool, Error> {
    let tag_refspec_to_ignore = fetch_tags
        .to_refspec()
        .filter(|_| matches!(fetch_tags, crate::remote::fetch::Tags::Included));
    let non_wildcard_specs_only = wants_shallow_change;
    match algo {
        Algorithm::Naive => {
            assert_eq!(round, 1, "Naive always finishes after the first round, it claims.");
            let mut has_missing_tracking_branch = false;
            for mapping in &ref_map.mappings {
                if tag_refspec_to_ignore.map_or(false, |tag_spec| {
                    mapping
                        .spec_index
                        .implicit_index()
                        .and_then(|idx| ref_map.extra_refspecs.get(idx))
                        .map_or(false, |spec| spec.to_ref() == tag_spec)
                }) {
                    continue;
                }
                if non_wildcard_specs_only
                    .and_then(|refspecs| mapping.spec_index.get(refspecs, &ref_map.extra_refspecs))
                    .map_or(false, |spec| {
                        spec.to_ref().local().map_or(false, |ref_| ref_.contains(&b'*'))
                    })
                {
                    continue;
                }
                let have_id = mapping.local.as_ref().and_then(|name| {
                    repo.find_reference(name)
                        .ok()
                        .and_then(|r| r.target().try_id().map(ToOwned::to_owned))
                });
                match have_id {
                    Some(have_id) => {
                        if let Some(want_id) = mapping.remote.as_id() {
                            if want_id != have_id || wants_shallow_change.is_some() {
                                arguments.want(want_id);
                                arguments.have(have_id);
                            }
                        }
                    }
                    None => {
                        if let Some(want_id) = mapping.remote.as_id() {
                            arguments.want(want_id);
                            has_missing_tracking_branch = true;
                        }
                    }
                }
            }

            if has_missing_tracking_branch || (wants_shallow_change.is_some() && arguments.is_empty()) {
                if let Ok(Some(r)) = repo.head_ref() {
                    if let Some(id) = r.target().try_id() {
                        arguments.have(id);
                        arguments.want(id);
                    }
                }
            }
            Ok(true)
        }
    }
}
