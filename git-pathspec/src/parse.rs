use bstr::{BString, ByteSlice};
use git_attributes::parse::Iter;

use crate::{MagicSignature, Pattern, SearchMode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Empty string is not a valid pathspec")]
    EmptyString,
    #[error("Found {:?}, which is not a valid keyword", found_keyword)]
    InvalidKeyword { found_keyword: BString },
    #[error("Missing ')' at the end of pathspec magic in {:?}", pathspec)]
    MissingClosingParenthesis { pathspec: BString },
    #[error("Attribute has non-ascii characters or starts with '-': {:?}", attribute)]
    InvalidAttribute { attribute: BString },
    #[error("Attribute specification cannot be empty")]
    EmptyAttribute,
    #[error("'literal' and 'glob' keywords cannot be used together in the same pathspec")]
    IncompatibleSearchModes,
    #[error("Only one attribute specification is allowed in the same pathspec")]
    MultipleAttributeSpecifications,
}

impl Pattern {
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        if input.is_empty() {
            return Err(Error::EmptyString);
        }

        let mut p = Pattern {
            path: BString::default(),
            signature: MagicSignature::empty(),
            search_mode: SearchMode::ShellGlob,
            attributes: Vec::new(),
        };
        let mut cursor = 0;
        if input.first() == Some(&b':') {
            cursor += 1;
            while let Some(&b) = input.get(cursor) {
                cursor += 1;
                match b {
                    b':' => break,
                    b'/' => p.signature |= MagicSignature::TOP,
                    b'^' | b'!' => p.signature |= MagicSignature::EXCLUDE,
                    b'(' => {
                        let end = input.find(")").ok_or(Error::MissingClosingParenthesis {
                            pathspec: BString::from(input),
                        })?;
                        let pat = parse_keywords(&input[cursor..end])?;

                        cursor = end + 1;
                        p.search_mode = pat.search_mode;
                        p.attributes = pat.attributes;
                        p.signature |= pat.signature;
                    }
                    _ => {
                        cursor -= 1;
                        break;
                    }
                }
            }
        }

        p.path = BString::from(&input[cursor..]);
        Ok(p)
    }
}

fn parse_keywords(input: &[u8]) -> Result<Pattern, Error> {
    let mut p = Pattern {
        path: BString::default(),
        signature: MagicSignature::empty(),
        search_mode: SearchMode::ShellGlob,
        attributes: Vec::new(),
    };

    debug_assert_eq!(p.search_mode, SearchMode::default());

    if input.is_empty() {
        return Ok(p);
    }

    for keyword in input.split(|&c| c == b',') {
        match keyword {
            b"top" => p.signature |= MagicSignature::TOP,
            b"icase" => p.signature |= MagicSignature::ICASE,
            b"exclude" => p.signature |= MagicSignature::EXCLUDE,
            b"attr" => p.signature |= MagicSignature::ATTR,
            b"literal" => match p.search_mode {
                SearchMode::PathAwareGlob => return Err(Error::IncompatibleSearchModes),
                _ => p.search_mode = SearchMode::Literal,
            },
            b"glob" => match p.search_mode {
                SearchMode::Literal => return Err(Error::IncompatibleSearchModes),
                _ => p.search_mode = SearchMode::PathAwareGlob,
            },
            s => {
                let attrs = s.strip_prefix(b"attr:").ok_or_else(|| Error::InvalidKeyword {
                    found_keyword: BString::from(s),
                })?;
                if attrs.is_empty() {
                    return Err(Error::EmptyAttribute);
                }
                if !p.attributes.is_empty() {
                    return Err(Error::MultipleAttributeSpecifications);
                }
                p.attributes = Iter::new(attrs.into(), 0)
                    .map(|res| res.map(|(attr, state)| (attr.into(), state.into())))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| match e {
                        git_attributes::parse::Error::AttributeName {
                            line_number: _,
                            attribute,
                        } => Error::InvalidAttribute { attribute },
                        _ => unreachable!("expecting only 'Error::AttributeName' but got {}", e),
                    })?;
                p.signature |= MagicSignature::ATTR
            }
        }
    }

    Ok(p)
}
