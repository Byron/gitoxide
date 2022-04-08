use bstr::{BStr, ByteSlice};
use git_attributes::{parse, State};
use git_glob::pattern::Mode;
use git_testtools::fixture_bytes;

#[test]
fn byte_order_marks_are_no_patterns() {
    assert_eq!(line("\u{feff}hello"), (pattern(r"hello", Mode::NO_SUB_DIR), vec![], 1));
    assert_eq!(
        line("\u{feff}\"hello\""),
        (pattern(r"hello", Mode::NO_SUB_DIR), vec![], 1)
    );
}

#[test]
fn line_numbers_are_counted_correctly() {
    let input = fixture_bytes("attributes/various.txt");
    assert_eq!(
        try_lines(&String::from_utf8(input).unwrap()).unwrap(),
        vec![
            (pattern(r"*.[oa]", Mode::NO_SUB_DIR), vec![set("c")], 2),
            (
                pattern(r"*.html", Mode::NO_SUB_DIR | Mode::ENDS_WITH),
                vec![set("a"), value("b", "c")],
                5
            ),
            (pattern(r"!foo.html", Mode::NO_SUB_DIR), vec![set("x")], 8),
            (pattern(r"#a/path", Mode::empty()), vec![unset("a")], 10),
            (pattern(r"/*", Mode::empty()), vec![unspecified("b")], 11),
        ]
    );
}

#[test]
fn line_endings_can_be_windows_or_unix() {
    assert_eq!(
        try_lines("unix\nwindows\r\nlast").unwrap(),
        vec![
            (pattern(r"unix", Mode::NO_SUB_DIR), vec![], 1),
            (pattern(r"windows", Mode::NO_SUB_DIR), vec![], 2),
            (pattern(r"last", Mode::NO_SUB_DIR), vec![], 3)
        ]
    );
}

#[test]
fn comment_lines_are_ignored_as_well_as_empty_ones() {
    assert!(git_attributes::parse(b"# hello world").next().is_none());
    assert!(git_attributes::parse(b"# \"hello world\"").next().is_none());
    assert!(
        git_attributes::parse(b" \t\r# \"hello world\"").next().is_none(),
        "also behind leading whitespace"
    );
    assert!(git_attributes::parse(b"\n\r\n\t\t   \n").next().is_none());
}

#[test]
fn leading_whitespace_is_ignored() {
    assert_eq!(line(" \r\tp"), (pattern(r"p", Mode::NO_SUB_DIR), vec![], 1));
    assert_eq!(line(" \r\t\"p\""), (pattern(r"p", Mode::NO_SUB_DIR), vec![], 1));
}

