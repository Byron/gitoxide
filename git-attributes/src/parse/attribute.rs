use crate::{Name, NameRef, State, StateRef, name};
use bstr::{BStr, BString, ByteSlice};
use std::borrow::Cow;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A pattern to match paths against
    Pattern(git_glob::Pattern),
    /// The name of the macro to define, always a valid attribute name
    // TODO: turn it into its own type for maximum safety
    Macro(BString),
}

mod error {
    use bstr::BString;
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Line {line_number} has a negative pattern, for literal characters use \\!: {line}")]
        PatternNegation { line_number: usize, line: BString },
        #[error("Attribute in line {line_number} has non-ascii characters or starts with '-': {attribute}")]
        AttributeName { line_number: usize, attribute: BString },
        #[error("Macro in line {line_number} has non-ascii characters or starts with '-': {macro_name}")]
        MacroName { line_number: usize, macro_name: BString },
        #[error("Could not unquote attributes line")]
        Unquote(#[from] git_quote::ansi_c::undo::Error),
    }
}
pub use error::Error;

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

pub struct Iter<'a> {
    attrs: bstr::Fields<'a>
}

impl<'a> Iter<'a> {
    pub fn new(attrs: &'a BStr) -> Self {
        Iter {
            attrs: attrs.fields()
        }
    }

    fn parse_attr(&self, attr: &'a [u8]) -> Result<NameRef<'a>, name::Error> {
        let mut tokens = attr.splitn(2, |b| *b == b'=');
        let attr = tokens.next().expect("attr itself").as_bstr();
        let possibly_value = tokens.next();
        let (attr, state) = if attr.first() == Some(&b'-') {
            (&attr[1..], StateRef::Unset)
        } else if attr.first() == Some(&b'!') {
            (&attr[1..], StateRef::Unspecified)
        } else {
            (
                attr,
                possibly_value
                    .map(|v| crate::StateRef::Value(v.as_bstr()))
                    .unwrap_or(crate::StateRef::Set),
            )
        };
        Ok(NameRef(check_attr(attr)?, state))
    }
}

fn check_attr(attr: &BStr) -> Result<&BStr, name::Error> {
    fn attr_valid(attr: &BStr) -> bool {
        if attr.first() == Some(&b'-') {
            return false;
        }

        attr.bytes().all(|b| {
            matches!(b, 
        b'-' | b'.' | b'_' | b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9')
        })
    }

    attr_valid(attr)
        .then(|| attr)
        .ok_or_else(|| name::Error { attribute: attr.into() })
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<NameRef<'a>, name::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let attr = self.attrs.next().filter(|a| !a.is_empty())?;
        self.parse_attr(attr).into()
    }
}

impl<'a> Lines<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        let bom = unicode_bom::Bom::from(buf);
        Lines {
            lines: buf[bom.len()..].lines(),
            line_no: 0,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = Result<(Kind, Iter<'a>, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        fn skip_blanks(line: &BStr) -> &BStr {
            line.find_not_byteset(BLANKS).map(|pos| &line[pos..]).unwrap_or(line)
        }
        for line in self.lines.by_ref() {
            self.line_no += 1;
            let line = skip_blanks(line.into());
            if line.first() == Some(&b'#') {
                continue;
            }
            match parse_line(line, self.line_no) {
                None => continue,
                Some(res) => return Some(res),
            }
        }
        None
    }
}

fn parse_line(line: &BStr, line_number: usize) -> Option<Result<(Kind, Iter<'_>, usize), Error>> {
    if line.is_empty() {
        return None;
    }

    let (line, attrs): (Cow<'_, _>, _) = if line.starts_with(b"\"") {
        let (unquoted, consumed) = match git_quote::ansi_c::undo(line) {
            Ok(res) => res,
            Err(err) => return Some(Err(err.into())),
        };
        (unquoted, &line[consumed..])
    } else {
        line.find_byteset(BLANKS)
            .map(|pos| (line[..pos].as_bstr().into(), line[pos..].as_bstr()))
            .unwrap_or((line.into(), [].as_bstr()))
    };

    let kind_res = match line.strip_prefix(b"[attr]") {
        Some(macro_name) => check_attr(macro_name.into())
            .map(|m| Kind::Macro(m.into()))
            .map_err(|err| Error::MacroName {
                line_number,
                macro_name: err.attribute,
            }),
        None => {
            let pattern = git_glob::Pattern::from_bytes(line.as_ref())?;
            if pattern.mode.contains(git_glob::pattern::Mode::NEGATIVE) {
                Err(Error::PatternNegation {
                    line: line.into_owned(),
                    line_number,
                })
            } else {
                Ok(Kind::Pattern(pattern))
            }
        }
    };
    let kind = match kind_res {
        Ok(kind) => kind,
        Err(err) => return Some(Err(err)),
    };
    Ok((kind, Iter::new(attrs), line_number)).into()
}

const BLANKS: &[u8] = b" \t\r";

impl<'a> NameRef<'a> {
    pub fn name(&self) -> &'a BStr {
        self.0
    }

    pub fn state(&self) -> StateRef<'a> {
        self.1.clone()
    }
}

impl<'a> From<NameRef<'a>> for Name {
    fn from(v: NameRef<'a>) -> Self {
        Name(v.0.to_owned(), v.1.into())
    }
}

impl Name {
    pub fn name(&self) -> &BString {
        &self.0
    }

    pub fn state(&self) -> &State {
        &self.1
    }
}