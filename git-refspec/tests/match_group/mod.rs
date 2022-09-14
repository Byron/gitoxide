use crate::matching::baseline;

#[test]
fn fetch_only() {
    baseline::agrees_with_fetch_specs(Some("refs/heads/main"));
    baseline::agrees_with_fetch_specs(Some("heads/main"));
    baseline::agrees_with_fetch_specs(Some("main"));
    baseline::agrees_with_fetch_specs(Some("v0.0-f1"));
    baseline::agrees_with_fetch_specs(Some("tags/v0.0-f2"));
    baseline::agrees_with_fetch_specs(Some("78b1c1be9421b33a49a7a8176d93eeeafa112da1"));
    // baseline::agrees_with_fetch_specs(Some("9d2fab1a0ba3585d0bc50922bfdd04ebb59361df")); // TODO: figure out how not to match 'main' here
}
