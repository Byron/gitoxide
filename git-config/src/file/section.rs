use crate::file::error::GitConfigError;
use crate::file::Index;
use crate::parser::{Event, Key};
use crate::values::{normalize_cow, normalize_vec};
use std::borrow::{Borrow, Cow};
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::iter::FusedIterator;
use std::ops::{Deref, Range};

/// A opaque type that represents a mutable reference to a section.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MutableSection<'borrow, 'event> {
    section: &'borrow mut SectionBody<'event>,
    implicit_newline: bool,
    whitespace: usize,
}

impl<'borrow, 'event> MutableSection<'borrow, 'event> {
    /// Adds an entry to the end of this section.
    pub fn push(&mut self, key: Key<'event>, value: Cow<'event, [u8]>) {
        if self.whitespace > 0 {
            self.section
                .0
                .push(Event::Whitespace(" ".repeat(self.whitespace).into()));
        }

        self.section.0.push(Event::Key(key));
        self.section.0.push(Event::KeyValueSeparator);
        self.section.0.push(Event::Value(value));
        if self.implicit_newline {
            self.section.0.push(Event::Newline("\n".into()));
        }
    }

    /// Removes all events until a key value pair is removed. This will also
    /// remove the whitespace preceding the key value pair, if any is found.
    pub fn pop(&mut self) -> Option<(Key, Cow<'event, [u8]>)> {
        let mut values = vec![];
        // events are popped in reverse order
        while let Some(e) = self.section.0.pop() {
            match e {
                Event::Key(k) => {
                    // pop leading whitespace
                    if let Some(Event::Whitespace(_)) = self.section.0.last() {
                        self.section.0.pop();
                    }

                    if values.len() == 1 {
                        let value = values.pop().expect("vec is non-empty but popped to empty value");
                        return Some((k, normalize_cow(value)));
                    }

                    return Some((
                        k,
                        normalize_vec(values.into_iter().rev().flat_map(|v: Cow<[u8]>| v.to_vec()).collect()),
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
    pub fn set(&mut self, key: Key<'event>, value: Cow<'event, [u8]>) -> Option<Cow<'event, [u8]>> {
        let range = self.get_value_range_by_key(&key);
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
    pub fn remove(&mut self, key: &Key<'event>) -> Option<Cow<'event, [u8]>> {
        let range = self.get_value_range_by_key(key);
        if range.is_empty() {
            return None;
        }
        Some(self.remove_internal(range))
    }

    /// Performs the removal, assuming the range is valid. This is used to
    /// avoid duplicating searching for the range in [`Self::set`].
    fn remove_internal(&mut self, range: Range<usize>) -> Cow<'event, [u8]> {
        self.section
            .0
            .drain(range)
            .fold(Cow::<[u8]>::Owned(vec![]), |acc, e| match e {
                Event::Value(v) | Event::ValueNotDone(v) | Event::ValueDone(v) => {
                    // This is fine because we start out with an owned
                    // variant, so we never actually clone the
                    // accumulator.
                    let mut acc = acc.into_owned();
                    acc.extend(&*v);
                    Cow::Owned(acc)
                }
                _ => acc,
            })
    }

    /// Adds a new line event. Note that you don't need to call this unless
    /// you've disabled implicit newlines.
    #[inline]
    pub fn push_newline(&mut self) {
        self.section.0.push(Event::Newline("\n".into()));
    }

    /// Enables or disables automatically adding newline events after adding
    /// a value. This is enabled by default.
    #[inline]
    pub fn implicit_newline(&mut self, on: bool) {
        self.implicit_newline = on;
    }

    /// Sets the number of spaces before the start of a key value. By default,
    /// this is set to two. Set to 0 to disable adding whitespace before a key
    /// value.
    #[inline]
    pub fn set_whitespace(&mut self, num: usize) {
        self.whitespace = num;
    }

    /// Returns the number of whitespace this section will insert before the
    /// beginning of a key.
    #[inline]
    #[must_use]
    pub const fn whitespace(&self) -> usize {
        self.whitespace
    }
}

// Internal methods that may require exact indices for faster operations.
impl<'borrow, 'event> MutableSection<'borrow, 'event> {
    #[inline]
    pub(super) fn new(section: &'borrow mut SectionBody<'event>) -> Self {
        Self {
            section,
            implicit_newline: true,
            whitespace: 2,
        }
    }

    pub(super) fn get<'key>(
        &self,
        key: &Key<'key>,
        start: Index,
        end: Index,
    ) -> Result<Cow<'_, [u8]>, GitConfigError<'key>> {
        let mut found_key = false;
        let mut latest_value = None;
        let mut partial_value = None;
        // section_id is guaranteed to exist in self.sections, else we have a
        // violated invariant.

        for event in &self.section.0[start.0..=end.0] {
            match event {
                Event::Key(event_key) if event_key == key => found_key = true,
                Event::Value(v) if found_key => {
                    found_key = false;
                    // Clones the Cow, doesn't copy underlying value if borrowed
                    latest_value = Some(v.clone());
                }
                Event::ValueNotDone(v) if found_key => {
                    latest_value = None;
                    partial_value = Some((*v).to_vec());
                }
                Event::ValueDone(v) if found_key => {
                    found_key = false;
                    partial_value.as_mut().unwrap().extend(&**v);
                }
                _ => (),
            }
        }

        latest_value
            .map(normalize_cow)
            .or_else(|| partial_value.map(normalize_vec))
            .ok_or(GitConfigError::KeyDoesNotExist)
    }

    #[inline]
    pub(super) fn delete(&mut self, start: Index, end: Index) {
        self.section.0.drain(start.0..=end.0);
    }

    pub(super) fn set_internal(&mut self, index: Index, key: Key<'event>, value: Vec<u8>) {
        self.section.0.insert(index.0, Event::Value(Cow::Owned(value)));
        self.section.0.insert(index.0, Event::KeyValueSeparator);
        self.section.0.insert(index.0, Event::Key(key));
    }
}

impl<'event> Deref for MutableSection<'_, 'event> {
    type Target = SectionBody<'event>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.section
    }
}

/// A opaque type that represents a section body.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug, Default)]
pub struct SectionBody<'event>(Vec<Event<'event>>);

impl<'event> SectionBody<'event> {
    pub(super) fn as_ref(&self) -> &[Event<'_>] {
        &self.0
    }

    pub(super) fn as_mut(&mut self) -> &mut Vec<Event<'event>> {
        &mut self.0
    }

    /// Constructs a new empty section body.
    #[inline]
    pub(super) fn new() -> Self {
        Self::default()
    }

    /// Retrieves the last matching value in a section with the given key.
    /// Returns None if the key was not found.
    // We hit this lint because of the unreachable!() call may panic, but this
    // is a clippy bug (rust-clippy#6699), so we allow this lint for this
    // function.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn value(&self, key: &Key) -> Option<Cow<'event, [u8]>> {
        let range = self.get_value_range_by_key(key);
        if range.is_empty() {
            return None;
        }

        if range.end - range.start == 1 {
            return self.0.get(range.start).map(|e| match e {
                Event::Value(v) => v.clone(),
                // range only has one element so we know it's a value event, so
                // it's impossible to reach this code.
                _ => unreachable!(),
            });
        }

        Some(normalize_cow(self.0[range].iter().fold(
            Cow::<[u8]>::Owned(vec![]),
            |acc, e| match e {
                Event::Value(v) | Event::ValueNotDone(v) | Event::ValueDone(v) => {
                    // This is fine because we start out with an owned
                    // variant, so we never actually clone the
                    // accumulator.
                    let mut acc = acc.into_owned();
                    acc.extend(&**v);
                    Cow::Owned(acc)
                }
                _ => acc,
            },
        )))
    }

