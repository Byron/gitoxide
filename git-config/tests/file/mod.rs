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
        File::at(fixture_path("repo-config.crlf")).unwrap();
    }
}

mod access;
mod from_env;
mod from_paths;
mod impls;
