use bstr::{BString, ByteSlice};

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
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
    type Item = (BString, git_glob::pattern::Mode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            self.line_no += 1;
            if line.first() == Some(&b'#') {
                continue;
            }
            match git_glob::parse(line) {
                None => continue,
                Some((line, flags)) => return Some((line, flags, self.line_no)),
            }
        }
        None
    }
}
