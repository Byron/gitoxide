use git_mailmap::Snapshot;
use git_testtools::fixture_bytes;

#[test]
#[ignore]
fn try_resolve() {
    let snapshot = snapshot();
    let sig = signature("Foo", "Joe@example.com");
    assert_eq!(
        snapshot.try_resolve(&sig.to_ref()),
        Some(signature("Joe R. Developer", "Joe@example.com")),
        "resolved signatures contain all original fields, but normalizes only what's in the mapping, lookup is case-insensitive"
    );
}

fn snapshot() -> Snapshot {
    Snapshot::from_bytes(&fixture_bytes("typical.txt"))
}

fn signature(name: &str, email: &str) -> git_actor::Signature {
    git_actor::Signature {
        name: name.into(),
        email: email.into(),
        time: git_actor::Time {
            // marker
            time: 42,
            offset: 53,
            sign: git_actor::Sign::Minus,
        },
    }
}
