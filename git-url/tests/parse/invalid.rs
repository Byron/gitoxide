use crate::parse::assert_failure;
use git_url::parse::Error;

#[test]
fn unknown_protocol() {
    assert_failure("foo://host.xz/path/to/repo.git/", "Protocol \"foo\" is not supported")
}

#[test]
fn ssh_missing_path() {
    assert_failure("ssh://host.xz", Error::MissingResourceLocation)
}

#[test]
fn git_missing_path() {
    assert_failure("git://host.xz", Error::MissingResourceLocation)
}

#[test]
fn file_missing_path() {
    assert_failure("file://", Error::MissingRepositoryPath);
}

#[test]
fn empty() {
    assert_failure("", Error::MissingRepositoryPath);
    assert_failure("file://..", Error::MissingRepositoryPath);
    assert_failure("file://.", Error::MissingRepositoryPath);
    #[cfg(not(windows))]
    {
        assert_failure("file://.\\", Error::MissingRepositoryPath);
    }
    assert_failure("file://a", Error::MissingRepositoryPath);
}

#[test]
fn missing_port_despite_indication() {
    assert_failure("ssh://host.xz:", Error::MissingResourceLocation)
}

#[test]
fn strange() {
    assert_failure("file:..", "\"file:..\" is not a valid local path")
}
