use std::{borrow::Cow, path::Path};

use git_path::absolutize;

fn p(input: &str) -> &Path {
    Path::new(input)
}

#[test]
fn no_change_if_there_are_no_trailing_relative_components() {
    for input in ["./a/b/c/d", "/absolute/path", "C:\\hello\\world"] {
        let path = p(input);
        assert_eq!(absolutize(path, std::env::current_dir().unwrap()).unwrap(), path);
    }
}

#[test]
fn special_cases_around_cwd() -> crate::Result {
    let cwd = std::env::current_dir()?;
    assert_eq!(
        absolutize(p("./../../.git/modules/src/llvm-project"), &cwd).unwrap(),
        cwd.parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(".git/modules/src/llvm-project"),
        "'.' is handled specifically to not fail to swap in the CWD"
    );
    assert_eq!(
        absolutize(&cwd, &cwd).unwrap(),
        cwd,
        "absolute inputs yield absolute outputs"
    );
    assert_eq!(
        absolutize(p("a/../.."), &cwd).unwrap(),
        cwd.parent().expect("parent"),
        "it automatically extends the pop-able items by using the current working dir"
    );
    assert_eq!(
        absolutize(p("a/.."), &cwd).unwrap(),
        p("."),
        "absolute CWDs are always shortened…"
    );
    assert_eq!(absolutize(p("./a/.."), &cwd).unwrap(), p("."), "…like this as well…");
    Ok(())
}

#[test]
fn parent_dirs_cause_the_cwd_to_be_used() {
    assert_eq!(
        absolutize(p("./a/b/../../.."), "/users/name").unwrap().as_ref(),
        p("/users")
    );
}

#[test]
fn walking_up_too_much_yield_none() {
    let cwd = "/users/name";
    assert_eq!(absolutize(p("./a/b/../../../../../."), cwd), None);
    assert_eq!(absolutize(p("./a/../../../.."), cwd), None);
}

#[test]
fn trailing_directories_after_too_numereous_parent_dirs_yield_none() {
    assert_eq!(
        absolutize(p("./a/b/../../../../../actually-invalid"), "/users").as_ref(),
        None,
    );
    assert_eq!(absolutize(p("/a/b/../../.."), "/does-not/matter").as_ref(), None,);
}

#[test]
fn trailing_relative_components_are_resolved() {
    let cwd = std::env::current_dir().unwrap();
    for (input, expected) in [
        ("./a/b/./c/../d/..", "./a/b"),
        ("/a/b/c/.././../.", "/a"),
        ("./a/..", "."),
        ("a/..", "."),
        ("./a", "./a"),
        ("./a/./b/..", "./a/."),
        ("/a/./b/c/.././../.", "/a"),
        ("/a/./b", "/a/./b"),
        ("/a/././c/.././../.", "/"),
        ("/a/b/../c/../..", "/"),
        ("C:/hello/../a", "C:/a"),
        ("./a/../b/..", "./"),
        ("/a/../b", "/b"),
    ] {
        let path = p(input);
        assert_eq!(
            absolutize(path, &cwd).unwrap_or_else(|| panic!("{path:?}")),
            Cow::Borrowed(p(expected)),
            "'{}' got an unexpected result",
            input
        );
    }
}
