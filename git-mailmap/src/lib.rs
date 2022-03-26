#![forbid(unsafe_code, rust_2018_idioms)]

pub mod parse {
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
    pub struct Lines<'a> {
        lines: bstr::Lines<'a>,
        line_no: usize,
    }
}
