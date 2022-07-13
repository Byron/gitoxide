use std::{
    borrow::Cow,
    iter::FusedIterator,
    ops::{Deref, Range},
};

use bstr::{BStr, BString, ByteSlice, ByteVec};

use crate::{
    file::{Index, Size},
    lookup, parse,
    parse::{section::Key, Event},
    value::{normalize, normalize_bstr, normalize_bstring},
};

/// A opaque type that represents a mutable reference to a section.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MutableSection<'a, 'event> {
    section: &'a mut SectionBody<'event>,
    implicit_newline: bool,
    whitespace: Whitespace<'event>,
}

/// Mutating methods.
impl<'a, 'event> MutableSection<'a, 'event> {
    /// Adds an entry to the end of this section.
    pub fn push(&mut self, key: Key<'event>, value: Cow<'event, BStr>) {
        if let Some(ws) = &self.whitespace.pre_key {
            self.section.0.push(Event::Whitespace(ws.clone()));
        }

        self.section.0.push(Event::SectionKey(key));
        self.section.0.extend(self.key_value_separators());
        self.section.0.push(Event::Value(escape_value(value.as_ref()).into()));
        if self.implicit_newline {
            self.section.0.push(Event::Newline(BString::from("\n").into()));
        }
    }

    /// Removes all events until a key value pair is removed. This will also
    /// remove the whitespace preceding the key value pair, if any is found.
    pub fn pop(&mut self) -> Option<(Key<'_>, Cow<'event, BStr>)> {
        let mut values = Vec::new();
        // events are popped in reverse order
        while let Some(e) = self.section.0.pop() {
            match e {
                Event::SectionKey(k) => {
                    // pop leading whitespace
                    if let Some(Event::Whitespace(_)) = self.section.0.last() {
                        self.section.0.pop();
                    }

                    if values.len() == 1 {
                        let value = values.pop().expect("vec is non-empty but popped to empty value");
                        return Some((k, normalize(value)));
                    }

                    return Some((
                        k,
                        normalize_bstring({
                            let mut s = BString::default();
                            for value in values.into_iter().rev() {
                                s.push_str(value.as_ref());
                            }
                            s
                        }),
                    ));
                }
                Event::Value(v) | Event::ValueNotDone(v) | Event::ValueDone(v) => values.push(v),
                _ => (),
            }
        }
        None
    }

    /// Sets the last key value pair if it exists, or adds the new value.
    /// Returns the previous value if it replaced a value, or None if it adds
    /// the value.
    pub fn set(&mut self, key: Key<'event>, value: Cow<'event, BStr>) -> Option<Cow<'event, BStr>> {
        let range = self.value_range_by_key(&key);
        if range.is_empty() {
            self.push(key, value);
            return None;
        }
        let range_start = range.start;
        let ret = self.remove_internal(range);
        self.section.0.insert(range_start, Event::Value(value));
        Some(ret)
    }

    /// Removes the latest value by key and returns it, if it exists.
    pub fn remove(&mut self, key: &Key<'event>) -> Option<Cow<'event, BStr>> {
        let range = self.value_range_by_key(key);
        if range.is_empty() {
            return None;
        }
        Some(self.remove_internal(range))
    }

    /// Adds a new line event. Note that you don't need to call this unless
    /// you've disabled implicit newlines.
    pub fn push_newline(&mut self) {
        self.section.0.push(Event::Newline(Cow::Borrowed("\n".into())));
    }

    /// Enables or disables automatically adding newline events after adding
    /// a value. This is _enabled by default_.
    pub fn set_implicit_newline(&mut self, on: bool) {
        self.implicit_newline = on;
    }

    /// Sets the exact whitespace to use before each newly created key-value pair,
    /// with only whitespace characters being permissible.
    ///
    /// The default is 2 tabs.
    /// Set to `None` to disable adding whitespace before a key value.
    ///
    /// # Panics
    ///
    /// If non-whitespace characters are used. This makes the method only suitable for validated
    /// or known input.
    pub fn set_leading_whitespace(&mut self, whitespace: Option<Cow<'event, BStr>>) {
        assert!(
            whitespace
                .as_deref()
                .map_or(true, |ws| ws.iter().all(|b| b.is_ascii_whitespace())),
            "input whitespace must only contain whitespace characters."
        );
        self.whitespace.pre_key = whitespace;
    }

    /// Returns the whitespace this section will insert before the
    /// beginning of a key, if any.
    #[must_use]
    pub fn leading_whitespace(&self) -> Option<&BStr> {
        self.whitespace.pre_key.as_deref()
    }

    /// Returns the whitespace to be used before and after the `=` between the key
    /// and the value.
    ///
    /// For example, `k = v` will have `(Some(" "), Some(" "))`, whereas `k=\tv` will
    /// have `(None, Some("\t"))`.
    #[must_use]
    pub fn separator_whitespace(&self) -> (Option<&BStr>, Option<&BStr>) {
        (self.whitespace.pre_sep.as_deref(), self.whitespace.post_sep.as_deref())
    }
}

