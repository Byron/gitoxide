use crate::{Error, Options};
use bstr::BString;

/// Ask the user given a `prompt`, returning the result.
pub fn ask(_prompt: &str, Options { secret }: Options) -> Result<BString, Error> {
    todo!()
}
