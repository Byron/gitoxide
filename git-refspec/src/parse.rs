#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty refspecs are invalid")]
    Empty,
    #[error("Negative refspecs cannot have destinations as they exclude sources")]
    NegativeWithDestination,
    #[error("Cannot push into an empty destination")]
    PushToEmpty,
    #[error("glob patterns may only involved a single '*' character, found {pattern:?}")]
    PatternUnsupported { pattern: bstr::BString },
    #[error("Both sides of the specification need a pattern, like 'a/*:b/*'")]
    PatternUnbalanced,
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
            None => {
                return match operation {
                    Operation::Push => Err(Error::Empty),
                    Operation::Fetch => Ok(RefSpecRef {
                        mode: Mode::Normal,
                        op: operation,
                        src: Some("HEAD".into()),
                        dst: None,
                    }),
                }
            }
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
                    (None, None) => match operation {
                        Operation::Push => (None, None),
                        Operation::Fetch => (Some("HEAD".into()), None),
                    },
                    (None, Some(dst)) => match operation {
                        Operation::Push => (None, Some(dst)),
                        Operation::Fetch => (Some("HEAD".into()), Some(dst)),
                    },
                    (Some(src), None) => match operation {
                        Operation::Push => return Err(Error::PushToEmpty),
                        Operation::Fetch => (Some(src), None),
                    },
                    (Some(src), Some(dst)) => (Some(src), Some(dst)),
                }
            }
            None => (Some(spec), None),
        };

        let (src, src_had_pattern) = validated(src)?;
        let (dst, dst_had_pattern) = validated(dst)?;
        if mode != Mode::Negative && src_had_pattern != dst_had_pattern {
            return Err(Error::PatternUnbalanced);
        }
        Ok(RefSpecRef {
            op: operation,
            mode,
            src,
            dst,
        })
    }

    fn validated(spec: Option<&BStr>) -> Result<(Option<&BStr>, bool), Error> {
        match spec {
            Some(spec) => {
                let glob_count = spec.iter().filter(|b| **b == b'*').take(2).count();
                if glob_count == 2 {
                    return Err(Error::PatternUnsupported { pattern: spec.into() });
                }
                Ok((Some(spec), glob_count == 1))
            }
            None => Ok((None, false)),
        }
    }
}