// Internal methods that may require exact indices for faster operations.
impl<'a, 'event> MutableSection<'a, 'event> {
    pub(crate) fn new(section: &'a mut SectionBody<'event>) -> Self {
        let whitespace = compute_whitespace(section);
        Self {
            section,
            implicit_newline: true,
            whitespace,
        }
    }

    pub(crate) fn get<'key>(
        &self,
        key: &Key<'key>,
        start: Index,
        end: Index,
    ) -> Result<Cow<'_, BStr>, lookup::existing::Error> {
        let mut expect_value = false;
        let mut concatenated_value = BString::default();

        for event in &self.section.0[start.0..end.0] {
            match event {
                Event::SectionKey(event_key) if event_key == key => expect_value = true,
                Event::Value(v) if expect_value => return Ok(normalize_bstr(v.as_ref())),
                Event::ValueNotDone(v) if expect_value => {
                    concatenated_value.push_str(v.as_ref());
                }
                Event::ValueDone(v) if expect_value => {
                    concatenated_value.push_str(v.as_ref());
                    return Ok(normalize_bstring(concatenated_value));
                }
                _ => (),
            }
        }

        Err(lookup::existing::Error::KeyMissing)
    }

    pub(crate) fn delete(&mut self, start: Index, end: Index) {
        self.section.0.drain(start.0..end.0);
    }

    pub(crate) fn set_internal(&mut self, index: Index, key: Key<'event>, value: &BStr) -> Size {
        let mut size = 0;

        self.section.0.insert(index.0, Event::Value(escape_value(value).into()));
        size += 1;

        let sep_events = self.key_value_separators();
        size += sep_events.len();
        self.section.0.insert_many(index.0, sep_events.into_iter().rev());

        self.section.0.insert(index.0, Event::SectionKey(key));
        size += 1;

        Size(size)
    }

    /// Performs the removal, assuming the range is valid.
    fn remove_internal(&mut self, range: Range<usize>) -> Cow<'event, BStr> {
        self.section
            .0
            .drain(range)
            .fold(Cow::Owned(BString::default()), |mut acc, e| {
                if let Event::Value(v) | Event::ValueNotDone(v) | Event::ValueDone(v) = e {
                    acc.to_mut().extend(&**v);
                }
                acc
            })
    }

    fn key_value_separators(&self) -> Vec<Event<'event>> {
        let mut out = Vec::with_capacity(3);
        if let Some(ws) = &self.whitespace.pre_sep {
            out.push(Event::Whitespace(ws.clone()));
        }
        out.push(Event::KeyValueSeparator);
        if let Some(ws) = &self.whitespace.post_sep {
            out.push(Event::Whitespace(ws.clone()));
        }
        out
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Whitespace<'a> {
    pre_key: Option<Cow<'a, BStr>>,
    pre_sep: Option<Cow<'a, BStr>>,
    post_sep: Option<Cow<'a, BStr>>,
}

