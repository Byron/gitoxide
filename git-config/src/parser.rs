//! This module handles parsing a `git-config` file. Generally speaking, you
//! want to use a higher abstraction such as [`GitConfig`] unless you have some
//! explicit reason to work with events instead.
//!
//! The general workflow for interacting with this is to use one of the
//! `parse_from_*` function variants. These will return a [`Parser`] on success,
//! which can be converted into an [`Event`] iterator. The [`Parser`] also has
//! additional methods for accessing leading comments or events by section.
//!
//! [`GitConfig`]: crate::file::GitConfig

use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_till, take_while};
use nom::character::complete::{char, none_of, one_of};
use nom::character::{is_newline, is_space};
use nom::combinator::{map, opt};
use nom::error::{Error as NomError, ErrorKind};
use nom::multi::{many0, many1};
use nom::sequence::delimited;
use nom::IResult;
use std::iter::FusedIterator;
use std::{borrow::Cow, hash::Hash};
use std::{convert::TryFrom, path::Path};
use std::{fmt::Display, io::Read};

/// Syntactic events that occurs in the config. Despite all these variants
/// holding a [`Cow`] instead over a simple reference, the parser will only emit
/// borrowed `Cow` variants.
///
/// The `Cow` smart pointer is used here for ease of inserting events in a
/// middle of an Event iterator. This is used, for example, in the [`GitConfig`]
/// struct when adding values.
///
/// [`Cow`]: std::borrow::Cow
/// [`GitConfig`]: crate::file::GitConfig
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
    Value(Cow<'a, [u8]>),
    /// Represents any token used to signify a new line character. On Unix
    /// platforms, this is typically just `\n`, but can be any valid newline
    /// sequence. Multiple newlines (such as `\n\n`) will be merged as a single
    /// newline event.
    Newline(Cow<'a, str>),
    /// Any value that isn't completed. This occurs when the value is continued
    /// onto the next line. A Newline event is guaranteed after, followed by
    /// either a ValueDone, a Whitespace, or another ValueNotDone.
    ValueNotDone(Cow<'a, [u8]>),
    /// The last line of a value which was continued onto another line.
    ValueDone(Cow<'a, [u8]>),
    /// A continuous section of insignificant whitespace. Values with internal
    /// spaces will not be separated by this event.
    Whitespace(Cow<'a, str>),
    /// This event is emitted when the parser counters a valid `=` character
    /// separating the key and value. This event is necessary as it eliminates
    /// the ambiguity for whitespace events between a key and value event.
    KeyValueSeparator,
}

impl Event<'_> {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.into()
    }

    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: Self::clone
    #[must_use]
    pub fn to_owned(&self) -> Event<'static> {
        match self {
            Event::Comment(e) => Event::Comment(e.to_owned()),
            Event::SectionHeader(e) => Event::SectionHeader(e.to_owned()),
            Event::Key(e) => Event::Key(e.to_owned()),
            Event::Value(e) => Event::Value(Cow::Owned(e.clone().into_owned())),
            Event::ValueNotDone(e) => Event::ValueNotDone(Cow::Owned(e.clone().into_owned())),
            Event::ValueDone(e) => Event::ValueDone(Cow::Owned(e.clone().into_owned())),
            Event::Newline(e) => Event::Newline(Cow::Owned(e.clone().into_owned())),
            Event::Whitespace(e) => Event::Whitespace(Cow::Owned(e.clone().into_owned())),
            Event::KeyValueSeparator => Event::KeyValueSeparator,
        }
    }
}

impl Display for Event<'_> {
    /// Note that this is a best-effort attempt at printing an `Event`. If
    /// there are non UTF-8 values in your config, this will _NOT_ render
    /// as read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(e) | Self::ValueNotDone(e) | Self::ValueDone(e) => match std::str::from_utf8(e) {
                Ok(e) => e.fmt(f),
                Err(_) => write!(f, "{:02x?}", e),
            },
            Self::Comment(e) => e.fmt(f),
            Self::SectionHeader(e) => e.fmt(f),
            Self::Key(e) => e.fmt(f),
            Self::Newline(e) | Self::Whitespace(e) => e.fmt(f),
            Self::KeyValueSeparator => write!(f, "="),
        }
    }
}

impl Into<Vec<u8>> for Event<'_> {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}

impl Into<Vec<u8>> for &Event<'_> {
    fn into(self) -> Vec<u8> {
        match self {
            Event::Value(e) | Event::ValueNotDone(e) | Event::ValueDone(e) => e.to_vec(),
            Event::Comment(e) => e.into(),
            Event::SectionHeader(e) => e.into(),
            Event::Key(e) => e.0.as_bytes().to_vec(),
            Event::Newline(e) | Event::Whitespace(e) => e.as_bytes().to_vec(),
            Event::KeyValueSeparator => vec![b'='],
        }
    }
}

/// A parsed section containing the header and the section events.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedSection<'a> {
    /// The section name and subsection name, if any.
    pub section_header: ParsedSectionHeader<'a>,
    /// The syntactic events found in this section.
    pub events: Vec<Event<'a>>,
}

impl ParsedSection<'_> {
    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: Self::clone
    #[must_use]
    pub fn to_owned(&self) -> ParsedSection<'static> {
        ParsedSection {
            section_header: self.section_header.to_owned(),
            events: self.events.iter().map(Event::to_owned).collect(),
        }
    }
}

