use crate::revision::spec::from_bytes::{parse_spec, parse_spec_no_baseline, repo};
use git_repository::revision::spec::parse::Error;

#[test]
fn nth_prior_checkout() {
    let repo = repo("complex_graph").unwrap();

    for (spec, prior_branch) in [
        ("@{-1}", "refs/heads/i"),
        ("@{-2}", "refs/heads/main"),
        ("@{-3}", "refs/heads/e"),
        ("@{-4}", "refs/heads/j"),
        ("@{-5}", "refs/heads/h"),
    ] {
        let parsed = parse_spec(spec, &repo).unwrap_or_else(|_| panic!("{} to be parsed successfully", spec));
        assert_eq!(parsed.first_reference().expect("present").name.as_bstr(), prior_branch);
    }

    assert_eq!(
        parse_spec("@{-6}", &repo).unwrap_err().to_string(),
        "HEAD has 5 prior checkouts and checkout number 6 is out of range"
    );
}

#[test]
fn by_date() {
    let repo = repo("complex_graph").unwrap();
    assert!(matches!(
        parse_spec_no_baseline("main@{1979-02-26 18:30:00}", &repo).unwrap_err(),
        Error::Planned { .. }
    ));
}
