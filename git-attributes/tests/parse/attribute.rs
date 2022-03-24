use bstr::{BStr, BString};
use git_attributes::ignore::pattern::Mode;
use git_attributes::{ignore, parse};
use git_testtools::fixture_path;

#[test]
fn byte_order_marks_are_no_patterns() {
    assert_eq!(line("\u{feff}hello"), (r"hello".into(), Mode::NO_SUB_DIR, vec![], 1));
}

#[test]
fn line_numbers_are_counted_correctly() {
    let ignore = std::fs::read(fixture_path("attributes/various.txt")).unwrap();
    assert_eq!(
        try_lines(&String::from_utf8(ignore).unwrap()).unwrap(),
        vec![
            (r"*.[oa]".into(), Mode::NO_SUB_DIR, vec![], 2),
            (r"*.html".into(), Mode::NO_SUB_DIR | Mode::ENDS_WITH, vec![], 5),
            (r"!foo.html".into(), Mode::NO_SUB_DIR, vec![], 8),
            (r"#a/path".into(), Mode::empty(), vec![], 10),
            (r"/*".into(), Mode::empty(), vec![], 11),
        ]
    );
}

#[test]
fn line_endings_can_be_windows_or_unix() {
    assert_eq!(
        try_lines("unix\nwindows\r\nlast").unwrap(),
        vec![
            (r"unix".into(), Mode::NO_SUB_DIR, vec![], 1),
            (r"windows".into(), Mode::NO_SUB_DIR, vec![], 2),
            (r"last".into(), Mode::NO_SUB_DIR, vec![], 3)
        ]
    );
}

#[test]
fn comment_lines_are_ignored() {
    assert!(git_attributes::parse(b"# hello world").next().is_none());
}

#[test]
fn comment_can_be_escaped_like_gitignore() {
    assert_eq!(
        line(r"\#hello"),
        (r"#hello".into(), Mode::NO_SUB_DIR, vec![], 1),
        "undocumented, but definitely works"
    );
}

#[test]
fn esclamation_marks_must_be_escaped_or_error_unlike_gitignore() {
    assert_eq!(line(r"\!hello"), (r"!hello".into(), Mode::NO_SUB_DIR, vec![], 1));
    assert!(matches!(
        try_line(r"!hello"),
        Err(parse::attribute::Error::PatternNegation { line_number: 1, .. })
    ));
}

#[test]
#[ignore]
fn attributes_are_parsed_behind_various_whitespace_characters() {
    // see https://github.com/git/git/blob/master/attr.c#L280:L280
    todo!()
}

type ExpandedAttribute<'a> = (
    BString,
    ignore::pattern::Mode,
    Vec<(&'a BStr, git_attributes::State<'a>)>,
    usize,
);

fn try_line(input: &str) -> Result<ExpandedAttribute, parse::attribute::Error> {
    let mut lines = git_attributes::parse(input.as_bytes());
    let res = expand(lines.next().unwrap())?;
    assert!(lines.next().is_none(), "expected only one line");
    Ok(res)
}

fn line(input: &str) -> ExpandedAttribute {
    let mut lines = git_attributes::parse(input.as_bytes());
    let res = expand(lines.next().unwrap()).unwrap();
    assert!(lines.next().is_none(), "expected only one line");
    res
}

fn try_lines(input: &str) -> Result<Vec<ExpandedAttribute>, parse::attribute::Error> {
    git_attributes::parse(input.as_bytes()).map(|l| expand(l)).collect()
}

fn expand(
    input: Result<(BString, ignore::pattern::Mode, parse::attribute::Iter<'_>, usize), parse::attribute::Error>,
) -> Result<ExpandedAttribute<'_>, parse::attribute::Error> {
    let (pattern, mode, attrs, line_no) = input?;
    let attrs = attrs.collect::<Result<Vec<_>, _>>()?;
    Ok((pattern, mode, attrs, line_no))
}
