use std::{convert::TryFrom, fs};

use bstr::BString;
use git_config::file::from_paths;
use git_ref::FullName;
use tempfile::tempdir;

use crate::file::{cow_str, from_paths::escape_backslashes};

#[test]
fn literal_branch_names_match() {
    assert_section_value(Options {
        condition: "literal-match",
        branch_name: "refs/heads/literal-match",
        expect: Value::OverrideByInclude,
    });
}

#[test]
fn full_ref_names_do_not_match() {
    assert_section_value(Options {
        condition: "refs/heads/simple",
        branch_name: "refs/heads/simple",
        expect: Value::Base,
    });
}

#[test]
fn non_branches_never_match() {
    assert_section_value(Options {
        condition: "good",
        branch_name: "refs/bisect/good",
        expect: Value::Base,
    });
}

#[test]
fn patterns_ending_with_slash_match_subdirectories_recursively() {
    assert_section_value(Options {
        condition: "feature/b/",
        branch_name: "refs/heads/feature/b/start",
        expect: Value::OverrideByInclude,
    });
    assert_section_value(Options {
        condition: "feature/",
        branch_name: "refs/heads/feature/b/start",
        expect: Value::OverrideByInclude,
    });
    assert_section_value_msg(
        Options {
            condition: "feature/b/start",
            branch_name: "refs/heads/feature/b/start",
            expect: Value::OverrideByInclude,
        },
        "just for good measure, we would expect branch paths to work as well".into(),
    );
}

#[test]
fn simple_glob_patterns() {
    assert_section_value(Options {
        condition: "prefix*",
        branch_name: "refs/heads/prefixsuffix",
        expect: Value::OverrideByInclude,
    });
    assert_section_value_msg(
        Options {
            condition: "prefix*",
            branch_name: "refs/heads/prefix/suffix",
            expect: Value::Base,
        },
        "single-stars do not cross component boundaries".into(),
    );
    assert_section_value(Options {
        condition: "*suffix",
        branch_name: "refs/heads/prefixsuffix",
        expect: Value::OverrideByInclude,
    });
    assert_section_value(Options {
        condition: "*/suffix",
        branch_name: "refs/heads/prefix/suffix",
        expect: Value::OverrideByInclude,
    });
    assert_section_value_msg(
        Options {
            condition: "*suffix",
            branch_name: "refs/heads/prefix/suffix",
            expect: Value::Base,
        },
        "single-stars do not cross component boundaries".into(),
    );
}

#[test]
fn simple_globs_do_not_cross_component_boundary() {
    assert_section_value(Options {
        condition: "feature/*/start",
        branch_name: "refs/heads/feature/a/start",
        expect: Value::OverrideByInclude,
    });
    assert_section_value_msg(
        Options {
            condition: "feature/*/start",
            branch_name: "refs/heads/feature/a/b/start",
            expect: Value::Base,
        },
        "path matching would never match 'a/b' as it cannot cross /".into(),
    );
}

#[test]
fn double_star_globs_cross_component_boundaries() {
    assert_section_value(Options {
        condition: "feature/**/start",
        branch_name: "refs/heads/feature/a/b/start",
        expect: Value::OverrideByInclude,
    });
}

enum Value {
    Base,
    OverrideByInclude,
}

struct Options<'a> {
    condition: &'a str,
    branch_name: &'a str,
    expect: Value,
}

fn assert_section_value(opts: Options) {
    assert_section_value_msg(opts, None)
}

fn assert_section_value_msg(
    Options {
        condition,
        branch_name,
        expect,
    }: Options,
    message: Option<&str>,
) {
    let dir = tempdir().unwrap();
    let root_config = dir.path().join("root.config");
    let included_config = dir.path().join("include.config");

    fs::write(
        &root_config,
        format!(
            r#"
[section]
value = base-value

[includeIf "onbranch:{}"]
path = {}"#,
            condition,
            escape_backslashes(&included_config),
        ),
    )
    .unwrap();

    fs::write(
        included_config,
        r#"
[section]
value = branch-override-by-include
"#,
    )
    .unwrap();

    let branch_name = FullName::try_from(BString::from(branch_name)).unwrap();
    let branch_name = branch_name.as_ref();
    let options = from_paths::Options {
        branch_name: Some(branch_name),
        ..Default::default()
    };

    let config = git_config::File::from_paths(Some(&root_config), options).unwrap();
    assert_eq!(
        config.string("section", None, "value"),
        Some(cow_str(match expect {
            Value::OverrideByInclude => "branch-override-by-include",
            Value::Base => "base-value",
        })),
        "{}, info: {:?}",
        match expect {
            Value::Base => "the base value should not be overridden as the branch does not match",
            Value::OverrideByInclude =>
                "the base value is overridden by an included file because the condition matches",
        },
        message
    );
}
