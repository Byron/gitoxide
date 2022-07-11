use crate::{MagicSignature, Pattern, SearchMode};
use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Empty string is not a valid pathspec")]
    EmptyString,
    #[error("Found {:?} in signature, which is not a valid keyword", keyword)]
    InvalidKeyword { keyword: BString },
    #[error("Unimplemented short keyword: {:?}", short_keyword)]
    Unimplemented { short_keyword: char },
    #[error("Missing ')' at the end of pathspec signature")]
    MissingClosingParenthesis,
    #[error("Attribute has non-ascii characters or starts with '-': {:?}", attribute)]
    InvalidAttribute { attribute: BString },
    #[error("Invalid character in attribute value: {:?}", character)]
    InvalidAttributeValue { character: char },
    #[error("Escape character '\\' is not allowed as the last character in an attribute value")]
    TrailingEscapeCharacter,
    #[error("Attribute specification cannot be empty")]
    EmptyAttribute,
    #[error("Only one attribute specification is allowed in the same pathspec")]
    MultipleAttributeSpecifications,
    #[error("'literal' and 'glob' keywords cannot be used together in the same pathspec")]
    IncompatibleSearchModes,
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
    let end = input.find(")").ok_or(Error::MissingClosingParenthesis)?;

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
    for window in input.windows(2) {
        i += 1;
        if window[0] != b'\\' && window[1] == split_char {
            keywords.push(&input[last..i]);
            last = i + 1;
        }
    }
    keywords.push(&input[last..]);
    keywords
}

fn parse_attributes(input: &[u8]) -> Result<Vec<git_attributes::Name>, Error> {
    if input.is_empty() {
        return Err(Error::EmptyAttribute);
    }

    let unescaped = unescape_attribute_values(input.into())?;

    git_attributes::parse::Iter::new(unescaped.as_bstr())
        .map(|res| res.map(|v| v.to_owned()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| Error::InvalidAttribute { attribute: e.attribute })
}

fn unescape_attribute_values(input: &BStr) -> Result<Cow<'_, BStr>, Error> {
    if !input.contains(&b'=') {
        return Ok(Cow::Borrowed(input));
    }

    let mut out: Cow<'_, BStr> = Cow::Borrowed("".into());

    for attr in input.split(|&c| c == b' ') {
        let split_point = attr.find_byte(b'=').map_or_else(|| attr.len(), |i| i + 1);
        let (name, value) = attr.split_at(split_point);

        if value.contains(&b'\\') {
            let out = out.to_mut();
            out.push_str(name);
            out.push_str(unescape_attribute_value(value.into())?);
            out.push(b' ');
        } else {
            let end = out.len() + attr.len() + 1;
            out = Cow::Borrowed(&input[0..end.min(input.len())])
        }
    }

    Ok(out)
}

fn unescape_attribute_value(value: &BStr) -> Result<BString, Error> {
    let mut out = BString::from(Vec::with_capacity(value.len()));
    let mut bytes = value.iter();
    while let Some(mut b) = bytes.next().copied() {
        if b == b'\\' {
            b = *bytes.next().ok_or(Error::TrailingEscapeCharacter)?;
        }
        if !b.is_ascii_alphanumeric() && !b",-_".contains(&b) {
            return Err(Error::InvalidAttributeValue { character: b as char });
        }

        out.push(b);
    }
    Ok(out)
}
