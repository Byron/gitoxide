mod decompose {
    use std::borrow::Cow;

    #[test]
    fn precomposed_unicode_is_decomposed() {
        let precomposed = "ä";
        let actual = gix_utils::str::decompose(precomposed.into());
        assert!(matches!(actual, Cow::Owned(_)), "new data is produced");
        assert_eq!(actual, "a\u{308}");
    }

    #[test]
    fn already_decomposed_does_not_copy() {
        let decomposed = "a\u{308}";
        let actual = gix_utils::str::decompose(decomposed.into());
        assert!(
            matches!(actual, Cow::Borrowed(_)),
            "pass-through as nothing needs to be done"
        );
        assert_eq!(actual, decomposed);
    }
}

mod precompose {
    use std::borrow::Cow;

    #[test]
    fn decomposed_unicode_is_precomposed() {
        let decomposed = "a\u{308}";
        let actual = gix_utils::str::precompose(decomposed.into());
        assert!(matches!(actual, Cow::Owned(_)), "new data is produced");
        assert_eq!(actual.chars().collect::<Vec<_>>(), ['ä']);
    }

    #[test]
    fn already_precomposed_does_not_copy() {
        let actual = gix_utils::str::precompose("ä".into());
        assert!(
            matches!(actual, Cow::Borrowed(_)),
            "pass-through as nothing needs to be done"
        );
        assert_eq!(actual.chars().collect::<Vec<_>>(), ['ä']);
    }
}
