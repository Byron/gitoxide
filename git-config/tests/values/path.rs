mod interpolate {
    use git_config::path;
    use std::{
        borrow::Cow,
        path::{Path, PathBuf},
    };

    use git_config::path::interpolate::Error;

    use crate::{file::cow_str, value::b};

    #[test]
    fn backslash_is_not_special_and_they_are_not_escaping_anything() -> crate::Result {
        for path in ["C:\\foo\\bar", "/foo/bar"] {
            let actual = git_config::Path::from(Cow::Borrowed(b(path))).interpolate(Default::default())?;
            assert_eq!(actual, Path::new(path));
            assert!(
                matches!(actual, Cow::Borrowed(_)),
                "it does not unnecessarily copy values"
            );
        }
        Ok(())
    }

    #[test]
    fn empty_path_is_error() {
        assert!(matches!(
            interpolate_without_context(""),
            Err(Error::Missing { what: "path" })
        ));
    }

    #[test]
    fn prefix_substitutes_git_install_dir() {
        for git_install_dir in &["/tmp/git", "C:\\git"] {
            for (val, expected) in &[("%(prefix)/foo/bar", "foo/bar"), ("%(prefix)/foo\\bar", "foo\\bar")] {
                let expected =
                    std::path::PathBuf::from(format!("{}{}{}", git_install_dir, std::path::MAIN_SEPARATOR, expected));
                assert_eq!(
                    git_config::Path::from(cow_str(val))
                        .interpolate(path::interpolate::Options {
                            git_install_dir: Path::new(git_install_dir).into(),
                            ..Default::default()
                        })
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
            git_config::Path::from(Cow::Borrowed(b(path)))
                .interpolate(path::interpolate::Options {
                    git_install_dir: Path::new(git_install_dir).into(),
                    ..Default::default()
                })
                .unwrap(),
            Path::new(path)
        );
    }

    #[test]
    fn tilde_alone_does_not_interpolate() -> crate::Result {
        assert_eq!(interpolate_without_context("~")?, Path::new("~"));
        Ok(())
    }

    #[test]
    fn tilde_slash_substitutes_current_user() -> crate::Result {
        let path = "~/user/bar";
        let home = std::env::current_dir()?;
        let expected = home.join("user").join("bar");
        assert_eq!(
            git_config::Path::from(cow_str(path))
                .interpolate(path::interpolate::Options {
                    home_dir: Some(&home),
                    home_for_user: Some(home_for_user),
                    ..Default::default()
                })
                .unwrap()
                .as_ref(),
            expected
        );
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn tilde_with_given_user_is_unsupported_on_windows() {
        assert!(matches!(
            interpolate_without_context("~baz/foo/bar"),
            Err(Error::UserInterpolationUnsupported)
        ));
    }

    #[cfg(not(windows))]
    #[test]
    fn tilde_with_given_user() -> crate::Result {
        let home = std::env::current_dir()?;

        for path_suffix in &["foo/bar", "foo\\bar", ""] {
            let path = format!("~user{}{}", std::path::MAIN_SEPARATOR, path_suffix);
            let expected = home.join("user").join(path_suffix);

            assert_eq!(interpolate_without_context(path)?, expected);
        }
        Ok(())
    }

    fn interpolate_without_context(
        path: impl AsRef<str>,
    ) -> Result<Cow<'static, Path>, git_config::path::interpolate::Error> {
        git_config::Path::from(Cow::Owned(path.as_ref().to_owned().into())).interpolate(path::interpolate::Options {
            home_for_user: Some(home_for_user),
            ..Default::default()
        })
    }

    fn home_for_user(name: &str) -> Option<PathBuf> {
        std::env::current_dir().unwrap().join(name).into()
    }
}