impl Default for Whitespace<'_> {
    fn default() -> Self {
        Whitespace {
            pre_key: Some(b"\t".as_bstr().into()),
            pre_sep: Some(b" ".as_bstr().into()),
            post_sep: Some(b" ".as_bstr().into()),
        }
    }
}

fn compute_whitespace<'a>(s: &mut SectionBody<'a>) -> Whitespace<'a> {
    let key_pos =
        s.0.iter()
            .enumerate()
            .find_map(|(idx, e)| matches!(e, Event::SectionKey(_)).then(|| idx));
    key_pos
        .map(|key_pos| {
            let pre_key = s.0[..key_pos].iter().rev().next().and_then(|e| match e {
                Event::Whitespace(s) => Some(s.clone()),
                _ => None,
            });
            let from_key = &s.0[key_pos..];
            let (pre_sep, post_sep) = from_key
                .iter()
                .enumerate()
                .find_map(|(idx, e)| matches!(e, Event::KeyValueSeparator).then(|| idx))
                .map(|sep_pos| {
                    (
                        from_key.get(sep_pos - 1).and_then(|e| match e {
                            Event::Whitespace(ws) => Some(ws.clone()),
                            _ => None,
                        }),
                        from_key.get(sep_pos + 1).and_then(|e| match e {
                            Event::Whitespace(ws) => Some(ws.clone()),
                            _ => None,
                        }),
                    )
                })
                .unwrap_or_default();
            Whitespace {
                pre_key,
                pre_sep,
                post_sep,
            }
        })
        .unwrap_or_default()
}

fn escape_value(value: &BStr) -> BString {
    let starts_with_whitespace = value.get(0).map_or(false, |b| b.is_ascii_whitespace());
    let ends_with_whitespace = value
        .get(value.len().saturating_sub(1))
        .map_or(false, |b| b.is_ascii_whitespace());
    let contains_comment_indicators = value.find_byteset(b";#").is_some();
    let quote = starts_with_whitespace || ends_with_whitespace || contains_comment_indicators;

    let mut buf: BString = Vec::with_capacity(value.len()).into();
    if quote {
        buf.push(b'"');
    }

    for b in value.iter().copied() {
        match b {
            b'\n' => buf.push_str("\\n"),
            b'\t' => buf.push_str("\\t"),
            b'"' => buf.push_str("\\\""),
            b'\\' => buf.push_str("\\\\"),
            _ => buf.push(b),
        }
    }

    if quote {
        buf.push(b'"');
    }
    buf
}

impl<'event> Deref for MutableSection<'_, 'event> {
    type Target = SectionBody<'event>;

    fn deref(&self) -> &Self::Target {
        self.section
    }
}

/// A opaque type that represents a section body.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug, Default)]
pub struct SectionBody<'event>(pub(crate) parse::section::Events<'event>);

impl<'event> SectionBody<'event> {
    pub(crate) fn as_ref(&self) -> &[Event<'_>] {
        &self.0
    }

    pub(crate) fn as_mut(&mut self) -> &mut parse::section::Events<'event> {
        &mut self.0
    }

    /// Returns the the range containing the value events for the `key`.
    /// If the value is not found, then this returns an empty range.
    fn value_range_by_key(&self, key: &Key<'_>) -> Range<usize> {
        let mut range = Range::default();
        for (i, e) in self.0.iter().enumerate().rev() {
            match e {
                Event::SectionKey(k) => {
                    if k == key {
                        break;
                    }
                    range = Range::default();
                }
                Event::Value(_) => {
                    (range.start, range.end) = (i, i);
                }
                Event::ValueNotDone(_) | Event::ValueDone(_) => {
                    if range.end == 0 {
                        range.end = i
                    } else {
                        range.start = i
                    };
                }
                _ => (),
            }
        }

        // value end needs to be offset by one so that the last value's index
        // is included in the range
        range.start..range.end + 1
    }
}

