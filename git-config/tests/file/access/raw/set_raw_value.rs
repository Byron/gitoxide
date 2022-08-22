fn file(input: &str) -> git_config::File<'static> {
    input.parse().unwrap()
}

fn assert_set_value(value: &str) {
    let mut file = file("[a]k=b\n[a]\nk=c\nk=d");
    file.set_existing_raw_value("a", None, "k", value.into()).unwrap();
    assert_eq!(file.raw_value("a", None, "k").unwrap().as_ref(), value);

    let file: git_config::File = file.to_string().parse().unwrap();
    assert_eq!(
        file.raw_value("a", None, "k").unwrap().as_ref(),
        value,
        "{:?} didn't have expected value {:?}",
        file.to_string(),
        value
    );
}

#[test]
fn single_line() {
    assert_set_value("hello world");
}

#[test]
fn starts_with_whitespace() {
    assert_set_value("\ta");
    assert_set_value(" a");
}

#[test]
fn ends_with_whitespace() {
    assert_set_value("a\t");
    assert_set_value("a ");
}

#[test]
fn quotes_and_backslashes() {
    assert_set_value(r#""hello"\"there"\\\b\x"#);
}

#[test]
fn multi_line() {
    assert_set_value("a\nb   \n\t   c");
}

#[test]
fn comment_included() {
    assert_set_value(";hello ");
    assert_set_value(" # hello");
}
