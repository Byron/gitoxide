use git_repository as git;

use crate::remote;

#[test]
fn push_defaults_to_fetch() -> crate::Result {
    let repo = remote::repo("many-fetchspecs");
    let head = repo.head()?;
    let branch = head.clone().try_into_referent().expect("history");
    assert_eq!(
        branch
            .remote_name(git::remote::Direction::Push)
            .expect("fallback to fetch"),
        branch.remote_name(git::remote::Direction::Fetch).expect("configured"),
        "push falls back to fetch"
    );
    assert_eq!(
        branch
            .remote(git::remote::Direction::Push)
            .expect("configured")?
            .name()
            .expect("set")
            .as_bstr(),
        "origin"
    );
    assert_eq!(
        head.into_remote(git::remote::Direction::Push)
            .expect("same with branch")?
            .name()
            .expect("set")
            .as_bstr(),
        "origin"
    );
    Ok(())
}

#[test]
fn separate_push_and_fetch() -> crate::Result {
    for name in ["push-default", "branch-push-remote"] {
        let repo = remote::repo(name);
        let head = repo.head()?;
        let branch = head.clone().try_into_referent().expect("history");

        assert_eq!(
            branch
                .remote_name(git::remote::Direction::Push)
                .expect("set")
                .as_symbol()
                .unwrap(),
            "myself"
        );
        assert_eq!(
            branch
                .remote_name(git::remote::Direction::Fetch)
                .expect("set")
                .as_symbol()
                .unwrap(),
            "new-origin"
        );

        assert_ne!(
            branch.remote(git::remote::Direction::Push).transpose()?,
            branch.remote(git::remote::Direction::Fetch).transpose()?
        );
        assert_ne!(
            head.clone().into_remote(git::remote::Direction::Push).transpose()?,
            head.into_remote(git::remote::Direction::Fetch).transpose()?
        );
    }
    Ok(())
}

#[test]
fn not_configured() -> crate::Result {
    let repo = remote::repo("base");
    let head = repo.head()?;
    let branch = head.clone().try_into_referent().expect("history");

    assert_eq!(branch.remote_name(git::remote::Direction::Push), None);
    assert_eq!(branch.remote_name(git::remote::Direction::Fetch), None);
    assert_eq!(branch.remote(git::remote::Direction::Fetch).transpose()?, None);
    assert_eq!(head.into_remote(git::remote::Direction::Fetch).transpose()?, None);

    Ok(())
}

#[test]
fn dot_remote_behind_symbol() -> crate::Result {
    let repo = remote::repo("branch-dot-remote");
    let head = repo.head()?;
    let branch = head.clone().try_into_referent().expect("history");

    assert_eq!(
        branch
            .remote_name(git::remote::Direction::Push)
            .expect("derived push")
            .as_url(),
        Some(".".into())
    );
    assert_eq!(
        branch
            .remote_name(git::remote::Direction::Fetch)
            .expect("fetch")
            .as_url(),
        Some(".".into())
    );

    {
        let remote = branch
            .remote(git::remote::Direction::Push)
            .transpose()?
            .expect("present");
        assert_eq!(remote.name(), None, "It's a url after all, anonymous");
        assert_eq!(remote.url(git::remote::Direction::Push).unwrap().path, ".");
        assert_eq!(remote.url(git::remote::Direction::Fetch).unwrap().path, ".");
    }

    Ok(())
}

#[test]
fn url_as_remote_name() -> crate::Result {
    let repo = remote::repo("remote-as-url");
    let branch = repo.head_ref()?.expect("history");

    assert_eq!(
        branch
            .remote_name(git::remote::Direction::Push)
            .expect("set")
            .as_url()
            .unwrap(),
        "https://example.com/push-path.git",
        "remote names can also be urls"
    );
    assert_eq!(
        branch
            .remote_name(git::remote::Direction::Fetch)
            .expect("set")
            .as_url()
            .unwrap(),
        "https://example.com/fetch-path.git"
    );
    {
        let remote = branch
            .remote(git::remote::Direction::Push)
            .transpose()?
            .expect("present");
        assert_eq!(remote.name(), None, "It's a url after all, anonymous");
        assert_eq!(remote.url(git::remote::Direction::Push).unwrap().path, "/push-path.git");
        assert_eq!(
            remote.url(git::remote::Direction::Fetch).unwrap().path,
            "/push-path.git",
            "this is an anonymous remote with just a single url configured"
        );
    }
    {
        let remote = branch
            .remote(git::remote::Direction::Fetch)
            .transpose()?
            .expect("present");
        assert_eq!(remote.name(), None, "It's a url after all, anonymous");
        assert_eq!(
            remote.url(git::remote::Direction::Fetch).unwrap().path,
            "/fetch-path.git",
            "anonymous remotes have a single url only"
        );
        assert_eq!(
            remote.url(git::remote::Direction::Push).unwrap().path,
            "/fetch-path.git"
        );
    }
    Ok(())
}
