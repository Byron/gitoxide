use gix_validate::submodule::name::Error;

#[test]
fn valid() {
    fn validate(name: &str) -> Result<(), Error> {
        gix_validate::submodule::name(name.into()).map(|_| ())
    }

    for valid_name in ["a/./b/..[", "..a/./b/", "..a\\./b\\", "你好"] {
        validate(valid_name).unwrap_or_else(|err| panic!("{valid_name} should be valid: {err:?}"));
    }
}

mod invalid {
    use bstr::ByteSlice;

    macro_rules! mktest {
        ($name:ident, $input:literal, $expected:ident) => {
            #[test]
            fn $name() {
                match gix_validate::submodule::name($input.as_bstr()) {
                    Err(gix_validate::submodule::name::Error::$expected) => {}
                    got => panic!("Wanted {}, got {:?}", stringify!($expected), got),
                }
            }
        };
    }

    mktest!(empty, b"", Empty);
    mktest!(starts_with_parent_component, b"../", ParentComponent);
    mktest!(parent_component_in_middle, b"hi/../ho", ParentComponent);
    mktest!(ends_with_parent_component, b"hi/ho/..", ParentComponent);
    mktest!(only_parent_component, b"..", ParentComponent);
    mktest!(starts_with_parent_component_backslash, b"..\\", ParentComponent);
    mktest!(parent_component_in_middle_backslash, b"hi\\..\\ho", ParentComponent);
    mktest!(ends_with_parent_component_backslash, b"hi\\ho\\..", ParentComponent);
}