impl Display for ParsedSection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_header)?;
        for event in &self.events {
            event.fmt(f)?;
        }
        Ok(())
    }
}

macro_rules! generate_case_insensitive {
    ($name:ident, $cow_inner_type:ty, $comment:literal) => {
        #[doc = $comment]
        #[derive(Clone, Eq, Ord, Debug, Default)]
        pub struct $name<'a>(pub Cow<'a, $cow_inner_type>);

        impl $name<'_> {
            /// Coerces into an owned instance. This differs from the standard
            /// [`clone`] implementation as calling clone will _not_ copy the
            /// borrowed variant, while this method will. In other words:
            ///
            /// | Borrow type | `.clone()` | `to_owned()` |
            /// | ----------- | ---------- | ------------ |
            /// | Borrowed    | Borrowed   | Owned        |
            /// | Owned       | Owned      | Owned        |
            ///
            /// This can be most effectively seen by the differing lifetimes
            /// between the two. This method guarantees a `'static` lifetime,
            /// while `clone` does not.
    ///
    /// [`clone`]: Self::clone
            #[must_use]
            pub fn to_owned(&self) -> $name<'static> {
                $name(Cow::Owned(self.0.clone().into_owned()))
            }
        }

        impl PartialEq for $name<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq_ignore_ascii_case(&other.0)
            }
        }

        impl Display for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl PartialOrd for $name<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0
                    .to_ascii_lowercase()
                    .partial_cmp(&other.0.to_ascii_lowercase())
            }
        }

        impl std::hash::Hash for $name<'_> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_ascii_lowercase().hash(state)
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            fn from(s: &'a str) -> Self {
                Self(Cow::Borrowed(s))
            }
        }

        impl<'a> From<Cow<'a, str>> for $name<'a> {
            fn from(s: Cow<'a, str>) -> Self {
                Self(s)
            }
        }

        impl<'a> std::ops::Deref for $name<'a> {
            type Target = $cow_inner_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

generate_case_insensitive!(
    SectionHeaderName,
    str,
    "Wrapper struct for section header names, since section headers are case-insensitive."
);

generate_case_insensitive!(
    Key,
    str,
    "Wrapper struct for key names, since keys are case-insensitive."
);

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
    pub separator: Option<Cow<'a, str>>,
    /// The subsection name without quotes if any exist.
    pub subsection_name: Option<Cow<'a, str>>,
}

impl ParsedSectionHeader<'_> {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.into()
    }

    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: Self::clone
    #[must_use]
    pub fn to_owned(&self) -> ParsedSectionHeader<'static> {
        ParsedSectionHeader {
            name: self.name.to_owned(),
            separator: self.separator.clone().map(|v| Cow::Owned(v.into_owned())),
            subsection_name: self.subsection_name.clone().map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

impl Display for ParsedSectionHeader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.name)?;

        if let Some(v) = &self.separator {
            // Separator must be utf-8
            v.fmt(f)?;
            let subsection_name = self.subsection_name.as_ref().unwrap();
            if v == "." {
                subsection_name.fmt(f)?;
            } else {
                write!(f, "\"{}\"", subsection_name)?;
            }
        }

        write!(f, "]")
    }
}

impl Into<Vec<u8>> for ParsedSectionHeader<'_> {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}

impl Into<Vec<u8>> for &ParsedSectionHeader<'_> {
    fn into(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl<'a> Into<Event<'a>> for ParsedSectionHeader<'a> {
    fn into(self) -> Event<'a> {
        Event::SectionHeader(self)
    }
}

/// A parsed comment event containing the comment marker and comment.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ParsedComment<'a> {
    /// The comment marker used. This is either a semicolon or octothorpe.
    pub comment_tag: char,
    /// The parsed comment.
    pub comment: Cow<'a, [u8]>,
}

impl ParsedComment<'_> {
    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: Self::clone
    #[must_use]
    pub fn to_owned(&self) -> ParsedComment<'static> {
        ParsedComment {
            comment_tag: self.comment_tag,
            comment: Cow::Owned(self.comment.to_vec()),
        }
    }
}

impl Display for ParsedComment<'_> {
    /// Note that this is a best-effort attempt at printing an comment. If
    /// there are non UTF-8 values in your config, this will _NOT_ render
    /// as read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.comment_tag.fmt(f)?;
        if let Ok(s) = std::str::from_utf8(&self.comment) {
            s.fmt(f)
        } else {
            write!(f, "{:02x?}", self.comment)
        }
    }
}

impl Into<Vec<u8>> for ParsedComment<'_> {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}

impl Into<Vec<u8>> for &ParsedComment<'_> {
    fn into(self) -> Vec<u8> {
        let mut values = vec![self.comment_tag as u8];
        values.extend(self.comment.iter());
        values
    }
}

/// A parser error reports the one-indexed line number where the parsing error
/// occurred, as well as the last parser node and the remaining data to be
/// parsed.
#[derive(PartialEq, Debug)]
pub struct Error<'a> {
    line_number: usize,
    last_attempted_parser: ParserNode,
    parsed_until: Cow<'a, [u8]>,
}

