use bstr::{BStr, BString, ByteSlice};
use compact_str::CompactStr;

use crate::{MagicSignature, Pattern, SearchMode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Empty string is not a valid pathspec")]
    EmptyString,
    #[error("Found {:?}, which is not a valid keyword", keyword)]
    InvalidKeyword { keyword: BString },
    #[error("Unimplemented pathspec magic {:?}", short_keyword)]
    Unimplemented { short_keyword: char },
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
            p.signature |= parse_short_keywords(input, &mut cursor)?;
            if let Some(b'(') = input.get(cursor) {
                cursor += 1;
                parse_long_keywords(input, &mut p, &mut cursor)?;
            }
        }

        p.path = BString::from(&input[cursor..]);
        Ok(p)
    }
}

fn parse_short_keywords(input: &[u8], cursor: &mut usize) -> Result<MagicSignature, Error> {
    let unimplemented_chars = vec![
        b'"', b'#', b'%', b'&', b'\'', b',', b'-', b';', b'<', b'=', b'>', b'@', b'_', b'`', b'~',
    ];

    let mut signature = MagicSignature::empty();
    while let Some(&b) = input.get(*cursor) {
        *cursor += 1;
        signature |= match b {
            b'/' => MagicSignature::TOP,
            b'^' | b'!' => MagicSignature::EXCLUDE,
            b':' => break,
            _ if unimplemented_chars.contains(&b) => {
                return Err(Error::Unimplemented {
                    short_keyword: b.into(),
                });
            }
            _ => {
                *cursor -= 1;
                break;
            }
        }
    }

    Ok(signature)
}

fn parse_long_keywords(input: &[u8], p: &mut Pattern, cursor: &mut usize) -> Result<(), Error> {
    let end = input.find(")").ok_or(Error::MissingClosingParenthesis {
        pathspec: BString::from(input),
    })?;

    let input = &input[*cursor..end];
    *cursor = end + 1;

    debug_assert_eq!(p.search_mode, SearchMode::default());

    if input.is_empty() {
        return Ok(());
    }

    for keyword in split_on_non_escaped_char(input, b',') {
        match keyword {
            b"attr" => continue,
            b"top" => p.signature |= MagicSignature::TOP,
            b"icase" => p.signature |= MagicSignature::ICASE,
            b"exclude" => p.signature |= MagicSignature::EXCLUDE,
            b"literal" => match p.search_mode {
                SearchMode::PathAwareGlob => return Err(Error::IncompatibleSearchModes),
                _ => p.search_mode = SearchMode::Literal,
            },
            b"glob" => match p.search_mode {
                SearchMode::Literal => return Err(Error::IncompatibleSearchModes),
                _ => p.search_mode = SearchMode::PathAwareGlob,
            },
            _ if keyword.starts_with(b"attr:") => {
                if p.attributes.is_empty() {
                    p.attributes = parse_attributes(&keyword[5..])?;
                } else {
                    return Err(Error::MultipleAttributeSpecifications);
                }
            }
            _ if keyword.starts_with(b"prefix:") => {
                // TODO: Needs research - what does 'prefix:' do
            }
            _ => {
                return Err(Error::InvalidKeyword {
                    keyword: BString::from(keyword),
                });
            }
        }
    }

    Ok(())
}

fn split_on_non_escaped_char(input: &[u8], split_char: u8) -> Vec<&[u8]> {
    let mut keywords = Vec::new();
    let mut i = 0;
    let mut last = 0;
    loop {
        if let Some(&b) = input.get(i + 1) {
            if b == split_char && input[i] != b'\\' {
                i += 1;
                keywords.push(&input[last..i]);
                last = i + 1;
            }
        } else {
            keywords.push(&input[last..]);
            break;
        }
        i += 1;
    }
    keywords
}

fn parse_attributes(input: &[u8]) -> Result<Vec<(BString, git_attributes::State)>, Error> {
    if input.is_empty() {
        return Err(Error::EmptyAttribute);
    }

    let unescaped = input.replace(r"\,", ",");

    let parsed_attrs = git_attributes::parse::Iter::new(unescaped.as_bstr(), 0)
        .map(|res| res.map(|(name, state)| (name.into(), state.into())))
        // .map(|res| res.map(|(name, state)| unescape_attribute_value((name, state))))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| match e {
            git_attributes::parse::Error::AttributeName {
                line_number: _,
                attribute,
            } => Error::InvalidAttribute { attribute },
            _ => unreachable!("expecting only 'Error::AttributeName' but got {}", e),
        })?;

    for (_, state) in parsed_attrs.iter() {
        match state {
            git_attributes::State::Value(v) => _check_attr_value(v.as_str().into())?,
            _ => {}
        }
    }

    Ok(parsed_attrs)
}

fn _unescape_attribute_value((name, state): (BString, git_attributes::State)) -> (BString, git_attributes::State) {
    match &state {
        git_attributes::State::Value(v) if v.contains("\\") => {
            let mut i = 0;
            let v = BString::from(v.to_string());
            let mut new_v = CompactStr::default();
            loop {
                if let Some(_) = v.get(i + 1) {
                    if v[i] == b'\\' {
                        i += 1;
                        new_v.push(v[i] as char);
                    } else {
                        new_v.push(v[i] as char);
                    }
                } else {
                    new_v.push(v[i] as char);
                    break;
                }
                i += 1;
            }
            (name, git_attributes::State::Value(new_v))
        }
        _ => (name, state),
    }
}

fn _check_attr_value(value: &BStr) -> Result<(), Error> {
    // the only characters allowed in the PATHSPEC attribute value
    let is_allowed_char = |c: u8| c.is_ascii_alphanumeric() || c == b'-' || c == b'_' || c == b',';

    if !value.bytes().all(is_allowed_char) {
        // TODO: return correct error (invalid character in attribute value)
        return Err(Error::InvalidAttribute {
            attribute: value.to_owned(),
        });
    }

    if value.ends_with(&[b'\\']) {
        // TODO: return correct error (escape char not allowed as last char)
        return Err(Error::InvalidAttribute {
            attribute: value.to_owned(),
        });
    }

    Ok(())
}
