use std::path::Path;

use gix_discover::parse;

#[test]
fn valid() -> crate::Result {
    assert_eq!(parse::gitdir(b"gitdir: a")?, Path::new("a"));
    assert_eq!(parse::gitdir(b"gitdir: relative/path")?, Path::new("relative/path"));
    assert_eq!(parse::gitdir(b"gitdir: ./relative/path")?, Path::new("./relative/path"));
    assert_eq!(parse::gitdir(b"gitdir: /absolute/path\n")?, Path::new("/absolute/path"));
    assert_eq!(
        parse::gitdir(b"gitdir: C:/hello/there\r\n")?,
        Path::new("C:/hello/there")
    );

    Ok(())
}

#[test]
fn invalid() {
    assert!(
        matches!(
            parse::gitdir(b"gitdir:"),
            Err(parse::gitdir::Error::InvalidFormat { .. })
        ),
        "missing prefix"
    );
    assert!(
        matches!(
            parse::gitdir(b"bogus: foo"),
            Err(parse::gitdir::Error::InvalidFormat { .. })
        ),
        "invalid prefix"
    );
    assert!(
        matches!(
            parse::gitdir(b"gitdir: "),
            Err(parse::gitdir::Error::InvalidFormat { .. })
        ),
        "empty path"
    );
}
