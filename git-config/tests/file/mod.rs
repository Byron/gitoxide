use std::borrow::Cow;

use bstr::{BStr, ByteSlice};

pub fn cow_str(s: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(s.as_bytes().as_bstr())
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
