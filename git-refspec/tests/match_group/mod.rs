use crate::matching::baseline;

#[test]
fn fetch_only() {
    baseline::agrees_with_fetch_specs(Some("refs/heads/main"));
    baseline::agrees_with_fetch_specs(Some("heads/main"));
    baseline::agrees_with_fetch_specs(Some("main"));
    baseline::agrees_with_fetch_specs(Some("v0.0-f1"));
    baseline::agrees_with_fetch_specs(Some("tags/v0.0-f2"));
}
