mod name {
    mod valid {
        use bstr::ByteSlice;
        use git_ref::validated;

        macro_rules! mktest {
            ($name:ident, $input:expr) => {
                #[test]
                fn $name() {
                    assert!(validated::name($input.as_bstr()).is_ok())
                }
            };
        }

        mktest!(an_at_sign, b"@");
        mktest!(chinese_unicode, "你好吗".as_bytes());
        mktest!(non_text, "😅🙌".as_bytes());
        mktest!(contains_an_at, b"hello@foo");
        mktest!(contains_dot_lock, b"file.lock.ext");
        mktest!(contains_brackets, b"this_{is-fine}_too");
        mktest!(contains_brackets_and_at, b"this_{@is-fine@}_too");
        mktest!(dot_in_the_middle, b"token.other");
        mktest!(dot_at_the_end, b"hello.");
        mktest!(slash_inbetween, b"hello/world");
    }

    mod invalid {
        use bstr::ByteSlice;
        use git_ref::validated;

        macro_rules! mktest {
            ($name:ident, $input:literal, $expected:ident) => {
                #[test]
                fn $name() {
                    match validated::name($input.as_bstr()) {
                        Err(validated::NameError::$expected) => {}
                        got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
                    }
                }
            };
        }
        macro_rules! mktestb {
            ($name:ident, $input:literal) => {
                #[test]
                fn $name() {
                    match validated::name($input.as_bstr()) {
                        Err(validated::NameError::InvalidByte(_)) => {}
                        got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
                    }
                }
            };
        }
        mktest!(contains_ref_log_portion, b"this_looks_like_a_@{reflog}", ReflogPortion);
        mktest!(suffix_is_dot_lock, b"prefix.lock", LockFileSuffix);
        mktest!(ends_with_slash, b"prefix/", EndsWithSlash);
        mktest!(is_dot_lock, b".lock", StartsWithDot);
        mktest!(contains_double_dot, b"with..double-dot", DoubleDot);
        mktest!(starts_with_double_dot, b"..with-double-dot", DoubleDot);
        mktest!(ends_with_double_dot, b"with-double-dot..", DoubleDot);
        mktest!(starts_with_asterisk, b"*suffix", Asterisk);
        mktest!(ends_with_asterisk, b"prefix*", Asterisk);
        mktest!(contains_asterisk, b"prefix*suffix", Asterisk);
        mktestb!(contains_null, b"prefix\0suffix");
        mktestb!(contains_bell, b"prefix\x07suffix");
        mktestb!(contains_backspace, b"prefix\x08suffix");
        mktestb!(contains_vertical_tab, b"prefix\x0bsuffix");
        mktestb!(contains_form_feed, b"prefix\x0csuffix");
        mktestb!(contains_ctrl_z, b"prefix\x1asuffix");
        mktestb!(contains_esc, b"prefix\x1bsuffix");
        mktestb!(contains_colon, b"prefix:suffix");
        mktestb!(contains_questionmark, b"prefix?suffix");
        mktestb!(contains_open_bracket, b"prefix[suffix");
        mktestb!(contains_backslash, b"prefix\\suffix");
        mktestb!(contains_circumflex, b"prefix^suffix");
        mktestb!(contains_tilde, b"prefix~suffix");
        mktestb!(contains_space, b"prefix suffix");
        mktestb!(contains_tab, b"prefix\tsuffix");
        mktestb!(contains_newline, b"prefix\nsuffix");
        mktestb!(contains_carriage_return, b"prefix\rsuffix");
        mktest!(starts_with_dot, b".with-dot", StartsWithDot);
        mktest!(empty, b"", Empty);
    }
}