impl Error<'_> {
    /// The one-indexed line number where the error occurred. This is determined
    /// by the number of newlines that were successfully parsed.
    #[must_use]
    pub const fn line_number(&self) -> usize {
        self.line_number + 1
    }

    /// The remaining data that was left unparsed.
    #[must_use]
    pub fn remaining_data(&self) -> &[u8] {
        &self.parsed_until
    }
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_size = self.parsed_until.len();
        let data = std::str::from_utf8(&self.parsed_until);
        write!(
            f,
            "Got an unexpected token on line {} while trying to parse a {}: ",
            self.line_number + 1,
            self.last_attempted_parser,
        )?;

        match (data, data_size) {
            (Ok(data), _) if data_size > 10 => {
                write!(f, "'{}' ... ({} characters omitted)", &data[..10], data_size - 10)
            }
            (Ok(data), _) => write!(f, "'{}'", data),
            (Err(_), _) if data_size > 10 => write!(
                f,
                "'{:02x?}' ... ({} characters omitted)",
                &self.parsed_until[..10],
                data_size - 10
            ),
            (Err(_), _) => write!(f, "'{:02x?}'", self.parsed_until),
        }
    }
}

impl std::error::Error for Error<'_> {}

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

impl Display for ParserOrIoError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserOrIoError::Parser(e) => e.fmt(f),
            ParserOrIoError::Io(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ParserOrIoError<'_> {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::error::Error for ParserOrIoError<'_> {}

/// A list of parsers that parsing can fail on. This is used for pretty-printing
/// errors
#[derive(PartialEq, Debug, Clone, Copy)]
enum ParserNode {
    SectionHeader,
    ConfigName,
}

impl Display for ParserNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SectionHeader => write!(f, "section header"),
            Self::ConfigName => write!(f, "config name"),
        }
    }
}

/// A zero-copy `git-config` file parser.
///
/// This is parser exposes low-level syntactic events from a `git-config` file.
/// Generally speaking, you'll want to use [`GitConfig`] as it wraps
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
/// #   name: SectionHeaderName(Cow::Borrowed("core")),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\n  autocrlf = input";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n")),
/// Event::Whitespace(Cow::Borrowed("  ")),
/// Event::Key(Key(Cow::Borrowed("autocrlf"))),
/// Event::Whitespace(Cow::Borrowed(" ")),
/// Event::KeyValueSeparator,
/// Event::Whitespace(Cow::Borrowed(" ")),
/// Event::Value(Cow::Borrowed(b"input")),
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
/// #   name: SectionHeaderName(Cow::Borrowed("core")),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\n  autocrlf";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n")),
/// Event::Whitespace(Cow::Borrowed("  ")),
/// Event::Key(Key(Cow::Borrowed("autocrlf"))),
/// Event::Value(Cow::Borrowed(b"")),
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
/// #   name: SectionHeaderName(Cow::Borrowed("core")),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\nautocrlf=true\"\"\nfilemode=fa\"lse\"";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n")),
/// Event::Key(Key(Cow::Borrowed("autocrlf"))),
/// Event::KeyValueSeparator,
/// Event::Value(Cow::Borrowed(br#"true"""#)),
/// Event::Newline(Cow::Borrowed("\n")),
/// Event::Key(Key(Cow::Borrowed("filemode"))),
/// Event::KeyValueSeparator,
/// Event::Value(Cow::Borrowed(br#"fa"lse""#)),
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
/// #   name: SectionHeaderName(Cow::Borrowed("some-section")),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[some-section]\nfile=a\\\n    c";
/// # assert_eq!(parse_from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n")),
/// Event::Key(Key(Cow::Borrowed("file"))),
/// Event::KeyValueSeparator,
/// Event::ValueNotDone(Cow::Borrowed(b"a")),
/// Event::Newline(Cow::Borrowed("\n")),
/// Event::ValueDone(Cow::Borrowed(b"    c")),
/// # ]);
/// ```
///
/// [`GitConfig`]: crate::file::GitConfig
/// [`.ini` file format]: https://en.wikipedia.org/wiki/INI_file
/// [`git`'s documentation]: https://git-scm.com/docs/git-config#_configuration_file
/// [`FromStr`]: std::str::FromStr
/// [`From<&'_ str>`]: std::convert::From
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Parser<'a> {
    frontmatter: Vec<Event<'a>>,
    sections: Vec<ParsedSection<'a>>,
}

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
        let mut to_return = vec![];
        std::mem::swap(&mut self.frontmatter, &mut to_return);
        to_return
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
    pub fn into_iter(self) -> impl Iterator<Item = Event<'a>> + FusedIterator {
        // Can't impl IntoIter without allocating.and using a generic associated type
        // TODO: try harder?
        let section_iter = self.sections.into_iter().flat_map(|section| {
            vec![Event::SectionHeader(section.section_header)]
                .into_iter()
                .chain(section.events)
        });
        self.frontmatter.into_iter().chain(section_iter)
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

