use crate::matching::baseline;

#[test]
fn fetch_only() {
    baseline::agrees_with_fetch_specs(Some("refs/heads/main"));
    baseline::agrees_with_fetch_specs(Some("heads/main"));
    baseline::agrees_with_fetch_specs(Some("main"));
    baseline::agrees_with_fetch_specs(Some("v0.0-f1"));
    baseline::agrees_with_fetch_specs(Some("tags/v0.0-f2"));
    baseline::provides_does_not_actually_match_object_names(
        Some("78b1c1be9421b33a49a7a8176d93eeeafa112da1"),
        ["refs/tags/annotated-v0.0"],
    );
    baseline::provides_does_not_actually_match_object_names(
        Some("9d2fab1a0ba3585d0bc50922bfdd04ebb59361df"),
        ["refs/heads/main", "refs/tags/annotated-v0.0"],
    );
}

#[test]
#[ignore]
fn fetch_and_update() {
    baseline::provides_does_not_actually_match_object_names_and_specific_local(
        Some("78b1c1be9421b33a49a7a8176d93eeeafa112da1:special"),
        ["refs/tags/annotated-v0.0:refs/heads/special"],
    );
    baseline::provides_does_not_actually_match_object_names_and_specific_local(
        Some("9d2fab1a0ba3585d0bc50922bfdd04ebb59361df:tags/special"),
        ["refs/tags/annotated-v0.0:refs/tags/special"],
    );
}
