#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty refspecs are invalid")]
    Empty,
    #[error("Negative refspecs cannot have destinations as they exclude sources")]
    NegativeWithDestination,
}

pub(crate) mod function {
    use crate::parse::Error;
    use crate::{Operation, RefSpecRef};
    use bstr::BStr;

    /// Parse `spec` for use in `operation` and return it if it is valid.
    pub fn parse(mut _spec: &BStr, _operation: Operation) -> Result<RefSpecRef<'_>, Error> {
        todo!()
    }
}