/// Parses a git config located at the provided path. On success, returns a
/// [`Parser`] that provides methods to accessing leading comments and sections
/// of a `git-config` file and can be converted into an iterator of [`Event`]
/// for higher level processing.
///
/// Note that since we accept a path rather than a reference to the actual
/// bytes, this function is _not_ zero-copy, as the Parser must own (and thus
/// copy) the bytes that it reads from. Consider one of the other variants if
/// performance is a concern.
///
/// # Errors
///
/// Returns an error if there was an IO error or the read file is not a valid
/// `git-config` This generally is due to either invalid names or if there's
/// extraneous data succeeding valid `git-config` data.
pub fn parse_from_path(path: &Path) -> Result<Parser<'static>, ParserOrIoError> {
    let mut bytes = vec![];
    let mut file = std::fs::File::open(path)?;
    file.read_to_end(&mut bytes)?;
    parse_from_bytes_owned(&bytes).map_err(ParserOrIoError::Parser)
}

/// Attempt to zero-copy parse the provided `&str`. On success, returns a
/// [`Parser`] that provides methods to accessing leading comments and sections
/// of a `git-config` file and can be converted into an iterator of [`Event`]
/// for higher level processing.
///
/// # Errors
///
/// Returns an error if the string provided is not a valid `git-config`.
/// This generally is due to either invalid names or if there's extraneous
/// data succeeding valid `git-config` data.
pub fn parse_from_str(input: &str) -> Result<Parser<'_>, Error> {
    parse_from_bytes(input.as_bytes())
}

/// Attempt to zero-copy parse the provided bytes. On success, returns a
/// [`Parser`] that provides methods to accessing leading comments and sections
/// of a `git-config` file and can be converted into an iterator of [`Event`]
/// for higher level processing.
///
/// # Errors
///
/// Returns an error if the string provided is not a valid `git-config`.
/// This generally is due to either invalid names or if there's extraneous
/// data succeeding valid `git-config` data.
#[allow(clippy::shadow_unrelated)]
pub fn parse_from_bytes(input: &[u8]) -> Result<Parser<'_>, Error> {
    let mut newlines = 0;
    let (i, frontmatter) = many0(alt((
        map(comment, Event::Comment),
        map(take_spaces, |whitespace| Event::Whitespace(Cow::Borrowed(whitespace))),
        map(take_newline, |(newline, counter)| {
            newlines += counter;
            Event::Newline(Cow::Borrowed(newline))
        }),
    )))(input)
    // I don't think this can panic. many0 errors if the child parser returns
    // a success where the input was not consumed, but alt will only return Ok
    // if one of its children succeed. However, all of it's children are
    // guaranteed to consume something if they succeed, so the Ok(i) == i case
    // can never occur.
    .expect("many0(alt(...)) panicked. Likely a bug in one of the children parser.");

    if i.is_empty() {
        return Ok(Parser {
            frontmatter,
            sections: vec![],
        });
    }

    let mut node = ParserNode::SectionHeader;

    let maybe_sections = many1(|i| section(i, &mut node))(i);
    let (i, sections) = maybe_sections.map_err(|_| Error {
        line_number: newlines,
        last_attempted_parser: node,
        parsed_until: i.into(),
    })?;

    let sections = sections
        .into_iter()
        .map(|(section, additional_newlines)| {
            newlines += additional_newlines;
            section
        })
        .collect();

    // This needs to happen after we collect sections, otherwise the line number
    // will be off.
    if !i.is_empty() {
        return Err(Error {
            line_number: newlines,
            last_attempted_parser: node,
            parsed_until: i.into(),
        });
    }

    Ok(Parser { frontmatter, sections })
}

/// Parses the provided bytes, returning an [`Parser`] that contains allocated
/// and owned events. This is similar to [`parse_from_bytes`], but performance
/// is degraded as it requires allocation for every event. However, this permits
/// the reference bytes to be dropped, allowing the parser to be passed around
/// without lifetime worries.
///
/// # Errors
///
/// Returns an error if the string provided is not a valid `git-config`.
/// This generally is due to either invalid names or if there's extraneous
/// data succeeding valid `git-config` data.
#[allow(clippy::shadow_unrelated)]
pub fn parse_from_bytes_owned(input: &[u8]) -> Result<Parser<'static>, Error<'static>> {
    // FIXME: This is duplication is necessary until comment, take_spaces, and take_newlines
    // accept cows instead, since we don't want to unnecessarily copy the frontmatter
    // events in a hypothetical parse_from_cow function.
    let mut newlines = 0;
    let (i, frontmatter) = many0(alt((
        map(comment, Event::Comment),
        map(take_spaces, |whitespace| Event::Whitespace(Cow::Borrowed(whitespace))),
        map(take_newline, |(newline, counter)| {
            newlines += counter;
            Event::Newline(Cow::Borrowed(newline))
        }),
    )))(input)
    // I don't think this can panic. many0 errors if the child parser returns
    // a success where the input was not consumed, but alt will only return Ok
    // if one of its children succeed. However, all of it's children are
    // guaranteed to consume something if they succeed, so the Ok(i) == i case
    // can never occur.
    .expect("many0(alt(...)) panicked. Likely a bug in one of the children parser.");
    let frontmatter = frontmatter.iter().map(Event::to_owned).collect();
    if i.is_empty() {
        return Ok(Parser {
            frontmatter,
            sections: vec![],
        });
    }

    let mut node = ParserNode::SectionHeader;

    let maybe_sections = many1(|i| section(i, &mut node))(i);
    let (i, sections) = maybe_sections.map_err(|_| Error {
        line_number: newlines,
        last_attempted_parser: node,
        parsed_until: Cow::Owned(i.into()),
    })?;

    let sections = sections
        .into_iter()
        .map(|(section, additional_newlines)| {
            newlines += additional_newlines;
            section.to_owned()
        })
        .collect();

    // This needs to happen after we collect sections, otherwise the line number
    // will be off.
    if !i.is_empty() {
        return Err(Error {
            line_number: newlines,
            last_attempted_parser: node,
            parsed_until: Cow::Owned(i.into()),
        });
    }

    Ok(Parser { frontmatter, sections })
}

