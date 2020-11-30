#![forbid(unsafe_code)]
// #![forbid(rust_2018_idioms)]
#![allow(dead_code)]

use std::ops::Range;

/// A span is a range into a set of bytes - see it as a selection into a Git config file.
///
/// Similar to [`std::ops::RangeInclusive`], but tailor made to work for us.
/// There are various issues with std ranges, which we don't have to opt into for the simple Range-like item we need.
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

/// A module with specialized value types as they exist within git config files.
pub mod value {
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
        /// Git-config paths can contain `~` and more, see [the git documentation](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/Documentation/config.txt#L295:L295)
        /// on what needs to be supported.
        pub fn path(_value: &BStr) -> Result<PathBuf, Error> {
            unimplemented!("resolve::path")
        }
    }
}

/// Spanned items refer to their content using [`Span`]s, thus they act like a pointer into a byte buffer representing the config file.
///
/// These are inherently read-only, as they do not own any data but rather point to a buffer they don't even know.
mod spanned {
    use crate::Span;
    // we parse leading and trailing whitespace into comments, avoiding the notion of whitespace.
    // This means we auto-trim whitespace otherwise, which I consider a feature
    pub(crate) type Comment = Span;

    /// A section or sub-section (in case `sub_name` is `Some()`), i.e.
    ///
    /// ```text
    /// [hello]
    ///
    /// [hello.world]
    /// ```
    pub(crate) struct Section {
        pub(crate) name: Span,
        pub(crate) sub_name: Option<Span>,
    }

    /// A key-value entry of a git-config file, like `name = value`
    pub(crate) struct Entry {
        pub(crate) name: Span,
        pub(crate) value: Option<Span>,
    }
}

/// Owned versions of what can be found in `spanned`, which allows these items to be altered.
///
/// All of these will *may* remember their originating `span` as `Some(…)`, which is the entire region in the config file they point to. This is important
/// in case of updates. New owned items thus don't have a `span`, represented by `None`.
mod owned {
    use crate::Span;
    use bstr::BString;

    /// A key-value entry of a git-config file, like `name = value`
    pub struct Entry {
        pub name: BString,
        pub value: Option<BString>,
        pub(crate) span: Option<Span>,
    }

    /// A section or sub-section (in case `sub_name` is `Some()`), with all their entries.
    ///
    /// For example
    /// ```text
    /// [hello]
    /// a = 2
    ///
    /// [hello.world]
    /// b = c
    /// x = y
    /// ```
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

/// Borrowed items are nothing more than a fancy 'handle' to an item stored in a file, which can be made editable to make updates.
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

    /// Decode an entry value - it can be [encoded as described in the git config documentation](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/Documentation/config.txt#L74:L80)
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
