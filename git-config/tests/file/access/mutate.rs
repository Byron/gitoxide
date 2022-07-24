mod rename_section {
    use std::{borrow::Cow, convert::TryFrom};

    use git_config::{file::rename_section, parse::section};

    #[test]
    fn section_renaming_validates_new_name() {
        let mut file = git_config::File::try_from("[core] a = b").unwrap();
        assert!(matches!(
            file.rename_section("core", None, "new_core", None),
            Err(rename_section::Error::Section(section::header::Error::InvalidName))
        ));

        assert!(matches!(
            file.rename_section("core", None, "new-core", Cow::from("a\nb")),
            Err(rename_section::Error::Section(
                section::header::Error::InvalidSubSection
            ))
        ));
    }
}
