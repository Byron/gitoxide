mod list {
    use std::path::Path;

    use gix_glob::{
        pattern::Case,
        search::{
            pattern::{List, Mapping},
            Pattern,
        },
    };

    #[derive(Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Default)]
    struct Dummy;

    impl Pattern for Dummy {
        type Value = ();

        fn bytes_to_patterns(_bytes: &[u8], _source: &Path) -> Vec<Mapping<Self::Value>> {
            vec![]
        }

        fn may_use_glob_pattern(_pattern: &gix_glob::Pattern) -> bool {
            unreachable!("won't be called")
        }
    }

    #[test]
    fn strip_base_handle_recompute_basename_pos() {
        let list = List::<Dummy>::from_bytes(&[], "a/b/source", Some(Path::new("")));
        let res = list.strip_base_handle_recompute_basename_pos("a/b/file".into(), Some(4), Case::Sensitive);
        assert_eq!(
            res,
            Some(("file".into(), None)),
            "files don't have a basename position anymore"
        );

        let res = list.strip_base_handle_recompute_basename_pos("a/B/c/File".into(), Some(6), Case::Fold);
        assert_eq!(
            res,
            Some(("c/File".into(), Some(2))),
            "otherwise the basename is recomputed, case folding is effective"
        );
    }
}
