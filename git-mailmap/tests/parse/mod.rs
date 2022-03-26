use git_mailmap::{parse, Entry};
use git_testtools::fixture_path;

#[test]
#[ignore]
fn line_numbers_are_counted_correctly_in_errors() {
    let input = std::fs::read(fixture_path("invalid.txt")).unwrap();
    let mut actual: Vec<_> = git_mailmap::parse(&input).collect();
    assert_eq!(actual.len(), 2);

    let err = actual.pop().expect("two items left").unwrap_err();
    assert!(matches!(err, parse::Error::Malformed { line_number: 3, .. }));

    let err = actual.pop().expect("one item left").unwrap_err();
    assert!(matches!(err, parse::Error::Malformed { line_number: 5, .. }));
}

#[test]
#[ignore]
fn empty_lines_and_comments_are_ignored() {
    assert!(git_mailmap::parse(b"# comment").next().is_none());
    assert!(git_mailmap::parse(b"\n\r\n\t\t   \n").next().is_none());
    assert_eq!(
        line(" # this is a name <email>"),
        Entry::change_name_by_email(" # this is a name", "email"),
        "whitespace before hashes counts as name though"
    );
}

#[test]
#[ignore]
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

fn line(input: &str) -> Entry<'_> {
    let mut lines = git_mailmap::parse(input.as_bytes());
    lines.next().expect("single line").unwrap()
}
