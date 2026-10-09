use gix::bstr::ByteSlice;

use crate::{Result, remote};

fn default_branch(repo: &gix::Repository, remote_name: &str) -> Result<Option<String>> {
    Ok(repo
        .find_remote(remote_name)?
        .default_branch()?
        .map(|name| name.as_bstr().to_string()))
}

#[test]
fn maps_the_remote_head_to_the_branch_on_the_remote() -> Result {
    let repo = remote::repo("remote-default-branch");
    let remote_repo = remote::repo("base");
    let remote_head_target = remote_repo.head_name()?.expect("the fixture has a symbolic HEAD");

    assert_eq!(
        default_branch(&repo, "origin")?.as_deref(),
        Some(remote_head_target.as_bstr().to_str()?),
        "`git clone` records the branch that the remote `HEAD` points to"
    );
    assert_eq!(
        default_branch(&repo, "other-head")?.as_deref(),
        Some("refs/heads/a"),
        "`git remote set-head` changes the record without looking at the remote, whose `HEAD` still points to `main`"
    );
    assert_eq!(
        default_branch(&repo, "renamed")?.as_deref(),
        Some("refs/heads/main"),
        "the fetch refspec maps the remote-tracking branch back to the branch on the remote, even if their names differ"
    );
    assert_eq!(
        default_branch(&repo, "team/origin")?.as_deref(),
        Some("refs/heads/main"),
        "remote names may contain slashes, which also nest their remote-tracking branches"
    );
    assert_eq!(
        default_branch(&repo, "dangling")?.as_deref(),
        Some("refs/heads/main"),
        "a `HEAD` whose remote-tracking branch was deleted still maps by name, like `git symbolic-ref` still prints it"
    );
    Ok(())
}

#[test]
fn preserves_fetch_sources_when_deriving_the_branch() -> Result {
    let repo = remote::repo("remote-default-branch");
    let remote_repo = remote::repo("base");
    let commit_id = remote_repo.find_reference("refs/heads/main")?.peel_to_id()?.to_string();
    let mut remote = repo.find_remote("origin")?;

    for (source, expected_branch) in [
        ("refs/heads/main", Some("refs/heads/main")),
        ("heads/main", Some("refs/heads/main")),
        ("refs/heads/*", Some("refs/heads/main")),
        ("heads/*", None),
        ("main", None),
        ("HEAD", None),
        ("@", None),
        ("", None),
        (commit_id.as_str(), None),
        ("refs/tags/b-tag", None),
        ("tags/b-tag", None),
        ("b-tag", None),
    ] {
        let destination = if source.contains('*') {
            "refs/remotes/origin/*"
        } else {
            "refs/remotes/origin/main"
        };
        let refspec = format!("+{source}:{destination}");
        remote.replace_refspecs(Some(refspec.as_str()), gix::remote::Direction::Fetch)?;

        assert_eq!(
            remote
                .default_branch()?
                .map(|name| name.as_bstr().to_string())
                .as_deref(),
            expected_branch,
            "`{refspec}` identifies a branch only when its source has a branch namespace"
        );
    }

    let remote = repo.find_remote("broad")?;
    assert_eq!(
        remote
            .default_branch()?
            .map(|name| name.as_bstr().to_string())
            .as_deref(),
        Some("refs/heads/main"),
        "a broad glob still identifies a branch when the reverse mapping has its full namespace"
    );
    Ok(())
}

#[test]
fn an_exact_source_identifies_a_branch_even_if_a_glob_produces_the_same_mapping() -> gix_testtools::TestResult {
    let repo = remote::repo("remote-default-branch");
    let mut remote = repo.find_remote("origin")?;
    for refspecs in [
        ["heads/*:refs/remotes/origin/*", "heads/main:refs/remotes/origin/main"],
        ["heads/main:refs/remotes/origin/main", "heads/*:refs/remotes/origin/*"],
    ] {
        remote.replace_refspecs(refspecs, gix::remote::Direction::Fetch)?;
        assert_eq!(
            remote
                .default_branch()?
                .map(|name| name.as_bstr().to_string())
                .as_deref(),
            Some("refs/heads/main"),
            "the exact source identifies the branch regardless of which duplicate mapping is retained"
        );
    }
    Ok(())
}

#[test]
fn fails_if_more_than_one_remote_reference_maps_to_the_remote_head_target() -> Result {
    let repo = remote::repo("remote-default-branch");
    let mut remote = repo.find_remote("ambiguous")?;
    let mut refspecs = vec![
        "+refs/heads/main:refs/remotes/ambiguous/main",
        "+refs/tags/main:refs/remotes/ambiguous/main",
    ];
    for num_mappings in [2, 3] {
        if num_mappings == 3 {
            refspecs.push("+refs/heads/a:refs/remotes/ambiguous/main");
        }
        remote.replace_refspecs(refspecs.iter().copied(), gix::remote::Direction::Fetch)?;
        let err = remote
            .default_branch()
            .expect_err("multiple remote references map to `refs/remotes/ambiguous/main`");
        let message = err.to_string();
        for ref_name in ["refs/heads/main", "refs/tags/main", "refs/remotes/ambiguous/main"] {
            assert!(message.contains(ref_name), "the error mentions `{ref_name}`: {message}");
        }
        assert!(
            message.contains(&format!("Found {num_mappings} mappings")),
            "the error counts all mappings, not just the two examples: {message}"
        );
    }
    Ok(())
}

#[test]
fn is_unknown_without_a_symbolic_remote_head_that_fetch_refspecs_map() -> Result {
    let repo = remote::repo("remote-default-branch");
    for (remote_name, reason) in [
        ("no-head", "there is no `refs/remotes/no-head/HEAD`"),
        ("detached", "`refs/remotes/detached/HEAD` isn't a symbolic reference"),
        (
            "unmapped",
            "no fetch refspec maps a remote reference to `refs/remotes/unmapped/main` anymore",
        ),
    ] {
        assert_eq!(default_branch(&repo, remote_name)?, None, "{reason}");
    }

    let anonymous = repo.remote_at("https://example.com/repo")?.with_refspecs(
        Some("+refs/heads/*:refs/remotes/origin/*"),
        gix::remote::Direction::Fetch,
    )?;
    assert_eq!(
        anonymous.default_branch()?,
        None,
        "without a name there is no `refs/remotes/<name>/HEAD`, even with the fetch refspecs of `origin`"
    );

    let repo = remote::repo("missing-urls");
    for remote_name in ["https://fallback.example/repo", "example.com:repo"] {
        assert_eq!(
            default_branch(&repo, remote_name)?,
            None,
            "`{remote_name}` can't be part of a reference name, so it can't have remote-tracking branches"
        );
    }
    Ok(())
}
