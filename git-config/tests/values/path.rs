use crate::values::b;
use git_config::values::path::interpolate;
use git_config::values::Path;
use std::borrow::Cow;

#[test]
fn no_interpolation_for_paths_without_tilde_or_prefix() {
    let path = &b"/foo/bar"[..];
    let actual = Path::from(Cow::Borrowed(path));
    assert_eq!(&*actual, path);
    assert!(
        matches!(&actual.value, Cow::Borrowed(_)),
        "it does not unnecessarily copy values"
    );
}

#[test]
fn empty_path_is_error() {
    assert!(matches!(
        Path::from(Cow::Borrowed(b(""))).interpolate(None),
        Err(interpolate::Error::Missing { what: "path" })
    ));
}

#[test]
fn prefix_substitutes_git_install_dir() {
    for git_install_dir in &["/tmp/git", "C:\\git"] {
        for (val, expected) in &[
            (&b"%(prefix)/foo/bar"[..], "foo/bar"),
            (b"%(prefix)/foo\\bar", "foo\\bar"),
        ] {
            let expected =
                &std::path::PathBuf::from(format!("{}{}{}", git_install_dir, std::path::MAIN_SEPARATOR, expected));
            assert_eq!(
                &*Path::from(Cow::Borrowed(*val))
                    .interpolate(Some(std::path::Path::new(git_install_dir)))
                    .unwrap(),
                expected,
                "prefix interpolation keeps separators as they are"
            );
        }
    }
}

#[test]
fn prefix_substitution_skipped_with_dot_slash() {
    let path = "./%(prefix)/foo/bar";
    let git_install_dir = "/tmp/git";
    assert_eq!(
        Path::from(Cow::Borrowed(b(path)))
            .interpolate(Some(std::path::Path::new(git_install_dir)))
            .unwrap(),
        std::path::Path::new(path)
    );
}

#[test]
fn tilde_substitutes_current_user() {
    let path = &b"~/foo/bar"[..];
    let expected = format!(
        "{}{}foo/bar",
        dirs::home_dir().expect("empty home").display(),
        std::path::MAIN_SEPARATOR
    );
    assert_eq!(
        Path::from(Cow::Borrowed(path)).interpolate(None).unwrap().as_ref(),
        std::path::Path::new(&expected),
        "note that path separators are not turned into slashes as we work with `std::path::Path`"
    );
}

#[cfg(target_os = "windows")]
#[test]
fn tilde_with_given_user_is_unsupported_on_windows() {
    assert!(matches!(
        Path::from(Cow::Borrowed(&b"~baz/foo/bar"[..])).interpolate(None),
        Err(interpolate::Error::UserInterpolationUnsupported)
    ));
}

#[cfg(not(target_os = "windows"))]
#[test]
fn tilde_with_given_user() {
    let user = std::env::var("USER").unwrap();
    let home = std::env::var("HOME").unwrap();
    let specific_user_home = format!("~{}", user);

    for path_suffix in &["foo/bar", "foo\\bar", ""] {
        let path = format!("{}{}{}", specific_user_home, std::path::MAIN_SEPARATOR, path_suffix);
        let expected = format!("{}{}{}", home, std::path::MAIN_SEPARATOR, path_suffix);
        assert_eq!(
            Path::from(Cow::Borrowed(b(&path))).interpolate(None).unwrap(),
            std::path::Path::new(&expected),
            "it keeps path separators as is"
        );
    }
}
