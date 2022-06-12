use crate::spec::parse::{parse, try_parse};
use git_revision::spec;

#[test]
#[ignore]
fn regex_parsing_ignores_ranges_as_opposed_to_git() {
    for spec in [":/a..b", ":/a...b"] {
        let rec = parse(spec);

        assert!(rec.kind.is_none());
        assert_eq!(
            rec.patterns,
            vec![(spec[2..].into(), false)],
            "git parses ranges but I think it's merely coincidental rather than intended, not doing so allows to use '.' more liberally"
        );
    }
}

#[test]
#[ignore]
fn index_lookups_ignores_ranges_as_opposed_to_git() {
    for spec in [":a..b", ":a...b"] {
        let rec = parse(spec);

        assert!(rec.kind.is_none());
        assert_eq!(
            rec.patterns,
            vec![(spec[1..].into(), false)],
            "git parses ranges but it's never useful as these specs only ever produce blob ids"
        );
    }
}

#[test]
#[ignore]
fn various_forms_of_regex() {
    for (spec, (regex, negated)) in [
        (":/simple", ("simple", false)),
        (":/!-negated", ("negated", true)),
        (":/^from start", ("^from start", false)),
        (":/!!leading exclamation mark", ("!leading exclamation mark", false)),
        (":/with count{1}", ("with count{1}", false)),
        (
            ":/all-consuming makes navigation impossible^5~10",
            ("all-consuming makes navigation impossible^5~10", false),
        ),
    ] {
        let rec = parse(spec);

        assert!(rec.kind.is_none());
        assert_eq!(rec.find_ref[0], None);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.patterns, vec![(regex.into(), negated)]);
        assert_eq!(rec.calls, 1);
    }
}

#[test]
#[ignore]
fn various_valid_index_lookups_by_path() {
    for spec in [
        ":path",
        ":dir/path",
        ":./relative-to.cwd",
        ":../relative-to-cwd-too",
        ":navigation/is/ignored~10^^^",
    ] {
        let rec = parse(spec);

        assert!(rec.kind.is_none());
        assert_eq!(rec.find_ref[0], None);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.index_lookups, vec![(spec[1..].into(), 0)]);
        assert_eq!(rec.peel_to, vec![], "peeling only works for anchors");
        assert_eq!(rec.calls, 1);
    }
}

#[test]
#[ignore]
fn various_valid_index_lookups_by_path_and_stage() {
    for (spec, path, stage) in [
        (":0:path", "path", 0),
        (":1:dir/path", "dir/path", 1),
        (":2:dir/path@{part-of-path}", "dir/path@{part-of-path}", 2),
    ] {
        let rec = parse(spec);

        assert!(rec.kind.is_none());
        assert_eq!(rec.find_ref[0], None);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.index_lookups, vec![(path.into(), stage)]);
        assert_eq!(rec.peel_to, vec![], "peeling only works for anchors");
        assert_eq!(rec.calls, 1);
    }
}

#[test]
#[ignore]
fn empty_top_level_regex_are_invalid() {
    let err = try_parse(":/").unwrap_err();
    assert!(
        matches!(err, spec::parse::Error::EmptyTopLevelRegex),
        "git also can't do it, finds nothing instead. It could be the youngest commit in theory, but isn't"
    )
}

#[test]
#[ignore]
fn needs_suffix() {
    let err = try_parse(":").unwrap_err();
    assert!(
        matches!(err, spec::parse::Error::MissingColonSuffix),
        "git also can't do it, finds nothing instead. It could be the youngest commit in theory, but isn't"
    )
}

#[test]
#[ignore]
fn invalid_index_stage_is_part_of_path() {
    for spec in [":4:file", ":5:file", ":01:file", ":10:file"] {
        let rec = parse(spec);

        assert!(rec.kind.is_none());
        assert_eq!(rec.find_ref[0], None);
        assert_eq!(rec.prefix[0], None);
        assert_eq!(rec.patterns, vec![]);
        assert_eq!(
            rec.index_lookups,
            vec![(spec[1..].into(), 0)],
            "these count as stage 0 lookups"
        );
        assert_eq!(rec.peel_to, vec![]);
        assert_eq!(rec.calls, 1);
    }
}
