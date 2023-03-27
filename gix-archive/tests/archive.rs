use gix_archive::{write, FailedToArchiveError, Format, Options};
use gix_hash::oid;
use gix_object::{Data, Kind};

#[test]
fn archive_succeeded() {
    assert!(
        write(
            oid::try_from_bytes("e2de15f23ea4ef15966a".as_bytes()).expect("valid sha1"),
            |_oid, _buf| Ok::<Data<'_>, FailedToArchiveError>(Data::new(Kind::Blob, &[])),
            vec![],
            Options::default(),
        )
        .is_ok(),
        "archive returns an error"
    )
}

#[test]
fn failed_to_archive() {
    assert!(
        write(
            oid::try_from_bytes("e2de15f23ea4ef15966a".as_bytes()).expect("valid sha1"),
            |_oid, _buf| Ok::<Data<'_>, FailedToArchiveError>(Data::new(Kind::Blob, &[])),
            std::io::stdout(),
            Options {
                format: Format::Zip(2),
                prefix: "".to_string(),
                modified_time: std::time::SystemTime::now(),
                use_worktree_attributes: false
            }
        )
        .is_err(),
        "archive did not return an error"
    )
}
