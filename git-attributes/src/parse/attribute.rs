use bstr::{BStr, BString, ByteSlice};

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

pub struct Iter<'a> {
    _attrs: bstr::Split<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a BStr, crate::State<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
    type Item = (BString, crate::ignore::pattern::Mode, Iter<'a>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            self.line_no += 1;
            if line.is_empty() || line.first() == Some(&b'#') {
                continue;
            }
            todo!("parse line")
        }
        None
    }
}
