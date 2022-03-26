use git_mailmap::{parse, Entry};
use git_testtools::fixture_path;

#[test]
fn line_numbers_are_counted_correctly_in_errors() {
    let input = std::fs::read(fixture_path("invalid.txt")).unwrap();
    let mut actual = git_mailmap::parse(&input).collect::<Vec<_>>().into_iter();
    assert_eq!(actual.len(), 2);

    let err = actual.next().expect("two items left").unwrap_err();
    assert!(matches!(err, parse::Error::Malformed { line_number: 3, .. }));

    let err = actual.next().expect("one item left").unwrap_err();
    assert!(matches!(err, parse::Error::UnconsumedInput { line_number: 5, .. }));
}

#[test]
fn empty_lines_and_comments_are_ignored() {
    assert!(git_mailmap::parse(b"# comment").next().is_none());
    assert!(git_mailmap::parse(b"\n\r\n\t\t   \n").next().is_none());
    assert_eq!(
        line(" # this is a name <email>"),
        Entry::change_name_by_email("# this is a name", "email"),
        "whitespace before hashes counts as name though"
    );
}

#[test]
fn valid_entries() {
    assert_eq!(
        line(" \t proper name   <commit-email>"),
        Entry::change_name_by_email("proper name", "commit-email")
    );
    assert_eq!(
        line("  <proper email>   <commit-email>  \t "),
        Entry::change_email_by_email("proper email", "commit-email")
    );
    assert_eq!(
        line("  proper name \t  <proper email> \t <commit-email>"),
        Entry::change_name_and_email_by_email("proper name", "proper email", "commit-email")
    );
    assert_eq!(
        line(" proper name  <proper email>\tcommit name\t<commit-email>\t"),
        Entry::change_name_and_email_by_name_and_email("proper name", "proper email", "commit name", "commit-email")
    );
}
#[test]
fn error_if_there_is_just_a_name() {
    assert!(matches!(
        try_line("just a name"),
        Err(parse::Error::UnconsumedInput { line_number: 1, .. })
    ));
}

#[test]
fn error_if_email_is_empty() {
    assert!(matches!(
        try_line("hello <"),
        Err(parse::Error::Malformed { line_number: 1, .. })
    ));
    assert!(matches!(
        try_line("hello < \t"),
        Err(parse::Error::Malformed { line_number: 1, .. })
    ));
    assert!(matches!(
        try_line("hello < \t\r >"),
        Err(parse::Error::Malformed { line_number: 1, .. })
    ));
}

fn line(input: &str) -> Entry<'_> {
    try_line(input).unwrap()
}

fn try_line(input: &str) -> Result<Entry<'_>, parse::Error> {
    let mut lines = git_mailmap::parse(input.as_bytes());
    let res = lines.next().expect("single line");
    assert!(lines.next().is_none(), "only one line provided");
    res
}
