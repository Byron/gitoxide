use bstr::{BStr, ByteSlice};
use std::borrow::Cow;

pub fn cow_str(s: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(s.as_bytes().as_bstr())
}

mod open {
    use git_config::File;
    use git_testtools::fixture_path;

    #[test]
    fn parse_config_with_windows_line_endings_successfully() {
        File::open(fixture_path("repo-config.crlf")).unwrap();
    }
}

mod display;
mod from_env;
mod from_paths;
mod mutable_multi_value;
mod mutable_value;
mod raw_multi_value;
mod raw_value;
mod value;