fn comment(i: &[u8]) -> IResult<&[u8], ParsedComment> {
    let (i, comment_tag) = one_of(";#")(i)?;
    let (i, comment) = take_till(|c| c == b'\n')(i)?;
    Ok((
        i,
        ParsedComment {
            comment_tag,
            comment: Cow::Borrowed(comment),
        },
    ))
}

fn section<'a, 'b>(i: &'a [u8], node: &'b mut ParserNode) -> IResult<&'a [u8], (ParsedSection<'a>, usize)> {
    let (mut i, section_header) = section_header(i)?;

    let mut newlines = 0;
    let mut items = vec![];

    // This would usually be a many0(alt(...)), the manual loop allows us to
    // optimize vec insertions
    loop {
        let old_i = i;

        if let Ok((new_i, v)) = take_spaces(i) {
            if old_i != new_i {
                i = new_i;
                items.push(Event::Whitespace(Cow::Borrowed(v)));
            }
        }

        if let Ok((new_i, (v, new_newlines))) = take_newline(i) {
            if old_i != new_i {
                i = new_i;
                newlines += new_newlines;
                items.push(Event::Newline(Cow::Borrowed(v)));
            }
        }

        if let Ok((new_i, _)) = section_body(i, node, &mut items) {
            if old_i != new_i {
                i = new_i;
            }
        }

        if let Ok((new_i, comment)) = comment(i) {
            if old_i != new_i {
                i = new_i;
                items.push(Event::Comment(comment));
            }
        }

        if old_i == i {
            break;
        }
    }

    Ok((
        i,
        (
            ParsedSection {
                section_header,
                events: items,
            },
            newlines,
        ),
    ))
}

fn section_header(i: &[u8]) -> IResult<&[u8], ParsedSectionHeader> {
    let (i, _) = char('[')(i)?;
    // No spaces must be between section name and section start
    let (i, name) = take_while(|c: u8| c.is_ascii_alphanumeric() || c == b'-' || c == b'.')(i)?;

    let name = std::str::from_utf8(name).map_err(|_| {
        nom::Err::Error(NomError::<&[u8]> {
            input: i,
            code: ErrorKind::AlphaNumeric,
        })
    })?;

    if let Ok((i, _)) = char::<_, NomError<&[u8]>>(']')(i) {
        // Either section does not have a subsection or using deprecated
        // subsection syntax at this point.
        let header = match memchr::memrchr(b'.', name.as_bytes()) {
            Some(index) => ParsedSectionHeader {
                name: SectionHeaderName(Cow::Borrowed(&name[..index])),
                separator: name.get(index..=index).map(|slice| Cow::Borrowed(slice)),
                subsection_name: name.get(index + 1..).map(|slice| Cow::Borrowed(slice)),
            },
            None => ParsedSectionHeader {
                name: SectionHeaderName(Cow::Borrowed(name)),
                separator: None,
                subsection_name: None,
            },
        };

        return Ok((i, header));
    }

    // Section header must be using modern subsection syntax at this point.

    let (i, whitespace) = take_spaces(i)?;
    let (i, subsection_name) = delimited(
        char('"'),
        opt(escaped(none_of("\"\\\n\0"), '\\', one_of(r#""\"#))),
        tag("\"]"),
    )(i)?;

    let subsection_name = subsection_name.map(std::str::from_utf8).transpose().map_err(|_| {
        nom::Err::Error(NomError::<&[u8]> {
            input: i,
            code: ErrorKind::AlphaNumeric,
        })
    })?;

    Ok((
        i,
        ParsedSectionHeader {
            name: SectionHeaderName(Cow::Borrowed(name)),
            separator: Some(Cow::Borrowed(whitespace)),
            // We know that there's some section name here, so if we get an
            // empty vec here then we actually parsed an empty section name.
            subsection_name: subsection_name.or(Some("")).map(Cow::Borrowed),
        },
    ))
}

fn section_body<'a, 'b, 'c>(
    i: &'a [u8],
    node: &'b mut ParserNode,
    items: &'c mut Vec<Event<'a>>,
) -> IResult<&'a [u8], ()> {
    // maybe need to check for [ here
    *node = ParserNode::ConfigName;
    let (i, name) = config_name(i)?;

    items.push(Event::Key(Key(Cow::Borrowed(name))));

    let (i, whitespace) = opt(take_spaces)(i)?;

    if let Some(whitespace) = whitespace {
        items.push(Event::Whitespace(Cow::Borrowed(whitespace)));
    }

    let (i, _) = config_value(i, items)?;
    Ok((i, ()))
}

/// Parses the config name of a config pair. Assumes the input has already been
/// trimmed of any leading whitespace.
fn config_name(i: &[u8]) -> IResult<&[u8], &str> {
    if i.is_empty() {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::NonEmpty,
        }));
    }

    if !(i[0] as char).is_alphabetic() {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Alpha,
        }));
    }

    let (i, v) = take_while(|c: u8| (c as char).is_alphanumeric() || c == b'-')(i)?;
    let v = std::str::from_utf8(v).map_err(|_| {
        nom::Err::Error(NomError::<&[u8]> {
            input: i,
            code: ErrorKind::AlphaNumeric,
        })
    })?;

    Ok((i, v))
}