    /// Retrieves the last matching value in a section with the given key, and
    /// attempts to convert the value into the provided type.
    ///
    /// # Errors
    ///
    /// Returns an error if the key was not found, or if the conversion failed.
    #[inline]
    pub fn value_as<T: TryFrom<Cow<'event, [u8]>>>(&self, key: &Key) -> Result<T, GitConfigError<'event>> {
        T::try_from(self.value(key).ok_or(GitConfigError::KeyDoesNotExist)?)
            .map_err(|_| GitConfigError::FailedConversion)
    }

    /// Retrieves all values that have the provided key name. This may return
    /// an empty vec, which implies there was values with the provided key.
    #[must_use]
    pub fn values(&self, key: &Key) -> Vec<Cow<'event, [u8]>> {
        let mut values = vec![];
        let mut found_key = false;
        let mut partial_value = None;

        // This can iterate forwards because we need to iterate over the whole
        // section anyways
        for event in &self.0 {
            match event {
                Event::Key(event_key) if event_key == key => found_key = true,
                Event::Value(v) if found_key => {
                    found_key = false;
                    // Clones the Cow, doesn't copy underlying value if borrowed
                    values.push(normalize_cow(v.clone()));
                    partial_value = None;
                }
                Event::ValueNotDone(v) if found_key => {
                    partial_value = Some((*v).to_vec());
                }
                Event::ValueDone(v) if found_key => {
                    found_key = false;
                    let mut value = partial_value
                        .take()
                        .expect("ValueDone event called before ValueNotDone");
                    value.extend(&**v);
                    values.push(normalize_cow(Cow::Owned(value)));
                }
                _ => (),
            }
        }

