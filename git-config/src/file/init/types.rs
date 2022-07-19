use crate::file::init;
use crate::parse;
use crate::parse::Event;
use crate::path::interpolate;

/// The error returned by [`File::from_bytes_no_includes()`][crate::File::from_bytes_no_includes()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Interpolate(#[from] interpolate::Error),
    #[error(transparent)]
    Includes(#[from] init::includes::Error),
}

/// Options when loading git config using [`File::from_paths_metadata()`][crate::File::from_paths_metadata()].
#[derive(Clone, Copy, Default)]
pub struct Options<'a> {
    /// Configure how to follow includes while handling paths.
    pub includes: init::includes::Options<'a>,
    /// If true, only value-bearing parse events will be kept to reduce memory usage and increase performance.
    ///
    /// Note that doing so will prevent [`write_to()`][crate::File::write_to()] to serialize itself meaningfully and correctly,
    /// as newlines will be missing. Use this only if it's clear that serialization will not be attempted.
    pub lossy: bool,
}

impl Options<'_> {
    pub(crate) fn to_event_filter(self) -> Option<fn(&Event<'_>) -> bool> {
        if self.lossy {
            Some(discard_nonessential_events)
        } else {
            None
        }
    }
}

fn discard_nonessential_events(e: &Event<'_>) -> bool {
    match e {
        Event::Whitespace(_) | Event::Comment(_) | Event::Newline(_) => false,
        Event::SectionHeader(_)
        | Event::SectionKey(_)
        | Event::KeyValueSeparator
        | Event::Value(_)
        | Event::ValueNotDone(_)
        | Event::ValueDone(_) => true,
    }
}
