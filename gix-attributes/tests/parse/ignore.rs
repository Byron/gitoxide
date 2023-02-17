use bstr::BString;
use gix_glob::{pattern::Mode, Pattern};
use gix_testtools::fixture_bytes;

#[test]
fn byte_order_marks_are_no_patterns() {
    assert_eq!(
        flatten(gix_attributes::parse::ignore("\u{feff}hello".as_bytes()).next()),
        Some((r"hello".into(), Mode::NO_SUB_DIR, 1))
    );
}

#[test]
fn line_numbers_are_counted_correctly() {
    let input = fixture_bytes("ignore/various.txt");
    let actual: Vec<_> = gix_attributes::parse::ignore(&input).map(flat_map).collect();
    assert_eq!(
        actual,
        vec![
            ("*.[oa]".into(), Mode::NO_SUB_DIR, 2),
            ("*.html".into(), Mode::NO_SUB_DIR | Mode::ENDS_WITH, 5),
            ("foo.html".into(), Mode::NO_SUB_DIR | Mode::NEGATIVE, 8),
            ("*".into(), Mode::NO_SUB_DIR | Mode::ENDS_WITH | Mode::ABSOLUTE, 11),
            ("foo".into(), Mode::NEGATIVE | Mode::NO_SUB_DIR | Mode::ABSOLUTE, 12),
            ("foo/*".into(), Mode::ABSOLUTE, 13),
            ("foo/bar".into(), Mode::ABSOLUTE | Mode::NEGATIVE, 14)
        ]
    );
}

#[test]
fn line_endings_can_be_windows_or_unix() {
    assert_eq!(
        gix_attributes::parse::ignore(b"unix\nwindows\r\nlast")
            .map(flat_map)
            .collect::<Vec<_>>(),
        vec![
            (r"unix".into(), Mode::NO_SUB_DIR, 1),
            (r"windows".into(), Mode::NO_SUB_DIR, 2),
            (r"last".into(), Mode::NO_SUB_DIR, 3)
        ]
    );
}

#[test]
fn comments_are_ignored_as_well_as_empty_ones() {
    assert!(gix_attributes::parse::ignore(b"# hello world").next().is_none());
    assert!(gix_attributes::parse::ignore(b"\n\r\n\t\t   \n").next().is_none());
}

#[test]
fn backslashes_before_hashes_are_no_comments() {
    assert_eq!(
        flatten(gix_attributes::parse::ignore(br"\#hello").next()),
        Some((r"#hello".into(), Mode::NO_SUB_DIR, 1))
    );
}

fn flatten(input: Option<(Pattern, usize)>) -> Option<(BString, gix_glob::pattern::Mode, usize)> {
    input.map(flat_map)
}

fn flat_map(input: (Pattern, usize)) -> (BString, gix_glob::pattern::Mode, usize) {
    (input.0.text, input.0.mode, input.1)
}
