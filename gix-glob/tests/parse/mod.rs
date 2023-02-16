use gix_glob::{pattern::Mode, Pattern};

#[test]
fn mark_ends_with_pattern_specifically() {
    assert_eq!(
        gix_glob::parse(br"*literal"),
        pat(r"*literal", Mode::NO_SUB_DIR | Mode::ENDS_WITH, Some(0))
    );
    assert_eq!(
        gix_glob::parse(br"**literal"),
        pat(r"**literal", Mode::NO_SUB_DIR, Some(0)),
        "double-asterisk won't allow for fast comparisons"
    );
    assert_eq!(
        gix_glob::parse(br"*litera[l]"),
        pat(r"*litera[l]", Mode::NO_SUB_DIR, Some(0))
    );
    assert_eq!(
        gix_glob::parse(br"*litera?"),
        pat(r"*litera?", Mode::NO_SUB_DIR, Some(0))
    );
    assert_eq!(
        gix_glob::parse(br"*litera\?"),
        pat(r"*litera\?", Mode::NO_SUB_DIR, Some(0)),
        "for now we don't handle escapes properly like git seems to do"
    );
}

fn pat(pattern: &str, mode: Mode, first_glob_char_pos: Option<usize>) -> Option<Pattern> {
    Some(Pattern {
        text: pattern.into(),
        mode,
        first_wildcard_pos: first_glob_char_pos,
    })
}

#[test]
fn whitespace_only_is_ignored() {
    assert!(gix_glob::parse(b"\n\r\n\t\t   \n").is_none());
}

#[test]
fn hash_symbols_are_not_special() {
    assert_eq!(
        gix_glob::parse(b"# hello world"),
        pat("# hello world", Mode::NO_SUB_DIR, None)
    );
}

#[test]
fn backslashes_before_hashes_are_considered_an_escape_sequence() {
    assert_eq!(gix_glob::parse(br"\#hello"), pat(r"#hello", Mode::NO_SUB_DIR, None));
}

#[test]
fn backslashes_are_part_of_the_pattern_if_not_in_specific_positions() {
    assert_eq!(
        gix_glob::parse(br"\hello\world"),
        pat(r"\hello\world", Mode::NO_SUB_DIR, Some(0))
    );
}

#[test]
fn leading_exclamation_mark_negates_pattern() {
    assert_eq!(
        gix_glob::parse(b"!hello"),
        pat("hello", Mode::NEGATIVE | Mode::NO_SUB_DIR, None)
    );
}

#[test]
fn leading_exclamation_marks_can_be_escaped_with_backslash() {
    assert_eq!(gix_glob::parse(br"\!hello"), pat("!hello", Mode::NO_SUB_DIR, None));
}

#[test]
fn leading_slashes_mark_patterns_as_absolute() {
    assert_eq!(
        gix_glob::parse(br"/absolute"),
        pat("absolute", Mode::NO_SUB_DIR | Mode::ABSOLUTE, None)
    );

    assert_eq!(
        gix_glob::parse(br"/absolute/path"),
        pat("absolute/path", Mode::ABSOLUTE, None)
    );
}

#[test]
fn absence_of_sub_directories_are_marked() {
    assert_eq!(gix_glob::parse(br"a/b"), pat("a/b", Mode::empty(), None));
    assert_eq!(gix_glob::parse(br"ab"), pat("ab", Mode::NO_SUB_DIR, None));
}

#[test]
fn trailing_slashes_are_marked_and_removed() {
    assert_eq!(
        gix_glob::parse(b"dir/"),
        pat("dir", Mode::MUST_BE_DIR | Mode::NO_SUB_DIR, None)
    );
    assert_eq!(
        gix_glob::parse(b"dir///"),
        pat("dir//", Mode::MUST_BE_DIR, None),
        "but only the last slash is removed"
    );
}

#[test]
fn trailing_spaces_are_ignored() {
    assert_eq!(gix_glob::parse(br"a   "), pat("a", Mode::NO_SUB_DIR, None));
    assert_eq!(
        gix_glob::parse(b"a\t\t  "),
        pat("a\t\t", Mode::NO_SUB_DIR, None),
        "trailing tabs are not ignored"
    );
}

#[test]
fn trailing_spaces_can_be_escaped_to_be_literal() {
    assert_eq!(
        gix_glob::parse(br"a  \ "),
        pat("a   ", Mode::NO_SUB_DIR, None),
        "a single escape in front of the last desired space is enough"
    );
    assert_eq!(
        gix_glob::parse(br"a  b  c "),
        pat("a  b  c", Mode::NO_SUB_DIR, None),
        "spaces in the middle are fine"
    );
    assert_eq!(
        gix_glob::parse(br"a\ \ \ "),
        pat("a   ", Mode::NO_SUB_DIR, None),
        "one can also escape every single one"
    );
    assert_eq!(
        gix_glob::parse(br"a \  "),
        pat("a  ", Mode::NO_SUB_DIR, None),
        "or just the one in the middle, losing the last actual space"
    );
    assert_eq!(
        gix_glob::parse(br"a   \"),
        pat("a   ", Mode::NO_SUB_DIR, None),
        "escaping nothing also works as a whitespace protection"
    );
    assert_eq!(
        gix_glob::parse(br"a   \\\ "),
        pat(r"a    ", Mode::NO_SUB_DIR, None),
        "strange things like these work too"
    );
    assert_eq!(
        gix_glob::parse(br"a   \\ "),
        pat(r"a   ", Mode::NO_SUB_DIR, None),
        "strange things like these work as well"
    );
}
