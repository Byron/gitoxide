use serde_git_config::parser::{parse_from_str, Event, ParsedSectionHeader, Parser};
use serde_git_config::values::Value;

fn fully_consumed<T>(t: T) -> (&'static str, T) {
    ("", t)
}

fn section_header(name: &'static str, subname: impl Into<Option<&'static str>>) -> Event<'static> {
    Event::SectionHeader(ParsedSectionHeader {
        name,
        subsection_name: subname.into(),
    })
}

fn name(name: &'static str) -> Event<'static> {
    Event::Key(name)
}

fn value(value: &'static str) -> Event<'static> {
    Event::Value(Value::from_str(value))
}

#[test]
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
            section_header("user", None),
            name("email"),
            value("code@eddie.sh"),
            name("name"),
            value("Edward Shen"),
            section_header("core", None),
            name("autocrlf"),
            value("input"),
            section_header("push", None),
            name("default"),
            value("simple"),
            section_header("commit", None),
            name("gpgsign"),
            value("true"),
            section_header("gpg", None),
            name("program"),
            value("gpg"),
            section_header("url", "ssh://git@github.com/"),
            name("insteadOf"),
            value("github://"),
            section_header("url", "ssh://git@git.eddie.sh/edward/"),
            name("insteadOf"),
            value("gitea://"),
            section_header("pull", None),
            name("ff"),
            value("only"),
            section_header("init", None),
            name("defaultBranch"),
            value("master"),
        ]
    );
}
