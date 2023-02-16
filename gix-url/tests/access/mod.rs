mod canonicalized {
    #[test]
    fn non_file_scheme_is_noop() -> crate::Result {
        let url = gix_url::parse("https://github.com/byron/gitoxide".into())?;
        assert_eq!(url.canonicalized()?, url);
        Ok(())
    }

    #[test]
    fn absolute_file_url_does_nothing() -> crate::Result {
        #[cfg(not(windows))]
        let url = gix_url::parse("/this/path/does/not/exist".into())?;
        #[cfg(windows)]
        let url = gix_url::parse("C:\\non\\existing".into())?;
        assert_eq!(url.canonicalized()?, url);
        Ok(())
    }

    #[test]
    fn file_that_is_current_dir_is_absolutized() -> crate::Result {
        let url = gix_url::parse(".".into())?;
        assert!(gix_path::from_bstr(url.path.as_ref()).is_relative());
        assert!(gix_path::from_bstr(url.canonicalized()?.path.as_ref()).is_absolute());
        Ok(())
    }
}
