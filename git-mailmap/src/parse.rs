mod error {
    use bstr::BString;
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            UnconsumedInput { line_number: usize, line: BString } {
                display("Line {} has too many names or emails: {}", line_number, line)
            }
            Malformed { line_number: usize, line: BString } {
                display("Line {} is malformed, an email address lacks the closing '>' bracket: {}", line_number, line)
            }
        }
    }
}

use crate::Entry;
use bstr::ByteSlice;
pub use error::Error;

pub struct Lines<'a> {
    lines: bstr::Lines<'a>,
    line_no: usize,
}

impl<'a> Lines<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Lines {
            lines: input.as_bstr().lines(),
            line_no: 0,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = Result<Entry<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            self.line_no += 1;
            match line.first() {
                None => continue,
                Some(b) if *b == b'#' => continue,
                Some(_) => {}
            }
        }
        None
    }
}
