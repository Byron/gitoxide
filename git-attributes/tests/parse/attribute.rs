use bstr::{BStr, BString, ByteSlice};
use git_attributes::ignore::pattern::Mode;
use git_attributes::{ignore, parse, State};
use git_testtools::fixture_path;

#[test]
fn byte_order_marks_are_no_patterns() {
    assert_eq!(line("\u{feff}hello"), (r"hello".into(), Mode::NO_SUB_DIR, vec![], 1));
    assert_eq!(
        line("\u{feff}\"hello\""),
        (r"hello".into(), Mode::NO_SUB_DIR, vec![], 1)
    );
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
    assert!(git_attributes::parse(b"# \"hello world\"").next().is_none());
    assert!(
        git_attributes::parse(b" \t\r# \"hello world\"").next().is_none(),
        "also behind leading whitespace"
    );
}

#[test]
fn leading_whitespace_is_ignored() {
    assert_eq!(line(" \r\tp"), (r"p".into(), Mode::NO_SUB_DIR, vec![], 1));
    assert_eq!(line(" \r\t\"p\""), (r"p".into(), Mode::NO_SUB_DIR, vec![], 1));
}

#[test]
fn comment_can_be_escaped_like_gitignore_or_quoted() {
    assert_eq!(
        line(r"\#hello"),
        (r"#hello".into(), Mode::NO_SUB_DIR, vec![], 1),
        "undocumented, but definitely works"
    );
    assert_eq!(line("\"# hello\""), (r"# hello".into(), Mode::NO_SUB_DIR, vec![], 1));
}

#[test]
fn exclamation_marks_must_be_escaped_or_error_unlike_gitignore() {
    assert_eq!(line(r"\!hello"), (r"!hello".into(), Mode::NO_SUB_DIR, vec![], 1));
    assert!(matches!(
        try_line(r"!hello"),
        Err(parse::attribute::Error::PatternNegation { line_number: 1, .. })
    ));
    assert!(
        matches!(
            try_line(r#""!hello""#),
            Err(parse::attribute::Error::PatternNegation { line_number: 1, .. }),
        ),
        "even in quotes they trigger…"
    );
    assert_eq!(
        line(r#""\\!hello""#),
        (r"!hello".into(), Mode::NO_SUB_DIR, vec![], 1),
        "…and must be double-escaped, once to get through quote, then to get through parse ignore line"
    );
}

#[test]
fn invalid_escapes_in_quotes_are_an_error() {
    assert!(matches!(
        try_line(r#""\!hello""#),
        Err(parse::attribute::Error::Unquote(_)),
    ),);
}

#[test]
#[ignore]
fn custom_macros_can_be_defined() {
    todo!("name validation, leave rejecting them based on location to the caller")
}

#[test]
fn attributes_are_parsed_behind_various_whitespace_characters() {
    assert_eq!(
        line(r#"p a b"#),
        ("p".into(), Mode::NO_SUB_DIR, vec![set("a"), set("b")], 1),
        "behind space"
    );
    assert_eq!(
        line(r#""p" a b"#),
        ("p".into(), Mode::NO_SUB_DIR, vec![set("a"), set("b")], 1),
        "behind space"
    );
    assert_eq!(
        line("p\ta\tb"),
        ("p".into(), Mode::NO_SUB_DIR, vec![set("a"), set("b")], 1),
        "behind tab"
    );
    assert_eq!(
        line("\"p\"\ta\tb"),
        ("p".into(), Mode::NO_SUB_DIR, vec![set("a"), set("b")], 1),
        "behind tab"
    );
    assert_eq!(
        line("p \t a \t b"),
        ("p".into(), Mode::NO_SUB_DIR, vec![set("a"), set("b")], 1),
        "behind a mix of space and tab"
    );
    assert_eq!(
        line("\"p\" \t a \t b"),
        ("p".into(), Mode::NO_SUB_DIR, vec![set("a"), set("b")], 1),
        "behind a mix of space and tab"
    );
}

#[test]
fn attributes_come_in_different_flavors_due_to_prefixes() {
    assert_eq!(
        line(r#"p set -unset !unspecified -set"#),
        (
            "p".into(),
            Mode::NO_SUB_DIR,
            vec![set("set"), unset("unset"), unspecified("unspecified"), unset("set")],
            1
        ),
        "the parser doesn't care about double-mentions either"
    );
}

#[test]
fn attributes_can_have_values() {
    assert_eq!(
        line(r#"p a=one b=2 c=你好 "#),
        (
            "p".into(),
            Mode::NO_SUB_DIR,
            vec![value("a", "one"), value("b", "2"), value("c", "你好")],
            1
        ),
        "only non-whitespace ascii values are allowed, no escaping or anything fancy is possible there"
    );
}

#[test]
fn attributes_see_state_adjustments_over_value_assignments() {
    assert_eq!(
        line(r#"p set -unset=a !unspecified=b"#),
        (
            "p".into(),
            Mode::NO_SUB_DIR,
            vec![set("set"), unset("unset"), unspecified("unspecified")],
            1
        )
    );
}

#[test]
fn trailing_whitespace_in_attributes_is_ignored() {
    assert_eq!(line("p a \r\t"), ("p".into(), Mode::NO_SUB_DIR, vec![set("a")], 1),);
    assert_eq!(line("\"p\" a \r\t"), ("p".into(), Mode::NO_SUB_DIR, vec![set("a")], 1),);
}

type ExpandedAttribute<'a> = (
    BString,
    ignore::pattern::Mode,
    Vec<(&'a BStr, git_attributes::State<'a>)>,
    usize,
);

fn set(attr: &str) -> (&BStr, State) {
    (attr.as_bytes().as_bstr(), State::Set)
}

fn unset(attr: &str) -> (&BStr, State) {
    (attr.as_bytes().as_bstr(), State::Unset)
}

fn unspecified(attr: &str) -> (&BStr, State) {
    (attr.as_bytes().as_bstr(), State::Unspecified)
}

fn value<'a, 'b>(attr: &'a str, value: &'b str) -> (&'a BStr, State<'b>) {
    (attr.as_bytes().as_bstr(), State::Value(value.as_bytes().as_bstr()))
}

fn try_line(input: &str) -> Result<ExpandedAttribute, parse::attribute::Error> {
    let mut lines = git_attributes::parse(input.as_bytes());
    let res = expand(lines.next().unwrap())?;
    assert!(lines.next().is_none(), "expected only one line");
    Ok(res)
}

fn line(input: &str) -> ExpandedAttribute {
    let mut lines = git_attributes::parse(input.as_bytes());
    let res = expand(lines.next().expect("single line")).unwrap();
    assert!(lines.next().is_none(), "expected only one line");
    res
}

fn try_lines(input: &str) -> Result<Vec<ExpandedAttribute>, parse::attribute::Error> {
    git_attributes::parse(input.as_bytes()).map(expand).collect()
}

fn expand(
    input: Result<(BString, ignore::pattern::Mode, parse::attribute::Iter<'_>, usize), parse::attribute::Error>,
) -> Result<ExpandedAttribute<'_>, parse::attribute::Error> {
    let (pattern, mode, attrs, line_no) = input?;
    let attrs = attrs.collect::<Result<Vec<_>, _>>()?;
    Ok((pattern, mode, attrs, line_no))
}
