use crate::matching::baseline;

#[test]
fn fetch_only() {
    baseline::agrees_with_fetch_specs(Some("refs/heads/main"));
}
