use gix_attributes::{AssignmentRef, NameRef, StateRef};

#[test]
fn display() {
    assert_eq!(adisplay("hello", StateRef::Unspecified), "!hello");
    assert_eq!(adisplay("hello", StateRef::Unset), "-hello");
    assert_eq!(adisplay("hello", StateRef::Set), "hello");
    assert_eq!(adisplay("hello", StateRef::Value("value".into())), "hello=value");
}

fn adisplay(name: &str, state: StateRef<'_>) -> String {
    AssignmentRef {
        name: NameRef::try_from(bstr::BStr::new(name.as_bytes())).expect("valid name"),
        state,
    }
    .to_string()
}
