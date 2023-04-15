use bstr::ByteSlice;

/// An iterator over line-wise ignore patterns parsed from a buffer.
pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

impl<'a> Lines<'a> {
    /// Create a new instance from `buf` to parse ignore patterns from.
    pub fn new(buf: &'a [u8]) -> Self {
        let bom = unicode_bom::Bom::from(buf);
        Lines {
            lines: buf[bom.len()..].lines(),
            line_no: 0,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = (gix_glob::Pattern, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            self.line_no += 1;
            if line.first() == Some(&b'#') {
                continue;
            }
            match gix_glob::Pattern::from_bytes(truncate_non_escaped_trailing_spaces(line)) {
                None => continue,
                Some(pattern) => return Some((pattern, self.line_no)),
            }
        }
        None
    }
}

/// We always copy just because that's ultimately needed anyway, not because we always have to.
fn truncate_non_escaped_trailing_spaces(buf: &[u8]) -> &[u8] {
    let mut last_space_pos = None;
    let mut bytes = buf.iter().enumerate();
    while let Some((pos, b)) = bytes.next() {
        match *b {
            b' ' => {
                last_space_pos.get_or_insert(pos);
                continue;
            }
            b'\\' => {
                if bytes.next().is_none() {
                    return buf;
                }
            }
            _ => {}
        }
        last_space_pos = None;
    }

    if let Some(pos) = last_space_pos {
        &buf[..pos]
    } else {
        buf
    }
}
