mod undo {
    use bstr::{ByteSlice, B};

    #[test]
    fn no_id_changes_nothing() {
        let mut buf = Vec::new();
        let changed = gix_filter::ident::undo(B("hello"), &mut buf);
        assert!(!changed, "the buffer is not touched");
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn empty() {
        let mut buf = Vec::new();
        assert!(
            !gix_filter::ident::undo(B(""), &mut buf),
            "nothing to be done in empty buffer"
        );
    }

    #[test]
    fn nothing_if_newline_between_dollars() {
        let mut buf = Vec::new();
        assert!(!gix_filter::ident::undo(B(" $Id: \n$"), &mut buf));
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn nothing_if_it_is_not_id() {
        let mut buf = Vec::new();
        assert!(
            !gix_filter::ident::undo(B(" $id: something$"), &mut buf),
            "it's matching case-sensitively"
        );
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn anything_between_dollar_id_dollar() {
        let mut buf = Vec::new();
        assert!(gix_filter::ident::undo(B(" $Id: something$\nhello"), &mut buf));
        assert_eq!(buf.as_bstr(), " $Id$\nhello");
    }

    #[test]
    fn multiple() {
        let mut buf = Vec::new();
        assert!(gix_filter::ident::undo(
            B("$Id: a\n$ $Id: something$\nhello$Id: hex$\nlast $Id:other$\n$Id: \n$"),
            &mut buf
        ));
        assert_eq!(buf.as_bstr(), "$Id: a\n$ $Id$\nhello$Id$\nlast $Id$\n$Id: \n$");

        assert!(gix_filter::ident::undo(
            B("$Id: a\n$$Id:$$Id: hex$\n$Id:other$$Id: $end"),
            &mut buf
        ));
        assert_eq!(buf.as_bstr(), "$Id: a\n$$Id$$Id$\n$Id$$Id$end");
    }
}

mod apply {
    use bstr::{ByteSlice, B};
    use gix_filter::ident;

    #[test]
    fn no_change() {
        let mut buf = Vec::new();
        for input_no_match in [
            "",
            "nothing",
            "$ID$ case sensitive matching",
            "$Id: expanded is ignored$",
        ] {
            let changed = ident::apply(input_no_match.as_bytes(), gix_hash::Kind::Sha1, &mut buf);
            assert!(!changed, "no substitution happens, nothing to do");
            assert_eq!(buf.len(), 0);
        }
    }

    #[test]
    fn simple() {
        let mut buf = Vec::new();
        assert!(
            ident::apply(B("$Id$"), gix_hash::Kind::Sha1, &mut buf),
            "a change happens"
        );
        assert_eq!(buf.as_bstr(), "$Id: b3f5ebfb5843bc43ceecff6d4f26bb37c615beb1$");

        assert!(ident::apply(B("$Id$ $Id$ foo"), gix_hash::Kind::Sha1, &mut buf));
        assert_eq!(
            buf.as_bstr(),
            "$Id: e230cff7a9624f59eaa28bfb97602c3a03651a49$ $Id: e230cff7a9624f59eaa28bfb97602c3a03651a49$ foo"
        );
    }

    #[test]
    fn round_trips() {
        let mut buf = Vec::new();
        for input in [
            "hi\n$Id$\nho\n\t$Id$$Id$$Id$",
            "$Id$",
            "$Id$ and one more $Id$ and done",
        ] {
            let changed = ident::apply(B(input), gix_hash::Kind::Sha1, &mut buf);
            assert!(changed, "the input was rewritten");
            assert!(ident::undo(&buf.clone(), &mut buf), "undo does something as well");
            assert_eq!(buf.as_bstr(), input, "the filter can be undone perfectly");
        }
    }
}