fn config_value<'a, 'b>(i: &'a [u8], events: &'b mut Vec<Event<'a>>) -> IResult<&'a [u8], ()> {
    if let (i, Some(_)) = opt(char('='))(i)? {
        events.push(Event::KeyValueSeparator);
        let (i, whitespace) = opt(take_spaces)(i)?;
        if let Some(whitespace) = whitespace {
            events.push(Event::Whitespace(Cow::Borrowed(whitespace)));
        }
        let (i, _) = value_impl(i, events)?;
        Ok((i, ()))
    } else {
        events.push(Event::Value(Cow::Borrowed(b"")));
        Ok((i, ()))
    }
}

/// Handles parsing of known-to-be values. This function handles both single
/// line values as well as values that are continuations.
///
/// # Errors
///
/// Returns an error if an invalid escape was used, if there was an unfinished
/// quote, or there was an escape but there is nothing left to escape.
fn value_impl<'a, 'b>(i: &'a [u8], events: &'b mut Vec<Event<'a>>) -> IResult<&'a [u8], ()> {
    let mut parsed_index: usize = 0;
    let mut offset: usize = 0;

    let mut was_prev_char_escape_char = false;
    // This is required to ignore comment markers if they're in a quote.
    let mut is_in_quotes = false;
    // Used to determine if we return a Value or Value{Not,}Done
    let mut partial_value_found = false;

    for (index, c) in i.iter().enumerate() {
        if was_prev_char_escape_char {
            was_prev_char_escape_char = false;
            match c {
                // We're escaping a newline, which means we've found a
                // continuation.
                b'\n' => {
                    partial_value_found = true;
                    events.push(Event::ValueNotDone(Cow::Borrowed(&i[offset..index - 1])));
                    events.push(Event::Newline(Cow::Borrowed(
                        std::str::from_utf8(&i[index..=index]).unwrap(),
                    )));
                    offset = index + 1;
                    parsed_index = 0;
                }
                b't' | b'\\' | b'n' | b'"' => (),
                _ => {
                    return Err(nom::Err::Error(NomError {
                        input: i,
                        code: ErrorKind::Escaped,
                    }));
                }
            }
        } else {
            match c {
                b'\n' => {
                    parsed_index = index;
                    break;
                }
                b';' | b'#' if !is_in_quotes => {
                    parsed_index = index;
                    break;
                }
                b'\\' => was_prev_char_escape_char = true,
                b'"' => is_in_quotes = !is_in_quotes,
                _ => {}
            }
        }
    }

    if parsed_index == 0 {
        parsed_index = i.len();
    }

    // Handle incomplete escape
    if was_prev_char_escape_char {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Escaped,
        }));
    }

    // Handle incomplete quotes
    if is_in_quotes {
        return Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Tag,
        }));
    }

    let (i, remainder_value) = {
        let mut new_index = parsed_index;
        for index in (offset..parsed_index).rev() {
            if !(i[index] as char).is_whitespace() {
                new_index = index + 1;
                break;
            }
        }
        (&i[new_index..], &i[offset..new_index])
    };

    if partial_value_found {
        events.push(Event::ValueDone(Cow::Borrowed(remainder_value)));
    } else {
        events.push(Event::Value(Cow::Borrowed(remainder_value)));
    }

    Ok((i, ()))
}

fn take_spaces(i: &[u8]) -> IResult<&[u8], &str> {
    let (i, v) = take_while(|c| (c as char).is_ascii() && is_space(c))(i)?;
    if v.is_empty() {
        Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Eof,
        }))
    } else {
        // v is guaranteed to be utf-8
        Ok((i, std::str::from_utf8(v).unwrap()))
    }
}

fn take_newline(i: &[u8]) -> IResult<&[u8], (&str, usize)> {
    let mut counter = 0;
    let (i, v) = take_while(|c| (c as char).is_ascii() && is_newline(c))(i)?;
    counter += v.len();
    if v.is_empty() {
        Err(nom::Err::Error(NomError {
            input: i,
            code: ErrorKind::Eof,
        }))
    } else {
        // v is guaranteed to be utf-8
        Ok((i, (std::str::from_utf8(v).unwrap(), counter)))
    }
}

#[cfg(test)]
mod comments {
    use super::comment;
    use crate::test_util::{comment as parsed_comment, fully_consumed};

    #[test]
    fn semicolon() {
        assert_eq!(
            comment(b"; this is a semicolon comment").unwrap(),
            fully_consumed(parsed_comment(';', " this is a semicolon comment")),
        );
    }

    #[test]
    fn octothorpe() {
        assert_eq!(
            comment(b"# this is an octothorpe comment").unwrap(),
            fully_consumed(parsed_comment('#', " this is an octothorpe comment")),
        );
    }

    #[test]
    fn multiple_markers() {
        assert_eq!(
            comment(b"###### this is an octothorpe comment").unwrap(),
            fully_consumed(parsed_comment('#', "##### this is an octothorpe comment")),
        );
    }
}

