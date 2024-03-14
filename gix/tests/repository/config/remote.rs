use gix::bstr::BStr;
use std::borrow::Cow;

use crate::remote;

fn remote_names<'a>(it: impl IntoIterator<Item = &'a str>) -> Vec<Cow<'a, BStr>> {
    it.into_iter().map(|n| Cow::Borrowed(n.into())).collect()
}

fn remote_name(name: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(name.into())
}

#[test]
fn remote_and_branch_names() {
    let repo = remote::repo("base");
    assert_eq!(repo.remote_names().len(), 0, "there are no remotes");
    assert_eq!(repo.branch_names().len(), 0, "there are no configured branches");
    assert_eq!(repo.remote_default_name(gix::remote::Direction::Fetch), None);
    assert_eq!(repo.remote_default_name(gix::remote::Direction::Push), None);

    let repo = remote::repo("clone");
    assert_eq!(
        Vec::from_iter(repo.remote_names().into_iter()),
        remote_names(["myself", "origin"])
    );
    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Fetch),
        Some(remote_name("origin"))
    );
    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Push),
        Some(remote_name("origin"))
    );
    assert_eq!(Vec::from_iter(repo.branch_names()), vec!["main"]);
}

#[test]
fn remote_default_name() {
    let repo = remote::repo("push-default");

    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Push),
        Some(remote_name("myself")),
        "overridden via remote.pushDefault"
    );

    assert_eq!(
        repo.remote_default_name(gix::remote::Direction::Fetch),
        None,
        "none if name origin, and there are multiple"
    );
}

mod branch_remote {
    use crate::util::named_subrepo_opts;
    use gix::config::tree::Push;
    use gix::remote;

    mod name {
        use crate::repository::config::remote::branch_remote::repo;
        use gix::remote;

        #[test]
        fn push() -> crate::Result {
            {
                let repo = repo("push-remote")?;

                assert_eq!(
                    repo.branch_remote_name("main", remote::Direction::Push)
                        .expect("Remote name exists")
                        .as_ref(),
                    "push-origin",
                    "branch.main.pushRemote is set"
                );
            }

            let repo = repo("push-remote-default")?;

            assert_eq!(
                repo.branch_remote_name("main", remote::Direction::Push)
                    .expect("Remote name exists")
                    .as_ref(),
                "push-origin",
                "remote.pushDefault is set"
            );

            Ok(())
        }
    }

