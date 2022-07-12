mod mutable_section {}

mod rename_section {
    use git_config::file::rename_section;
    use git_config::parse::section;
    use std::borrow::Cow;
    use std::convert::TryFrom;

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
