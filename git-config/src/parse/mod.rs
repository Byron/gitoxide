//! This module handles parsing a `git-config` file. Generally speaking, you
//! want to use a higher abstraction such as [`File`] unless you have some
//! explicit reason to work with events instead.
//!
//! The general workflow for interacting with this is to use one of the
//! `parse_from_*` function variants. These will return a [`Events`] on success,
//! which can be converted into an [`Event`] iterator. The [`Events`] also has
//! additional methods for accessing leading comments or events by section.
//!
//! [`File`]: crate::File

use bstr::BStr;
use std::{borrow::Cow, hash::Hash};

///
pub mod state;

/// Syntactic events that occurs in the config. Despite all these variants
/// holding a [`Cow`] instead over a simple reference, the parser will only emit
/// borrowed `Cow` variants.
///
/// The `Cow` smart pointer is used here for ease of inserting events in a
/// middle of an Event iterator. This is used, for example, in the [`File`]
/// struct when adding values.
///
/// [`Cow`]: std::borrow::Cow
/// [`File`]: crate::File
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Event<'a> {
    /// A comment with a comment tag and the comment itself. Note that the
    /// comment itself may contain additional whitespace and comment markers
    /// at the beginning.
    Comment(Comment<'a>),
    /// A section header containing the section name and a subsection, if it
    /// exists, like `remote "origin"`.
    SectionHeader(section::Header<'a>),
    /// A name to a value in a section, like `url` in `remote.origin.url`.
    SectionKey(section::Key<'a>),
    /// A completed value. This may be any string, including the empty string,
    /// if an implicit boolean value is used. Note that these values may contain
    /// spaces and any special character. This value is also unprocessed, so it
    /// it may contain double quotes that should be replaced.
    Value(Cow<'a, BStr>),
    /// Represents any token used to signify a new line character. On Unix
    /// platforms, this is typically just `\n`, but can be any valid newline
    /// sequence. Multiple newlines (such as `\n\n`) will be merged as a single
    /// newline event.
    Newline(Cow<'a, BStr>),
    /// Any value that isn't completed. This occurs when the value is continued
    /// onto the next line. A Newline event is guaranteed after, followed by
    /// either a ValueDone, a Whitespace, or another ValueNotDone.
    ValueNotDone(Cow<'a, BStr>),
    /// The last line of a value which was continued onto another line.
    ValueDone(Cow<'a, BStr>),
    /// A continuous section of insignificant whitespace. Values with internal
    /// spaces will not be separated by this event.
    Whitespace(Cow<'a, BStr>),
    /// This event is emitted when the parser counters a valid `=` character
    /// separating the key and value. This event is necessary as it eliminates
    /// the ambiguity for whitespace events between a key and value event.
    KeyValueSeparator,
}

///
pub mod event;
#[path = "events.rs"]
mod events_type;
pub use events_type::Events;

///
pub mod events {
    ///
    pub mod from_path {
        /// An error type representing a Parser [`Error`] or an [`IO error`]. This is
        /// returned from functions that will perform IO on top of standard parsing,
        /// such as reading from a file.
        ///
        /// [`IO error`]: std::io::Error
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            Parse(#[from] crate::parse::Error),
            #[error(transparent)]
            Io(#[from] std::io::Error),
        }
    }
}

/// A parsed section containing the header and the section events.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Section<'a> {
    /// The section name and subsection name, if any.
    pub section_header: section::Header<'a>,
    /// The syntactic events found in this section.
    pub events: Vec<Event<'a>>,
}

///
pub mod section;

/// A parsed comment event containing the comment marker and comment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Comment<'a> {
    /// The comment marker used. This is either a semicolon or octothorpe.
    pub comment_tag: u8,
    /// The parsed comment.
    pub comment: Cow<'a, BStr>,
}

mod comment;

/// A parser error reports the one-indexed line number where the parsing error
/// occurred, as well as the last parser node and the remaining data to be
/// parsed.
#[derive(PartialEq, Debug)]
pub struct Error {
    line_number: usize,
    last_attempted_parser: error::ParseNode,
    parsed_until: bstr::BString,
}

mod error;

mod nom;
pub use self::nom::from_bytes;

#[cfg(test)]
pub(crate) mod tests;
