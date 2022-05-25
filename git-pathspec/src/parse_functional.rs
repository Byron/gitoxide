use crate::parse::Error;
use bstr::{BString, ByteSlice};
use git_attributes::{parse::Iter, State};

use crate::{MagicSignature, Pattern};

impl Pattern {
    pub fn empty() -> Self {
        Pattern {
            path: BString::default(),
            signature: None,
            attributes: Vec::new(),
        }
    }

    pub fn from_bytes_functional(input: &[u8]) -> Result<Self, Error> {
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
            s if s.starts_with(b"attr:") => Iter::new(s[5..].into(), 0)
                .map(|res| res.map(|(attr, state)| (BString::from(attr), State::from(state))))
                .collect::<Result<Vec<_>, _>>()
                .map(|v| {
                    attributes = v;
                    MagicSignature::ATTR
                })?,
            s => {
                return Err(Error::InvalidSignature {
                    found_signature: BString::from(s),
                });
            }
        }
    }

    Ok((signature, attributes))
}