#[cfg(test)]
mod section_headers {
    use super::section_header;
    use crate::test_util::{fully_consumed, section_header as parsed_section_header};

    #[test]
    fn no_subsection() {
        assert_eq!(
            section_header(b"[hello]").unwrap(),
            fully_consumed(parsed_section_header("hello", None)),
        );
    }

    #[test]
    fn modern_subsection() {
        assert_eq!(
            section_header(br#"[hello "world"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "world"))),
        );
    }

    #[test]
    fn escaped_subsection() {
        assert_eq!(
            section_header(br#"[hello "foo\\bar\""]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", r#"foo\\bar\""#))),
        );
    }

    #[test]
    fn deprecated_subsection() {
        assert_eq!(
            section_header(br#"[hello.world]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (".", "world")))
        );
    }

    #[test]
    fn empty_legacy_subsection_name() {
        assert_eq!(
            section_header(br#"[hello.]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (".", "")))
        );
    }

    #[test]
    fn empty_modern_subsection_name() {
        assert_eq!(
            section_header(br#"[hello ""]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "")))
        );
    }

    #[test]
    fn newline_in_header() {
        assert!(section_header(b"[hello\n]").is_err())
    }

    #[test]
    fn null_byte_in_header() {
        assert!(section_header(b"[hello\0]").is_err())
    }

    #[test]
    fn right_brace_in_subsection_name() {
        assert_eq!(
            section_header(br#"[hello "]"]"#).unwrap(),
            fully_consumed(parsed_section_header("hello", (" ", "]")))
        );
    }
}

#[cfg(test)]
mod config_name {
    use super::config_name;
    use crate::test_util::fully_consumed;

    #[test]
    fn just_name() {
        assert_eq!(config_name(b"name").unwrap(), fully_consumed("name"));
    }

    #[test]
    fn must_start_with_alphabetic() {
        assert!(config_name(b"4aaa").is_err());
        assert!(config_name(b"-aaa").is_err());
    }

    #[test]
    fn cannot_be_empty() {
        assert!(config_name(b"").is_err())
    }
}

#[cfg(test)]
mod section_body {
    use super::{section_body, Event, ParserNode};
    use crate::test_util::{name_event, value_event, whitespace_event};

    #[test]
    fn whitespace_is_not_ambigious() {
        let mut node = ParserNode::SectionHeader;
        let mut vec = vec![];
        assert!(section_body(b"a =b", &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            vec![
                name_event("a"),
                whitespace_event(" "),
                Event::KeyValueSeparator,
                value_event("b")
            ]
        );

        let mut vec = vec![];
        assert!(section_body(b"a= b", &mut node, &mut vec).is_ok());
        assert_eq!(
            vec,
            vec![
                name_event("a"),
                Event::KeyValueSeparator,
                whitespace_event(" "),
                value_event("b")
            ]
        );
    }
}

#[cfg(test)]
mod value_no_continuation {
    use super::value_impl;
    use crate::test_util::value_event;

    #[test]
    fn no_comment() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello", &mut events).unwrap().0, b"");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn no_comment_newline() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello\na", &mut events).unwrap().0, b"\na");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn semicolon_comment_not_consumed() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello;world", &mut events).unwrap().0, b";world");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn octothorpe_comment_not_consumed() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello#world", &mut events).unwrap().0, b"#world");
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn values_with_extraneous_whitespace_without_comment() {
        let mut events = vec![];
        assert_eq!(
            value_impl(b"hello               ", &mut events).unwrap().0,
            b"               "
        );
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn values_with_extraneous_whitespace_before_comment() {
        let mut events = vec![];
        assert_eq!(
            value_impl(b"hello             #world", &mut events).unwrap().0,
            b"             #world"
        );
        assert_eq!(events, vec![value_event("hello")]);

        let mut events = vec![];
        assert_eq!(
            value_impl(b"hello             ;world", &mut events).unwrap().0,
            b"             ;world"
        );
        assert_eq!(events, vec![value_event("hello")]);
    }

    #[test]
    fn trans_escaped_comment_marker_not_consumed() {
        let mut events = vec![];
        assert_eq!(value_impl(br##"hello"#"world; a"##, &mut events).unwrap().0, b"; a");
        assert_eq!(events, vec![value_event(r##"hello"#"world"##)]);
    }

    #[test]
    fn complex_test() {
        let mut events = vec![];
        assert_eq!(value_impl(br#"value";";ahhhh"#, &mut events).unwrap().0, b";ahhhh");
        assert_eq!(events, vec![value_event(r#"value";""#)]);
    }

    #[test]
    fn garbage_after_continution_is_err() {
        assert!(value_impl(b"hello \\afwjdls", &mut vec![]).is_err());
    }

    #[test]
    fn incomplete_quote() {
        assert!(value_impl(br#"hello "world"#, &mut vec![]).is_err());
    }

    #[test]
    fn incomplete_escape() {
        assert!(value_impl(br#"hello world\"#, &mut vec![]).is_err());
    }
}

#[cfg(test)]
mod value_continuation {
    use super::value_impl;
    use crate::test_util::{newline_event, value_done_event, value_not_done_event};

    #[test]
    fn simple_continuation() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello\\\nworld", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            vec![
                value_not_done_event("hello"),
                newline_event(),
                value_done_event("world")
            ]
        );
    }

    #[test]
    fn continuation_with_whitespace() {
        let mut events = vec![];
        assert_eq!(value_impl(b"hello\\\n        world", &mut events).unwrap().0, b"");
        assert_eq!(
            events,
            vec![
                value_not_done_event("hello"),
                newline_event(),
                value_done_event("        world")
            ]
        );
    }

    #[test]
    fn complex_continuation_with_leftover_comment() {
        let mut events = vec![];
        assert_eq!(
            value_impl(b"1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut events)
                .unwrap()
                .0,
            b" # \"b\t ; c"
        );
        assert_eq!(
            events,
            vec![
                value_not_done_event(r#"1    "\""#),
                newline_event(),
                value_not_done_event(r#"a ; e "\""#),
                newline_event(),
                value_done_event("d")
            ]
        );
    }

    #[test]
    fn quote_split_over_two_lines_with_leftover_comment() {
        let mut events = vec![];
        assert_eq!(value_impl(b"\"\\\n;\";a", &mut events).unwrap().0, b";a");
        assert_eq!(
            events,
            vec![value_not_done_event("\""), newline_event(), value_done_event(";\"")]
        );
    }
}

#[cfg(test)]
mod section {
    use super::{section, Event, ParsedSection, ParserNode};
    use crate::test_util::{
        comment_event, fully_consumed, name_event, newline_event, section_header as parsed_section_header,
        value_done_event, value_event, value_not_done_event, whitespace_event,
    };

    #[test]
    fn empty_section() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[test]", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("test", None),
                    events: vec![]
                },
                0
            )),
        );
    }

    #[test]
    fn simple_section() {
        let mut node = ParserNode::SectionHeader;
        let section_data = br#"[hello]
            a = b
            c
            d = "lol""#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![
                        newline_event(),
                        whitespace_event("            "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("b"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("c"),
                        value_event(""),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("d"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("\"lol\"")
                    ]
                },
                3
            ))
        )
    }

    #[test]
    fn section_single_line() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[hello] c", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![whitespace_event(" "), name_event("c"), value_event("")]
                },
                0
            ))
        );
    }

    #[test]
    fn section_very_commented() {
        let mut node = ParserNode::SectionHeader;
        let section_data = br#"[hello] ; commentA
            a = b # commentB
            ; commentC
            ; commentD
            c = d"#;
        assert_eq!(
            section(section_data, &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("hello", None),
                    events: vec![
                        whitespace_event(" "),
                        comment_event(';', " commentA"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("b"),
                        whitespace_event(" "),
                        comment_event('#', " commentB"),
                        newline_event(),
                        whitespace_event("            "),
                        comment_event(';', " commentC"),
                        newline_event(),
                        whitespace_event("            "),
                        comment_event(';', " commentD"),
                        newline_event(),
                        whitespace_event("            "),
                        name_event("c"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_event("d"),
                    ]
                },
                4
            ))
        );
    }

    #[test]
    fn complex_continuation() {
        let mut node = ParserNode::SectionHeader;
        // This test is absolute hell. Good luck if this fails.
        assert_eq!(
            section(b"[section] a = 1    \"\\\"\\\na ; e \"\\\"\\\nd # \"b\t ; c", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("section", None),
                    events: vec![
                        whitespace_event(" "),
                        name_event("a"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        whitespace_event(" "),
                        value_not_done_event(r#"1    "\""#),
                        newline_event(),
                        value_not_done_event(r#"a ; e "\""#),
                        newline_event(),
                        value_done_event("d"),
                        whitespace_event(" "),
                        comment_event('#', " \"b\t ; c"),
                    ]
                },
                0
            ))
        );
    }

    #[test]
    fn quote_split_over_two_lines() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[section \"a\"] b =\"\\\n;\";a", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("section", (" ", "a")),
                    events: vec![
                        whitespace_event(" "),
                        name_event("b"),
                        whitespace_event(" "),
                        Event::KeyValueSeparator,
                        value_not_done_event("\""),
                        newline_event(),
                        value_done_event(";\""),
                        comment_event(';', "a"),
                    ]
                },
                0
            ))
        )
    }

    #[test]
    fn section_handles_extranous_whitespace_before_comment() {
        let mut node = ParserNode::SectionHeader;
        assert_eq!(
            section(b"[s]hello             #world", &mut node).unwrap(),
            fully_consumed((
                ParsedSection {
                    section_header: parsed_section_header("s", None),
                    events: vec![
                        name_event("hello"),
                        whitespace_event("             "),
                        value_event(""),
                        comment_event('#', "world"),
                    ]
                },
                0
            ))
        );
    }
}

#[cfg(test)]
mod error {
    use super::parse_from_str;

    #[test]
    fn line_no_is_one_indexed() {
        assert_eq!(parse_from_str("[hello").unwrap_err().line_number(), 1);
    }

    #[test]
    fn remaining_data_contains_bad_tokens() {
        assert_eq!(parse_from_str("[hello").unwrap_err().remaining_data(), b"[hello");
    }

    #[test]
    fn to_string_truncates_extra_values() {
        assert_eq!(
            parse_from_str("[1234567890").unwrap_err().to_string(),
            "Got an unexpected token on line 1 while trying to parse a section header: '[123456789' ... (1 characters omitted)"
        );
    }
}
