use bstr::BStr;
use std::borrow::Cow;

fn cowstr(input: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(input.into())
}
mod undo {
    use crate::ident::cowstr;
    use std::borrow::Cow;

    #[test]
    fn no_id_changes_nothing() {
        let cow = gix_filter::ident::undo(cowstr("hello"));
        assert!(matches!(cow, Cow::Borrowed(_)), "the buffer is not touched");
        assert_eq!(cow.as_ref(), "hello");
    }

    #[test]
    fn empty() {
        assert!(matches!(gix_filter::ident::undo(cowstr("")), Cow::Borrowed(_)));
    }

    #[test]
    fn nothing_if_newline_between_dollars() {
        assert!(matches!(gix_filter::ident::undo(cowstr(" $Id: \n$")), Cow::Borrowed(_)));
    }

    #[test]
    fn nothing_if_it_is_not_id() {
        assert!(
            matches!(gix_filter::ident::undo(cowstr(" $id: something$")), Cow::Borrowed(_)),
            "it's matching case-sensitively"
        );
    }

    #[test]
    fn anything_between_dollar_id_dollar() {
        assert_eq!(
            gix_filter::ident::undo(cowstr(" $Id: something$\nhello")).as_ref(),
            " $Id$\nhello"
        );
    }

    #[test]
    fn multiple() {
        assert_eq!(
            gix_filter::ident::undo(cowstr(
                "$Id: a\n$ $Id: something$\nhello$Id: hex$\nlast $Id:other$\n$Id: \n$"
            ))
            .as_ref(),
            "$Id: a\n$ $Id$\nhello$Id$\nlast $Id$\n$Id: \n$",
        );
        assert_eq!(
            gix_filter::ident::undo(cowstr("$Id: a\n$$Id:$$Id: hex$\n$Id:other$$Id: $end")).as_ref(),
            "$Id: a\n$$Id$$Id$\n$Id$$Id$end",
        );
    }
}

mod apply {
    use crate::ident::cowstr;
    use gix_filter::ident;
    use std::borrow::Cow;

    #[test]
    fn no_change() {
        for input_no_match in [
            "",
            "nothing",
            "$ID$ case sensitive matching",
            "$Id: expanded is ignored$",
        ] {
            let res = ident::apply(cowstr(input_no_match), gix_hash::Kind::Sha1);
            assert!(
                matches!(res, Cow::Borrowed(_)),
                "no substitution happens, so no mutable version of the Cow is created"
            );
            assert_eq!(res.as_ref(), input_no_match, "there definitely is no change");
        }
    }

    #[test]
    fn simple() {
        assert_eq!(
            ident::apply(cowstr("$Id$"), gix_hash::Kind::Sha1).as_ref(),
            "$Id: b3f5ebfb5843bc43ceecff6d4f26bb37c615beb1$"
        );

        assert_eq!(
            ident::apply(cowstr("$Id$ $Id$"), gix_hash::Kind::Sha1).as_ref(),
            "$Id: f6f3176060328ef7030a8b8eeda57fbf0587b2f9$ $Id: f6f3176060328ef7030a8b8eeda57fbf0587b2f9$"
        );
    }

    #[test]
    fn round_trips() {
        for input in [
            "hi\n$Id$\nho\n\t$Id$$Id$$Id$",
            "$Id$",
            "$Id$ and one more $Id$ and done",
        ] {
            let res = ident::apply(cowstr(input), gix_hash::Kind::Sha1);
            assert_ne!(res.as_ref(), input, "the input was rewritten");
            assert_eq!(ident::undo(res).as_ref(), input, "the filter can be undone perfectly");
        }
    }
}
