use git_discover::parse;
use std::path::Path;

#[test]
fn valid() {
    assert_eq!(parse::git_dir(b"gitdir: a").unwrap(), Path::new("a"));
    assert_eq!(
        parse::git_dir(b"gitdir: relative/path").unwrap(),
        Path::new("relative/path")
    );
    assert_eq!(
        parse::git_dir(b"gitdir: ./relative/path").unwrap(),
        Path::new("./relative/path")
    );
    assert_eq!(
        parse::git_dir(b"gitdir: /absolute/path\n").unwrap(),
        Path::new("/absolute/path")
    );
    assert_eq!(
        parse::git_dir(b"gitdir: C:/hello/there\r\n").unwrap(),
        Path::new("C:/hello/there")
    );
}

#[test]
fn invalid() {
    assert!(
        matches!(
            parse::git_dir(b"gitdir:"),
            Err(parse::git_dir::Error::InvalidFormat { .. })
        ),
        "missing prefix"
    );
    assert!(
        matches!(
            parse::git_dir(b"bogus: foo"),
            Err(parse::git_dir::Error::InvalidFormat { .. })
        ),
        "invalid prefix"
    );
    assert!(
        matches!(
            parse::git_dir(b"gitdir: "),
            Err(parse::git_dir::Error::InvalidFormat { .. })
        ),
        "empty path"
    );
}
