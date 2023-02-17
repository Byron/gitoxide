use gix_credentials::{program::Kind, Program};

#[test]
fn script() {
    assert!(
        matches!(Program::from_custom_definition("!exe").kind, Kind::ExternalShellScript(script) if script == "exe")
    );
}

#[test]
fn name_with_args() {
    let input = "name --arg --bar=\"a b\"";
    let expected = "git credential-name --arg --bar=\"a b\"";
    assert!(
        matches!(Program::from_custom_definition(input).kind, Kind::ExternalName{name_and_args} if name_and_args == expected)
    );
}

#[test]
fn name() {
    let input = "name";
    let expected = "git credential-name";
    assert!(
        matches!(Program::from_custom_definition(input).kind, Kind::ExternalName{name_and_args} if name_and_args == expected)
    );
}

#[test]
fn path_with_args() {
    let input = "/abs/name --arg --bar=\"a b\"";
    assert!(
        matches!(Program::from_custom_definition(input).kind, Kind::ExternalPath{path_and_args} if path_and_args == input)
    );
}

#[test]
fn path() {
    let input = "/abs/name";
    assert!(
        matches!(Program::from_custom_definition(input).kind, Kind::ExternalPath{path_and_args} if path_and_args == input)
    );
}
