use bstr::ByteSlice;
use gix_path::{to_unix_separators, to_windows_separators};

#[test]
fn assure_unix_separators() {
    assert_eq!(to_unix_separators(b"no-backslash".as_bstr()).as_bstr(), "no-backslash");

    assert_eq!(to_unix_separators(b"\\a\\b\\\\".as_bstr()).as_bstr(), "/a/b//");
}

#[test]
fn assure_windows_separators() {
    assert_eq!(
        to_windows_separators(b"no-backslash".as_bstr()).as_bstr(),
        "no-backslash"
    );

    assert_eq!(to_windows_separators(b"/a/b//".as_bstr()).as_bstr(), "\\a\\b\\\\");
}

mod normalize;

mod join_bstr_unix_pathsep {
    use bstr::BStr;
    use gix_path::join_bstr_unix_pathsep;

    fn b(s: &str) -> &BStr {
        s.into()
    }

    #[test]
    fn typical_with_double_slash_avoidance() {
        assert_eq!(join_bstr_unix_pathsep(b("base"), "path"), b("base/path"));
        assert_eq!(
            join_bstr_unix_pathsep(b("base/"), "path"),
            b("base/path"),
            "no double slashes"
        );
        assert_eq!(join_bstr_unix_pathsep(b("/base"), "path"), b("/base/path"));
        assert_eq!(join_bstr_unix_pathsep(b("/base/"), "path"), b("/base/path"));
    }
    #[test]
    fn relative_base_or_path_are_nothing_special() {
        assert_eq!(join_bstr_unix_pathsep(b("base"), "."), b("base/."));
        assert_eq!(join_bstr_unix_pathsep(b("base"), ".."), b("base/.."));
        assert_eq!(join_bstr_unix_pathsep(b("base"), "../dir"), b("base/../dir"));
    }
    #[test]
    fn absolute_path_produces_double_slashes() {
        assert_eq!(join_bstr_unix_pathsep(b("/base"), "/root"), b("/base//root"));
        assert_eq!(join_bstr_unix_pathsep(b("base/"), "/root"), b("base//root"));
    }
    #[test]
    fn empty_path_makes_base_end_with_a_slash() {
        assert_eq!(join_bstr_unix_pathsep(b("base"), ""), b("base/"));
        assert_eq!(join_bstr_unix_pathsep(b("base/"), ""), b("base/"));
    }
    #[test]
    fn empty_base_leaves_everything_untouched() {
        assert_eq!(join_bstr_unix_pathsep(b(""), ""), b(""));
        assert_eq!(join_bstr_unix_pathsep(b(""), "hi"), b("hi"));
        assert_eq!(join_bstr_unix_pathsep(b(""), "/hi"), b("/hi"));
    }
}

mod relativize_with_prefix {
    fn r(path: &str, prefix: &str) -> String {
        gix_path::to_unix_separators_on_windows(
            gix_path::os_str_into_bstr(gix_path::relativize_with_prefix(path.as_ref(), prefix.as_ref()).as_os_str())
                .expect("no illformed UTF-8"),
        )
        .to_string()
    }

    #[test]
    fn basics() {
        assert_eq!(
            r("a", "a"),
            ".",
            "reaching the prefix is signalled by a '.', the current dir"
        );
        assert_eq!(r("a/b/c", "a/b"), "c", "'c' is clearly within the current directory");
        assert_eq!(
            r("c/b/c", "a/b"),
            "../../c/b/c",
            "when there is a complete disjoint prefix, we have to get out of it with ../"
        );
        assert_eq!(
            r("a/a", "a/b"),
            "../a",
            "when there is mismatch, we have to get out of the CWD"
        );
        assert_eq!(
            r("a/a", ""),
            "a/a",
            "empty prefix means nothing happens (and no work is done)"
        );
        assert_eq!(r("", ""), "", "empty stays empty");
    }
}
