use crate::bstr::{BStr, ByteSlice};

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
/// Returns `true` if the negotiation is done from our side so the server won't keep asking.
pub(crate) fn one_round(
    algo: Algorithm,
    round: usize,
    remote: &crate::Remote<'_>,
    ref_map: &crate::remote::fetch::RefMap<'_>,
    arguments: &mut git_protocol::fetch::Arguments,
    _previous_response: Option<&git_protocol::fetch::Response>,
) -> Result<bool, Error> {
    let repo = remote.repo;
    match algo {
        Algorithm::Naive => {
            assert_eq!(round, 1, "Naive always finishes after the first round, and claims.");
            let mut has_missing_tracking_branch = false;
            for mapping in &ref_map.mappings {
                let have_id = mapping.local.as_ref().and_then(|name| {
                    repo.find_reference(name)
                        .ok()
                        .and_then(|r| r.target().try_id().map(ToOwned::to_owned))
                });
                match have_id {
                    Some(have_id) if mapping.remote.as_id() != have_id => {
                        arguments.want(mapping.remote.as_id());
                        arguments.have(have_id);
                    }
                    Some(_) => {}
                    None => {
                        arguments.want(mapping.remote.as_id());
                        has_missing_tracking_branch = true;
                    }
                }
            }

            if has_missing_tracking_branch {
                let our_url = remote
                    .url(crate::remote::Direction::Fetch)
                    .expect("url present or we wouldn't be here");
                let our_repo_name = strip_git_suffix(our_url.path.as_ref());
                dbg!(our_repo_name);
                for other_remote in repo.remote_names().iter().filter_map(|name| match remote.name() {
                    Some(our_name) if our_name == *name => None,
                    Some(_) | None => repo.find_remote(name).ok(),
                }) {
                    if let Some(other_url) = other_remote.url(crate::remote::Direction::Fetch) {
                        if strip_git_suffix(other_url.path.as_ref()) == our_repo_name {
                            dbg!(&other_url, &our_url);
                        }
                    }
                }
            }
            Ok(true)
        }
    }
}

fn strip_git_suffix(repo_path: &BStr) -> &BStr {
    let repo_path = repo_path.strip_suffix(b"/.git").unwrap_or(repo_path);
    let repo_path = repo_path
        .rfind_byte(b'/')
        .map(|slash| &repo_path[slash + 1..])
        .unwrap_or(repo_path);
    repo_path
        .strip_suffix(b".git")
        .map(Into::into)
        .unwrap_or(repo_path.into())
}

#[cfg(test)]
mod strip_git_suffix_tests {
    use super::strip_git_suffix;

    #[test]
    fn dot_git_dir() {
        assert_eq!(strip_git_suffix("a/repo/.git".into()), "repo");
    }

    #[test]
    fn dot_git_suffix() {
        assert_eq!(strip_git_suffix("/a/repo.git".into()), "repo");
    }

    #[test]
    fn no_git_suffix() {
        assert_eq!(strip_git_suffix("a/b/repo".into()), "repo");
    }
}
