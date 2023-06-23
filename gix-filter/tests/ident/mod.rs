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
