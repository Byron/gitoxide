use git::prelude::ObjectIdExt;
use git::RevSpec;
use git_object::bstr;
use git_object::bstr::BStr;
use git_ref::bstr::{BString, ByteSlice};
use git_repository as git;
use git_revision::spec::Kind;
use git_testtools::{hex_to_id, once_cell::sync::Lazy};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

const FIXTURE_NAME: &str = "make_rev_spec_parse_repos.sh";
static BASELINE: Lazy<HashMap<PathBuf, HashMap<BString, Option<git_revision::Spec>>>> = Lazy::new(|| {
    fn kind_of(spec: &BStr) -> git_revision::spec::Kind {
        if spec.starts_with(b"^") {
            git_revision::spec::Kind::IncludeReachable
        } else if spec.contains_str(b"...") {
            git_revision::spec::Kind::ReachableToMergeBase
        } else if spec.contains_str(b"..") {
            git_revision::spec::Kind::RangeBetween
        } else if spec.ends_with(b"^!") {
            git_revision::spec::Kind::ExcludeReachableFromParents
        } else if spec.ends_with(b"^@") {
            unreachable!("BUG: cannot use rev^@ as it won't list the actual commit")
        } else {
            git_revision::spec::Kind::IncludeReachable
        }
    }
    fn lines_of(kind: git_revision::spec::Kind) -> Option<usize> {
        Some(match kind {
            Kind::ExcludeReachable | Kind::IncludeReachable => 1,
            Kind::RangeBetween => 2,
            Kind::ReachableToMergeBase => 3,
            Kind::IncludeReachableFromParents | Kind::ExcludeReachableFromParents => return None,
        })
    }
    fn object_id_of_next(lines: &mut std::iter::Peekable<bstr::Lines<'_>>) -> git_hash::ObjectId {
        let hex_hash = lines.next().expect("valid respect yields enough lines");
        object_id_of(hex_hash).expect("git yields full objects ids")
    }
    fn object_id_of(input: &[u8]) -> Option<git_hash::ObjectId> {
        let hex_hash = input.strip_prefix(b"^").unwrap_or(input);
        git_hash::ObjectId::from_str(hex_hash.to_str().expect("hex is ascii")).ok()
    }
    let mut baseline_map = HashMap::new();
    let base = git_testtools::scripted_fixture_repo_read_only(FIXTURE_NAME).unwrap();
    for baseline_entry in walkdir::WalkDir::new(base)
        .max_depth(2)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok().and_then(|e| (e.file_name() == "baseline.git").then(|| e)))
    {
        let map = baseline_map
            .entry(baseline_entry.path().parent().expect("file in directory").into())
            .or_insert_with(HashMap::default);
        let baseline = std::fs::read(baseline_entry.path()).unwrap();
        let mut lines = baseline.lines().peekable();
        while let Some(spec) = lines.next() {
            let exit_code_or_hash = lines.next().expect("exit code or single hash").to_str().unwrap();
            let kind = kind_of(spec.as_bstr());
            let first_hash = match u8::from_str(exit_code_or_hash) {
                Ok(_exit_code) => {
                    let is_duplicate = map.insert(spec.into(), None).is_some();
                    assert!(!is_duplicate, "Duplicate spec '{}' cannot be handled", spec.as_bstr());
                    continue;
                }
                Err(_) => match git::ObjectId::from_str(exit_code_or_hash) {
                    Ok(hash) => hash,
                    Err(_) => break, // for now bail out, we can't parse multi-line results yet
                },
            };
            let num_lines = lines_of(kind);
            let rev_spec = match num_lines {
                Some(line_count) => match line_count {
                    1 if kind == git_revision::spec::Kind::IncludeReachable => git_revision::Spec::Include(first_hash),
                    1 if kind == git_revision::spec::Kind::ExcludeReachable => git_revision::Spec::Exclude(first_hash),
                    2 | 3 => {
                        let second_hash = object_id_of_next(&mut lines);
                        if line_count == 2 {
                            git_revision::Spec::Range {
                                from: second_hash,
                                to: first_hash,
                            }
                        } else {
                            lines.next().expect("merge-base to consume");
                            git_revision::Spec::Merge {
                                theirs: first_hash,
                                ours: second_hash,
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                None => {
                    let rev_spec = match kind {
                        git_revision::spec::Kind::ExcludeReachableFromParents => {
                            git_revision::Spec::ExcludeFromParents { from: first_hash }
                        }
                        _ => unreachable!(),
                    };
                    while let Some(_oid) = lines.peek().map(|hex| object_id_of(hex)) {
                        lines.next();
                    }
                    rev_spec
                }
            };
            let is_duplicate = map.insert(spec.into(), Some(rev_spec)).is_some();
            assert!(!is_duplicate, "Duplicate spec '{}' cannot be handled", spec.as_bstr());
            if num_lines.filter(|count| *count > 1).is_some() {
                // git always considers these errors for some reason, so skip it.
                lines.next();
            }
        }
    }
    baseline_map
});

fn parse_spec_no_baseline<'a>(
    spec: &str,
    repo: &'a git::Repository,
) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
    parse_spec_no_baseline_opts(spec, repo, Default::default())
}

enum BaselineExpectation {
    /// We have the same result as git
    Same,
    /// Git can't do something that we can
    GitFailsWeSucceed,
}

/// Git can't do that, but we can
fn parse_spec_better_than_baseline<'a>(
    spec: &str,
    repo: &'a git::Repository,
) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
    let res = RevSpec::from_bstr(spec, repo, Default::default());
    compare_with_baseline(&res, repo, spec, BaselineExpectation::GitFailsWeSucceed);
    res
}

