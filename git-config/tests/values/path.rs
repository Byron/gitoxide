mod interpolate {
    use std::borrow::Cow;
    use std::path::Path;

    use git_config::values::Path as InterpolatingPath;

    use crate::file::cow_str;
    use crate::values::b;
    use git_config::values::path::interpolate::Error;

    #[test]
    fn backslash_is_not_special_and_they_are_not_escaping_anything() -> crate::Result {
        for path in ["C:\\foo\\bar", "/foo/bar"] {
            let actual = InterpolatingPath::from(Cow::Borrowed(b(path))).interpolate(None, None)?;
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
                    InterpolatingPath::from(cow_str(val))
                        .interpolate(Path::new(git_install_dir).into(), None)
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
            InterpolatingPath::from(Cow::Borrowed(b(path)))
                .interpolate(Path::new(git_install_dir).into(), None)
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
    fn tilde_slash_substitutes_current_user() {
        let path = "~/foo/bar";
        let home = std::env::current_dir().unwrap();
        let expected = format!("{}{}foo/bar", home.display(), std::path::MAIN_SEPARATOR);
        assert_eq!(
            InterpolatingPath::from(cow_str(path))
                .interpolate(None, Some(&home))
                .unwrap()
                .as_ref(),
            Path::new(&expected),
            "note that path separators are not turned into slashes as we work with `std::path::Path`"
        );
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
        let user = std::env::var("USER")?;
        let home = std::env::var("HOME")?;
        let specific_user_home = format!("~{}", user);

        for path_suffix in &["foo/bar", "foo\\bar", ""] {
            let path = format!("{}{}{}", specific_user_home, std::path::MAIN_SEPARATOR, path_suffix);
            let expected = format!("{}{}{}", home, std::path::MAIN_SEPARATOR, path_suffix);
            assert_eq!(
                interpolate_without_context(path)?,
                Path::new(&expected),
                "it keeps path separators as is"
            );
        }
        Ok(())
    }

    fn interpolate_without_context(
        path: impl AsRef<str>,
    ) -> Result<Cow<'static, Path>, git_config::values::path::interpolate::Error> {
        InterpolatingPath::from(Cow::Owned(path.as_ref().to_owned().into())).interpolate(None, None)
    }
}
