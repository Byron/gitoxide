use bstr::{BStr, BString, ByteSlice};
use std::borrow::Cow;

mod error {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            PatternNegation { line_number: usize, line: BString } {
                display("Line {} has a negative pattern, for literal characters use \\!: {}", line_number, line)
            }
            Unquote(err: git_quote::ansi_c::undo::Error) {
                display("Could not unquote attributes line")
                from()
                source(err)
            }
        }
    }
}
use crate::ignore;
pub use error::Error;

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

pub struct Iter<'a> {
    attrs: bstr::Fields<'a>,
}

impl<'a> Iter<'a> {
    pub fn new(attrs: &'a BStr) -> Self {
        Iter { attrs: attrs.fields() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<(&'a BStr, crate::State<'a>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let attr = self.attrs.next().filter(|a| !a.is_empty())?;
        Some(Ok((attr.as_bstr(), crate::State::Set)))
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
    type Item = Result<(BString, crate::ignore::pattern::Mode, Iter<'a>, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            self.line_no += 1;
            let line = skip_blanks(line.into());
            if line.first() == Some(&b'#') {
                continue;
            }
            match parse_line(line) {
                None => continue,
                Some(Ok((pattern, flags, attrs))) => {
                    return Some(if flags.contains(ignore::pattern::Mode::NEGATIVE) {
                        Err(Error::PatternNegation {
                            line: line.into(),
                            line_number: self.line_no,
                        })
                    } else {
                        Ok((pattern, flags, attrs, self.line_no))
                    })
                }
                Some(Err(err)) => return Some(Err(err)),
            }
        }
        None
    }
}

fn parse_line(line: &BStr) -> Option<Result<(BString, crate::ignore::pattern::Mode, Iter<'_>), Error>> {
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

    let (pattern, flags) = super::ignore::parse_line(line.as_ref())?;
    Ok((pattern, flags, Iter::new(attrs))).into()
}

const BLANKS: &[u8] = b" \t\r";

fn skip_blanks(line: &BStr) -> &BStr {
    line.find_not_byteset(BLANKS).map(|pos| &line[pos..]).unwrap_or(line)
}
