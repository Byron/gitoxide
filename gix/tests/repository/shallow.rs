use serial_test::parallel;

use crate::util::{hex_to_id, named_subrepo_opts};

#[test]
#[parallel]
fn no() -> crate::Result {
    for name in ["base", "empty"] {
        let repo = named_subrepo_opts("make_shallow_repo.sh", name, crate::restricted())?;
        assert!(!repo.is_shallow());
        assert!(repo.shallow_commits()?.is_none());
        let commits: Vec<_> = repo
            .head_id()?
            .ancestors()
            .all()?
            .map(|c| c.map(|c| c.id))
            .collect::<Result<_, _>>()?;
        let expected = if name == "base" {
            vec![
                hex_to_id("30887839de28edf7ab66c860e5c58b4d445f6b12"),
                hex_to_id("d8523dfd5a7aa16562fa1c3e1d3b4a4494f97876"),
                hex_to_id("05dc291f5376cde200316cb0b74b00cfebc79ea4"),
            ]
        } else {
            vec![hex_to_id("05dc291f5376cde200316cb0b74b00cfebc79ea4")]
        };
        assert_eq!(commits, expected);
    }
    Ok(())
}

#[test]
#[parallel]
fn yes() -> crate::Result {
    for name in ["shallow.git", "shallow"] {
        let repo = named_subrepo_opts("make_shallow_repo.sh", name, crate::restricted())?;
        assert!(repo.is_shallow());
        assert_eq!(
            repo.shallow_commits()?.expect("present").as_slice(),
            [hex_to_id("30887839de28edf7ab66c860e5c58b4d445f6b12")]
        );
    }
    Ok(())
}

mod traverse {
    use gix_traverse::commit::Sorting;
    use serial_test::parallel;

    use crate::util::{hex_to_id, named_subrepo_opts};

    #[test]
    #[parallel]
    fn boundary_is_detected_triggering_no_error() -> crate::Result {
        for toggle in [false, true] {
            for name in ["shallow.git", "shallow"] {
                let repo = named_subrepo_opts("make_shallow_repo.sh", name, crate::restricted())?;
                let commits: Vec<_> = repo
                    .head_id()?
                    .ancestors()
                    .use_commit_graph(toggle)
                    .all()?
                    .map(|c| c.map(|c| c.id))
                    .collect::<Result<_, _>>()?;
                assert_eq!(commits, [hex_to_id("30887839de28edf7ab66c860e5c58b4d445f6b12")]);
            }
        }
        Ok(())
    }

    #[test]
    #[parallel]
    fn complex_graphs_can_be_iterated_despite_multiple_shallow_boundaries() -> crate::Result {
        let base =
            gix_path::realpath(&gix_testtools::scripted_fixture_read_only("make_remote_repos.sh")?.join("base"))?;
        let shallow_base = gix_testtools::scripted_fixture_read_only_with_args(
            "make_complex_shallow_repo.sh",
            Some(base.to_string_lossy()),
        )?;
        for toggle in [false, true] {
            for name in ["shallow.git", "shallow"] {
                let repo = gix::open_opts(shallow_base.join(name), crate::restricted())?;
                assert_eq!(
                    repo.shallow_commits()?.expect("present").as_slice(),
                    [
                        hex_to_id("27e71576a6335294aa6073ab767f8b36bdba81d0"),
                        hex_to_id("82024b2ef7858273337471cbd1ca1cedbdfd5616"),
                        hex_to_id("b5152869aedeb21e55696bb81de71ea1bb880c85"),
                    ]
                );
                let commits: Vec<_> = repo
                    .head_id()?
                    .ancestors()
                    .use_commit_graph(toggle)
                    .sorting(Sorting::ByCommitTimeNewestFirst)
                    .all()?
                    .map(|c| c.map(|c| c.id))
                    .collect::<Result<_, _>>()?;
                assert_eq!(
                    commits,
                    [
                        "f99771fe6a1b535783af3163eba95a927aae21d5",
                        "2d9d136fb0765f2e24c44a0f91984318d580d03b",
                        "dfd0954dabef3b64f458321ef15571cc1a46d552",
                        "b5152869aedeb21e55696bb81de71ea1bb880c85",
                        "27e71576a6335294aa6073ab767f8b36bdba81d0",
                        "82024b2ef7858273337471cbd1ca1cedbdfd5616",
                    ]
                    .into_iter()
                    .map(hex_to_id)
                    .collect::<Vec<_>>()
                );

                // should be
                // *   f99771f - (HEAD -> main, origin/main, origin/HEAD) A (18 years ago) <A U Thor>
                // | * 2d9d136 - C (18 years ago) <A U Thor>
                // *-. | dfd0954 - (tag: b-tag) B (18 years ago) <A U Thor>
                // | | * 27e7157 - (grafted) F (18 years ago) <A U Thor>
                //     | * b515286 - (grafted) E (18 years ago) <A U Thor>
                //     * 82024b2 - (grafted) D (18 years ago) <A U Thor>
            }
        }
        Ok(())
    }
}
