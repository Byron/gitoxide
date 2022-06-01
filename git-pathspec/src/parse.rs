use bstr::{BString, ByteSlice};
use git_attributes::{parse::Iter, State};

use crate::{MagicSignature, Pattern, SearchMode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Empty string is not a valid pathspec")]
    EmptyString,

    #[error("Found \"{}\", which is not a valid keyword", found_keyword)]
    InvalidKeyword { found_keyword: BString },

    #[error("Missing ')' at the end of pathspec magic in '{}'", pathspec)]
    MissingClosingParenthesis { pathspec: BString },

    #[error("Attribute has non-ascii characters or starts with '-': {}", attribute)]
    InvalidAttribute { attribute: BString },

    #[error("'literal' and 'globl' keywords can not be used together in the same pathspec")]
    IncompatibleSearchmodes,
}

impl Pattern {
    pub fn empty() -> Self {
        Pattern {
            path: BString::default(),
            signature: MagicSignature::empty(),
            searchmode: SearchMode::Default,
            attributes: Vec::new(),
        }
    }

    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        if input.is_empty() {
            return Err(Error::EmptyString);
        }

        let mut cursor = 0;
        let mut signature = MagicSignature::empty();
        let mut searchmode = SearchMode::Default;
        let mut attributes = Vec::new();

        if input.first() == Some(&b':') {
            cursor += 1;
            while let Some(&b) = input.get(cursor) {
                cursor += 1;
                match b {
                    b':' => break,
                    b'/' => signature |= MagicSignature::TOP,
                    b'^' | b'!' => signature |= MagicSignature::EXCLUDE,
                    b'(' => {
                        let end = input.find(")").ok_or(Error::MissingClosingParenthesis {
                            pathspec: BString::from(input),
                        })?;
                        let (sig, sm, attrs) = parse_keywords(&input[cursor..end])?;

                        cursor = end + 1;
                        signature |= sig;
                        searchmode = sm;
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
            signature,
            searchmode,
            attributes,
        })
    }
}

fn parse_keywords(input: &[u8]) -> Result<(MagicSignature, SearchMode, Vec<(BString, State)>), Error> {
    let mut signature = MagicSignature::empty();
    let mut searchmode = SearchMode::Default;
    let mut attributes = Vec::new();

    if input.is_empty() {
        return Ok((signature, searchmode, attributes));
    }

    for keyword in input.split(|&c| c == b',') {
        match keyword {
            b"top" => signature |= MagicSignature::TOP,
            b"icase" => signature |= MagicSignature::ICASE,
            b"exclude" => signature |= MagicSignature::EXCLUDE,
            b"attr" => signature |= MagicSignature::ATTR,
            b"literal" => match searchmode {
                SearchMode::Default => searchmode = SearchMode::Literal,
                _ => return Err(Error::IncompatibleSearchmodes),
            },
            b"glob" => match searchmode {
                SearchMode::Default => searchmode = SearchMode::Glob,
                _ => return Err(Error::IncompatibleSearchmodes),
            },
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
                    signature |= MagicSignature::ATTR
                } else {
                    return Err(Error::InvalidKeyword {
                        found_keyword: BString::from(s),
                    });
                }
            }
        }
    }

    Ok((signature, searchmode, attributes))
}
