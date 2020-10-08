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
    pub fn as_section(&self) -> Option<&spanned::Section> {
        match self {
            Token::Section(v) => Some(v),
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
}

impl File {
    pub fn sections(&self) -> impl Iterator<Item = borrowed::Section<'_>> {
        self.tokens
            .iter()
            .enumerate()
            .filter_map(move |(index, t)| t.as_section().map(|_| borrowed::Section { parent: self, index }))
    }
}

impl<'a> borrowed::Section<'a> {
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

mod edit {
    use crate::{borrowed, file::File, owned, Span};

    impl Into<Edit> for owned::Section {
        fn into(self) -> Edit {
            Edit::SetSection(self)
        }
    }

    impl Into<Edit> for owned::Entry {
        fn into(self) -> Edit {
            Edit::SetEntry(self)
        }
    }

    enum Edit {
        Delete(Span), // section or entry
        SetSection(owned::Section),
        SetEntry(owned::Entry),
    }

    /// Collects edits to be applied to a [`File`], to be written out eventually.
    pub struct Edits<'a> {
        parent: &'a File,
        edits: Vec<Edit>,
    }

    impl<'a> Edits<'a> {
        pub fn delete_section(&mut self, section: &borrowed::Section<'_>) -> &mut Self {
            self.edits.push(Edit::Delete(
                self.parent.token(section.index).as_section().expect("section").name,
            ));
            self
        }
        pub fn delete_entry(&mut self, entry: &borrowed::Entry<'_>) -> &mut Self {
            self.edits.push(Edit::Delete(
                self.parent.token(entry.index).as_entry().expect("entry").name,
            ));
            self
        }
        // Use with [`owned::Section`].
        //
        // Newly instantiated sections will be appended, and existing ones can be edited
        // by calling [`borrowed::Section::edit()`].
        pub fn create_or_update_section(&mut self, section: owned::Section) -> &mut Self {
            self.edits.push(Edit::SetSection(section));
            self
        }
        pub fn create_or_update_entry(&mut self, entry: owned::Entry) -> &mut Self {
            self.edits.push(Edit::SetEntry(entry));
            self
        }
    }
}
pub use edit::Edits;
