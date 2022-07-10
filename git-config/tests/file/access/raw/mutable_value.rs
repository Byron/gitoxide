use std::convert::TryFrom;

use git_config::File;

fn init_config() -> File<'static> {
    File::try_from(
        r#"[core]
            a=b"100"
        [core]
            c=d
            e=f"#,
    )
    .unwrap()
}

#[test]
fn value_is_correct() {
    let mut git_config = init_config();

    let value = git_config.raw_value_mut("core", None, "a").unwrap();
    assert_eq!(&*value.get().unwrap(), "b100");
}

#[test]
fn set_string_cleanly_updates() {
    let mut git_config = init_config();

    let mut value = git_config.raw_value_mut("core", None, "a").unwrap();
    value.set_string("hello world".to_string());
    assert_eq!(
        git_config.to_string(),
        r#"[core]
            a=hello world
        [core]
            c=d
            e=f"#,
    );

    let mut value = git_config.raw_value_mut("core", None, "e").unwrap();
    value.set_string(String::new());
    assert_eq!(
        git_config.to_string(),
        r#"[core]
            a=hello world
        [core]
            c=d
            e="#,
    );
}

#[test]
fn delete_value() {
    let mut git_config = init_config();

    let mut value = git_config.raw_value_mut("core", None, "a").unwrap();
    value.delete();
    assert_eq!(
        git_config.to_string(),
        "[core]\n            \n        [core]\n            c=d\n            e=f",
    );

    let mut value = git_config.raw_value_mut("core", None, "c").unwrap();
    value.delete();
    assert_eq!(
        git_config.to_string(),
        "[core]\n            \n        [core]\n            \n            e=f",
    );
}

#[test]
fn get_value_after_deleted() {
    let mut git_config = init_config();

    let mut value = git_config.raw_value_mut("core", None, "a").unwrap();
    value.delete();
    assert!(value.get().is_err());
}

#[test]
fn set_string_after_deleted() {
    let mut git_config = init_config();

    let mut value = git_config.raw_value_mut("core", None, "a").unwrap();
    value.delete();
    value.set_string("hello world".to_string());
    assert_eq!(
        git_config.to_string(),
        r#"[core]
            a=hello world
        [core]
            c=d
            e=f"#,
    );
}

#[test]
fn subsequent_delete_calls_are_noop() {
    let mut git_config = init_config();

    let mut value = git_config.raw_value_mut("core", None, "a").unwrap();
    for _ in 0..10 {
        value.delete();
    }
    assert_eq!(
        git_config.to_string(),
        "[core]\n            \n        [core]\n            c=d\n            e=f"
    );
}

#[test]
fn partial_values_are_supported() {
    let mut git_config = File::try_from(
        r#"[core]
            a=b"100"\
c\
b
        [core]
            c=d
            e=f"#,
    )
    .unwrap();
    let mut value = git_config.raw_value_mut("core", None, "a").unwrap();
    assert_eq!(&*value.get().unwrap(), "b100cb");
    value.delete();
    assert_eq!(
        git_config.to_string(),
        "[core]\n            \n        [core]\n            c=d\n            e=f"
    );
}
