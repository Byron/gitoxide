mod get {
    use crate::file::cow_str;
    use crate::file::mutable::multi_value::init_config;

    #[test]
    fn single_lines() -> crate::Result {
        let mut config = init_config();

        let value = config.raw_values_mut("core", None, "a")?;
        assert_eq!(&*value.get()?, vec![cow_str("b100"), cow_str("d"), cow_str("f"),]);
        Ok(())
    }

    #[test]
    fn multi_line() -> crate::Result {
        let mut config: git_config::File = r#"[core]
            a=b\
"100"
        [core]
            a=d\
b
            a=f\
a"#
        .parse()?;

        let mut values = config.raw_values_mut("core", None, "a")?;
        assert_eq!(&*values.get()?, vec![cow_str("b100"), cow_str("db"), cow_str("fa"),]);

        values.delete_all();
        assert!(values.get().is_err());

        Ok(())
    }
}

mod access {
    use crate::file::mutable::multi_value::init_config;

    #[test]
    fn non_empty_sizes() -> crate::Result {
        let mut config = init_config();
        assert_eq!(config.raw_values_mut("core", None, "a")?.len(), 3);
        assert!(!config.raw_values_mut("core", None, "a")?.is_empty());
        Ok(())
    }
}

mod set {
    use crate::file::mutable::multi_value::init_config;

    #[test]
    fn single_at_start() -> crate::Result {
        let mut config = init_config();
        let mut values = config.raw_values_mut("core", None, "a")?;
        values.set_string(0, "Hello".into());
        assert_eq!(
            config.to_string(),
            "[core]\n    a=Hello\n    [core]\n        a=d\n        a=f"
        );
        Ok(())
    }

    #[test]
    fn single_at_end() -> crate::Result {
        let mut config = init_config();
        let mut values = config.raw_values_mut("core", None, "a")?;
        values.set_string(2, "Hello".into());
        assert_eq!(
            config.to_string(),
            "[core]\n    a=b\"100\"\n    [core]\n        a=d\n        a=Hello"
        );
        Ok(())
    }

    #[test]
    fn all() -> crate::Result {
        let mut config = init_config();
        let mut values = config.raw_values_mut("core", None, "a")?;
        values.set_owned_values_all("Hello");
        assert_eq!(
            config.to_string(),
            "[core]\n    a=Hello\n    [core]\n        a=Hello\n        a=Hello"
        );
        Ok(())
    }
}

mod delete {
    use crate::file::mutable::multi_value::init_config;

    #[test]
    fn single_at_start_and_end() -> crate::Result {
        let mut config = init_config();
        {
            let mut values = config.raw_values_mut("core", None, "a")?;
            values.delete(0);
            assert_eq!(config.to_string(), "[core]\n    \n    [core]\n        a=d\n        a=f",);
        }

        let mut values = config.raw_values_mut("core", None, "a")?;
        values.delete(1);
        assert_eq!(config.to_string(), "[core]\n    \n    [core]\n        a=d\n        ",);
        Ok(())
    }

    #[test]
    fn all() -> crate::Result {
        let mut config = init_config();
        let mut values = config.raw_values_mut("core", None, "a")?;
        values.delete_all();
        assert!(values.get().is_err());
        assert_eq!(config.to_string(), "[core]\n    \n    [core]\n        \n        ",);
        Ok(())
    }
}

#[test]
#[ignore]
fn empty_value() {
    todo!()
}

fn init_config() -> git_config::File<'static> {
    r#"[core]
    a=b"100"
    [core]
        a=d
        a=f"#
        .parse()
        .unwrap()
}
