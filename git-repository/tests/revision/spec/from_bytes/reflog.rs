use crate::revision::spec::from_bytes::{parse_spec, repo};

#[test]
#[ignore]
fn nth_prior_checkout() {
    let repo = repo("complex_graph").unwrap();

    for spec in ["@{-1}", "@{-2}", "@{-3}", "@{-4}", "@{-5}"] {
        assert!(parse_spec(spec, &repo).is_ok(), "spec {} should be valid", spec);
    }

    assert_eq!(
        parse_spec("@{-6}", &repo).unwrap_err().to_string(),
        "HEAD has 5 prior checkouts and checkout 6 is out of range"
    );
}
