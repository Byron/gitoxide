mod parse {
    use git_object::bstr::{BStr, BString};
    use git_revision::spec;

    #[derive(Default, Debug)]
    struct Recorder {
        resolve_ref_input: Option<BString>,
        kind: Option<spec::Kind>,
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

        fn kind(&mut self, kind: spec::Kind) {
            self.kind = Some(kind);
        }
    }

    fn parse(spec: &str) -> Recorder {
        let mut rec = Recorder::default();
        spec::parse(spec.into(), &mut rec).unwrap();
        rec
    }

    #[test]
    #[ignore]
    fn leading_caret_is_range_kind() {
        let rec = parse("^HEAD");
        assert_eq!(rec.kind.unwrap(), spec::Kind::Range);
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn at_by_iteself_is_shortcut_for_head() {
        let rec = parse("@");
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }

    #[test]
    fn refname_head() {
        let rec = parse("HEAD");
        assert!(rec.kind.is_none());
        assert_eq!(rec.resolve_ref_input.unwrap(), "HEAD");
    }
}
