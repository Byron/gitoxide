use serde_git_config::parser::{parse_from_str, Event, ParsedSectionHeader};
use serde_git_config::values::Value;

fn fully_consumed<T>(t: T) -> (&'static str, T) {
    ("", t)
}

fn gen_section_header(
    name: &str,
    subsection: impl Into<Option<(&'static str, &'static str)>>,
) -> Event<'_> {
    Event::SectionHeader(
        if let Some((separator, subsection_name)) = subsection.into() {
            ParsedSectionHeader {
                name,
                separator: Some(separator),
                subsection_name: Some(subsection_name),
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
    Event::Key(name)
}

fn value(value: &'static str) -> Event<'static> {
    Event::Value(Value::from_str(value))
}

fn newline() -> Event<'static> {
    Event::Newline("\n")
}

fn whitespace(value: &'static str) -> Event<'static> {
    Event::Whitespace(value)
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
            .into_iter()
            .collect::<Vec<_>>(),
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