#[test]
fn quotes_separate_attributes_even_without_whitespace() {
    assert_eq!(
        line(r#""path"a b"#),
        (pattern(r"path", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1)
    );
}

#[test]
fn comment_can_be_escaped_like_gitignore_or_quoted() {
    assert_eq!(
        line(r"\#hello"),
        (pattern(r"#hello", Mode::NO_SUB_DIR), vec![], 1),
        "undocumented, but definitely works"
    );
    assert_eq!(line("\"# hello\""), (pattern(r"# hello", Mode::NO_SUB_DIR), vec![], 1));
}

#[test]
fn exclamation_marks_must_be_escaped_or_error_unlike_gitignore() {
    assert_eq!(line(r"\!hello"), (pattern(r"!hello", Mode::NO_SUB_DIR), vec![], 1));
    assert!(matches!(
        try_line(r"!hello"),
        Err(parse::Error::PatternNegation { line_number: 1, .. })
    ));
    assert!(
        matches!(
            try_line(r#""!hello""#),
            Err(parse::Error::PatternNegation { line_number: 1, .. }),
        ),
        "even in quotes they trigger…"
    );
    assert_eq!(
        line(r#""\\!hello""#),
        (pattern(r"!hello", Mode::NO_SUB_DIR), vec![], 1),
        "…and must be double-escaped, once to get through quote, then to get through parse ignore line"
    );
}

#[test]
fn invalid_escapes_in_quotes_are_an_error() {
    assert!(matches!(try_line(r#""\!hello""#), Err(parse::Error::Unquote(_)),),);
}

#[test]
fn custom_macros_can_be_differentiated() {
    assert_eq!(
        line(r#"[attr]foo bar -baz"#),
        (macro_(r"foo"), vec![set("bar"), unset("baz")], 1)
    );

    assert_eq!(
        line(r#""[attr]foo" bar -baz"#),
        (macro_(r"foo"), vec![set("bar"), unset("baz")], 1),
        "it works after unquoting even, making it harder to denote a file name with [attr] prefix"
    );
}

#[test]
fn custom_macros_must_be_valid_attribute_names() {
    assert!(matches!(
        try_line(r"[attr]-prefixdash"),
        Err(parse::Error::MacroName { line_number: 1, .. })
    ));
    assert!(matches!(
        try_line(r"[attr]你好"),
        Err(parse::Error::MacroName { line_number: 1, .. })
    ));
}

#[test]
fn attribute_names_must_not_begin_with_dash_and_must_be_ascii_only() {
    assert!(matches!(
        try_line(r"p !-a"),
        Err(parse::Error::AttributeName { line_number: 1, .. })
    ));
    assert!(
        matches!(
            try_line(r#"p !!a"#),
            Err(parse::Error::AttributeName { line_number: 1, .. })
        ),
        "exclamation marks aren't allowed either"
    );
    assert!(
        matches!(
            try_line(r#"p 你好"#),
            Err(parse::Error::AttributeName { line_number: 1, .. })
        ),
        "nor is utf-8 encoded characters - gitoxide could consider to relax this when established"
    );
}

#[test]
fn attributes_are_parsed_behind_various_whitespace_characters() {
    assert_eq!(
        line(r#"p a b"#),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1),
        "behind space"
    );
    assert_eq!(
        line(r#""p" a b"#),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1),
        "behind space"
    );
    assert_eq!(
        line("p\ta\tb"),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1),
        "behind tab"
    );
    assert_eq!(
        line("\"p\"\ta\tb"),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1),
        "behind tab"
    );
    assert_eq!(
        line("p \t a \t b"),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1),
        "behind a mix of space and tab"
    );
    assert_eq!(
        line("\"p\" \t a \t b"),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a"), set("b")], 1),
        "behind a mix of space and tab"
    );
}

#[test]
fn attributes_come_in_different_flavors_due_to_prefixes() {
    assert_eq!(
        line(r#"p set -unset !unspecified -set"#),
        (
            pattern("p", Mode::NO_SUB_DIR),
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
            pattern("p", Mode::NO_SUB_DIR),
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
            pattern("p", Mode::NO_SUB_DIR),
            vec![set("set"), unset("unset"), unspecified("unspecified")],
            1
        )
    );
}

#[test]
fn trailing_whitespace_in_attributes_is_ignored() {
    assert_eq!(line("p a \r\t"), (pattern("p", Mode::NO_SUB_DIR), vec![set("a")], 1),);
    assert_eq!(
        line("\"p\" a \r\t"),
        (pattern("p", Mode::NO_SUB_DIR), vec![set("a")], 1),
    );
}

type ExpandedAttribute<'a> = (parse::Kind, Vec<(&'a BStr, git_attributes::State<'a>)>, usize);

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

fn pattern(name: &str, flags: git_glob::pattern::Mode) -> parse::Kind {
    parse::Kind::Pattern(git_glob::Pattern {
        text: name.into(),
        mode: flags,
        no_wildcard_len: 0,
    })
}

fn macro_(name: &str) -> parse::Kind {
    parse::Kind::Macro(name.into())
}

fn try_line(input: &str) -> Result<ExpandedAttribute, parse::Error> {
    let mut lines = git_attributes::parse(input.as_bytes());
    let res = expand(lines.next().unwrap())?;
    assert!(lines.next().is_none(), "expected only one line");
    Ok(res)
}

fn line(input: &str) -> ExpandedAttribute {
    try_line(input).unwrap()
}

fn try_lines(input: &str) -> Result<Vec<ExpandedAttribute>, parse::Error> {
    git_attributes::parse(input.as_bytes()).map(expand).collect()
}

fn expand(
    input: Result<(parse::Kind, parse::Iter<'_>, usize), parse::Error>,
) -> Result<ExpandedAttribute<'_>, parse::Error> {
    let (pattern, attrs, line_no) = input?;
    let attrs = attrs.collect::<Result<Vec<_>, _>>()?;
    Ok((pattern, attrs, line_no))
}
