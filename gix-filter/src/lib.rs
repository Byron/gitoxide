//! A library for implementing everything needed to deal with git filter pipelines.
//!
//! Generally, multiple filters are applied in a row forming a pipeline, with each filter being a stage in that pipeline.
//! This pipeline is pre-determined with each stage being configurable.
//!
//! The transformation on an input buffer goes in two ways: either a filter is applied, or its effects are undone. Differentiating
//! between these states is important to avoid comparing unfiltered buffers with filtered ones, for example.
//!
//! This crate implements the building blocks in terms of applying and undoing filters, along with logic to decide whether
//! or not to apply such a filter.
#![deny(rust_2018_idioms, missing_docs)]
#![forbid(unsafe_code)]

///
pub mod ident;

/// utilities related to handling line endings in buffers
pub mod eol {
    use crate::clear_and_set_capacity;
    use bstr::ByteSlice;

    /// The combination of `crlf`, `text` and `eol` attributes into one neat package.
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum AttributesDigest {
        /// Equivalent to the `-text` attribute.
        Binary,
        /// Equivalent to the `text` attribute.
        Text,
        /// Equivalent to the `text eol=lf` attributes.
        TextInput,
        /// Equivalent to the `text eol=crlf` attributes.
        TextCrlf,
        /// Equivalent to the `text=auto` attributes.
        TextAuto,
        /// Equivalent to the `text=auto eol=crlf` attributes.
        TextAutoCrlf,
        /// Equivalent to the `text=auto eol=lf` attributes.
        TextAutoInput,
    }

    ///
    pub mod convert_to_git {
        /// The error returned by [convert_to_git()][super::convert_to_git()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("{msg}")]
            RoundTrip { msg: &'static str },
            #[error("Could not obtain index object to check line endings for")]
            FetchObjectFromIndex(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
        }
    }

    /// Given a `src` buffer, change it `git` (`\n`) line endings and store the result in `buf`.
    /// Return `true` if `buf` was written or `false` if nothing had to be done.
    /// `action` is used to determine if ultimately a conversion should be done or not.
    /// When `action` takes certain values, `index_object` is called to write the version of `src` as stored in the index
    /// into the buffer and if it is a blob, or return `Ok(None)` if no such object exists.
    /// If renormalization is desired, let it return `Ok(None)` at all times to not let it have any influence over the
    /// outcome of this function.
    pub fn convert_to_git<E>(
        src: &[u8],
        action: AttributesDigest,
        buf: &mut Vec<u8>,
        index_object: impl FnOnce(&mut Vec<u8>) -> Result<Option<()>, E>,
    ) -> Result<bool, convert_to_git::Error>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        if action == AttributesDigest::Binary || src.is_empty() {
            return Ok(false);
        }

        let stats = Stats::from_bytes(src);
        let mut convert_crlf_to_lf = stats.crlf > 0;
        if matches!(
            action,
            AttributesDigest::TextAuto | AttributesDigest::TextAutoCrlf | AttributesDigest::TextAutoInput
        ) {
            // In this mode, we are supposed to figure out ourselves if we should convert or not.
            if stats.is_binary() {
                return Ok(false);
            }

            if let Some(()) =
                index_object(buf).map_err(|err| convert_to_git::Error::FetchObjectFromIndex(Box::new(err)))?
            {
                let has_crlf_in_index = buf
                    .find_byte(b'\r')
                    .map(|_| Stats::from_bytes(buf))
                    .filter(|s| !s.is_binary() && s.crlf > 0)
                    .is_some();
                if has_crlf_in_index {
                    convert_crlf_to_lf = false;
                }
            }
        }

        Ok(if convert_crlf_to_lf {
            clear_and_set_capacity(buf, src.len() - stats.crlf);
            if stats.lone_cr == 0 {
                buf.extend(src.iter().filter(|b| **b != b'\r'));
            } else {
                let mut bytes = src.iter().peekable();
                while let Some(b) = bytes.next() {
                    if !(*b == b'\r' && bytes.peek() == Some(&&b'\n')) {
                        buf.push(*b);
                    }
                }
            }
            true
        } else {
            false
        })
    }

    /// Statistics about a buffer that helps to safely perform EOL conversions
    #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Stats {
        /// The amount of null bytes.
        pub null: usize,
        /// The amount of lone carriage returns (`\r`).
        pub lone_cr: usize,
        /// The amount of lone line feeds (`\n`).
        pub lone_lf: usize,
        /// The amount carriage returns followed by line feeds
        pub crlf: usize,
        /// The estimate of printable characters.
        pub printable: usize,
        /// The estimate of characters that can't be printed.
        pub non_printable: usize,
    }

    impl Stats {
        /// Gather statistics from the given `bytes`.
        ///
        /// Note that the entire buffer will be scanned.
        pub fn from_bytes(bytes: &[u8]) -> Self {
            let mut bytes = bytes.iter().peekable();
            let mut null = 0;
            let mut lone_cr = 0;
            let mut lone_lf = 0;
            let mut crlf = 0;
            let mut printable = 0;
            let mut non_printable = 0;
            while let Some(b) = bytes.next() {
                if *b == b'\r' {
                    match bytes.peek() {
                        Some(n) if **n == b'\n' => {
                            bytes.next();
                            crlf += 1
                        }
                        _ => lone_cr += 1,
                    }
                    continue;
                }
                if *b == b'\n' {
                    lone_lf += 1;
                    continue;
                }
                if *b == 127 {
                    non_printable += 1;
                } else if *b < 32 {
                    match *b {
                        8 /* \b */ | b'\t' | 27 /* \033 */ | 12 /* \014 */ => printable += 1,
                        0 => {
                            non_printable += 1;
                            null += 1;
                        },
                        _ => non_printable += 1,
                    }
                } else {
                    printable += 1;
                }
            }

            Self {
                null,
                lone_cr,
                lone_lf,
                crlf,
                printable,
                non_printable,
            }
        }

        /// Returns `true` if these statistics are typical for a binary file.
        pub fn is_binary(&self) -> bool {
            self.lone_cr > 0 || self.null > 0 || (self.printable >> 7) < self.non_printable
        }
    }
}

fn clear_and_set_capacity(buf: &mut Vec<u8>, cap: usize) {
    buf.clear();
    if buf.capacity() < cap {
        buf.reserve(cap - buf.capacity());
    }
}
