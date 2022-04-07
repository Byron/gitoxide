use git_glob::pattern::Mode;
use git_testtools::fixture_bytes;

#[test]
fn byte_order_marks_are_no_patterns() {
    assert_eq!(
        git_attributes::parse::ignore("\u{feff}hello".as_bytes()).next(),
        Some((r"hello".into(), Mode::NO_SUB_DIR, 1))
    );
}

#[test]
fn line_numbers_are_counted_correctly() {
    let input = fixture_bytes("ignore/various.txt");
    let actual: Vec<_> = git_attributes::parse::ignore(&input).collect();
    assert_eq!(
        actual,
        vec![
            ("*.[oa]".into(), Mode::NO_SUB_DIR, 2),
            ("*.html".into(), Mode::NO_SUB_DIR | Mode::ENDS_WITH, 5),
            ("foo.html".into(), Mode::NO_SUB_DIR | Mode::NEGATIVE, 8),
            ("/*".into(), Mode::empty(), 11),
            ("/foo".into(), Mode::NEGATIVE, 12),
            ("/foo/*".into(), Mode::empty(), 13),
            ("/foo/bar".into(), Mode::NEGATIVE, 14)
        ]
    );
}

#[test]
fn line_endings_can_be_windows_or_unix() {
    assert_eq!(
        git_attributes::parse::ignore(b"unix\nwindows\r\nlast").collect::<Vec<_>>(),
        vec![
            (r"unix".into(), Mode::NO_SUB_DIR, 1),
            (r"windows".into(), Mode::NO_SUB_DIR, 2),
            (r"last".into(), Mode::NO_SUB_DIR, 3)
        ]
    );
}

#[test]
fn comments_are_ignored_as_well_as_empty_ones() {
    assert!(git_attributes::parse::ignore(b"# hello world").next().is_none());
    assert!(git_attributes::parse::ignore(b"\n\r\n\t\t   \n").next().is_none());
}

#[test]
fn backslashes_before_hashes_are_no_comments() {
    assert_eq!(
        git_attributes::parse::ignore(br"\#hello").next(),
        Some((r"#hello".into(), Mode::NO_SUB_DIR, 1))
    );
}
