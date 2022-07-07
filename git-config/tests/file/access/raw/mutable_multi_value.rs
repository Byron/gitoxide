use std::{borrow::Cow, convert::TryFrom};

use git_config::File;
use git_testtools::fixture_path;

fn init_config() -> File<'static> {
    let mut buf = Vec::new();
    File::from_path_with_buf(&fixture_path("multi-core.txt"), &mut buf).unwrap()
}

#[test]
fn value_is_correct() {
    let mut git_config = init_config();

    let value = git_config.raw_multi_value_mut("core", None, "a").unwrap();
    assert_eq!(
        &*value.get().unwrap(),
        vec![
            Cow::<[u8]>::Owned(b"b100".to_vec()),
            Cow::<[u8]>::Borrowed(b"d"),
            Cow::<[u8]>::Borrowed(b"f"),
        ]
    );
}

#[test]
fn non_empty_sizes_are_correct() {
    let mut git_config = init_config();
    assert_eq!(git_config.raw_multi_value_mut("core", None, "a").unwrap().len(), 3);
    assert!(!git_config.raw_multi_value_mut("core", None, "a").unwrap().is_empty());
}

#[test]
fn set_value_at_start() {
    let mut git_config = init_config();
    let mut values = git_config.raw_multi_value_mut("core", None, "a").unwrap();
    values.set_string(0, "Hello".to_string());
    assert_eq!(
        git_config.to_string(),
        "[core]\n    a=Hello\n    [core]\n        a=d\n        a=f"
    );
}

#[test]
fn set_value_at_end() {
    let mut git_config = init_config();
    let mut values = git_config.raw_multi_value_mut("core", None, "a").unwrap();
    values.set_string(2, "Hello".to_string());
    assert_eq!(
        git_config.to_string(),
        "[core]\n    a=b\"100\"\n    [core]\n        a=d\n        a=Hello"
    );
}

#[test]
fn set_values_all() {
    let mut git_config = init_config();
    let mut values = git_config.raw_multi_value_mut("core", None, "a").unwrap();
    values.set_owned_values_all("Hello");
    assert_eq!(
        git_config.to_string(),
        "[core]\n    a=Hello\n    [core]\n        a=Hello\n        a=Hello"
    );
}

#[test]
fn delete() {
    let mut git_config = init_config();
    let mut values = git_config.raw_multi_value_mut("core", None, "a").unwrap();
    values.delete(0);
    assert_eq!(
        git_config.to_string(),
        "[core]\n    \n    [core]\n        a=d\n        a=f",
    );
}

#[test]
fn delete_all() {
    let mut git_config = init_config();
    let mut values = git_config.raw_multi_value_mut("core", None, "a").unwrap();
    values.delete_all();
    assert!(values.get().is_err());
    assert_eq!(git_config.to_string(), "[core]\n    \n    [core]\n        \n        ",);
}

#[test]
fn partial_values_are_supported() {
    let mut git_config = File::try_from(
        r#"[core]
            a=b\
"100"
        [core]
            a=d\
b
            a=f\
a"#,
    )
    .unwrap();
    let mut values = git_config.raw_multi_value_mut("core", None, "a").unwrap();

    assert_eq!(
        &*values.get().unwrap(),
        vec![
            Cow::<[u8]>::Owned(b"b100".to_vec()),
            Cow::<[u8]>::Borrowed(b"db"),
            Cow::<[u8]>::Borrowed(b"fa"),
        ]
    );

    values.delete_all();
    assert!(values.get().is_err());
}
