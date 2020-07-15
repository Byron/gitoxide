mod name {
    mod valid {
        use crate::validated;
        use bstr::ByteSlice;

        macro_rules! mktest {
            ($name:ident, $input:literal) => {
                #[test]
                fn $name() {
                    assert!(validated::name($input.as_bstr()).is_ok())
                }
            };
        }

        mktest!(dot_in_the_middle, b"token.other");
        mktest!(dot_at_the_end, b"hello.");
    }
    mod invalid {
        use crate::validated;
        use bstr::ByteSlice;

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

        mktest!(starts_with_dot, b".with-dot", StartsWithDot);
        mktest!(empty, b"", Empty);
    }
}
