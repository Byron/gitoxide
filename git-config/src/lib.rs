#![forbid(unsafe_code)]
// #![forbid(rust_2018_idioms)]
#![allow(dead_code)]

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
    use crate::{borrowed, spanned, Span};
    use bstr::{BStr, ByteSlice};

    pub(crate) enum Token {
        Section(spanned::Section),
        Entry(spanned::Entry),
        Comment(spanned::Comment),
    }

    impl Token {
        pub fn as_entry(&self) -> Option<&spanned::Entry> {
            match self {
                Token::Entry(v) => Some(v),
                _ => None,
            }
        }
    }

    pub struct File {
        buf: Vec<u8>,
        tokens: Vec<Token>, // but how do we get fast lookups and proper value lookup based on decoded values?
                            // On the fly is easier, otherwise we have to deal with a lookup cache of sorts and
                            // many more allocations up front (which might be worth it). Cow<'a, _> would bind to
                            // our buffer so the cache can't be in this type.
                            // Probably it could be the 'Config' type which handles multiple files and treats them as one,
                            // and only if there is any need.
    }

    impl File {
        pub(crate) fn bytes_at(&self, span: Span) -> &BStr {
            &self.buf[span.to_range()].as_bstr()
        }

        pub(crate) fn token(&self, index: usize) -> &Token {
            &self.tokens[index]
        }
        // access for sections
    }

    impl<'a> borrowed::Section<'a> {
        // Access for entries
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

mod spanned {
    use crate::Span;
    // we parse leading and trailing whitespace into comments, avoiding the notion of whitespace.
    // This means we auto-trim whitespace otherwise, which I consider a feature
    pub(crate) type Comment = Span;

    pub(crate) struct Section {
        pub(crate) name: Span,
        pub(crate) sub_name: Option<Span>,
    }

    pub(crate) struct Entry {
        pub(crate) name: Span,
        pub(crate) value: Option<Span>,
    }
}

mod borrowed {
    use crate::file::File;

    pub struct Entry<'a> {
        pub(crate) parent: &'a File,
        pub(crate) index: usize,
    }

    pub struct Section<'a> {
        parent: &'a File,
        index: usize,
    }
}

mod decode {
    use crate::{borrowed, value};
    use bstr::BStr;
    use quick_error::quick_error;
    use std::{borrow::Cow, path::PathBuf};

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("let's see what can go wrong and how we do it")
            }
            NoValue {
                display("Entry has no value (TODO: much more error information)")
            }
        }
    }

    pub fn value(_input: &BStr) -> Result<Cow<'_, BStr>, Error> {
        unimplemented!("decode value from bstr")
    }

    impl<'a> borrowed::Entry<'a> {
        pub fn as_string(&self) -> Result<Cow<'a, BStr>, Error> {
            value(
                self.parent.bytes_at(
                    self.parent
                        .token(self.index)
                        .as_entry()
                        .expect("entry")
                        .value
                        .ok_or_else(|| Error::NoValue)?,
                ),
            )
            .map_err(Into::into)
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
        pub fn as_color(&self) -> Result<value::Color, Error> {
            unimplemented!("as bool")
        }
    }
}
