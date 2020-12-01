use crate::{borrowed, spanned, Span};
use bstr::{BStr, ByteSlice};

#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
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
    pub fn as_section(&self) -> Option<&spanned::Section> {
        match self {
            Token::Section(v) => Some(v),
            _ => None,
        }
    }
}

/// The entry point into reading and writing git config files.
///
/// After reading a configuration file its contents is stored verbatim and indexed to allow retrieval
/// of sections and entry values on demand. These are returned as [`borrowed`] items, which are read-only but
/// can be transformed into editable items.
#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct File {
    buf: Vec<u8>,
    /// A config file as parsed into tokens, where each [`Token`] is one of the three relevant items in git config files.
    tokens: Vec<Token>, // but how do we get fast lookups and proper value lookup based on decoded values?
                        // On the fly is easier, otherwise we have to deal with a lookup cache of sorts and
                        // many more allocations up front (which might be worth it only once we have measurements).
                        // Cow<'a, _> would bind to our buffer so the cache can't be in this type.
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
}

impl File {
    /// Returns an iterator over all sections and sub-sections of the configuration file.
    ///
    /// Note that every entry must be part of a section, that is global entries/key-value pairs are not allowed.
    pub fn sections(&self) -> impl Iterator<Item = borrowed::Section<'_>> {
        self.tokens
            .iter()
            .enumerate()
            .filter_map(move |(index, t)| t.as_section().map(|_| borrowed::Section { parent: self, index }))
    }
}

impl<'a> borrowed::Section<'a> {
    /// Returns an iterator over all entries in a section.
    pub fn entries(&self) -> impl Iterator<Item = borrowed::Entry<'_>> {
        struct Iter<'a> {
            inner: Option<&'a [Token]>,
            parent: &'a File,
            index: usize,
            offset: usize,
        }
        impl<'a> Iterator for Iter<'a> {
            type Item = borrowed::Entry<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.inner.as_ref() {
                    Some(s) => {
                        let r = loop {
                            break match s.get(self.index) {
                                None | Some(Token::Section(_)) => {
                                    self.inner = None;
                                    None
                                }
                                Some(Token::Entry(_)) => Some(borrowed::Entry {
                                    parent: self.parent,
                                    index: self.index + self.offset,
                                }),
                                Some(Token::Comment(_)) => continue,
                            };
                        };
                        self.index += 1;
                        r
                    }
                    None => None,
                }
            }
        }
        let start_of_entries = self.index + 1;
        Iter {
            inner: self.parent.tokens.get(start_of_entries..),
            parent: self.parent,
            index: 0,
            offset: start_of_entries,
        }
    }
}

mod edit;
pub use edit::Edits;
