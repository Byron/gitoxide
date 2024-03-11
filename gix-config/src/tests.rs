mod impls {
    use bstr::ByteSlice;

    use crate::Key;

    #[test]
    fn key_impl_for_str() {
        assert_eq!("", "".section_name());
        assert_eq!(None, "".subsection_name());
        assert_eq!("", "".name());

        assert_eq!("", "foo".section_name());
        assert_eq!(None, "foo".subsection_name());
        assert_eq!("foo", "foo".name());

        assert_eq!("foo", "foo.bar".section_name());
        assert_eq!(None, "foo.bar".subsection_name());
        assert_eq!("bar", "foo.bar".name());

        assert_eq!("foo", "foo.bar.baz".section_name());
        assert_eq!(Some("bar".into()), "foo.bar.baz".subsection_name());
        assert_eq!("baz", "foo.bar.baz".name());

        assert_eq!("foo", "foo.quux.bar.baz".section_name());
        assert_eq!(Some("quux.bar".into()), "foo.quux.bar.baz".subsection_name());
        assert_eq!("baz", "foo.quux.bar.baz".name());

        assert_eq!(
            Some("gitdir/i:C:\\bare.git".into()),
            "includeIf.gitdir/i:C:\\bare.git.path".subsection_name()
        )
    }

    #[test]
    fn key_impl_for_bstr() {
        assert_eq!("", "".as_bytes().as_bstr().section_name());
        assert_eq!(None, "".as_bytes().as_bstr().subsection_name());
        assert_eq!("", "".as_bytes().as_bstr().name());

        assert_eq!("", "foo".as_bytes().as_bstr().section_name());
        assert_eq!(None, "foo".as_bytes().as_bstr().subsection_name());
        assert_eq!("foo", "foo".as_bytes().as_bstr().name());

        assert_eq!("foo", "foo.bar".as_bytes().as_bstr().section_name());
        assert_eq!(None, "foo.bar".as_bytes().as_bstr().subsection_name());
        assert_eq!("bar", "foo.bar".as_bytes().as_bstr().name());

        assert_eq!("foo", "foo.bar.baz".as_bytes().as_bstr().section_name());
        assert_eq!(Some("bar".into()), "foo.bar.baz".as_bytes().as_bstr().subsection_name());
        assert_eq!("baz", "foo.bar.baz".as_bytes().as_bstr().name());

        assert_eq!("foo", "foo.quux.bar.baz".as_bytes().as_bstr().section_name());
        assert_eq!(
            Some("quux.bar".into()),
            "foo.quux.bar.baz".as_bytes().as_bstr().subsection_name()
        );
        assert_eq!("baz", "foo.quux.bar.baz".as_bytes().as_bstr().name());

        assert_eq!(
            Some("gitdir/i:C:\\bare.git".into()),
            "includeIf.gitdir/i:C:\\bare.git.path"
                .as_bytes()
                .as_bstr()
                .subsection_name()
        )
    }
}
