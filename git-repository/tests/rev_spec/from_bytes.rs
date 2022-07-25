use git::prelude::ObjectIdExt;
use git::RevSpec;
use git_ref::bstr::{BString, ByteSlice};
use git_repository as git;
use git_testtools::{hex_to_id, once_cell::sync::Lazy};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

const FIXTURE_NAME: &str = "make_rev_spec_parse_repos.sh";
static BASELINE: Lazy<HashMap<PathBuf, HashMap<BString, Option<git::ObjectId>>>> = Lazy::new(|| {
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
            let hash = match u8::from_str(exit_code_or_hash) {
                Ok(_exit_code) => {
                    map.insert(spec.into(), None);
                    continue;
                    // assert_eq!(
                    //     map.insert(spec.into(), None),
                    //     None,
                    //     "Duplicate spec '{}' cannot be handled",
                    //     spec.as_bstr()
                    // );
                }
                Err(_) => match git::ObjectId::from_str(exit_code_or_hash) {
                    Ok(hash) => hash,
                    Err(_) => break, // for now bail out, we can't parse multi-line results yet
                },
            };
            map.insert(spec.into(), Some(hash));
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
    let actual = res.as_ref().ok().and_then(|rs| rs.from().map(|id| id.detach()));
    let spec: BString = spec.into();
    assert_eq!(
        &actual,
        BASELINE
            .get(repo.work_dir().unwrap_or_else(|| repo.git_dir()))
            .unwrap_or_else(|| panic!("No baseline for {:?}", repo))
            .get(&spec)
            .unwrap_or_else(|| panic!("'{}' revspec not found in git baseline", spec)),
        "{}: git baseline boiled down to success or failure must match our outcome",
        spec
    );
    res
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
        parse_spec, parse_spec_no_baseline, parse_spec_no_baseline_opts, parse_spec_opts,
    };
    use git_repository::prelude::ObjectIdExt;
    use git_repository::rev_spec::parse::{Options, RefsHint};
    use git_repository::RevSpec;
    use git_testtools::hex_to_id;

    #[test]
    #[ignore]
    fn prefix() {
        {
            let repo = repo("blob.prefix").unwrap();
            assert_eq!(
                parse_spec("dead", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with dead\nThe ref partially named 'dead' could not be found",
                "our messages aren't yet as good for ambiguous objects as we don't provide additional information"
            );
            assert_eq!(
                parse_spec("beef", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with beef\nThe ref partially named 'beef' could not be found"
            );
        }

        {
            let repo = repo("blob.bad").unwrap();
            assert_eq!(
                parse_spec("bad0", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with bad0\nThe ref partially named 'bad0' could not be found",
                "git is able to also detect that the object has an invalid type because it tries to provide additional context, we don't"
            );
        }
    }

    #[test]
    #[ignore]
    fn blob_and_tree_can_be_disambiguated_by_type_some_day() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        assert_eq!(
                parse_spec("0000000000", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with 0000000000\nThe ref partially named '0000000000' could not be found",
                concat!("in theory one could disambiguate with 0000000000^{{tree}} (which works in git) or 0000000000^{{blob}} which doesn't work for some reason.",
                        "but to do that we would have to know the list of object candidates and a lot more logic which right now we don't.")
            );
    }

    #[test]
    #[ignore]
    fn trees_can_be_disambiguated_by_blob_access_some_day() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        assert_eq!(
            parse_spec_no_baseline("0000000000:a0blgqsjc", &repo).unwrap_err().to_string(),
            "Found more than one object prefixed with 0000000000\nThe ref partially named '0000000000' could not be found",
            "git can do this, but we can't just yet. It requires to deal with multiple candidates and try to apply transformations to them, discarding ones that don't work"
        );
    }

    #[test]
    #[ignore]
    fn commits_can_be_disambiguated_with_commit_specific_transformations_one_day() {
        let repo = repo("ambiguous_blob_tree_commit").unwrap();
        for spec in ["0000000000^0", "0000000000^{commit}"] {
            assert_eq!(
                parse_spec_no_baseline(spec, &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with 0000000000\nThe ref partially named '0000000000' could not be found",
                "git can do this, but we can't just yet"
            );
        }
    }

    #[test]
    #[ignore]
    fn tags_can_be_disambiguated_with_commit_specific_transformations_one_day() {
        let repo = repo("ambiguous_commits").unwrap();
        assert_eq!(
                parse_spec("0000000000^{tag}", &repo).unwrap_err().to_string(),
                "Found more than one object prefixed with 0000000000\nThe ref partially named '0000000000' could not be found",
                "git also cannot do this actually"
            );
    }

    #[test]
    #[ignore]
    fn duplicates_are_deduplicated_across_all_odb_types_on_day() {
        let repo = repo("duplicate_ambiguous_objects").unwrap();
        assert_eq!(
            parse_spec_no_baseline("0000000000", &repo).unwrap_err().to_string(),
            "Found more than one object prefixed with 0000000000\nThe ref partially named '0000000000' could not be found",
            "One day we want to see 16 objects here, and not 32 just because they exist in the loose and the packed odb"
        );
    }

    fn opts_ref_hint(hint: RefsHint) -> Options {
        Options { refs_hint: hint }
    }

    #[test]
    #[ignore]
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
            "The short hash 0000000000e4f9fbd19cf1e932319e5ad0d1d00b matched both the reference refs/heads/0000000000e4f9fbd19cf1e932319e5ad0d1d00b and the object 0000000000e4f9fbd19cf1e932319e5ad0d1d00b"
        );
    }

    #[test]
    #[ignore]
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
            parse_spec_no_baseline_opts(
                spec,
                &repo,
                opts_ref_hint(RefsHint::Fail)
            )
                .unwrap_err()
                .to_string(),
            "The short hash 0000000000e matched both the reference refs/heads/0000000000e and the object 0000000000e4f9fbd19cf1e932319e5ad0d1d00b",
            "users who don't want this ambiguity, could fail like this."
        );
    }

    #[test]
    #[ignore]
    fn disambiguation_hints_can_be_provided_to_choose_one() {
        let repo = repo("ambiguous_commits_disambiguation_config").unwrap();
        assert_eq!(
            parse_spec("0000000000f", &repo).unwrap(),
            RevSpec::from_id(hex_to_id("0000000000f8f5507ab27a0d7bd3c75c0f64ffe0").attach(&repo)),
            "we read the 'core.disambiguate' value and apply it to auto-disambiguate"
        );

        assert_eq!(
            parse_spec("0000000000f^{tree}", &repo).unwrap_err().to_string(),
            "Found more than one object prefixed with 0000000000f\nThe ref partially named '0000000000f' could not be found",
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
#[ignore]
fn access_blob_through_tree() {
    let repo = repo("ambiguous_blob_tree_commit").unwrap();
    assert_eq!(
        parse_spec("0000000000cdc:a0blgqsjc", &repo).unwrap(),
        RevSpec::from_id(hex_to_id("0000000000b36b6aa7ea4b75318ed078f55505c3").attach(&repo))
    );
}

#[test]
#[ignore]
fn find_ref() {}
