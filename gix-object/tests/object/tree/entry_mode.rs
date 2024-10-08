use gix_object::tree::{EntryKind, EntryMode};

#[test]
fn size_in_bytes() {
    assert_eq!(
        std::mem::size_of::<EntryMode>(),
        2,
        "it should not change without notice"
    );
}

#[test]
fn is_methods() {
    fn mode(kind: EntryKind) -> EntryMode {
        kind.into()
    }

    assert!(mode(EntryKind::Blob).is_blob());
    assert!(EntryMode(0o100645).is_blob());
    assert_eq!(EntryMode(0o100645).kind(), EntryKind::Blob);
    assert!(!EntryMode(0o100675).is_executable());
    assert!(EntryMode(0o100700).is_executable());
    assert_eq!(EntryMode(0o100700).kind(), EntryKind::BlobExecutable);
    assert!(!mode(EntryKind::Blob).is_link());
    assert!(mode(EntryKind::BlobExecutable).is_blob());
    assert!(mode(EntryKind::BlobExecutable).is_executable());
    assert!(mode(EntryKind::Blob).is_blob_or_symlink());
    assert!(mode(EntryKind::BlobExecutable).is_blob_or_symlink());

    assert!(!mode(EntryKind::Link).is_blob());
    assert!(mode(EntryKind::Link).is_link());
    assert!(EntryMode(0o121234).is_link());
    assert_eq!(EntryMode(0o121234).kind(), EntryKind::Link);
    assert!(mode(EntryKind::Link).is_blob_or_symlink());
    assert!(mode(EntryKind::Tree).is_tree());
    assert!(EntryMode(0o040101).is_tree());
    assert_eq!(EntryMode(0o040101).kind(), EntryKind::Tree);
    assert!(mode(EntryKind::Commit).is_commit());
    assert!(EntryMode(0o167124).is_commit());
    assert_eq!(EntryMode(0o167124).kind(), EntryKind::Commit);
    assert_eq!(
        EntryMode(0o000000).kind(),
        EntryKind::Commit,
        "commit is really 'anything else' as `kind()` can't fail"
    );
}

#[test]
fn as_bytes() {
    let mut buf = Default::default();
    for (mode, expected) in [
        (EntryMode::from(EntryKind::Tree), EntryKind::Tree.as_octal_str()),
        (EntryKind::Blob.into(), EntryKind::Blob.as_octal_str()),
        (
            EntryKind::BlobExecutable.into(),
            EntryKind::BlobExecutable.as_octal_str(),
        ),
        (EntryKind::Link.into(), EntryKind::Link.as_octal_str()),
        (EntryKind::Commit.into(), EntryKind::Commit.as_octal_str()),
        (
            EntryMode::try_from(b"100744 ".as_ref()).expect("valid"),
            "100744".into(),
        ),
        (
            EntryMode::try_from(b"100644 ".as_ref()).expect("valid"),
            "100644".into(),
        ),
    ] {
        assert_eq!(mode.as_bytes(&mut buf), expected);
    }
}
