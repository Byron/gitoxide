mod remove_section {
    use std::convert::TryFrom;

    #[test]
    fn removal_of_all_sections_programmatically_with_sections_and_ids_by_name() {
        let mut file = git_config::File::try_from("[core] \na = b\nb=c\n\n[core \"name\"]\nd = 1\ne = 2").unwrap();
        for id in file
            .sections_and_ids_by_name("core")
            .expect("2 sections present")
            .map(|(_, id)| id)
            .collect::<Vec<_>>()
        {
            file.remove_section_by_id(id);
        }
        assert!(file.is_void());
        assert_eq!(file.sections().count(), 0);
    }

    #[test]
    fn removal_of_all_sections_programmatically_with_sections_and_ids() {
        let mut file = git_config::File::try_from("[core] \na = b\nb=c\n\n[core \"name\"]\nd = 1\ne = 2").unwrap();
        for id in file.sections_and_ids().map(|(_, id)| id).collect::<Vec<_>>() {
            file.remove_section_by_id(id);
        }
        assert!(file.is_void());
        assert_eq!(file.sections().count(), 0);
    }

    #[test]
    fn removal_is_complete_and_sections_can_be_readded() {
        let mut file = git_config::File::try_from("[core] \na = b\nb=c\n\n[core \"name\"]\nd = 1\ne = 2").unwrap();
        assert_eq!(file.sections().count(), 2);

        let removed = file.remove_section("core", None).expect("removed correct section");
        assert_eq!(removed.header().name(), "core");
        assert_eq!(removed.header().subsection_name(), None);
        assert_eq!(file.sections().count(), 1);

        let removed = file.remove_section("core", Some("name".into())).expect("found");
        assert_eq!(removed.header().name(), "core");
        assert_eq!(removed.header().subsection_name().expect("present"), "name");
        assert_eq!(file.sections().count(), 0);

        file.section_mut_or_create_new("core", None).expect("creation succeeds");
        file.section_mut_or_create_new("core", Some("name".into()))
            .expect("creation succeeds");
    }
}
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
            file.rename_section("core", None, "new-core", Some(Cow::Borrowed("a\nb".into()))),
            Err(rename_section::Error::Section(
                section::header::Error::InvalidSubSection
            ))
        ));
    }
}
