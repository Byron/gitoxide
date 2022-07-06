//! This module handles parsing a `git-config` file. Generally speaking, you
//! want to use a higher abstraction such as [`File`] unless you have some
//! explicit reason to work with events instead.
//!
//! The general workflow for interacting with this is to use one of the
//! `parse_from_*` function variants. These will return a [`Parser`] on success,
//! which can be converted into an [`Event`] iterator. The [`Parser`] also has
//! additional methods for accessing leading comments or events by section.
//!
//! [`File`]: crate::File

use bstr::BStr;
use std::{borrow::Cow, hash::Hash};

/// A zero-copy `git-config` file parser.
///
/// This is parser exposes low-level syntactic events from a `git-config` file.
/// Generally speaking, you'll want to use [`File`] as it wraps
/// around the parser to provide a higher-level abstraction to a `git-config`
/// file, including querying, modifying, and updating values.
///
/// This parser guarantees that the events emitted are sufficient to
/// reconstruct a `git-config` file identical to the source `git-config`.
///
/// # Differences between a `.ini` parser
///
/// While the `git-config` format closely resembles the [`.ini` file format],
/// there are subtle differences that make them incompatible. For one, the file
/// format is not well defined, and there exists no formal specification to
/// adhere to. Thus, attempting to use an `.ini` parser on a `git-config` file
/// may successfully parse invalid configuration files.
///
/// For concrete examples, some notable differences are:
/// - `git-config` sections permit subsections via either a quoted string
/// (`[some-section "subsection"]`) or via the deprecated dot notation
/// (`[some-section.subsection]`). Successful parsing these section names is not
/// well defined in typical `.ini` parsers. This parser will handle these cases
/// perfectly.
/// - Comment markers are not strictly defined either. This parser will always
/// and only handle a semicolon or octothorpe (also known as a hash or number
/// sign).
/// - Global properties may be allowed in `.ini` parsers, but is strictly
/// disallowed by this parser.
/// - Only `\t`, `\n`, `\b` `\\` are valid escape characters.
/// - Quoted and semi-quoted values will be parsed (but quotes will be included
/// in event outputs). An example of a semi-quoted value is `5"hello world"`,
/// which should be interpreted as `5hello world`.
/// - Line continuations via a `\` character is supported.
/// - Whitespace handling similarly follows the `git-config` specification as
/// closely as possible, where excess whitespace after a non-quoted value are
/// trimmed, and line continuations onto a new line with excess spaces are kept.
/// - Only equal signs (optionally padded by spaces) are valid name/value
/// delimiters.
///
/// Note that that things such as case-sensitivity or duplicate sections are
/// _not_ handled. This parser is a low level _syntactic_ interpreter (as a
/// parser should be), and higher level wrappers around this parser (which may
/// or may not be zero-copy) should handle _semantic_ values. This also means
/// that string-like values are not interpreted. For example, `hello"world"`
/// would be read at a high level as `helloworld` but this parser will return
/// the former instead, with the extra quotes. This is because it is not the
/// responsibility of the parser to interpret these values, and doing so would
/// necessarily require a copy, which this parser avoids.
///
/// # Trait Implementations
///
/// - This struct does _not_ implement [`FromStr`] due to lifetime
/// constraints implied on the required `from_str` method. Instead, it provides
/// [`From<&'_ str>`].
///
/// # Idioms
///
/// If you do want to use this parser, there are some idioms that may help you
/// with interpreting sequences of events.
///
/// ## `Value` events do not immediately follow `Key` events
///
/// Consider the following `git-config` example:
///
/// ```text
/// [core]
///   autocrlf = input
/// ```
///
/// Because this parser guarantees perfect reconstruction, there are many
/// non-significant events that occur in addition to the ones you may expect:
///
/// ```
/// # use git_config::parser::{Event, ParsedSectionHeader, parse_from_str, SectionHeaderName, Key};
/// # use std::borrow::Cow;
/// # let section_header = ParsedSectionHeader {
/// #   name: SectionHeaderName(Cow::Borrowed("core".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\n  autocrlf = input";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Whitespace(Cow::Borrowed("  ".into())),
/// Event::Key(Key(Cow::Borrowed("autocrlf".into()))),
/// Event::Whitespace(Cow::Borrowed(" ".into())),
/// Event::KeyValueSeparator,
/// Event::Whitespace(Cow::Borrowed(" ".into())),
/// Event::Value(Cow::Borrowed("input".into())),
/// # ]);
/// ```
///
/// Note the two whitespace events between the key and value pair! Those two
/// events actually refer to the whitespace between the name and value and the
/// equal sign. So if the config instead had `autocrlf=input`, those whitespace
/// events would no longer be present.
///
/// ## `KeyValueSeparator` event is not guaranteed to emit
///
/// Consider the following `git-config` example:
///
/// ```text
/// [core]
///   autocrlf
/// ```
///
/// This is a valid config with a `autocrlf` key having an implicit `true`
/// value. This means that there is not a `=` separating the key and value,
/// which means that the corresponding event won't appear either:
///
/// ```
/// # use git_config::parser::{Event, ParsedSectionHeader, parse_from_str, SectionHeaderName, Key};
/// # use std::borrow::Cow;
/// # let section_header = ParsedSectionHeader {
/// #   name: SectionHeaderName(Cow::Borrowed("core".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\n  autocrlf";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Whitespace(Cow::Borrowed("  ".into())),
/// Event::Key(Key(Cow::Borrowed("autocrlf".into()))),
/// Event::Value(Cow::Borrowed("".into())),
/// # ]);
/// ```
///
/// ## Quoted values are not unquoted
///
/// Consider the following `git-config` example:
///
/// ```text
/// [core]
/// autocrlf=true""
/// filemode=fa"lse"
/// ```
///
/// Both these events, when fully processed, should normally be `true` and
/// `false`. However, because this parser is zero-copy, we cannot process
/// partially quoted values, such as the `false` example. As a result, to
/// maintain consistency, the parser will just take all values as literals. The
/// relevant event stream emitted is thus emitted as:
///
/// ```
/// # use git_config::parser::{Event, ParsedSectionHeader, parse_from_str, SectionHeaderName, Key};
/// # use std::borrow::Cow;
/// # let section_header = ParsedSectionHeader {
/// #   name: SectionHeaderName(Cow::Borrowed("core".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\nautocrlf=true\"\"\nfilemode=fa\"lse\"";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Key(Key(Cow::Borrowed("autocrlf".into()))),
/// Event::KeyValueSeparator,
/// Event::Value(Cow::Borrowed(r#"true"""#.into())),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Key(Key(Cow::Borrowed("filemode".into()))),
/// Event::KeyValueSeparator,
/// Event::Value(Cow::Borrowed(r#"fa"lse""#.into())),
/// # ]);
/// ```
///
/// ## Whitespace after line continuations are part of the value
///
/// Consider the following `git-config` example:
///
/// ```text
/// [some-section]
/// file=a\
///     c
/// ```
///
/// Because how `git-config` treats continuations, the whitespace preceding `c`
/// are in fact part of the value of `file`. The fully interpreted key/value
/// pair is actually `file=a    c`. As a result, the parser will provide this
/// split value accordingly:
///
/// ```
/// # use git_config::parser::{Event, ParsedSectionHeader, parse_from_str, SectionHeaderName, Key};
/// # use std::borrow::Cow;
/// # let section_header = ParsedSectionHeader {
/// #   name: SectionHeaderName(Cow::Borrowed("some-section".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[some-section]\nfile=a\\\n    c";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Key(Key(Cow::Borrowed("file".into()))),
/// Event::KeyValueSeparator,
/// Event::ValueNotDone(Cow::Borrowed("a".into())),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::ValueDone(Cow::Borrowed("    c".into())),
/// # ]);
/// ```
///
/// [`File`]: crate::File
/// [`.ini` file format]: https://en.wikipedia.org/wiki/INI_file
/// [`git`'s documentation]: https://git-scm.com/docs/git-config#_configuration_file
/// [`FromStr`]: std::str::FromStr
/// [`From<&'_ str>`]: std::convert::From
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Parser<'a> {
    frontmatter: Vec<Event<'a>>,
    sections: Vec<ParsedSection<'a>>,
}

