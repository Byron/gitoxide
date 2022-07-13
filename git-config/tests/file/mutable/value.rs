mod get {
    use bstr::BString;

    fn file_get(input: &str) -> BString {
        let mut file: git_config::File = input.parse().unwrap();
        file.raw_value_mut("a", None, "k").unwrap().get().unwrap().into_owned()
    }

    #[test]
    #[ignore]
    fn empty() {
        assert_eq!(file_get("[a] k"), "");
    }
}

mod set_string {
    fn file() -> git_config::File<'static> {
        "[a] k=v".parse().unwrap()
    }

    #[test]
    #[ignore]
    fn leading_whitespace_causes_double_quotes() -> crate::Result {
        let mut file = file();
        let mut v = file.raw_value_mut("a", None, "k")?;
        assert_eq!(v.get()?.as_ref(), "v");
        v.set_string(" v");
        assert_eq!(v.get()?.as_ref(), " v");
        Ok(())
    }
}
