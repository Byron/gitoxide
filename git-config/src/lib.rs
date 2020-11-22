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

pub mod file;
pub use file::File;

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

mod owned {
    use crate::Span;
    use bstr::BString;

    pub struct Entry {
        pub name: BString,
        pub value: Option<BString>,
        pub(crate) span: Option<Span>,
    }

    pub struct Section {
        pub name: BString,
        pub sub_name: Option<BString>,
        pub entries: Vec<Entry>,
        pub(crate) span: Option<Span>,
    }

    impl Entry {
        pub fn new(name: BString, value: Option<BString>) -> Self {
            Entry {
                name,
                value,
                span: None,
            }
        }

        pub fn name(mut self, name: impl Into<BString>) -> Self {
            self.name = name.into();
            self
        }
        pub fn value(mut self, name: Option<BString>) -> Self {
            self.value = name;
            self
        }
    }

    impl Section {
        pub fn new(name: BString, sub_name: Option<BString>, entries: Vec<Entry>) -> Self {
            Section {
                name,
                sub_name,
                entries,
                span: None,
            }
        }

        pub fn name(mut self, name: impl Into<BString>) -> Self {
            self.name = name.into();
            self
        }
        pub fn sub_name(mut self, sub_name: Option<BString>) -> Self {
            self.sub_name = sub_name;
            self
        }
    }
}

mod borrowed {
    use crate::{file::File, owned};

    pub struct Entry<'a> {
        pub(crate) parent: &'a File,
        pub(crate) index: usize,
    }

    impl<'a> Entry<'a> {
        pub fn to_editable(&self) -> owned::Entry {
            let entry = self.parent.token(self.index).as_entry().expect("entry");
            owned::Entry {
                name: self.parent.bytes_at(entry.name).into(),
                value: entry.value.map(|span| self.parent.bytes_at(span).into()),
                span: Some(entry.name),
            }
        }
    }

    pub struct Section<'a> {
        pub(crate) parent: &'a File,
        pub(crate) index: usize,
    }

    impl<'a> Section<'a> {
        pub fn to_editable(&self) -> owned::Section {
            let section = self.parent.token(self.index).as_section().expect("section");
            owned::Section {
                name: self.parent.bytes_at(section.name).into(),
                sub_name: section.sub_name.map(|span| self.parent.bytes_at(span).into()),
                span: Some(section.name),
                entries: self.entries().map(|e| e.to_editable()).collect(),
            }
        }
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
                        .ok_or(Error::NoValue)?,
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