mod state {
    use crate::parser::{parse_from_bytes, parse_from_str, Error, Event, ParsedSection, Parser};
    use std::convert::TryFrom;

    impl<'a> Parser<'a> {
        /// Returns the leading events (any comments, whitespace, or newlines before
        /// a section) from the parser. Consider [`Parser::take_frontmatter`] if
        /// you need an owned copy only once. If that function was called, then this
        /// will always return an empty slice.
        #[must_use]
        pub fn frontmatter(&self) -> &[Event<'a>] {
            &self.frontmatter
        }

        /// Takes the leading events (any comments, whitespace, or newlines before
        /// a section) from the parser. Subsequent calls will return an empty vec.
        /// Consider [`Parser::frontmatter`] if you only need a reference to the
        /// frontmatter
        pub fn take_frontmatter(&mut self) -> Vec<Event<'a>> {
            std::mem::take(&mut self.frontmatter)
        }

        /// Returns the parsed sections from the parser. Consider
        /// [`Parser::take_sections`] if you need an owned copy only once. If that
        /// function was called, then this will always return an empty slice.
        #[must_use]
        pub fn sections(&self) -> &[ParsedSection<'a>] {
            &self.sections
        }

        /// Takes the parsed sections from the parser. Subsequent calls will return
        /// an empty vec. Consider [`Parser::sections`] if you only need a reference
        /// to the comments.
        pub fn take_sections(&mut self) -> Vec<ParsedSection<'a>> {
            let mut to_return = vec![];
            std::mem::swap(&mut self.sections, &mut to_return);
            to_return
        }

        /// Consumes the parser to produce a Vec of Events.
        #[must_use]
        pub fn into_vec(self) -> Vec<Event<'a>> {
            self.into_iter().collect()
        }

        /// Consumes the parser to produce an iterator of Events.
        #[must_use = "iterators are lazy and do nothing unless consumed"]
        #[allow(clippy::should_implement_trait)]
        pub fn into_iter(self) -> impl Iterator<Item = Event<'a>> + std::iter::FusedIterator {
            self.frontmatter
                .into_iter()
                .chain(self.sections.into_iter().flat_map(|section| {
                    std::iter::once(Event::SectionHeader(section.section_header)).chain(section.events)
                }))
        }
    }

    impl<'a> TryFrom<&'a str> for Parser<'a> {
        type Error = Error<'a>;

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            parse_from_str(value)
        }
    }

    impl<'a> TryFrom<&'a [u8]> for Parser<'a> {
        type Error = Error<'a>;

        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            parse_from_bytes(value)
        }
    }
}

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
    Comment(ParsedComment<'a>),
    /// A section header containing the section name and a subsection, if it
    /// exists.
    SectionHeader(ParsedSectionHeader<'a>),
    /// A name to a value in a section.
    Key(Key<'a>),
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

mod event;

/// A parsed section containing the header and the section events.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedSection<'a> {
    /// The section name and subsection name, if any.
    pub section_header: ParsedSectionHeader<'a>,
    /// The syntactic events found in this section.
    pub events: Vec<Event<'a>>,
}

/// A parsed section header, containing a name and optionally a subsection name.
///
/// Note that section headers must be parsed as valid ASCII, and thus all valid
/// instances must also necessarily be valid UTF-8, which is why we use a
/// [`str`] instead of [`[u8]`].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedSectionHeader<'a> {
    /// The name of the header.
    pub name: SectionHeaderName<'a>,
    /// The separator used to determine if the section contains a subsection.
    /// This is either a period `.` or a string of whitespace. Note that
    /// reconstruction of subsection format is dependent on this value. If this
    /// is all whitespace, then the subsection name needs to be surrounded by
    /// quotes to have perfect reconstruction.
    pub separator: Option<Cow<'a, BStr>>,
    /// The subsection name without quotes if any exist.
    pub subsection_name: Option<Cow<'a, BStr>>,
}

mod section;

mod types;
pub use types::{Key, SectionHeaderName};

/// A parsed comment event containing the comment marker and comment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedComment<'a> {
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
pub struct Error<'a> {
    line_number: usize,
    last_attempted_parser: error::ParserNode,
    parsed_until: Cow<'a, BStr>,
}

/// An error type representing a Parser [`Error`] or an [`IO error`]. This is
/// returned from functions that will perform IO on top of standard parsing,
/// such as reading from a file.
///
/// [`IO error`]: std::io::Error
#[derive(Debug)]
#[allow(missing_docs, clippy::module_name_repetitions)]
pub enum ParserOrIoError<'a> {
    Parser(Error<'a>),
    Io(std::io::Error),
}

mod error;

mod nom;
pub use self::nom::{parse_from_bytes, parse_from_bytes_owned, parse_from_path, parse_from_str};

#[cfg(test)]
pub(crate) mod tests;