fn parse_spec_no_baseline_opts<'a>(
    spec: &str,
    repo: &'a git::Repository,
    opts: git::rev_spec::parse::Options,
) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
    RevSpec::from_bstr(spec, repo, opts)
}

fn parse_spec_opts<'a>(
    spec: &str,
    repo: &'a git::Repository,
    opts: git::rev_spec::parse::Options,
) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
    let res = RevSpec::from_bstr(spec, repo, opts);
    compare_with_baseline(&res, repo, spec, BaselineExpectation::Same);
    res
}

fn rev_parse<'a>(spec: &str, repo: &'a git::Repository) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
    let res = repo.rev_parse(spec);
    compare_with_baseline(&res, repo, spec, BaselineExpectation::Same);
    res
}

fn compare_with_baseline(
    res: &Result<RevSpec<'_>, git::rev_spec::parse::Error>,
    repo: &git::Repository,
    spec: &str,
    expectation: BaselineExpectation,
) {
    let actual = res.as_deref().ok().copied();
    let spec: BString = spec.into();
    let expected = *BASELINE
        .get(repo.work_dir().unwrap_or_else(|| repo.git_dir()))
        .unwrap_or_else(|| panic!("No baseline for {:?}", repo))
        .get(&spec)
        .unwrap_or_else(|| panic!("'{}' revspec not found in git baseline", spec));
    match expectation {
        BaselineExpectation::Same => {
            assert_eq!(
                actual, expected,
                "{}: left (ours) should match right (git): {:?}",
                spec, res
            );
        }
        BaselineExpectation::GitFailsWeSucceed => {
            assert_eq!(expected, None, "Git should fail here");
        }
    }
}

fn parse_spec<'a>(spec: &str, repo: &'a git::Repository) -> Result<RevSpec<'a>, git::rev_spec::parse::Error> {
    parse_spec_opts(spec, repo, Default::default())
}

fn repo(name: &str) -> crate::Result<git::Repository> {
    let base = git_testtools::scripted_fixture_repo_read_only(FIXTURE_NAME)?;
    Ok(git::open(base.join(name))?)
}

mod ambiguous {
    use super::repo;
    use crate::rev_spec::from_bytes::{
        parse_spec, parse_spec_better_than_baseline, parse_spec_no_baseline, parse_spec_no_baseline_opts,
        parse_spec_opts, rev_parse,
    };
    use git_repository::prelude::ObjectIdExt;
    use git_repository::rev_spec::parse::{Options, RefsHint};
    use git_repository::RevSpec;
    use git_testtools::hex_to_id;

    #[test]
    fn prefix() {
        {
            let repo = repo("blob.prefix").unwrap();
            assert_eq!(
                parse_spec("dead", &repo).unwrap_err().to_string(),
                "Short id dead is ambiguous. Candidates are:\n\tdead7b2 blob\n\tdead9d3 blob"
            );
            assert_eq!(
                parse_spec("beef", &repo).unwrap_err().to_string(),
                "Short id beef is ambiguous. Candidates are:\n\tbeef2b0 blob\n\tbeefc9b blob"
            );
        }

        {
            let repo = repo("blob.bad").unwrap();
            assert_eq!(
                parse_spec("bad0", &repo).unwrap_err().to_string(),
                "Short id bad0 is ambiguous. Candidates are:\n\tbad0853 lookup error: An error occurred while obtaining an object from the loose object store\n\tbad0bd4 lookup error: An error occurred while obtaining an object from the loose object store",
                "git provides a much worse hint messages and fails to provide information for both bad objects"
            );
        }
    }

