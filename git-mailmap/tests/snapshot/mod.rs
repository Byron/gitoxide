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
fn non_name_and_name_mappings_will_not_clash() {
    let entries = vec![
        // add mapping from email
        git_mailmap::Entry::change_name_and_email_by_email("new-name", "new-email", "old-email"),
        // add mapping from name and email
        git_mailmap::Entry::change_name_and_email_by_name_and_email(
            "other-new-name",
            "other-new-email",
            "old-name",
            "old-email",
        ),
    ];
    for entries in vec![entries.clone().into_iter().rev().collect::<Vec<_>>(), entries] {
        let snapshot = Snapshot::new(entries);

        assert_eq!(
            snapshot.try_resolve(&signature("replace-by-email", "old-email").to_ref()),
            Some(signature("new-name", "new-email")),
            "it can match by email only"
        );
        assert_eq!(
            snapshot.try_resolve(&signature("old-name", "old-email").to_ref()),
            Some(signature("other-new-name", "other-new-email")),
            "it can match by email and name as well"
        );
    }
}

#[test]
fn overwrite_entries() {
    let snapshot = Snapshot::from_bytes(&fixture_bytes("overwrite.txt"));
    assert_eq!(
        snapshot.try_resolve(&signature("does not matter", "old-a-email").to_ref()),
        Some(signature("A-overwritten", "old-a-email")),
        "email only by email"
    );

    assert_eq!(
        snapshot.try_resolve(&signature("to be replaced", "old-b-EMAIL").to_ref()),
        Some(signature("B-overwritten", "new-b-email-overwritten")),
        "name and email by email"
    );

    assert_eq!(
        snapshot.try_resolve(&signature("old-c", "old-C-email").to_ref()),
        Some(signature("C-overwritten", "new-c-email-overwritten")),
        "name and email by name and email"
    );

    assert_eq!(
        snapshot.try_resolve(&signature("unchanged", "old-d-email").to_ref()),
        Some(signature("unchanged", "new-d-email-overwritten")),
        "email by email"
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
