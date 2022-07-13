fn file(input: &str) -> git_config::File<'static> {
    input.parse().unwrap()
}

#[test]
fn single_line() -> crate::Result {
    let mut file = file("[a]k=b\n[a]\nk=c\nk=d");
    file.set_raw_value("a", None, "k", "e".into())?;
    assert_eq!(file.raw_value("a", None, "k")?.as_ref(), "e");
    Ok(())
}
