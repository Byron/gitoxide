use std::cmp::Ordering;

use gix::{
    config::tree::{Core, Key},
    prelude::ObjectIdExt,
};
use gix_object::bstr::BString;

/// Convert a hexadecimal hash into its corresponding `ObjectId` or _panic_.
fn hex_to_id(hex: &str) -> gix_hash::ObjectId {
    gix_hash::ObjectId::from_hex(hex.as_bytes()).expect("40 bytes hex")
}

#[test]
fn prefix() -> crate::Result {
    let repo = crate::repo("make_repo_with_fork_and_dates.sh")?.to_thread_local();
    let work_dir = repo.work_dir().expect("non-bare");
    let id = hex_to_id("288e509293165cb5630d08f4185bdf2445bf6170").attach(&repo);
    let prefix = id.shorten()?;
    assert_eq!(prefix.cmp_oid(&id), Ordering::Equal);
    assert_eq!(prefix.hex_len(), 7, "preconfigured via core.abbrev default value");

    let repo = gix::open_opts(
        work_dir,
        gix::open::Options::isolated().config_overrides(Core::ABBREV.validated_assignment("5".into())),
    )
    .unwrap();
    let id = id.detach().attach(&repo);
    let prefix = id.shorten()?;
    assert_eq!(prefix.cmp_oid(&id), Ordering::Equal);
    assert_eq!(prefix.hex_len(), 5, "preconfigured via core.abbrev");

    assert!(
        gix::open_opts(
            work_dir,
            gix::open::Options::isolated().config_overrides(Some(BString::from("core.abbrev=invalid")))
        )
        .is_ok(),
        "By default gitoxide acts like `libgit2` here and we prefer to be lenient when possible"
    );

    assert!(
        matches!(
            gix::open_opts(
                work_dir,
                gix::open::Options::isolated()
                    .strict_config(true)
                    .config_overrides(Some(BString::from("core.abbrev=invalid")))
            )
            .unwrap_err(),
            gix::open::Error::Config(gix::config::Error::CoreAbbrev(_))
        ),
        "an empty core.abbrev fails the open operation in strict config mode, emulating git behaviour"
    );
    Ok(())
}

#[test]
fn display_and_debug() -> crate::Result {
    let repo = crate::basic_repo()?;
    let id = repo.head_id()?;
    assert_eq!(
        format!("{id} {id:?}"),
        "3189cd3cb0af8586c39a838aa3e54fd72a872a41 Sha1(3189cd3cb0af8586c39a838aa3e54fd72a872a41)"
    );
    Ok(())
}

mod ancestors {
    use gix_traverse::commit;

    use crate::id::hex_to_id;

    #[test]
    fn all() -> crate::Result {
        let repo = crate::repo("make_repo_with_fork_and_dates.sh")?.to_thread_local();
        for toggle in [false, true] {
            let head = repo.head()?.into_fully_peeled_id().expect("born")?;
            let commits_graph_order = head
                .ancestors()
                .use_commit_graph(toggle)
                .all()?
                .map(|c| c.map(|c| c.detach()))
                .collect::<Result<Vec<_>, _>>()?;
            assert_eq!(commits_graph_order.len(), 4, "need a specific amount of commits");

            let commits_by_commit_date = head
                .ancestors()
                .use_commit_graph(!toggle)
                .sorting(commit::Sorting::ByCommitTimeNewestFirst)
                .all()?
                .map(|c| c.map(|c| c.detach()))
                .collect::<Result<Vec<_>, _>>()?;
            assert_eq!(
                commits_by_commit_date.len(),
                4,
                "need a specific amount of commits, ordering doesn't affect that"
            );
            assert_ne!(
                commits_by_commit_date, commits_graph_order,
                "these are ordered differently"
            );

            assert_eq!(
                head.ancestors()
                    .first_parent_only()
                    .use_commit_graph(toggle)
                    .all()?
                    .count(),
                3,
                "It skips merges this way."
            );
        }
        Ok(())
    }

    #[test]
    fn filtered() -> crate::Result {
        let repo = crate::repo("make_repo_with_fork_and_dates.sh")?.to_thread_local();
        let head = repo.head()?.into_fully_peeled_id().expect("born")?;

        for use_commit_graph in [false, true] {
            for sorting in [
                commit::Sorting::BreadthFirst,
                commit::Sorting::ByCommitTimeNewestFirst,
                commit::Sorting::ByCommitTimeNewestFirstCutoffOlderThan {
                    time_in_seconds_since_epoch: 0,
                },
            ] {
                let commits_graph_order = head
                    .ancestors()
                    .sorting(sorting)
                    .use_commit_graph(use_commit_graph)
                    .selected(|id| {
                        let _assert_lifetime_works = &repo; // assure we can use repo here.
                        id != hex_to_id("9902e3c3e8f0c569b4ab295ddf473e6de763e1e7")
                            && id != hex_to_id("bcb05040a6925f2ff5e10d3ae1f9264f2e8c43ac")
                    })?
                    .map(|c| c.map(|c| c.id))
                    .collect::<Result<Vec<_>, _>>()?;
                assert_eq!(
                    commits_graph_order,
                    &[hex_to_id("288e509293165cb5630d08f4185bdf2445bf6170")],
                    "we ignore all but the first"
                );
            }
        }
        Ok(())
    }
}
