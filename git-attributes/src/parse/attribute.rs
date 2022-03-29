use std::borrow::Cow;

use bstr::{BStr, BString, ByteSlice};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// A pattern to match paths against
    Pattern(BString, crate::ignore::pattern::Mode),
    /// The name of the macro to define, always a valid attribute name
    Macro(BString),
}

mod error {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            PatternNegation { line_number: usize, line: BString } {
                display("Line {} has a negative pattern, for literal characters use \\!: {}", line_number, line)
            }
            AttributeName { line_number: usize, attribute: BString } {
                display("Attribute in line {} has non-ascii characters or starts with '-': {}", line_number, attribute)
            }
            MacroName { line_number: usize, macro_name: BString } {
                display("Macro in line {} has non-ascii characters or starts with '-': {}", line_number, macro_name)
            }
            Unquote(err: git_quote::ansi_c::undo::Error) {
                display("Could not unquote attributes line")
                from()
                source(err)
            }
        }
    }
}
pub use error::Error;

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

pub struct Iter<'a> {
    attrs: bstr::Fields<'a>,
    line_no: usize,
}

impl<'a> Iter<'a> {
    pub fn new(attrs: &'a BStr, line_no: usize) -> Self {
        Iter {
            attrs: attrs.fields(),
            line_no,
        }
    }

    fn parse_attr(&self, attr: &'a [u8]) -> Result<(&'a BStr, crate::State<'a>), Error> {
        let mut tokens = attr.splitn(2, |b| *b == b'=');
        let attr = tokens.next().expect("attr itself").as_bstr();
        let possibly_value = tokens.next();
        let (attr, state) = if attr.first() == Some(&b'-') {
            (&attr[1..], crate::State::Unset)
        } else if attr.first() == Some(&b'!') {
            (&attr[1..], crate::State::Unspecified)
        } else {
            (
                attr,
                possibly_value
                    .map(|v| crate::State::Value(v.as_bstr()))
                    .unwrap_or(crate::State::Set),
            )
        };
        Ok((check_attr(attr, self.line_no)?, state))
    }
}

fn check_attr(attr: &BStr, line_number: usize) -> Result<&BStr, Error> {
    fn attr_valid(attr: &BStr) -> bool {
        if attr.first() == Some(&b'-') {
            return false;
        }

        attr.bytes().all(|b| {
            matches!(b, 
        b'-' | b'.' | b'_' | b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9')
        })
    }

    attr_valid(attr).then(|| attr).ok_or_else(|| Error::AttributeName {
        line_number,
        attribute: attr.into(),
    })
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<(&'a BStr, crate::State<'a>), Error>;

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
        Some(macro_name) => check_attr(macro_name.into(), line_number)
            .map(|m| Kind::Macro(m.into()))
            .map_err(|err| match err {
                Error::AttributeName { line_number, attribute } => Error::MacroName {
                    line_number,
                    macro_name: attribute,
                },
                _ => unreachable!("BUG: check_attr() must only return attribute errors"),
            }),
        None => {
            let (pattern, flags) = super::ignore::parse_line(line.as_ref())?;
            if flags.contains(crate::ignore::pattern::Mode::NEGATIVE) {
                Err(Error::PatternNegation {
                    line: line.into_owned(),
                    line_number,
                })
            } else {
                Ok(Kind::Pattern(pattern, flags))
            }
        }
    };
    let kind = match kind_res {
        Ok(kind) => kind,
        Err(err) => return Some(Err(err)),
    };
    Ok((kind, Iter::new(attrs, line_number), line_number)).into()
}

const BLANKS: &[u8] = b" \t\r";
