mod valid {
    use bstr::ByteSlice;
    use git_validate;
    macro_rules! mktest {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                assert!(git_validate::refname($input.as_bstr()).is_ok())
            }
        };
    }

    mktest!(a_path, b"/refs/heads/main");
    mktest!(a_path_with_underscores_and_dashes, b"/refs/heads/main-2nd_ext");
    mktest!(uppercase_name, b"Main");
    mktest!(all_uppercase, b"MAIN");
}

mod invalid {
    // use git_ref::validate;

    // macro_rules! mktest {
    //     ($name:ident, $input:literal, $expected:ident) => {
    //         #[test]
    //         fn $name() {
    //             match git_validate::refname($input.as_bstr()) {
    //                 Err(validate::refname::Error::$expected) => {}
    //                 got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
    //             }
    //         }
    //     };
    // }

    // mktest!(lowercase_name_without_path, b"main");
    // mktest!(starts_with_underscore, b"_private");
    // mktest!(ends_with_underscore, b"private_");
    // mktest!(starts_with_slash, b"/etc/foo");
    // mktest!(refs_starts_with_slash, b"/refs/heads/main");
    // mktest!(ends_with_slash, b"refs/heads/main/");
    // mktest!(a_path_with_duplicate_slashes, b"refs//heads/main");
}
