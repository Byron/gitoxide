#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty refspecs are invalid")]
    Empty,
    #[error("Negative refspecs cannot have destinations as they exclude sources")]
    NegativeWithDestination,
}

pub(crate) mod function {
    use crate::parse::Error;
    use crate::{Mode, Operation, RefSpecRef};
    use bstr::{BStr, ByteSlice};

    /// Parse `spec` for use in `operation` and return it if it is valid.
    pub fn parse(mut spec: &BStr, _operation: Operation) -> Result<RefSpecRef<'_>, Error> {
        let mode = match spec.get(0) {
            Some(&b'^') => {
                spec = &spec[1..];
                Mode::Negative
            }
            Some(_) => Mode::Normal,
            None => return Err(Error::Empty),
        };

        match spec.find_byte(b':') {
            Some(pos) => {
                let (_src, _dst) = spec.split_at(pos);
                if mode == Mode::Negative {
                    return Err(Error::NegativeWithDestination);
                }
                todo!("with colon")
            }
            None => todo!("no colon"),
        }
    }
}
