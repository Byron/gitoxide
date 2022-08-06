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
    pub fn parse(mut spec: &BStr, operation: Operation) -> Result<RefSpecRef<'_>, Error> {
        let mode = match spec.get(0) {
            Some(&b'^') => {
                spec = &spec[1..];
                Mode::Negative
            }
            Some(&b'+') => {
                spec = &spec[1..];
                Mode::Force
            }
            Some(_) => Mode::Normal,
            None => return Err(Error::Empty),
        };

        let (src, dst) = match spec.find_byte(b':') {
            Some(pos) => {
                let (src, dst) = spec.split_at(pos);
                let dst = &dst[1..];
                if mode == Mode::Negative {
                    return Err(Error::NegativeWithDestination);
                }
                let src = (!src.is_empty()).then(|| src.as_bstr());
                let dst = (!dst.is_empty()).then(|| dst.as_bstr());
                match (src, dst) {
                    (None, None) => (None, None), // match all
                    (None, Some(dst)) => match operation {
                        Operation::Push => (None, Some(dst)),
                        Operation::Fetch => (Some("HEAD".into()), Some(dst)),
                    },
                    _ => todo!("src or dst handling"),
                }
            }
            None => todo!("no colon"),
        };

        Ok(RefSpecRef {
            op: operation,
            mode,
            src,
            dst,
        })
    }
}