/// Access
impl<'event> SectionBody<'event> {
    /// Retrieves the last matching value in a section with the given key, if present.
    #[must_use]
    pub fn value(&self, key: &str) -> Option<Cow<'_, BStr>> {
        let key = Key::from_str_unchecked(key);
        let range = self.value_range_by_key(&key);
        if range.is_empty() {
            return None;
        }
        let mut concatenated = BString::default();

        for event in &self.0[range] {
            match event {
                Event::Value(v) => {
                    return Some(normalize_bstr(v.as_ref()));
                }
                Event::ValueNotDone(v) => {
                    concatenated.push_str(v.as_ref());
                }
                Event::ValueDone(v) => {
                    concatenated.push_str(v.as_ref());
                    return Some(normalize_bstring(concatenated));
                }
                _ => (),
            }
        }
        None
    }

    /// Retrieves all values that have the provided key name. This may return
    /// an empty vec, which implies there were no values with the provided key.
    #[must_use]
    pub fn values(&self, key: &str) -> Vec<Cow<'_, BStr>> {
        let key = &Key::from_str_unchecked(key);
        let mut values = Vec::new();
        let mut expect_value = false;
        let mut concatenated_value = BString::default();

        for event in &self.0 {
            match event {
                Event::SectionKey(event_key) if event_key == key => expect_value = true,
                Event::Value(v) if expect_value => {
                    expect_value = false;
                    values.push(normalize_bstr(v.as_ref()));
                }
                Event::ValueNotDone(v) if expect_value => {
                    concatenated_value.push_str(v.as_ref());
                }
                Event::ValueDone(v) if expect_value => {
                    expect_value = false;
                    concatenated_value.push_str(v.as_ref());
                    values.push(normalize_bstring(std::mem::take(&mut concatenated_value)));
                }
                _ => (),
            }
        }

        values
    }

    /// Returns an iterator visiting all keys in order.
    pub fn keys(&self) -> impl Iterator<Item = &Key<'event>> {
        self.0
            .iter()
            .filter_map(|e| if let Event::SectionKey(k) = e { Some(k) } else { None })
    }

    /// Returns true if the section containss the provided key.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        let key = &Key::from_str_unchecked(key);
        self.0.iter().any(|e| {
            matches!(e,
                Event::SectionKey(k) if k == key
            )
        })
    }

    /// Returns the number of values in the section.
    #[must_use]
    pub fn num_values(&self) -> usize {
        self.0.iter().filter(|e| matches!(e, Event::SectionKey(_))).count()
    }

    /// Returns if the section is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// An owning iterator of a section body. Created by [`SectionBody::into_iter`], yielding
/// un-normalized (`key`, `value`) pairs.
// TODO: tests
pub struct SectionBodyIter<'event>(smallvec::IntoIter<[Event<'event>; 64]>);

impl<'event> IntoIterator for SectionBody<'event> {
    type Item = (Key<'event>, Cow<'event, BStr>);

    type IntoIter = SectionBodyIter<'event>;

    fn into_iter(self) -> Self::IntoIter {
        SectionBodyIter(self.0.into_iter())
    }
}

impl<'event> Iterator for SectionBodyIter<'event> {
    type Item = (Key<'event>, Cow<'event, BStr>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = None;
        let mut partial_value = BString::default();
        let mut value = None;

        for event in self.0.by_ref() {
            match event {
                Event::SectionKey(k) => key = Some(k),
                Event::Value(v) => {
                    value = Some(v);
                    break;
                }
                Event::ValueNotDone(v) => partial_value.push_str(v.as_ref()),
                Event::ValueDone(v) => {
                    partial_value.push_str(v.as_ref());
                    value = Some(partial_value.into());
                    break;
                }
                _ => (),
            }
        }

        key.zip(value.map(normalize))
    }
}

impl FusedIterator for SectionBodyIter<'_> {}
