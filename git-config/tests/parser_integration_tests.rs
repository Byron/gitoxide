use std::borrow::Cow;

use serde_git_config::parser::{parse_from_str, Event, ParsedSectionHeader};

fn gen_section_header(
    name: &str,
    subsection: impl Into<Option<(&'static str, &'static str)>>,
) -> Event<'_> {
    let name = Cow::Borrowed(name);
    Event::SectionHeader(
        if let Some((separator, subsection_name)) = subsection.into() {
            ParsedSectionHeader {
                name,
                separator: Some(Cow::Borrowed(separator)),
                subsection_name: Some(Cow::Borrowed(subsection_name)),
            }
        } else {
            ParsedSectionHeader {
                name,
                separator: None,
                subsection_name: None,
            }
        },
    )
}
fn name(name: &'static str) -> Event<'static> {
    Event::Key(Cow::Borrowed(name))
}

fn value(value: &'static str) -> Event<'static> {
    Event::Value(Cow::Borrowed(value))
}

fn newline() -> Event<'static> {
    Event::Newline(Cow::Borrowed("\n"))
}

fn whitespace(value: &'static str) -> Event<'static> {
    Event::Whitespace(Cow::Borrowed(value))
}

#[test]
#[rustfmt::skip]
fn personal_config() {
    let config = r#"[user]
        email = code@eddie.sh
        name = Edward Shen
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
            gen_section_header("user", None),
            newline(),

            whitespace("        "),
            name("email"),
            whitespace(" "),
            whitespace(" "),
            value("code@eddie.sh"),
            newline(),

            whitespace("        "),
            name("name"),
            whitespace(" "),
            whitespace(" "),
            value("Edward Shen"),
            newline(),

            gen_section_header("core", None),
            newline(),

            whitespace("        "),
            name("autocrlf"),
            whitespace(" "),
            whitespace(" "),
            value("input"),
            newline(),

            gen_section_header("push", None),
            newline(),

            whitespace("        "),
            name("default"),
            whitespace(" "),
            whitespace(" "),
            value("simple"),
            newline(),

            gen_section_header("commit", None),
            newline(),

            whitespace("        "),
            name("gpgsign"),
            whitespace(" "),
            whitespace(" "),
            value("true"),
            newline(),

            gen_section_header("gpg", None),
            newline(),

            whitespace("        "),
            name("program"),
            whitespace(" "),
            whitespace(" "),
            value("gpg"),
            newline(),

            gen_section_header("url", (" ", "ssh://git@github.com/")),
            newline(),

            whitespace("        "),
            name("insteadOf"),
            whitespace(" "),
            whitespace(" "),
            value("\"github://\""),
            newline(),

            gen_section_header("url", (" ", "ssh://git@git.eddie.sh/edward/")),
            newline(),

            whitespace("        "),
            name("insteadOf"),
            whitespace(" "),
            whitespace(" "),
            value("\"gitea://\""),
            newline(),

            gen_section_header("pull", None),
            newline(),

            whitespace("        "),
            name("ff"),
            whitespace(" "),
            whitespace(" "),
            value("only"),
            newline(),

            gen_section_header("init", None),
            newline(),

            whitespace("        "),
            name("defaultBranch"),
            whitespace(" "),
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
        vec![
            newline(),
            whitespace("   "),
            newline(),
            whitespace(" "),
            newline(),
        ]
    )
}

#[test]
fn newline_events_are_merged() {
    assert_eq!(
        parse_from_str("\n\n\n\n\n").unwrap().into_vec(),
        vec![Event::Newline("\n\n\n\n\n".into())]
    );
}
