#![forbid(unsafe_code)]
// #![forbid(rust_2018_idioms)]

use std::ops::Range;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Span {
    pub start: usize,
    pub end_inclusive: usize,
}

impl From<Span> for Range<usize> {
    fn from(Span { start, end_inclusive }: Span) -> Self {
        Range {
            start,
            end: end_inclusive + 1,
        }
    }
}

impl Span {
    fn to_range(&self) -> Range<usize> {
        self.clone().into()
    }
}

mod file {
    use crate::Span;
    use bstr::{BStr, ByteSlice};

    pub struct File {
        buf: Vec<u8>,
    }

    impl File {
        pub(crate) fn bytes_at(&self, span: Span) -> &BStr {
            &self.buf[span.to_range()].as_bstr()
        }
    }
}

mod value {
    pub enum Color {
        Red,
        BrightRed,
        Ansi { r: u8, g: u8, c: u8 },
    }

    mod resolve {
        use bstr::BStr;
        use quick_error::quick_error;
        use std::path::PathBuf;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Tbd {
                    display("TBD")
                }
            }
        }
        pub fn path(_value: &BStr) -> Result<PathBuf, Error> {
            unimplemented!("path_resolve")
        }
    }
}

mod decode {
    use bstr::BStr;
    use quick_error::quick_error;
    use std::borrow::Cow;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("let's see what can go wrong and how we do it")
            }
        }
    }

    pub fn value(_input: &BStr) -> Result<Cow<'_, BStr>, Error> {
        unimplemented!("decode value from bstr")
    }
}

mod borrowed {
    use crate::{decode, file::File, value::Color, Span};
    use bstr::BStr;
    use quick_error::quick_error;
    use std::{borrow::Cow, path::PathBuf};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Decode(err: decode::Error) {
                display("A string value could not be decoded")
                from()
                source(err)
            }
        }
    }

    struct Entry<'a> {
        parent: &'a File,
        section: Section,
        name: Span,
        value: Span,
    }

    impl<'a> Entry<'a> {
        pub fn as_string(&self) -> Result<Cow<'a, BStr>, Error> {
            decode::value(self.parent.bytes_at(self.value)).map_err(Into::into)
        }
        pub fn as_int(&self) -> Result<i64, Error> {
            unimplemented!("as int")
        }
        pub fn as_bool(&self) -> Result<bool, Error> {
            unimplemented!("as bool")
        }
        pub fn as_path(&self) -> Result<PathBuf, Error> {
            unimplemented!("as bool")
        }
        pub fn as_color(&self) -> Result<Color, Error> {
            unimplemented!("as bool")
        }
    }

    enum Section {
        Level1 { name: Span },
        Level2 { parent_name: Span, name: Span },
    }
}
