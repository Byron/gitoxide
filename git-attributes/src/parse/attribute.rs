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
        }
    }
}
pub use error::Error;

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

pub struct Iter<'a> {
    attrs: bstr::Split<'a>,
}

impl<'a> Iter<'a> {
    pub fn new(attrs: &'a [u8]) -> Self {
        Iter {
            attrs: attrs.as_bstr().split_str(b" "),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<(&'a BStr, crate::State<'a>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let _attr = self.attrs.next().filter(|a| !a.is_empty())?;
        todo!("parse attribute")
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
            match parse_line(line) {
                None => continue,
                Some(res) => return Some(res.map(|(line, flags, attrs)| (line, flags, attrs, self.line_no))),
            }
        }
        None
    }
}

fn parse_line(line: &[u8]) -> Option<Result<(BString, crate::ignore::pattern::Mode, Iter<'_>), Error>> {
    if line.is_empty() {
        return None;
    }

    let (line, attrs): (Cow<'_, _>, _) = if line.starts_with(b"\"") {
        todo!("unquote, need length of consumed bytes to know where attrs start")
    } else {
        let mut tokens = line.splitn(2, |n| *n == b' ');
        (
            tokens.next().expect("at least a line").into(),
            tokens.next().unwrap_or_default(),
        )
    };

    let (pattern, flags) = super::ignore::parse_line(line.as_ref())?;
    Ok((pattern, flags, Iter::new(attrs))).into()
}
