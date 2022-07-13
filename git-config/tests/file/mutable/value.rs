mod get {
    use bstr::BString;

    use crate::file::mutable::value::init_config;

    fn config_get(input: &str) -> BString {
        let mut file: git_config::File = input.parse().unwrap();
        file.raw_value_mut("a", None, "k").unwrap().get().unwrap().into_owned()
    }

    #[test]
    fn empty() {
        assert_eq!(config_get("[a] k"), "");
    }

    #[test]
    fn single_line_before_comment() {
        assert_eq!(config_get("[a] k = hello there ; comment"), "hello there");
    }

    #[test]
    fn quoted_single_line_before_comment() {
        assert_eq!(config_get("[a] k = \" hello\tthere \"; comment"), " hello\tthere ");
    }

    #[test]
    fn multi_line_before_comment() {
        assert_eq!(config_get("[a] k = a\\\n  b\\\n  c ; comment"), "a  b  c");
    }

    #[test]
    fn value_is_correct() -> crate::Result {
        let mut config = init_config();

        let value = config.raw_value_mut("core", None, "a")?;
        assert_eq!(&*value.get()?, "b100");
        Ok(())
    }
}

mod set_string {
    use crate::file::mutable::value::init_config;

    fn file() -> git_config::File<'static> {
        "[a] k = v".parse().unwrap()
    }

    fn assert_set_string(expected: &str) {
        let mut file = file();
        let mut v = file.raw_value_mut("a", None, "k").unwrap();
        assert_eq!(v.get().unwrap().as_ref(), "v");
        v.set_string(expected);

        assert_eq!(v.get().unwrap().as_ref(), expected);

        let file: git_config::File = match file.to_string().parse() {
            Ok(f) => f,
            Err(err) => panic!("{:?} failed with: {}", file.to_string(), err),
        };
        assert_eq!(file.raw_value("a", None, "k").expect("present").as_ref(), expected);
    }

    #[test]
    fn empty() {
        assert_set_string("");
    }

    #[test]
    fn just_whitespace() {
        assert_set_string("\t ");
    }

    #[test]
    fn leading_whitespace_causes_double_quotes() {
        assert_set_string(" v");
    }

    #[test]
    fn single_line() {
        assert_set_string("hello world");
    }

    #[test]
    fn starts_with_whitespace() {
        assert_set_string("\ta");
        assert_set_string(" a");
    }

    #[test]
    fn ends_with_whitespace() {
        assert_set_string("a\t");
        assert_set_string("a ");
    }

    #[test]
    fn quotes_and_backslashes() {
        assert_set_string(r#""hello"\"there"\\\b\x"#);
    }

    #[test]
    fn multi_line() {
        assert_set_string("a\nb   \n\t   c");
    }

    #[test]
    fn comment_included() {
        assert_set_string(";hello ");
        assert_set_string(" # hello");
        assert_set_string("value then seemingly # comment");
    }

    #[test]
    fn simple_value_and_empty_string() -> crate::Result {
        let mut config = init_config();

        let mut value = config.raw_value_mut("core", None, "a")?;
        value.set_string("hello world");
        assert_eq!(
            config.to_string(),
            r#"[core]
            a=hello world
        [core]
            c=d
            e=f"#,
        );

        let mut value = config.raw_value_mut("core", None, "e")?;
        value.set_string(String::new());
        assert_eq!(
            config.to_string(),
            r#"[core]
            a=hello world
        [core]
            c=d
            e="#,
        );
        Ok(())
    }
}

mod delete {
    use crate::file::mutable::value::init_config;

    #[test]
    fn single_line_value() -> crate::Result {
        let mut config = init_config();

        let mut value = config.raw_value_mut("core", None, "a")?;
        value.delete();
        assert_eq!(
            config.to_string(),
            "[core]\n            \n        [core]\n            c=d\n            e=f",
        );

        let mut value = config.raw_value_mut("core", None, "c")?;
        value.delete();
        assert_eq!(
            config.to_string(),
            "[core]\n            \n        [core]\n            \n            e=f",
        );
        Ok(())
    }

    #[test]
    fn get_value_after_deleted() -> crate::Result {
        let mut config = init_config();

        let mut value = config.raw_value_mut("core", None, "a")?;
        value.delete();
        assert!(value.get().is_err());
        Ok(())
    }

    #[test]
    fn set_string_after_deleted() -> crate::Result {
        let mut config = init_config();

        let mut value = config.raw_value_mut("core", None, "a")?;
        value.delete();
        value.set_string("hello world");
        assert_eq!(
            config.to_string(),
            r#"[core]
            a=hello world
        [core]
            c=d
            e=f"#,
        );
        Ok(())
    }

    #[test]
    fn idempotency() -> crate::Result {
        let mut config = init_config();

        let mut value = config.raw_value_mut("core", None, "a")?;
        for _ in 0..3 {
            value.delete();
        }
        assert_eq!(
            config.to_string(),
            "[core]\n            \n        [core]\n            c=d\n            e=f"
        );
        Ok(())
    }

    #[test]
    fn multi_line_value() -> crate::Result {
        let mut config: git_config::File = r#"[core]
            a=b"100"\
c\
b
        [core]
            c=d
            e=f"#
            .parse()?;
        let mut value = config.raw_value_mut("core", None, "a")?;
        assert_eq!(&*value.get()?, "b100cb");
        value.delete();
        assert_eq!(
            config.to_string(),
            "[core]\n            \n        [core]\n            c=d\n            e=f"
        );
        Ok(())
    }
}

fn init_config() -> git_config::File<'static> {
    use std::convert::TryFrom;
    git_config::File::try_from(
        r#"[core]
            a=b"100"
        [core]
            c=d
            e=f"#,
    )
    .unwrap()
}
