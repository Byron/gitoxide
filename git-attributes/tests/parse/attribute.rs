use bstr::{BStr, BString};
use git_attributes::ignore::pattern::Mode;
use git_attributes::{ignore, parse};

#[test]
#[ignore]
fn byte_order_marks_are_no_patterns() {
    git_attributes::parse("\u{feff}hello".as_bytes()).next();
    todo!();
}

#[test]
#[ignore]
fn line_numbers_are_counted_correctly() {
    todo!()
}

#[test]
#[ignore]
fn line_endings_can_be_windows_or_unix() {
    let _ = git_attributes::parse(b"unix\nwindows\r\nlast").collect::<Vec<_>>();
    todo!()
}

#[test]
fn comment_lines_are_ignored() {
    assert!(git_attributes::parse(b"# hello world").next().is_none());
}

#[test]
#[ignore]
fn comment_cannot_be_escaped_like_gitignore() {
    assert_eq!(line(r"\#hello"), (r"\#hello".into(), Mode::empty(), vec![], 0));
}

#[test]
#[ignore]
fn backslashes_before_hashes_are_part_of_the_path() {
    git_attributes::parse(br"\#hello").next();
    todo!();
}

type ExpandedAttribute<'a> = (
    BString,
    ignore::pattern::Mode,
    Vec<(&'a BStr, git_attributes::State<'a>)>,
    usize,
);

fn line(input: &str) -> ExpandedAttribute {
    let mut lines = git_attributes::parse(input.as_bytes());
    let res = expand(lines.next().unwrap()).unwrap();
    assert!(lines.next().is_none(), "expected only one line");
    res
}

fn expand(
    input: Result<(BString, ignore::pattern::Mode, parse::attribute::Iter<'_>, usize), parse::attribute::Error>,
) -> Result<ExpandedAttribute<'_>, parse::attribute::Error> {
    let (pattern, mode, attrs, line_no) = input?;
    let attrs = attrs.collect::<Result<Vec<_>, _>>()?;
    Ok((pattern, mode, attrs, line_no))
}