    #[test]
    fn fully_failed_disambiguation_still_yields_an_ambiguity_error() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        assert_eq!(
            parse_spec("0000000000^{tag}", &repo).unwrap_err().to_string(),
            "Short id 0000000000 is ambiguous. Candidates are:\n\t0000000000e commit 1112911993 -0700 \"a2onsxbvj\"\n\t0000000000c tree\n\t0000000000b blob",
            "without special treatment, one would see a bunch of failed transformations with the impression that the first of them is the root cause, which isn't correct."
        );
    }

    #[test]
    #[ignore]
    fn ranges_are_auto_disambiguating_by_committish() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        let expected = RevSpec::from_id(hex_to_id("0000000000cdcf04beb2fab69e65622616294984").attach(&repo));
        assert_eq!(
            parse_spec("000000000..000000000", &repo).unwrap(),
            expected,
            "as ranges need a commit, this is assumed when disambiguating"
        );
    }

    #[test]
    fn blob_and_tree_can_be_disambiguated_by_type() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        assert_eq!(
            parse_spec("0000000000", &repo).unwrap_err().to_string(),
            "Short id 0000000000 is ambiguous. Candidates are:\n\t0000000000e commit 1112911993 -0700 \"a2onsxbvj\"\n\t0000000000c tree\n\t0000000000b blob",
            "in theory one could disambiguate with 0000000000^{{tree}} (which works in git) or 0000000000^{{blob}} which doesn't work for some reason."
            );

        assert_eq!(
            parse_spec("0000000000cdc^{tree}", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000cdcf04beb2fab69e65622616294984").attach(&repo)),
            "this is unambiguous anyway, but also asserts for tree which is naturally the case"
        );

        assert_eq!(
            parse_spec_better_than_baseline("0000000000^{tree}", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000cdcf04beb2fab69e65622616294984").attach(&repo)),
            "the commit refers to the tree which also starts with this prefix, so ultimately the result is unambiguous. Git can't do that yet."
        );

        assert_eq!(
            parse_spec("0000000000^{commit}", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000e4f9fbd19cf1e932319e5ad0d1d00b").attach(&repo)),
            "disambiguation with committish"
        );

        assert_eq!(
            parse_spec("0000000000e", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000e4f9fbd19cf1e932319e5ad0d1d00b").attach(&repo)),
            "no disambiguation needed here"
        );
    }

    #[test]
    fn trees_can_be_disambiguated_by_blob_access() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        assert_eq!(
            parse_spec_better_than_baseline("0000000000:a0blgqsjc", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000b36b6aa7ea4b75318ed078f55505c3").attach(&repo)),
            "we can disambiguate by providing a path, but git cannot"
        );
    }

    #[test]
    fn commits_can_be_disambiguated_with_commit_specific_transformations() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        for spec in ["0000000000^0", "0000000000^{commit}"] {
            assert_eq!(
                parse_spec(spec, &repo).unwrap(),
                RevSpec::from_id(hex_to_id("0000000000e4f9fbd19cf1e932319e5ad0d1d00b").attach(&repo))
            );
        }
    }

    #[test]
    fn tags_can_be_disambiguated_with_commit_specific_transformations() {
        let repo = repo("ambiguous_commits").unwrap();
        assert_eq!(
            parse_spec_better_than_baseline("0000000000^{tag}", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000f8f5507ab27a0d7bd3c75c0f64ffe0").attach(&repo)),
            "disambiguation is possible by type, and git can't do that for some reason"
        );
    }

    #[test]
    fn duplicates_are_deduplicated_across_all_odb_types() {
        let repo = repo("duplicate_ambiguous_objects").unwrap();
        assert_eq!(
            parse_spec_no_baseline("0000000000", &repo).unwrap_err().to_string(),
            "Short id 0000000000 is ambiguous. Candidates are:\n\t0000000000f8 tag \"v1.0.0\"\n\t000000000004 commit 1112912053 -0700 \"czy8f73t\"\n\t00000000006 commit 1112912233 -0700 \"ad2uee\"\n\t00000000008 commit 1112912113 -0700 \"ioiley5o\"\n\t0000000000e commit 1112911993 -0700 \"a2onsxbvj\"\n\t000000000002 tree\n\t00000000005 tree\n\t00000000009 tree\n\t0000000000c tree\n\t0000000000fd tree\n\t00000000001 blob\n\t00000000003 blob\n\t0000000000a blob\n\t0000000000b blob\n\t0000000000f2 blob",
            "One day we want to see 16 objects here, and not 32 just because they exist in the loose and the packed odb"
        );
    }

    fn opts_ref_hint(hint: RefsHint) -> Options {
        Options {
            refs_hint: hint,
            object_kind_hint: None,
        }
    }

    #[test]
    fn ambiguous_40hex_refs_are_ignored_and_we_prefer_the_object_of_the_same_name() {
        let repo = repo("ambiguous_refs").unwrap();
        let spec = "0000000000e4f9fbd19cf1e932319e5ad0d1d00b";
        assert_eq!(
            parse_spec(spec, &repo).unwrap(),
            RevSpec::from_id(hex_to_id(spec).attach(&repo)),
            "git shows an advisory here and ignores the ref, which makes it easy to just ignore it too. We are unable to show anything though, maybe traces?"
        );

        assert_eq!(
            parse_spec_opts(spec, &repo, opts_ref_hint(RefsHint::PreferObject)).unwrap(),
            RevSpec::from_id(hex_to_id(spec).attach(&repo)),
            "preferring objects yields the same result here"
        );

        assert_eq!(
            parse_spec_no_baseline_opts(spec, &repo, opts_ref_hint(RefsHint::PreferRef)).unwrap(),
            RevSpec::from_id(hex_to_id("cc60d25ccfee90e4a4105e73df36059db383d5ce").attach(&repo)),
            "we can prefer refs in any case, too"
        );

        assert_eq!(
            parse_spec_no_baseline_opts(
                spec,
                &repo,
                opts_ref_hint(RefsHint::Fail)
            )
            .unwrap_err()
            .to_string(),
            "The short hash 0000000000e4f9fbd19cf1e932319e5ad0d1d00b matched both the reference refs/heads/0000000000e4f9fbd19cf1e932319e5ad0d1d00b and at least one object"
        );
    }

    #[test]
    fn ambiguous_short_refs_are_dereferenced() {
        let repo = repo("ambiguous_refs").unwrap();
        let spec = "0000000000e";
        assert_eq!(
            parse_spec(spec, &repo).unwrap(),
            RevSpec::from_id(hex_to_id("cc60d25ccfee90e4a4105e73df36059db383d5ce").attach(&repo)),
            "git shows a warning here and we show nothing but have dials to control how to handle these cases"
        );

        assert_eq!(
            parse_spec_opts(spec, &repo, opts_ref_hint(RefsHint::PreferRef)).unwrap(),
            RevSpec::from_id(hex_to_id("cc60d25ccfee90e4a4105e73df36059db383d5ce").attach(&repo)),
            "this does the same, but independently of the length of the ref"
        );

        assert_eq!(
            parse_spec_no_baseline_opts(spec, &repo, opts_ref_hint(RefsHint::PreferObject)).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000e4f9fbd19cf1e932319e5ad0d1d00b").attach(&repo)),
            "we can always prefer objects, too"
        );

        assert_eq!(
            parse_spec_no_baseline_opts(spec, &repo, opts_ref_hint(RefsHint::Fail))
                .unwrap_err()
                .to_string(),
            "The short hash 0000000000e matched both the reference refs/heads/0000000000e and at least one object",
            "users who don't want this ambiguity, could fail like this."
        );
    }

    #[test]
    fn repository_local_disambiguation_hints_disambiguate() {
        let r = repo("ambiguous_objects_disambiguation_config_committish").unwrap();
        assert_eq!(
            rev_parse("0000000000f", &r).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000f8f5507ab27a0d7bd3c75c0f64ffe0").attach(&r)),
            "we read the 'core.disambiguate' value and apply it to auto-disambiguate"
        );
        assert_eq!(
            rev_parse("0000000000", &r).unwrap_err().to_string(),
            "Short id 0000000000 is ambiguous. Candidates are:\n\t0000000000f8 tag \"v1.0.0\"\n\t000000000004 commit 1112912053 -0700 \"czy8f73t\"\n\t00000000006 commit 1112912233 -0700 \"ad2uee\"\n\t00000000008 commit 1112912113 -0700 \"ioiley5o\"\n\t0000000000e commit 1112911993 -0700 \"a2onsxbvj\"",
        );

        let r = repo("ambiguous_objects_disambiguation_config_treeish").unwrap();
        assert_eq!(
            rev_parse("0000000000f", &r).unwrap_err().to_string(),
            "Short id 0000000000f is ambiguous. Candidates are:\n\t0000000000f8 tag \"v1.0.0\"\n\t0000000000fd tree",
            "disambiguation might not always work either."
        );

        let r = repo("ambiguous_objects_disambiguation_config_tree").unwrap();
        assert_eq!(
            rev_parse("0000000000f", &r).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000fd8bcc566027a4d16bde8434cac1a4").attach(&r)),
            "disambiguation may work precisely even with a simple object type constraint"
        );

        let r = repo("ambiguous_objects_disambiguation_config_commit").unwrap();
        assert_eq!(
            rev_parse("0000000000f", &r).unwrap_err().to_string(),
            "Short id 0000000000f is ambiguous. Candidates are:\n\t0000000000f8 tag \"v1.0.0\"\n\t0000000000fd tree\n\t0000000000f2 blob",
        );
        assert_eq!(
            rev_parse("0000000000", &r).unwrap_err().to_string(),
            "Short id 0000000000 is ambiguous. Candidates are:\n\t000000000004 commit 1112912053 -0700 \"czy8f73t\"\n\t00000000006 commit 1112912233 -0700 \"ad2uee\"\n\t00000000008 commit 1112912113 -0700 \"ioiley5o\"\n\t0000000000e commit 1112911993 -0700 \"a2onsxbvj\"",
        );

        let r = repo("ambiguous_objects_disambiguation_config_blob").unwrap();
        assert_eq!(
            rev_parse("0000000000f", &r).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000f2fdf63f36c0d76aece18a79ab64f2").attach(&r)),
        );
    }

    #[test]
    fn repository_local_disambiguation_hints_are_overridden_by_specific_ones() {
        let repo = repo("ambiguous_objects_disambiguation_config_committish").unwrap();
        assert_eq!(
            rev_parse("0000000000f^{tree}", &repo).unwrap_err().to_string(),
            "Short id 0000000000f is ambiguous. Candidates are:\n\t0000000000c tree\n\t0000000000fd tree",
            "spec overrides overrule the configuration value, which makes this particular object ambiguous between tree and tag"
        );
    }
}

#[test]
fn bad_objects_are_valid_until_they_are_actually_read_from_the_odb() {
    {
        let repo = repo("blob.bad").unwrap();
        assert_eq!(
            parse_spec("e328", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("e32851d29feb48953c6f40b2e06d630a3c49608a").attach(&repo)),
            "we are able to return objects even though they are 'bad' when trying to decode them, like git",
        );
        assert_eq!(
            format!("{:?}", parse_spec("e328^{object}", &repo).unwrap_err()),
            r#"FindObject(Find(Loose(Decode(ObjectHeader(InvalidObjectKind("bad"))))))"#,
            "Now we enforce the object to exist and be valid, as ultimately it wants to match with a certain type"
        );
    }

    {
        let repo = repo("blob.corrupt").unwrap();
        assert_eq!(
            parse_spec("cafea", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("cafea31147e840161a1860c50af999917ae1536b").attach(&repo))
        );
        assert_eq!(
            &format!("{:?}", parse_spec("cafea^{object}", &repo).unwrap_err())[..80],
            r#"FindObject(Find(Loose(DecompressFile { source: Inflate(DecompressError(General {"#
        );
    }
}

#[test]
fn access_blob_through_tree() {
    let repo = repo("ambiguous_blob_tree_commit").unwrap();
    assert_eq!(
        parse_spec("0000000000cdc:a0blgqsjc", &repo).unwrap(),
        RevSpec::from_id(hex_to_id("0000000000b36b6aa7ea4b75318ed078f55505c3").attach(&repo))
    );

    assert_eq!(
        parse_spec("0000000000cdc:missing", &repo).unwrap_err().to_string(),
        "Could not find path \"missing\" in tree 0000000000c of parent object 0000000000c"
    );
}

#[test]
#[ignore]
fn find_ref() {}