    #[test]
    fn fetch() -> crate::Result {
        let repo = repo("fetch")?;

        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Fetch)
                .expect("Remote Merge ref exists")
                .expect("Remote Merge ref is valid")
                .shorten(),
            "main"
        );
        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Fetch)
                .expect("Remote Merge ref exists")
                .expect("Remote Merge ref is valid")
                .as_bstr(),
            "refs/remotes/remote_repo/main"
        );
        for direction in [remote::Direction::Fetch, remote::Direction::Push] {
            assert_eq!(
                repo.branch_remote_name("main", direction)
                    .expect("Remote name exists")
                    .as_ref(),
                "remote_repo"
            );
        }

        let merge_branch_invalid_msg = "The configured name of the remote ref to merge wasn't valid";
        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/broken".try_into()?, remote::Direction::Fetch)
                .expect("Remote Merge ref exists")
                .unwrap_err()
                .to_string(),
            merge_branch_invalid_msg
        );
        assert!(repo
            .branch_remote_ref_name("refs/heads/missing".try_into()?, remote::Direction::Fetch)
            .is_none());
        for direction in [remote::Direction::Fetch, remote::Direction::Push] {
            assert_eq!(
                repo.branch_remote_name("broken", direction).expect("is set").as_bstr(),
                "remote_repo"
            );
        }
        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/broken".try_into()?, remote::Direction::Fetch)
                .expect("err")
                .unwrap_err()
                .to_string(),
            "Could not get the remote reference to translate into the local tracking branch",
            "the merge ref is broken, hence there can't be a tracking ref",
        );

        Ok(())
    }

    #[test]
    fn push_default() -> crate::Result {
        let repo = repo("fetch")?;

        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .shorten(),
            "main",
            "by default, there is a 1:1 mapping due to `push.default=simple`"
        );

        for direction in [remote::Direction::Fetch, remote::Direction::Push] {
            assert_eq!(
                repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, direction)
                    .expect("exists")?
                    .as_bstr(),
                "refs/remotes/remote_repo/main",
                "this is a 'simple' mapping of an existing branch, using push.default=simple and the default refspec"
            );
        }

        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/broken".try_into()?, remote::Direction::Push).expect("has err").unwrap_err().to_string(),
            "Could not get the remote reference to translate into the local tracking branch",
            "push.default = simple, hence we need to verify the merge-branch is the same as us, but retrieving it fails",
        );

        Ok(())
    }

    #[test]
    fn push_mapped() -> crate::Result {
        let repo = repo("push-mapped")?;

        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .shorten(),
            "remapped-main",
            "the first matching push-spec maps the branch to another head"
        );

        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .as_bstr(),
            "refs/remotes/origin/remapped-main",
            "the first matching push-spec maps the branch to another head, then it's mapped with fetch-specs"
        );
        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Fetch)
                .expect("exists")?
                .as_bstr(),
            "refs/remotes/origin/main",
            "push.simple is set (or the default), hence it's a one-one mapping along with the standard refspec"
        );

        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/feature".try_into()?, remote::Direction::Fetch)
                .expect("exists")?
                .shorten(),
            "main",
            "branch.feature.merge=refs/heads/main is causing the fetch remote to be remapped"
        );

        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/feature".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .shorten(),
            "remapped-feature",
            "this branch is mapped with push-specs, so we don't run into a failing push.default"
        );

        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/feature".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .as_bstr(),
            "refs/remotes/origin/remapped-feature",
            "this branch is mapped with push-specs, then it's mapped with fetch-specs as well"
        );
        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/feature".try_into()?, remote::Direction::Fetch)
                .expect("exists")?
                .as_bstr(),
            "refs/remotes/origin/main",
            "remapping by branch.feature.merge=main, then mapped by refspec"
        );

        Ok(())
    }

    #[test]
    fn push_missing() -> crate::Result {
        let repo = repo("push-missing")?;

        assert!(
            repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .is_none(),
            "there were push specs, but none matched, and we don't regard the push.default in this case, so end up with no match"
        );

        assert!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .is_none(),
            "the same thing happens when getting the tracking branch - after all it depends on the remote reference"
        );

        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Fetch)
                .expect("exists")?
                .shorten(),
            "origin/main",
            "fetch specs are specified, hence we can get the tracking branch"
        );

        Ok(())
    }

    #[test]
    fn push_default_current() -> crate::Result {
        let mut repo = repo("push-default-current")?;

        for same_name_default in ["current", "matching"] {
            repo.config_snapshot_mut()
                .set_value(&Push::DEFAULT, same_name_default)?;
            assert_eq!(
                repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                    .expect("exists")?
                    .shorten(),
                "main",
                "there was no push spec, `branch.main.merge` points to another branch, but we have a config override"
            );
            assert_eq!(
                repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                    .expect("exists")?
                    .shorten(),
                "origin/main",
                "same as above, but retrieves the tracking branch that the remote reference would be tracked under"
            );
        }

        repo.config_snapshot_mut().set_value(&Push::DEFAULT, "upstream")?;
        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .shorten(),
            "other",
            "`branch.main.merge` is configured as `refs/heads/other`, which is what we use with `push.default=upstream`"
        );
        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .expect("exists")?
                .shorten(),
            "origin/other",
            "as above, but we convert it to the tracking branch"
        );

        repo.config_snapshot_mut().set_value(&Push::DEFAULT, "simple")?;
        assert_eq!(
            repo.branch_remote_ref_name("refs/heads/main".try_into()?, remote::Direction::Push).transpose()?,
            None,
            "simple requires that the upstream matches the current branch, which isn't the case as `branch.main.merge` points to 'other'"
        );
        assert_eq!(
            repo.branch_remote_tracking_ref_name("refs/heads/main".try_into()?, remote::Direction::Push)
                .transpose()?,
            None,
        );
        Ok(())
    }

    fn repo(name: &str) -> Result<gix::Repository, gix::open::Error> {
        named_subrepo_opts("make_remote_config_repos.sh", name, gix::open::Options::isolated())
    }
}
