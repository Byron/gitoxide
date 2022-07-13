use std::borrow::Cow;

use bstr::{BStr, ByteSlice};

pub fn cow_str(s: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(s.as_bytes().as_bstr())
}

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<git_config::File<'_>>(),
        1032,
        "This shouldn't change without us noticing"
    );
}

mod open {
    use git_config::File;
    use git_testtools::fixture_path;

    #[test]
    fn parse_config_with_windows_line_endings_successfully() {
        let mut buf = Vec::new();
        File::from_path_with_buf(&fixture_path("repo-config.crlf"), &mut buf).unwrap();
    }
}

mod access;
mod impls;
mod init;
mod mutable;
