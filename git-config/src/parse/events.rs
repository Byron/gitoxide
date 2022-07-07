use crate::{
    parse,
    parse::{events::from_path, Event, Section},
};
use std::convert::TryFrom;

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
/// # use git_config::parse::{Event, Events, section};
/// # use std::borrow::Cow;
/// # let section_header = section::Header {
/// #   name: section::Name(Cow::Borrowed("core".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\n  autocrlf = input";
/// # assert_eq!(Events::from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Whitespace(Cow::Borrowed("  ".into())),
/// Event::SectionKey(section::Key(Cow::Borrowed("autocrlf".into()))),
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
/// # use git_config::parse::{Event, Events, section};
/// # use std::borrow::Cow;
/// # let section_header = section::Header {
/// #   name: section::Name(Cow::Borrowed("core".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\n  autocrlf";
/// # assert_eq!(Events::from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::Whitespace(Cow::Borrowed("  ".into())),
/// Event::SectionKey(section::Key(Cow::Borrowed("autocrlf".into()))),
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
/// # use git_config::parse::{Event, Events, section};
/// # use std::borrow::Cow;
/// # let section_header = section::Header {
/// #   name: section::Name(Cow::Borrowed("core".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[core]\nautocrlf=true\"\"\nfilemode=fa\"lse\"";
/// # assert_eq!(Events::from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::SectionKey(section::Key(Cow::Borrowed("autocrlf".into()))),
/// Event::KeyValueSeparator,
/// Event::Value(Cow::Borrowed(r#"true"""#.into())),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::SectionKey(section::Key(Cow::Borrowed("filemode".into()))),
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
/// # use git_config::parse::{Event, Events, section};
/// # use std::borrow::Cow;
/// # let section_header = section::Header {
/// #   name: section::Name(Cow::Borrowed("some-section".into())),
/// #   separator: None,
/// #   subsection_name: None,
/// # };
/// # let section_data = "[some-section]\nfile=a\\\n    c";
/// # assert_eq!(Events::from_str(section_data).unwrap().into_vec(), vec![
/// Event::SectionHeader(section_header),
/// Event::Newline(Cow::Borrowed("\n".into())),
/// Event::SectionKey(section::Key(Cow::Borrowed("file".into()))),
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
pub struct Events<'a> {
    pub(crate) frontmatter: Vec<Event<'a>>,
    pub(crate) sections: Vec<Section<'a>>,
}

impl parse::Delegate for Events<'static> {
    fn front_matter(&mut self, event: Event<'_>) {
        self.frontmatter.push(event.to_owned());
    }

    fn section(&mut self, section: Section<'_>) {
        self.sections.push(section.to_owned());
    }
}

impl Events<'static> {
    /// Parses a git config located at the provided path. On success, returns a
    /// [`Events`] that provides methods to accessing leading comments and sections
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
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Events<'static>, from_path::Error> {
        let bytes = std::fs::read(path)?;
        Ok(Self::from_bytes_owned(&bytes)?)
    }

    /// Parses the provided bytes, returning an [`Events`] that contains allocated
    /// and owned events. This is similar to [`Events::from_bytes()`], but performance
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
    pub fn from_bytes_owned(bytes: &[u8]) -> Result<Events<'static>, parse::Error<'static>> {
        let mut events = Events::default();
        {
            // SAFETY: we don't actually keep the bytes around for 'static, but help the borrow checker who
            //         cannot see that this is fine to do. Fortunately it's not possible to use these as 'static
            //         either in the delegate.
            #[allow(unsafe_code)]
            let bytes: &'static [u8] = unsafe { std::mem::transmute(bytes) };
            parse::from_bytes_1(bytes, &mut events)?;
        }
        Ok(events)
    }
}

impl<'a> Events<'a> {
    /// Attempt to zero-copy parse the provided `&str`. On success, returns a
    /// [`Events`] that provides methods to accessing leading comments and sections
    /// of a `git-config` file and can be converted into an iterator of [`Event`]
    /// for higher level processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the string provided is not a valid `git-config`.
    /// This generally is due to either invalid names or if there's extraneous
    /// data succeeding valid `git-config` data.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &'a str) -> Result<Events<'a>, parse::Error<'a>> {
        crate::parse::nom::from_bytes(input.as_bytes())
    }

    /// Attempt to zero-copy parse the provided bytes. On success, returns a
    /// [`Events`] that provides methods to accessing leading comments and sections
    /// of a `git-config` file and can be converted into an iterator of [`Event`]
    /// for higher level processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the string provided is not a valid `git-config`.
    /// This generally is due to either invalid names or if there's extraneous
    /// data succeeding valid `git-config` data.
    #[allow(clippy::shadow_unrelated)]
    pub fn from_bytes(input: &'a [u8]) -> Result<Events<'a>, parse::Error<'a>> {
        crate::parse::nom::from_bytes(input)
    }
}

impl<'a> Events<'a> {
    /// Returns the leading events (any comments, whitespace, or newlines before
    /// a section) from the parser. Consider [`Events::take_frontmatter`] if
    /// you need an owned copy only once. If that function was called, then this
    /// will always return an empty slice.
    #[must_use]
    pub fn frontmatter(&self) -> &[Event<'a>] {
        &self.frontmatter
    }

    /// Takes the leading events (any comments, whitespace, or newlines before
    /// a section) from the parser. Subsequent calls will return an empty vec.
    /// Consider [`Events::frontmatter`] if you only need a reference to the
    /// frontmatter
    pub fn take_frontmatter(&mut self) -> Vec<Event<'a>> {
        std::mem::take(&mut self.frontmatter)
    }

    /// Returns the parsed sections from the parser. Consider
    /// [`Events::take_sections`] if you need an owned copy only once. If that
    /// function was called, then this will always return an empty slice.
    #[must_use]
    pub fn sections(&self) -> &[Section<'a>] {
        &self.sections
    }

    /// Takes the parsed sections from the parser. Subsequent calls will return
    /// an empty vec. Consider [`Events::sections`] if you only need a reference
    /// to the comments.
    pub fn take_sections(&mut self) -> Vec<Section<'a>> {
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
        self.frontmatter.into_iter().chain(
            self.sections.into_iter().flat_map(|section| {
                std::iter::once(Event::SectionHeader(section.section_header)).chain(section.events)
            }),
        )
    }
}

impl<'a> TryFrom<&'a str> for Events<'a> {
    type Error = parse::Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for Events<'a> {
    type Error = parse::Error<'a>;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        crate::parse::nom::from_bytes(value)
    }
}
