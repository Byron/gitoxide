mod parse {
    use git_object::bstr::{BStr, BString};
    use git_revision::spec;
    #[derive(Default, Debug)]
    struct Recorder {
        resolve_ref_input: Option<BString>,
    }
    impl spec::parse::Delegate for Recorder {
        fn resolve_ref(&mut self, input: &BStr) -> Option<()> {
            assert!(
                self.resolve_ref_input.is_none(),
                "called resolve_ref twice with '{}'",
                input
            );
            self.resolve_ref_input = input.to_owned().into();
            Some(())
        }

        fn find_by_prefix(&mut self, _input: &BStr) -> Option<()> {
            todo!()
        }

        fn nth_ancestor(&mut self, _n: usize) -> Option<()> {
            todo!()
        }

        fn nth_parent(&mut self, _n: usize) -> Option<()> {
            todo!()
        }
    }

    fn b(s: &str) -> &BStr {
        s.into()
    }

    #[test]
    fn at_alone_is_shortcut_for_head() {
        let mut rec = Recorder::default();
        spec::parse(b("@"), &mut rec).unwrap();
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn refname() {
        let mut rec = Recorder::default();
        spec::parse(b("HEAD"), &mut rec).unwrap();
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }
}
