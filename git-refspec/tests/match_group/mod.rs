use crate::matching::baseline;

#[test]
fn fetch_only() {
    baseline::agrees_with_fetch_specs(Some("refs/heads/main"));
    baseline::agrees_with_fetch_specs(Some("heads/main"));
    baseline::agrees_with_fetch_specs(Some("main"));
    baseline::agrees_with_fetch_specs(Some("v0.0-f1"));
    baseline::agrees_with_fetch_specs(Some("tags/v0.0-f2"));
    baseline::of_objects_always_matches_if_the_server_has_the_object(Some("78b1c1be9421b33a49a7a8176d93eeeafa112da1"));
    baseline::of_objects_always_matches_if_the_server_has_the_object(Some("9d2fab1a0ba3585d0bc50922bfdd04ebb59361df"));
}

#[test]
fn fetch_and_update() {
    baseline::of_objects_with_destinations_are_written_into_given_local_branches(
        Some("78b1c1be9421b33a49a7a8176d93eeeafa112da1:special"),
        ["78b1c1be9421b33a49a7a8176d93eeeafa112da1:refs/heads/special"],
    );
    baseline::of_objects_with_destinations_are_written_into_given_local_branches(
        Some("78b1c1be9421b33a49a7a8176d93eeeafa112da1:1111111111111111111111111111111111111111"),
        ["78b1c1be9421b33a49a7a8176d93eeeafa112da1:refs/heads/1111111111111111111111111111111111111111"],
    );
    // baseline::of_objects_with_destinations_are_written_into_given_local_branches(
    //     Some("9d2fab1a0ba3585d0bc50922bfdd04ebb59361df:tags/special"),
    //     ["9d2fab1a0ba3585d0bc50922bfdd04ebb59361df:refs/tags/special"],
    // );
}
