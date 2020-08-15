fn assert_err_display<T: std::fmt::Debug, E: std::error::Error>(res: Result<T, E>, expected: impl AsRef<str>) {
    match res {
        Ok(v) => assert!(false, "Expected error '{}', got value {:?}", expected.as_ref(), v),
        Err(err) => assert_eq!(err.to_string(), expected.as_ref()),
    }
}

mod decode;
mod encode;
mod read;
