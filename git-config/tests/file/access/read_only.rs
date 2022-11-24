use std::{borrow::Cow, convert::TryFrom};

use bstr::BStr;
use git_config::{
    color,
    file::{init, Metadata},
    integer, path, Boolean, Color, File, Integer,
};

use crate::file::cow_str;

/// Asserts we can cast into all variants of our type
#[test]
fn get_value_for_all_provided_values() -> crate::Result {
    let config = r#"
        [core]
            other-quoted = "hello"
        [core]
            bool-explicit = false
            bool-implicit
            integer-no-prefix = 10
            integer-prefix = 10g
            color = brightgreen red \
            bold
            other = hello world
            other-quoted = "hello world"
            location = ~/tmp
            location-quoted = "~/quoted"
    "#;
    for lossy in [false, true] {
        let config = File::from_bytes_no_includes(
            config.as_bytes(),
            Metadata::api(),
            init::Options {
                lossy,
                ..Default::default()
            },
        )?;

        assert!(!config.value::<Boolean>("core", None, "bool-explicit")?.0);
        assert!(!config.boolean("core", None, "bool-explicit").expect("exists")?);
        assert!(!config.boolean_by_key("core.bool-explicit").expect("exists")?);

        assert!(
            config.value::<Boolean>("core", None, "bool-implicit").is_err(),
            "this cannot work like in git as the original value isn't there for us"
        );
        assert!(
            config.boolean("core", None, "bool-implicit").expect("present")?,
            "implicit booleans resolve to being true"
        );
        assert_eq!(
            config.string("core", None, "bool-implicit"),
            None,
            "unset values are not present"
        );
        assert_eq!(
            config.strings("core", None, "bool-implicit").expect("present"),
            &[cow_str("")],
            "unset values show up as empty within a string array"
        );
        assert_eq!(
            config.strings_by_key("core.bool-implicit").expect("present"),
            &[cow_str("")],
        );

        assert_eq!(config.string("doesnt", None, "exist"), None);

        assert_eq!(
            config.value::<Integer>("core", None, "integer-no-prefix")?,
            Integer {
                value: 10,
                suffix: None
            }
        );

        assert_eq!(
            config.value::<Integer>("core", None, "integer-no-prefix")?,
            Integer {
                value: 10,
                suffix: None
            }
        );

        assert_eq!(
            config.value::<Integer>("core", None, "integer-prefix")?,
            Integer {
                value: 10,
                suffix: Some(integer::Suffix::Gibi),
            }
        );

        assert_eq!(
            config.value::<Color>("core", None, "color")?,
            Color {
                foreground: Some(color::Name::BrightGreen),
                background: Some(color::Name::Red),
                attributes: color::Attribute::BOLD
            }
        );

        {
            let string = config.value::<Cow<'_, BStr>>("core", None, "other")?;
            assert_eq!(string, cow_str("hello world"));
            assert!(
                matches!(string, Cow::Borrowed(_)),
                "no copy is made, we reference the `file` itself"
            );
        }

        assert_eq!(
            config.string("core", None, "other-quoted").unwrap(),
            cow_str("hello world")
        );

        {
            let strings = config.strings("core", None, "other-quoted").unwrap();
            assert_eq!(strings, vec![cow_str("hello"), cow_str("hello world")]);
            assert!(matches!(strings[0], Cow::Borrowed(_)));
            assert!(matches!(strings[1], Cow::Borrowed(_)));
        }

        {
            let cow = config.string("core", None, "other").expect("present");
            assert_eq!(cow.as_ref(), "hello world");
            assert!(matches!(cow, Cow::Borrowed(_)));
        }
        assert_eq!(
            config.string("core", None, "other-quoted").expect("present").as_ref(),
            "hello world"
        );

        {
            let actual = config.value::<git_config::Path>("core", None, "location")?;
            assert_eq!(&*actual, "~/tmp", "no interpolation occurs when querying a path");

            let home = std::env::current_dir()?;
            let expected = home.join("tmp");
            assert!(matches!(actual.value, Cow::Borrowed(_)));
            assert_eq!(
                actual
                    .interpolate(path::interpolate::Context {
                        home_dir: home.as_path().into(),
                        ..Default::default()
                    })
                    .unwrap(),
                expected
            );
        }

        let actual = config.path("core", None, "location").expect("present");
        assert_eq!(&*actual, "~/tmp");
        let actual = config.path_by_key("core.location").expect("present");
        assert_eq!(&*actual, "~/tmp");

        let actual = config.path("core", None, "location-quoted").expect("present");
        assert_eq!(&*actual, "~/quoted");

        let actual = config.value::<git_config::Path>("core", None, "location-quoted")?;
        assert_eq!(&*actual, "~/quoted", "but the path is unquoted");
    }

    Ok(())
}

#[test]
fn get_value_looks_up_all_sections_before_failing() -> crate::Result {
    let config = r#"
        [core]
            bool-explicit = false
            bool-implicit = false
        [core]
            bool-implicit
    "#;

    let file = File::try_from(config)?;

    // Checks that we check the last entry first still
    assert!(
        !file.value::<Boolean>("core", None, "bool-implicit")?.0,
        "implicit bool is invisible to `value` and boolean is the only value we want. Would have to special case it."
    );
    assert!(
        file.boolean("core", None, "bool-implicit").expect("present")?,
        "correct handling of booleans is implemented specifically"
    );

    assert!(
        !file.value::<Boolean>("core", None, "bool-explicit")?.0,
        "explicit values always work"
    );

    Ok(())
}

