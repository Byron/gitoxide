use git_mailmap::Snapshot;
use git_testtools::fixture_bytes;

#[test]
fn try_resolve() {
    let snapshot = snapshot();
    assert_eq!(
        snapshot.try_resolve(&signature("Foo", "Joe@example.com").to_ref()),
        Some(signature("Joe R. Developer", "Joe@example.com")),
        "resolved signatures contain all original fields, but normalizes only what's in the mapping, lookup is case-insensitive"
    );
    assert_eq!(
        snapshot.try_resolve(&signature("Joe", "bugs@example.com").to_ref()),
        Some(signature("Joe R. Developer", "joe@example.com")),
        "name and email can be mapped specifically"
    );

    assert_eq!(
        snapshot.try_resolve(&signature("Jane", "jane@laptop.(none)").to_ref()),
        Some(signature("Jane Doe", "jane@example.com")),
        "fix name and email by email"
    );
    assert_eq!(
        snapshot.try_resolve(&signature("Jane", "jane@desktop.(none)").to_ref()),
        Some(signature("Jane Doe", "jane@example.com")),
        "fix name and email by other email"
    );

    assert_eq!(
        snapshot.try_resolve(&signature("janE", "Bugs@example.com").to_ref()),
        Some(signature("Jane Doe", "jane@example.com")),
        "name and email can be mapped specifically, case insensitive matching of name"
    );

    let sig = signature("Jane", "other@example.com");
    assert_eq!(snapshot.try_resolve(&sig.to_ref()), None, "unmatched email");

    assert_eq!(
        snapshot.resolve(&sig.to_ref()),
        sig,
        "resolution always works here, returning a copy of the original"
    );

    let sig = signature("Jean", "bugs@example.com");
    assert_eq!(
        snapshot.try_resolve(&sig.to_ref()),
        None,
        "matched email, unmatched name"
    );
    assert_eq!(snapshot.resolve(&sig.to_ref()), sig);
}

#[test]
#[ignore]
fn non_name_and_name_mappings_will_not_clash() {
    // add mapping from email
    // add mapping from name and email
    // both should be accessible

    // the same the other way around
    todo!()
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
