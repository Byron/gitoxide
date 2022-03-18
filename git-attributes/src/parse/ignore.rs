use crate::ignore;
use bstr::{BStr, BString, ByteSlice};

pub struct Iter<'a> {
    cursor: &'a BStr,
    line_no: usize,
}

impl<'a> Iter<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Iter {
            cursor: buf.as_bstr(),
            line_no: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (BString, ignore::pattern::Mode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }
        let mut lines = self.cursor.lines_with_terminator();
        let mut res = None;
        let mut offset = 0;
        for mut line in lines.by_ref() {
            self.line_no += 1;
            offset += line.len();
            line = trim_newline(line);
            let mut mode = ignore::pattern::Mode::empty();
            if line.is_empty() {
                continue;
            };
            if line.first() == Some(&b'#') {
                continue;
            } else if line.first() == Some(&b'!') {
                mode |= ignore::pattern::Mode::NEGATIVE;
                line = &line[1..];
            } else if line.first() == Some(&b'\\') {
                let second = line.get(1);
                if second == Some(&b'!') || second == Some(&b'#') {
                    line = &line[1..];
                }
            }
            let mut line = truncate_non_escaped_trailing_spaces(line);
            if line.last() == Some(&b'/') {
                mode |= ignore::pattern::Mode::MUST_BE_DIR;
                line.pop();
            }
            if !line.contains(&b'/') {
                mode |= ignore::pattern::Mode::NO_SUB_DIR;
            }
            res = Some((line, mode, self.line_no));
            break;
        }
        self.cursor = &self.cursor[offset..];
        res
    }
}

#[inline]
fn trim_newline(mut line: &[u8]) -> &[u8] {
    if line.last_byte() == Some(b'\n') {
        line = &line[..line.len() - 1];
        if line.last_byte() == Some(b'\r') {
            line = &line[..line.len() - 1];
        }
    }
    line
}

/// We always copy just because that's ultimately needed anyway, not because we always have to.
fn truncate_non_escaped_trailing_spaces(buf: &[u8]) -> BString {
    match buf.rfind_not_byteset(br"\ ") {
        Some(pos) if pos + 1 == buf.len() => buf.into(), // does not end in (escaped) whitespace
        None => buf.into(),
        Some(start_of_non_space) => {
            // This seems a bit strange but attempts to recreate the git implementation while
            // actually removing the escape characters before spaces. We leave other backslashes
            // for escapes to be handled by `glob/globset`.
            let mut res: BString = buf[..start_of_non_space + 1].into();

            let mut trailing_bytes = buf[start_of_non_space + 1..].iter();
            let mut bare_spaces = 0;
            while let Some(b) = trailing_bytes.next() {
                match b {
                    b' ' => {
                        bare_spaces += 1;
                    }
                    b'\\' => {
                        res.extend(std::iter::repeat(b' ').take(bare_spaces));
                        bare_spaces = 0;
                        // Skip what follows, like git does, but keep spaces if possible.
                        if trailing_bytes.next() == Some(&b' ') {
                            res.push(b' ');
                        }
                    }
                    _ => unreachable!("BUG: this must be either backslash or space"),
                }
            }
            res
        }
    }
}
