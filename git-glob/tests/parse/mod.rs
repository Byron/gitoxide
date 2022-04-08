use git_glob::pattern::Mode;

#[test]
fn mark_ends_with_pattern_specifically() {
    assert_eq!(
        git_glob::parse(br"*literal"),
        Some((r"*literal".into(), Mode::NO_SUB_DIR | Mode::ENDS_WITH, Some(0)))
    );
    assert_eq!(
        git_glob::parse(br"**literal"),
        Some((r"**literal".into(), Mode::NO_SUB_DIR, Some(0))),
        "double-asterisk won't allow for fast comparisons"
    );
    assert_eq!(
        git_glob::parse(br"*litera[l]"),
        Some((r"*litera[l]".into(), Mode::NO_SUB_DIR, Some(0)))
    );
    assert_eq!(
        git_glob::parse(br"*litera?"),
        Some((r"*litera?".into(), Mode::NO_SUB_DIR, Some(0)))
    );
    assert_eq!(
        git_glob::parse(br"*litera\?"),
        Some((r"*litera\?".into(), Mode::NO_SUB_DIR, Some(0))),
        "for now we don't handle escapes properly like git seems to do"
    );
}

#[test]
fn whitespace_only_is_ignored() {
    assert!(git_glob::parse(b"\n\r\n\t\t   \n").is_none());
}

#[test]
fn hash_symbols_are_not_special() {
    assert_eq!(
        git_glob::parse(b"# hello world"),
        Some(("# hello world".into(), Mode::NO_SUB_DIR, None))
    );
}

#[test]
fn backslashes_before_hashes_are_considered_an_escape_sequence() {
    assert_eq!(
        git_glob::parse(br"\#hello"),
        Some((r"#hello".into(), Mode::NO_SUB_DIR, None))
    );
}

#[test]
fn backslashes_are_part_of_the_pattern_if_not_in_specific_positions() {
    assert_eq!(
        git_glob::parse(br"\hello\world"),
        Some((r"\hello\world".into(), Mode::NO_SUB_DIR, Some(0)))
    );
}

#[test]
fn leading_exclamation_mark_negates_pattern() {
    assert_eq!(
        git_glob::parse(b"!hello"),
        Some(("hello".into(), Mode::NEGATIVE | Mode::NO_SUB_DIR, None))
    );
}

#[test]
fn leading_exclamation_marks_can_be_escaped_with_backslash() {
    assert_eq!(
        git_glob::parse(br"\!hello"),
        Some(("!hello".into(), Mode::NO_SUB_DIR, None))
    );
}

#[test]
fn absence_of_sub_directories_are_marked() {
    assert_eq!(git_glob::parse(br"a/b"), Some(("a/b".into(), Mode::empty(), None)));
    assert_eq!(git_glob::parse(br"ab"), Some(("ab".into(), Mode::NO_SUB_DIR, None)));
}

#[test]
fn trailing_slashes_are_marked_and_removed() {
    assert_eq!(
        git_glob::parse(b"dir/"),
        Some(("dir".into(), Mode::MUST_BE_DIR | Mode::NO_SUB_DIR, None))
    );
    assert_eq!(
        git_glob::parse(b"dir///"),
        Some(("dir//".into(), Mode::MUST_BE_DIR, None)),
        "but only the last slash is removed"
    );
}

#[test]
fn trailing_spaces_are_ignored() {
    assert_eq!(git_glob::parse(br"a   "), Some(("a".into(), Mode::NO_SUB_DIR, None)));
    assert_eq!(
        git_glob::parse(b"a\t\t  "),
        Some(("a\t\t".into(), Mode::NO_SUB_DIR, None)),
        "trailing tabs are not ignored"
    );
}

#[test]
fn trailing_spaces_can_be_escaped_to_be_literal() {
    assert_eq!(
        git_glob::parse(br"a  \ "),
        Some(("a   ".into(), Mode::NO_SUB_DIR, None)),
        "a single escape in front of the last desired space is enough"
    );
    assert_eq!(
        git_glob::parse(br"a  b  c "),
        Some(("a  b  c".into(), Mode::NO_SUB_DIR, None)),
        "spaces in the middle are fine"
    );
    assert_eq!(
        git_glob::parse(br"a\ \ \ "),
        Some(("a   ".into(), Mode::NO_SUB_DIR, None)),
        "one can also escape every single one"
    );
    assert_eq!(
        git_glob::parse(br"a \  "),
        Some(("a  ".into(), Mode::NO_SUB_DIR, None)),
        "or just the one in the middle, losing the last actual space"
    );
    assert_eq!(
        git_glob::parse(br"a   \"),
        Some(("a   ".into(), Mode::NO_SUB_DIR, None)),
        "escaping nothing also works as a whitespace protection"
    );
    assert_eq!(
        git_glob::parse(br"a   \\\ "),
        Some((r"a    ".into(), Mode::NO_SUB_DIR, None)),
        "strange things like these work too"
    );
    assert_eq!(
        git_glob::parse(br"a   \\ "),
        Some((r"a   ".into(), Mode::NO_SUB_DIR, None)),
        "strange things like these work as well"
    );
}
