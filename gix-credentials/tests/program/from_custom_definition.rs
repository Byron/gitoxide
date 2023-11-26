use gix_credentials::{helper, program::Kind, Program};

#[cfg(windows)]
const GIT: &str = "git.exe";
#[cfg(not(windows))]
const GIT: &str = "git";

#[cfg(windows)]
const SH: &str = "sh";
#[cfg(not(windows))]
const SH: &str = "/bin/sh";

#[test]
fn empty() {
    let prog = Program::from_custom_definition("");
    assert!(matches!(&prog.kind, Kind::ExternalName { name_and_args } if name_and_args == ""));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        format!(r#""{GIT}" "credential-" "store""#),
        "not useful, but allowed, would have to be caught elsewhere"
    );
}

#[test]
fn simple_script_in_path() {
    let prog = Program::from_custom_definition("!exe");
    assert!(matches!(&prog.kind, Kind::ExternalShellScript(script) if script == "exe"));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        r#""exe" "store""#,
        "it didn't detect anything shell-scripty, and thus doesn't use a shell"
    );
}

#[test]
fn name_with_args() {
    let input = "name --arg --bar=\"a b\"";
    let prog = Program::from_custom_definition(input);
    assert!(matches!(&prog.kind, Kind::ExternalName{name_and_args} if name_and_args == input));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        format!(r#""{GIT}" "credential-name" "--arg" "--bar=a b" "store""#)
    );
}

#[test]
fn name_with_special_args() {
    let input = "name --arg --bar=~/folder/in/home";
    let prog = Program::from_custom_definition(input);
    assert!(matches!(&prog.kind, Kind::ExternalName{name_and_args} if name_and_args == input));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        format!(r#""{SH}" "-c" "{GIT} credential-name --arg --bar=~/folder/in/home \"$@\"" "--" "store""#)
    );
}

#[test]
fn name() {
    let input = "name";
    let prog = Program::from_custom_definition(input);
    assert!(matches!(&prog.kind, Kind::ExternalName{name_and_args} if name_and_args == input));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        format!(r#""{GIT}" "credential-name" "store""#),
        "we detect that this can run without shell, which is also more portable on windows"
    );
}

#[test]
fn path_with_args_that_definitely_need_shell() {
    let input = "/abs/name --arg --bar=\"a b\"";
    let prog = Program::from_custom_definition(input);
    assert!(matches!(&prog.kind, Kind::ExternalPath{path_and_args} if path_and_args == input));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        if cfg!(windows) {
            r#""/abs/name" "--arg" "--bar=a b" "store""#.to_owned()
        } else {
            format!(r#""{SH}" "-c" "/abs/name --arg --bar=\"a b\" \"$@\"" "--" "store""#)
        }
    );
}

#[test]
fn path_without_args() {
    let input = "/abs/name";
    let prog = Program::from_custom_definition(input);
    assert!(matches!(&prog.kind, Kind::ExternalPath{path_and_args} if path_and_args == input));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        r#""/abs/name" "store""#,
        "no shell is used"
    );
}

#[test]
fn path_with_simple_args() {
    let input = "/abs/name a b";
    let prog = Program::from_custom_definition(input);
    assert!(matches!(&prog.kind, Kind::ExternalPath{path_and_args} if path_and_args == input));
    assert_eq!(
        format!("{:?}", prog.to_command(&helper::Action::Store("egal".into()))),
        if cfg!(windows) {
            r#""/abs/name" "a" "b" "store""#.to_owned()
        } else {
            format!(r#""{SH}" "-c" "/abs/name a b \"$@\"" "--" "store""#)
        },
        "a shell is used as there are arguments, and it's generally more flexible, but on windows we split ourselves"
    );
}
