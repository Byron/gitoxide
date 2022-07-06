use crate::parse::{Error, Event, ParsedSection, ParserOrIoError, State};
use std::convert::TryFrom;
use std::io::Read;

impl State<'static> {
    /// Parses a git config located at the provided path. On success, returns a
    /// [`State`] that provides methods to accessing leading comments and sections
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
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<State<'static>, ParserOrIoError<'static>> {
        let mut bytes = vec![];
        let mut file = std::fs::File::open(path)?;
        file.read_to_end(&mut bytes)?;
        crate::parse::nom::from_bytes_owned(&bytes).map_err(ParserOrIoError::Parser)
    }

    /// Parses the provided bytes, returning an [`State`] that contains allocated
    /// and owned events. This is similar to [`State::from_bytes()`], but performance
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
    pub fn from_bytes_owned(input: &[u8]) -> Result<State<'static>, Error<'static>> {
        crate::parse::nom::from_bytes_owned(input)
    }
}

impl<'a> State<'a> {
    /// Attempt to zero-copy parse the provided `&str`. On success, returns a
    /// [`State`] that provides methods to accessing leading comments and sections
    /// of a `git-config` file and can be converted into an iterator of [`Event`]
    /// for higher level processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the string provided is not a valid `git-config`.
    /// This generally is due to either invalid names or if there's extraneous
    /// data succeeding valid `git-config` data.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &'a str) -> Result<State<'a>, Error<'a>> {
        crate::parse::nom::from_bytes(input.as_bytes())
    }

    /// Attempt to zero-copy parse the provided bytes. On success, returns a
    /// [`State`] that provides methods to accessing leading comments and sections
    /// of a `git-config` file and can be converted into an iterator of [`Event`]
    /// for higher level processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the string provided is not a valid `git-config`.
    /// This generally is due to either invalid names or if there's extraneous
    /// data succeeding valid `git-config` data.
    #[allow(clippy::shadow_unrelated)]
    pub fn from_bytes(input: &'a [u8]) -> Result<State<'a>, Error<'a>> {
        crate::parse::nom::from_bytes(input)
    }
}

impl<'a> State<'a> {
    /// Returns the leading events (any comments, whitespace, or newlines before
    /// a section) from the parser. Consider [`State::take_frontmatter`] if
    /// you need an owned copy only once. If that function was called, then this
    /// will always return an empty slice.
    #[must_use]
    pub fn frontmatter(&self) -> &[Event<'a>] {
        &self.frontmatter
    }

    /// Takes the leading events (any comments, whitespace, or newlines before
    /// a section) from the parser. Subsequent calls will return an empty vec.
    /// Consider [`State::frontmatter`] if you only need a reference to the
    /// frontmatter
    pub fn take_frontmatter(&mut self) -> Vec<Event<'a>> {
        std::mem::take(&mut self.frontmatter)
    }

    /// Returns the parsed sections from the parser. Consider
    /// [`State::take_sections`] if you need an owned copy only once. If that
    /// function was called, then this will always return an empty slice.
    #[must_use]
    pub fn sections(&self) -> &[ParsedSection<'a>] {
        &self.sections
    }

    /// Takes the parsed sections from the parser. Subsequent calls will return
    /// an empty vec. Consider [`State::sections`] if you only need a reference
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
        self.frontmatter.into_iter().chain(
            self.sections.into_iter().flat_map(|section| {
                std::iter::once(Event::SectionHeader(section.section_header)).chain(section.events)
            }),
        )
    }
}

impl<'a> TryFrom<&'a str> for State<'a> {
    type Error = Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for State<'a> {
    type Error = Error<'a>;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        crate::parse::nom::from_bytes(value)
    }
}
