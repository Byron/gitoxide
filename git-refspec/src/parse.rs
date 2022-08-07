#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty refspecs are invalid")]
    Empty,
    #[error("Negative refspecs cannot have destinations as they exclude sources")]
    NegativeWithDestination,
    #[error("Negative specs must not be empty")]
    NegativeEmpty,
    #[error("Cannot push into an empty destination")]
    PushToEmpty,
    #[error("glob patterns may only involved a single '*' character, found {pattern:?}")]
    PatternUnsupported { pattern: bstr::BString },
    #[error("Both sides of the specification need a pattern, like 'a/*:b/*'")]
    PatternUnbalanced,
    #[error(transparent)]
    ReferenceName(#[from] git_validate::refname::Error),
    #[error(transparent)]
    RevSpec(#[from] git_revision::spec::parse::Error),
}

pub(crate) mod function {
    use crate::parse::Error;
    use crate::{Mode, Operation, RefSpecRef};
    use bstr::{BStr, ByteSlice};

    /// Parse `spec` for use in `operation` and return it if it is valid.
    pub fn parse(mut spec: &BStr, operation: Operation) -> Result<RefSpecRef<'_>, Error> {
        fn fetch_head_only(mode: Mode) -> RefSpecRef<'static> {
            RefSpecRef {
                mode,
                op: Operation::Fetch,
                src: Some("HEAD".into()),
                dst: None,
            }
        }

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
                    Operation::Fetch => Ok(fetch_head_only(Mode::Normal)),
                }
            }
        };

        let (mut src, dst) = match spec.find_byte(b':') {
            Some(pos) => {
                if mode == Mode::Negative {
                    return Err(Error::NegativeWithDestination);
                }

                let (src, dst) = spec.split_at(pos);
                let dst = &dst[1..];
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
            None => {
                let src = (!spec.is_empty()).then(|| spec);
                if Operation::Fetch == operation && mode != Mode::Negative && src.is_none() {
                    return Ok(fetch_head_only(mode));
                } else {
                    (src, None)
                }
            }
        };

        if mode == Mode::Negative && src.is_none() {
            return Err(Error::NegativeEmpty);
        }

        if let Some(spec) = src.as_mut() {
            if *spec == "@" {
                *spec = "HEAD".into();
            }
        }
        let (src, src_had_pattern) = validated(src, operation == Operation::Push)?;
        let (dst, dst_had_pattern) = validated(dst, false)?;
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

    fn validated(spec: Option<&BStr>, allow_revspecs: bool) -> Result<(Option<&BStr>, bool), Error> {
        match spec {
            Some(spec) => {
                let glob_count = spec.iter().filter(|b| **b == b'*').take(2).count();
                if glob_count > 1 {
                    return Err(Error::PatternUnsupported { pattern: spec.into() });
                }
                let has_globs = glob_count == 1;
                if has_globs {
                    let mut buf = smallvec::SmallVec::<[u8; 256]>::with_capacity(spec.len());
                    buf.extend_from_slice(spec);
                    let glob_pos = buf.find_byte(b'*').expect("glob present");
                    buf[glob_pos] = b'a';
                    git_validate::reference::name_partial(buf.as_bstr())?;
                } else {
                    git_validate::reference::name_partial(spec)
                        .map_err(Error::from)
                        .or_else(|err| {
                            if allow_revspecs {
                                match git_revision::spec::parse(spec, &mut super::revparse::Noop) {
                                    Ok(_) => {
                                        if spec.iter().any(|b| b.is_ascii_whitespace()) {
                                            Err(err)
                                        } else {
                                            Ok(spec)
                                        }
                                    }
                                    Err(err) => Err(err.into()),
                                }
                            } else {
                                Err(err)
                            }
                        })?;
                }
                Ok((Some(spec), has_globs))
            }
            None => Ok((None, false)),
        }
    }
}

mod revparse {
    use bstr::BStr;
    use git_revision::spec::parse::delegate::{
        Kind, Navigate, PeelTo, PrefixHint, ReflogLookup, Revision, SiblingBranch, Traversal,
    };

    pub(crate) struct Noop;

    impl Revision for Noop {
        fn find_ref(&mut self, _name: &BStr) -> Option<()> {
            Some(())
        }

        fn disambiguate_prefix(&mut self, _prefix: git_hash::Prefix, _hint: Option<PrefixHint<'_>>) -> Option<()> {
            Some(())
        }

        fn reflog(&mut self, _query: ReflogLookup) -> Option<()> {
            Some(())
        }

        fn nth_checked_out_branch(&mut self, _branch_no: usize) -> Option<()> {
            Some(())
        }

        fn sibling_branch(&mut self, _kind: SiblingBranch) -> Option<()> {
            Some(())
        }
    }

    impl Navigate for Noop {
        fn traverse(&mut self, _kind: Traversal) -> Option<()> {
            Some(())
        }

        fn peel_until(&mut self, _kind: PeelTo<'_>) -> Option<()> {
            Some(())
        }

        fn find(&mut self, _regex: &BStr, _negated: bool) -> Option<()> {
            Some(())
        }

        fn index_lookup(&mut self, _path: &BStr, _stage: u8) -> Option<()> {
            Some(())
        }
    }

    impl Kind for Noop {
        fn kind(&mut self, _kind: git_revision::spec::Kind) -> Option<()> {
            Some(())
        }
    }

    impl git_revision::spec::parse::Delegate for Noop {
        fn done(&mut self) {}
    }
}