        values
    }

    /// Retrieves all values that have the provided key name. This may return
    /// an empty vec, which implies there was values with the provided key.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion failed.
    #[inline]
    pub fn values_as<T: TryFrom<Cow<'event, [u8]>>>(&self, key: &Key) -> Result<Vec<T>, GitConfigError<'event>> {
        self.values(key)
            .into_iter()
            .map(T::try_from)
            .collect::<Result<Vec<T>, _>>()
            .map_err(|_| GitConfigError::FailedConversion)
    }

    /// Returns an iterator visiting all keys in order.
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &Key<'event>> {
        self.0
            .iter()
            .filter_map(|e| if let Event::Key(k) = e { Some(k) } else { None })
    }

    /// Checks if the section contains the provided key.
    #[must_use]
    pub fn contains_key(&self, key: &Key) -> bool {
        self.0.iter().any(|e| *e == Event::Key(*key))
    }

    /// Returns the number of entries in the section.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.iter().filter(|e| matches!(e, Event::Key(_))).count()
    }

    /// Returns if the section is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the the range containing the value events for the section.
    /// If the value is not found, then this returns an empty range.
    fn get_value_range_by_key(&self, key: &Key<'event>) -> Range<usize> {
        let mut values_start = 0;
        // value end needs to be offset by one so that the last value's index
        // is included in the range
        let mut values_end = 0;
        for (i, e) in self.0.iter().enumerate().rev() {
            match e {
                Event::Key(k) => {
                    if k == key {
                        break;
                    }
                    values_start = 0;
                    values_end = 0;
                }
                Event::Value(_) => {
                    values_end = i + 1;
                    values_start = i;
                }
                Event::ValueNotDone(_) | Event::ValueDone(_) => {
                    if values_end == 0 {
                        values_end = i + 1;
                    } else {
                        values_start = i;
                    }
                }
                _ => (),
            }
        }

        values_start..values_end
    }
}

impl<'event> IntoIterator for SectionBody<'event> {
    type Item = (Key<'event>, Cow<'event, [u8]>);

    type IntoIter = SectionBodyIter<'event>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SectionBodyIter(self.0.into())
    }
}

/// An owning iterator of a section body. Created by [`SectionBody::into_iter`].
pub struct SectionBodyIter<'event>(VecDeque<Event<'event>>);

impl<'event> Iterator for SectionBodyIter<'event> {
    type Item = (Key<'event>, Cow<'event, [u8]>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = None;
        let mut partial_value: Vec<u8> = Vec::new();
        let mut value = None;

        while let Some(event) = self.0.pop_front() {
            match event {
                Event::Key(k) => key = Some(k),
                Event::Value(v) => {
                    value = Some(v);
                    break;
                }
                Event::ValueNotDone(v) => partial_value.extend::<&[u8]>(v.borrow()),
                Event::ValueDone(v) => {
                    partial_value.extend::<&[u8]>(v.borrow());
                    value = Some(Cow::Owned(partial_value));
                    break;
                }
                _ => (),
            }
        }

        key.zip(value.map(normalize_cow))
    }
}

impl FusedIterator for SectionBodyIter<'_> {}

impl<'event> From<Vec<Event<'event>>> for SectionBody<'event> {
    #[inline]
    fn from(e: Vec<Event<'event>>) -> Self {
        Self(e)
    }
}
