use bstr::{BString, ByteSlice};
use git_attributes::{parse::Iter, State};

use crate::{MagicSignature, Pattern};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Found \"{}\", which is not a valid signature", found_signature)]
    InvalidSignature { found_signature: BString },

    #[error("Missing ')' at the end of pathspec magic in '{}'", pathspec)]
    MissingClosingParenthesis { pathspec: BString },

    #[error("Attribute has non-ascii characters or starts with '-': {}", attribute)]
    InvalidAttribute { attribute: BString },
}

impl Pattern {
    pub fn empty() -> Self {
        Pattern {
            path: BString::default(),
            signature: None,
            attributes: Vec::new(),
        }
    }

    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        if input.is_empty() {
            return Ok(Pattern::empty());
        }

        let mut cursor = 0;
        let mut signature = MagicSignature::empty();
        let mut attributes = Vec::new();

        if input.first() == Some(&b':') {
            while let Some(&b) = input.get(cursor) {
                cursor += 1;
                match b {
                    b':' => {
                        if !signature.is_empty() {
                            break;
                        }
                    }
                    b'/' => signature |= MagicSignature::TOP,
                    b'^' | b'!' => signature |= MagicSignature::EXCLUDE,
                    b'(' => {
                        let end = input.find(")").ok_or(Error::MissingClosingParenthesis {
                            pathspec: BString::from(input),
                        })?;
                        let (signatures, attrs) = parse_keywords(&input[cursor..end])?;

                        cursor = end + 1;
                        signature |= signatures;
                        attributes = attrs;
                    }
                    _ => {
                        cursor -= 1;
                        break;
                    }
                }
            }
        }

        Ok(Pattern {
            path: BString::from(&input[cursor..]),
            signature: (!signature.is_empty()).then(|| signature),
            attributes,
        })
    }
}

fn parse_keywords(input: &[u8]) -> Result<(MagicSignature, Vec<(BString, State)>), Error> {
    let mut signature = MagicSignature::empty();
    let mut attributes = Vec::new();

    if input.is_empty() {
        return Ok((signature, attributes));
    }

    for keyword in input.split(|&c| c == b',') {
        signature |= match keyword {
            b"top" => MagicSignature::TOP,
            b"literal" => MagicSignature::LITERAL,
            b"icase" => MagicSignature::ICASE,
            b"glob" => MagicSignature::GLOB,
            b"attr" => MagicSignature::ATTR,
            b"exclude" => MagicSignature::EXCLUDE,
            s => {
                if let Some(attrs) = s.strip_prefix(b"attr:") {
                    attributes = Iter::new(attrs.into(), 0)
                        .map(|res| res.map(|(attr, state)| (attr.into(), state.into())))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| match e {
                            git_attributes::parse::Error::AttributeName {
                                line_number: _,
                                attribute,
                            } => Error::InvalidAttribute { attribute },
                            _ => unreachable!("expecting only 'Error::AttributeName' but got {}", e),
                        })?;
                    MagicSignature::ATTR
                } else {
                    return Err(Error::InvalidSignature {
                        found_signature: BString::from(s),
                    });
                }
            }
        }
    }

    Ok((signature, attributes))
}
