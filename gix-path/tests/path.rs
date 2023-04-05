pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod convert;
mod realpath;
mod home_dir {
    #[test]
    fn returns_existing_directory() {
        if let Some(home) = gix_path::env::home_dir() {
            assert!(
                home.is_dir(),
                "the home directory would typically exist, even though on unix we don't test for that."
            );
        }
    }
}

mod xdg_config_path {
    use std::ffi::OsStr;

    #[test]
    fn prefers_xdg_config_bases() {
        let actual = gix_path::env::xdg_config("test", &mut |n| {
            (n == OsStr::new("XDG_CONFIG_HOME")).then(|| "marker".into())
        })
        .expect("set");
        #[cfg(unix)]
        assert_eq!(actual.to_str(), Some("marker/git/test"));
        #[cfg(windows)]
        assert_eq!(actual.to_str(), Some("marker\\git\\test"));
    }

    #[test]
    fn falls_back_to_home() {
        let actual = gix_path::env::xdg_config("test", &mut |n| (n == OsStr::new("HOME")).then(|| "marker".into()))
            .expect("set");
        #[cfg(unix)]
        assert_eq!(actual.to_str(), Some("marker/.config/git/test"));
        #[cfg(windows)]
        assert_eq!(actual.to_str(), Some("marker\\.config\\git\\test"));
    }
}
mod util;
