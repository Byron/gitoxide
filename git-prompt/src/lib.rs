#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[derive(Debug, thiserror::Error)]
#[error("TBD")]
pub struct Error;

/// Ask for information typed by the user after showing the prompt`, like `"Username: `.
pub fn openly(_prompt: &str) -> Result<String, Error> {
    todo!("open")
}

/// Ask for information _securely_ after showing the `prompt` (like `"password: "`) by not showing what's typed.
pub fn securely(_prompt: &str) -> Result<String, Error> {
    todo!("securely")
}