#[test]
fn section_names_are_case_insensitive() -> crate::Result {
    let config = "[core] a=true";
    let file = File::try_from(config)?;
    assert_eq!(
        file.value::<Boolean>("core", None, "a").unwrap(),
        file.value::<Boolean>("CORE", None, "a").unwrap()
    );

    Ok(())
}

#[test]
fn value_names_are_case_insensitive() -> crate::Result {
    let config = "[core]
        a = true
        A = false";
    let file = File::try_from(config)?;
    assert_eq!(file.values::<Boolean>("core", None, "a")?.len(), 2);
    assert_eq!(
        file.value::<Boolean>("core", None, "a").unwrap(),
        file.value::<Boolean>("core", None, "A").unwrap()
    );

    Ok(())
}

#[test]
fn single_section() {
    let config = File::try_from("[core]\na=b\nc").unwrap();
    let first_value = config.string("core", None, "a").unwrap();
    assert_eq!(first_value, cow_str("b"));

    assert!(
        config.raw_value("core", None, "c").is_err(),
        "value is considered false as it is without '=', so it's like not present"
    );

    assert!(
        config.boolean("core", None, "c").expect("present").unwrap(),
        "asking for a boolean is true true, as per git rules"
    );
}

#[test]
fn sections_by_name() {
    let config = r#"
    [core]
        repositoryformatversion = 0
        filemode = true
        bare = false
        logallrefupdates = true
    [remote "origin"]
        url = git@github.com:Byron/gitoxide.git
        fetch = +refs/heads/*:refs/remotes/origin/*
    "#;

    let config = File::try_from(config).unwrap();
    let value = config.string("remote", Some("origin".into()), "url").unwrap();
    assert_eq!(value, cow_str("git@github.com:Byron/gitoxide.git"));
}

#[test]
fn multi_line_value_plain() {
    let config = r#"
[alias]
   save = !git status \
        && git add -A \
        && git commit -m \"$1\" \
        && git push -f \
        && git log -1 \
        && :            # comment
    "#;

    let config = File::try_from(config).unwrap();

    let expected = r#"!git status         && git add -A         && git commit -m "$1"         && git push -f         && git log -1         && :"#;
    assert_eq!(config.raw_value("alias", None, "save").unwrap().as_ref(), expected);
    assert_eq!(config.string("alias", None, "save").unwrap().as_ref(), expected);
}

#[test]
fn complex_quoted_values() {
    let config = r#"
    [core]
            escape-sequence = "hi\nho\n\tthere\bi\\\" \""
"#;
    let config = File::try_from(config).unwrap();
    let expected = "hi\nho\n\ttheri\\\" \"";
    assert_eq!(
        config.raw_value("core", None, "escape-sequence").unwrap().as_ref(),
        expected,
        "raw_value is normalized…"
    );
    assert_eq!(
        config.string("core", None, "escape-sequence").unwrap().as_ref(),
        expected,
        "…and so is the comfort API"
    );
}

#[test]
fn multi_line_value_outer_quotes_unescaped_inner_quotes() {
    let config = r#"
[alias]
   save = "!f() { \
           git status; \
           git add -A; \
           git commit -m "$1"; \
           git push -f; \
           git log -1;  \
        }; \
        f;  \
        unset f"
"#;
    let config = File::try_from(config).unwrap();
    let expected = r#"!f() {            git status;            git add -A;            git commit -m $1;            git push -f;            git log -1;          };         f;          unset f"#;
    assert_eq!(config.raw_value("alias", None, "save").unwrap().as_ref(), expected);
}

#[test]
fn multi_line_value_outer_quotes_escaped_inner_quotes() {
    let config = r#"
[alias]
   save = "!f() { \
           git status; \
           git add -A; \
           git commit -m \"$1\"; \
           git push -f; \
           git log -1;  \
        }; \
        f;  \
        unset f"
"#;
    let config = File::try_from(config).unwrap();
    let expected = r#"!f() {            git status;            git add -A;            git commit -m "$1";            git push -f;            git log -1;          };         f;          unset f"#;
    assert_eq!(config.raw_value("alias", None, "save").unwrap().as_ref(), expected);
}

#[test]
fn overrides_with_implicit_booleans_work_in_single_section() {
    let config = r#"
        [a]
            b = false
            b
        "#;
    let config = File::try_from(config).unwrap();
    assert_eq!(
        config.boolean("a", None, "b"),
        Some(Ok(true)),
        "empty implicit booleans "
    );
}

#[test]
fn overrides_with_implicit_booleans_work_across_sections() {
    let config = r#"
        [a]
            b = false
        [a]
            b
        "#;
    let config = File::try_from(config).unwrap();
    assert_eq!(
        config.boolean("a", None, "b"),
        Some(Ok(true)),
        "empty implicit booleans "
    );
}
