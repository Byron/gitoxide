#![forbid(unsafe_code, rust_2018_idioms)]

pub mod ignore {
    pub mod pattern {
        use bitflags::bitflags;

        bitflags! {
            pub struct Mode: u32 {
                const NO_DIR = 1 << 0;
                // TODO: find a much better name!
                const ENDS_WITH = 1 << 1;
                const MUST_BE_DIR = 1 << 2;
                const NEGATIVE = 1 << 3;
            }
        }
    }
}

pub mod parse {
    pub mod ignore {
        use crate::ignore;
        use bstr::{BStr, BString, ByteSlice};

        pub struct Iter<'a> {
            cursor: &'a BStr,
        }

        impl<'a> Iter<'a> {
            pub fn new(buf: &'a [u8]) -> Self {
                Iter { cursor: buf.as_bstr() }
            }
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = (BString, ignore::pattern::Mode);

            fn next(&mut self) -> Option<Self::Item> {
                if self.cursor.is_empty() {
                    return None;
                }
                let mut lines = self.cursor.lines();
                let res = None;
                while let Some(line) = lines.next() {
                    if line.starts_with(b"#") {
                        continue;
                    }
                    todo!("handle escapes and trim trailing non-escaped whitespace")
                }
                if let Some(next_line) = lines.next() {
                    self.cursor = next_line.as_bstr();
                }
                res
            }
        }
    }
    pub fn ignore(buf: &[u8]) -> ignore::Iter<'_> {
        ignore::Iter::new(buf)
    }
}
