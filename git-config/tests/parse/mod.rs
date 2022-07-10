use std::borrow::Cow;

use git_config::parse::{section, Event, Events, Section};

pub fn section_header_event(name: &str, subsection: impl Into<Option<(&'static str, &'static str)>>) -> Event<'_> {
    Event::SectionHeader(section_header(name, subsection))
}

pub fn section_header(name: &str, subsection: impl Into<Option<(&'static str, &'static str)>>) -> section::Header<'_> {
    let name = section::Name(Cow::Borrowed(name.into()));
    if let Some((separator, subsection_name)) = subsection.into() {
        section::Header {
            name,
            separator: Some(Cow::Borrowed(separator.into())),
            subsection_name: Some(Cow::Borrowed(subsection_name.into())),
        }
    } else {
        section::Header {
            name,
            separator: None,
            subsection_name: None,
        }
    }
}

fn name(name: &'static str) -> Event<'static> {
    Event::SectionKey(section::Key(Cow::Borrowed(name.into())))
}

fn value(value: &'static str) -> Event<'static> {
    Event::Value(Cow::Borrowed(value.into()))
}

fn newline() -> Event<'static> {
    newline_custom("\n")
}

fn newline_custom(value: &'static str) -> Event<'static> {
    Event::Newline(Cow::Borrowed(value.into()))
}

fn whitespace(value: &'static str) -> Event<'static> {
    Event::Whitespace(Cow::Borrowed(value.into()))
}

fn separator() -> Event<'static> {
    Event::KeyValueSeparator
}

#[test]
fn size_in_memory() {
    assert_eq!(
        std::mem::size_of::<Section<'_>>(),
        6768,
        "This shouldn't change without us noticing"
    );
    assert_eq!(
        std::mem::size_of::<Event<'_>>(),
        104,
        "This shouldn't change without us noticing"
    );
    assert_eq!(
        std::mem::size_of::<Events<'_>>(),
        872,
        "This shouldn't change without us noticing"
    );
}

#[test]
#[rustfmt::skip]
fn personal_config() {
    let config = r#"[user]
        email = code@eddie.sh
        name = Foo Bar
[core]
        autocrlf = input
[push]
        default = simple
[commit]
        gpgsign = true
[gpg]
        program = gpg
[url "ssh://git@github.com/"]
        insteadOf = "github://"
[url "ssh://git@git.eddie.sh/edward/"]
        insteadOf = "gitea://"
[pull]
        ff = only
[init]
        defaultBranch = master"#;

    assert_eq!(
        Events::from_str(config)
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![
            section_header_event("user", None),
            newline(),

            whitespace("        "),
            name("email"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("code@eddie.sh"),
            newline(),

            whitespace("        "),
            name("name"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("Foo Bar"),
            newline(),

            section_header_event("core", None),
            newline(),

            whitespace("        "),
            name("autocrlf"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("input"),
            newline(),

            section_header_event("push", None),
            newline(),

            whitespace("        "),
            name("default"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("simple"),
            newline(),

            section_header_event("commit", None),
            newline(),

            whitespace("        "),
            name("gpgsign"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("true"),
            newline(),

            section_header_event("gpg", None),
            newline(),

            whitespace("        "),
            name("program"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("gpg"),
            newline(),

            section_header_event("url", (" ", "ssh://git@github.com/")),
            newline(),

            whitespace("        "),
            name("insteadOf"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("\"github://\""),
            newline(),

            section_header_event("url", (" ", "ssh://git@git.eddie.sh/edward/")),
            newline(),

            whitespace("        "),
            name("insteadOf"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("\"gitea://\""),
            newline(),

            section_header_event("pull", None),
            newline(),

            whitespace("        "),
            name("ff"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("only"),
            newline(),

            section_header_event("init", None),
            newline(),

            whitespace("        "),
            name("defaultBranch"),
            whitespace(" "),
            separator(),
            whitespace(" "),
            value("master"),
        ]
    );
}

#[test]
fn parse_empty() {
    assert_eq!(Events::from_str("").unwrap().into_vec(), vec![]);
}

#[test]
fn parse_whitespace() {
    assert_eq!(
        Events::from_str("\n   \n \n").unwrap().into_vec(),
        vec![newline(), whitespace("   "), newline(), whitespace(" "), newline()]
    )
}

#[test]
fn newline_events_are_merged() {
    assert_eq!(
        Events::from_str("\n\n\n\n\n").unwrap().into_vec(),
        vec![newline_custom("\n\n\n\n\n")]
    );
}

#[test]
fn error() {
    let input = "[a_b]\n c=d";
    assert_eq!(
        Events::from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 1 while trying to parse a section header: '[a_b]\n c=d'",
        "underscores in section names aren't allowed and will be rejected by git"
    );
    let input = "[core] a=b\\\n cd\n[core]\n\n 4a=3";
    assert_eq!(
        Events::from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 5 while trying to parse a name: '4a=3'"
    );
    let input = "[core] a=b\\\n cd\n 4a=3";
    assert_eq!(
        Events::from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 3 while trying to parse a name: '4a=3'"
    );
    let input = "[core] a=b\n 4a=3";
    assert_eq!(
        Events::from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 2 while trying to parse a name: '4a=3'"
    );
    let input = "[core] a=b\n =3";
    assert_eq!(
        Events::from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 2 while trying to parse a name: '=3'"
    );
    let input = "[core";
    assert_eq!(
        Events::from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 1 while trying to parse a section header: '[core'"
    );
}

mod key {
    use std::cmp::Ordering;

    use crate::parse::section::Key;

    #[test]
    fn case_insentive_eq() {
        assert_eq!(Key::from("aBc"), Key::from("AbC"));
    }

    #[test]
    fn case_insentive_ord() {
        assert_eq!(Key::from("a").cmp(&Key::from("a")), Ordering::Equal);
        assert_eq!(Key::from("aBc").cmp(&Key::from("AbC")), Ordering::Equal);
    }

    #[test]
    fn case_insentive_hash() {
        fn calculate_hash<T: std::hash::Hash>(t: T) -> u64 {
            use std::hash::Hasher;
            let mut s = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        assert_eq!(calculate_hash(Key::from("aBc")), calculate_hash(Key::from("AbC")));
    }
}

mod events {
    use crate::parse::Events;

    #[test]
    fn parser_skips_bom() {
        let bytes = b"
        [core]
            a = 1
    ";
        let bytes_with_gb18030_bom = "\u{feff}
        [core]
            a = 1
    ";

        assert_eq!(
            Events::from_bytes(bytes),
            Events::from_bytes(bytes_with_gb18030_bom.as_bytes())
        );
        assert_eq!(
            Events::from_bytes_owned(bytes, None),
            Events::from_bytes_owned(bytes_with_gb18030_bom.as_bytes(), None)
        );
    }
}

#[cfg(test)]
mod error {
    use crate::parse::Events;

    #[test]
    fn line_no_is_one_indexed() {
        assert_eq!(Events::from_str("[hello").unwrap_err().line_number(), 1);
    }

    #[test]
    fn remaining_data_contains_bad_tokens() {
        assert_eq!(Events::from_str("[hello").unwrap_err().remaining_data(), b"[hello");
    }

    #[test]
    fn to_string_truncates_extra_values() {
        assert_eq!(
            Events::from_str("[1234567890").unwrap_err().to_string(),
            "Got an unexpected token on line 1 while trying to parse a section header: '[123456789' ... (1 characters omitted)"
        );
    }

    #[test]
    fn detected_by_fuzz() {
        assert!(Events::from_str("[]I=").is_err());
    }
}
