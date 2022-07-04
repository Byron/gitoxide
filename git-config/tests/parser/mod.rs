use std::borrow::Cow;

use git_config::parser::{parse_from_str, Event, Key, ParsedSectionHeader, SectionHeaderName};

pub fn section_header_event(name: &str, subsection: impl Into<Option<(&'static str, &'static str)>>) -> Event<'_> {
    Event::SectionHeader(section_header(name, subsection))
}

pub fn section_header(
    name: &str,
    subsection: impl Into<Option<(&'static str, &'static str)>>,
) -> ParsedSectionHeader<'_> {
    let name = SectionHeaderName(Cow::Borrowed(name.into()));
    if let Some((separator, subsection_name)) = subsection.into() {
        ParsedSectionHeader {
            name,
            separator: Some(Cow::Borrowed(separator.into())),
            subsection_name: Some(Cow::Borrowed(subsection_name.into())),
        }
    } else {
        ParsedSectionHeader {
            name,
            separator: None,
            subsection_name: None,
        }
    }
}

fn name(name: &'static str) -> Event<'static> {
    Event::Key(Key(Cow::Borrowed(name.into())))
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
        parse_from_str(config)
            .unwrap()
            .into_vec(),
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
    assert_eq!(parse_from_str("").unwrap().into_vec(), vec![]);
}

#[test]
fn parse_whitespace() {
    assert_eq!(
        parse_from_str("\n   \n \n").unwrap().into_vec(),
        vec![newline(), whitespace("   "), newline(), whitespace(" "), newline()]
    )
}

#[test]
fn newline_events_are_merged() {
    assert_eq!(
        parse_from_str("\n\n\n\n\n").unwrap().into_vec(),
        vec![newline_custom("\n\n\n\n\n")]
    );
}

#[test]
fn error() {
    let input = "[a_b]\n c=d";
    assert_eq!(
        parse_from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 1 while trying to parse a section header: '[a_b]\n c=d'",
        "underscores in section names aren't allowed and will be rejected by git"
    );
    let input = "[core] a=b\n 4a=3";
    assert_eq!(
        parse_from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 2 while trying to parse a config name: '4a=3'"
    );
    let input = "[core] a=b\n =3";
    assert_eq!(
        parse_from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 2 while trying to parse a config name: '=3'"
    );
    let input = "[core";
    assert_eq!(
        parse_from_str(input).unwrap_err().to_string(),
        "Got an unexpected token on line 1 while trying to parse a section header: '[core'"
    );
}
