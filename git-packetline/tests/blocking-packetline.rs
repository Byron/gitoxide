pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub fn assert_err_display<T: std::fmt::Debug, E: std::error::Error>(
    res: std::result::Result<T, E>,
    expected: impl AsRef<str>,
) {
    match res {
        Ok(v) => assert!(false, "Expected error '{}', got value {:?}", expected.as_ref(), v),
        Err(err) => assert_eq!(err.to_string(), expected.as_ref()),
    }
}

#[cfg(feature = "blocking-io")]
mod blocking;
