use bstr::{BStr, BString, ByteVec};
use std::{
    borrow::Cow,
    collections::VecDeque,
    convert::TryFrom,
    iter::FusedIterator,
    ops::{Deref, Range},
};

use crate::{
    file::Index,
    lookup,
    parser::{Event, Key},
    values::{normalize_bstring, normalize_cow},
};

/// A opaque type that represents a mutable reference to a section.
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MutableSection<'borrow, 'event> {
    section: &'borrow mut SectionBody<'event>,
    implicit_newline: bool,
    whitespace: usize,
}

impl<'borrow, 'event> MutableSection<'borrow, 'event> {
    /// Adds an entry to the end of this section.
    pub fn push(&mut self, key: Key<'event>, value: Cow<'event, BStr>) {
        if self.whitespace > 0 {
            self.section.0.push(Event::Whitespace({
                let mut s = BString::default();
                s.extend(std::iter::repeat(b' ').take(self.whitespace));
                s.into()
            }));
        }

        self.section.0.push(Event::Key(key));
        self.section.0.push(Event::KeyValueSeparator);
        self.section.0.push(Event::Value(value));
        if self.implicit_newline {
            self.section.0.push(Event::Newline(BString::from("\n").into()));
        }
    }

    /// Removes all events until a key value pair is removed. This will also
    /// remove the whitespace preceding the key value pair, if any is found.
    pub fn pop(&mut self) -> Option<(Key<'_>, Cow<'event, BStr>)> {
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

    /// Performs the removal, assuming the range is valid. This is used to
    /// avoid duplicating searching for the range in [`Self::set`].
    fn remove_internal(&mut self, range: Range<usize>) -> Cow<'event, BStr> {
        self.section
            .0
            .drain(range)
            .fold(Cow::Owned(BString::default()), |acc, e| match e {
                Event::Value(v) | Event::ValueNotDone(v) | Event::ValueDone(v) => {
                    // This is fine because we start out with an owned
                    // variant, so we never actually clone the
                    // accumulator.
                    let mut acc = acc.into_owned();
                    acc.extend(&**v);
                    Cow::Owned(acc)
                }
                _ => acc,
            })
    }

    /// Adds a new line event. Note that you don't need to call this unless
    /// you've disabled implicit newlines.
    pub fn push_newline(&mut self) {
        self.section.0.push(Event::Newline(Cow::Borrowed("\n".into())));
    }

    /// Enables or disables automatically adding newline events after adding
    /// a value. This is enabled by default.
    pub fn implicit_newline(&mut self, on: bool) {
        self.implicit_newline = on;
    }

    /// Sets the number of spaces before the start of a key value. By default,
    /// this is set to two. Set to 0 to disable adding whitespace before a key
    /// value.
    pub fn set_whitespace(&mut self, num: usize) {
        self.whitespace = num;
    }

    /// Returns the number of whitespace this section will insert before the
    /// beginning of a key.
    #[must_use]
    pub const fn whitespace(&self) -> usize {
        self.whitespace
    }
}

// Internal methods that may require exact indices for faster operations.
impl<'borrow, 'event> MutableSection<'borrow, 'event> {
    pub(crate) fn new(section: &'borrow mut SectionBody<'event>) -> Self {
        Self {
            section,
            implicit_newline: true,
            whitespace: 2,
        }
    }

    pub(crate) fn get<'key>(
        &self,
        key: &Key<'key>,
        start: Index,
        end: Index,
    ) -> Result<Cow<'_, BStr>, lookup::existing::Error> {
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
                    partial_value = Some(v.as_ref().to_owned());
                }
                Event::ValueDone(v) if found_key => {
                    found_key = false;
                    partial_value.as_mut().unwrap().push_str(v.as_ref());
                }
                _ => (),
            }
        }

        latest_value
            .map(normalize_cow)
            .or_else(|| partial_value.map(normalize_bstring))
            .ok_or(lookup::existing::Error::KeyMissing)
    }

    pub(crate) fn delete(&mut self, start: Index, end: Index) {
        self.section.0.drain(start.0..=end.0);
    }

    pub(crate) fn set_internal(&mut self, index: Index, key: Key<'event>, value: BString) {
        self.section.0.insert(index.0, Event::Value(Cow::Owned(value)));
        self.section.0.insert(index.0, Event::KeyValueSeparator);
        self.section.0.insert(index.0, Event::Key(key));
    }
}

impl<'event> Deref for MutableSection<'_, 'event> {
    type Target = SectionBody<'event>;

    fn deref(&self) -> &Self::Target {
        self.section
    }
}

/// A opaque type that represents a section body.
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug, Default)]
pub struct SectionBody<'event>(Vec<Event<'event>>);

impl<'event> SectionBody<'event> {
    pub(crate) fn as_ref(&self) -> &[Event<'_>] {
        &self.0
    }

    pub(crate) fn as_mut(&mut self) -> &mut Vec<Event<'event>> {
        &mut self.0
    }

    /// Constructs a new empty section body.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Retrieves the last matching value in a section with the given key.
    /// Returns None if the key was not found.
    // We hit this lint because of the unreachable!() call may panic, but this
    // is a clippy bug (rust-clippy#6699), so we allow this lint for this
    // function.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn value(&self, key: &Key<'_>) -> Option<Cow<'event, BStr>> {
        let range = self.value_range_by_key(key);
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

        normalize_bstring(self.0[range].iter().fold(BString::default(), |mut acc, e| match e {
            Event::Value(v) | Event::ValueNotDone(v) | Event::ValueDone(v) => {
                acc.push_str(v.as_ref());
                acc
            }
            _ => acc,
        }))
        .into()
    }

    /// Retrieves the last matching value in a section with the given key, and
    /// attempts to convert the value into the provided type.
    ///
    /// # Errors
    ///
    /// Returns an error if the key was not found, or if the conversion failed.
    pub fn value_as<T: TryFrom<Cow<'event, BStr>>>(&self, key: &Key<'_>) -> Result<T, lookup::Error<T::Error>> {
        T::try_from(self.value(key).ok_or(lookup::existing::Error::KeyMissing)?)
            .map_err(lookup::Error::FailedConversion)
    }

    /// Retrieves all values that have the provided key name. This may return
    /// an empty vec, which implies there were no values with the provided key.
    #[must_use]
    pub fn values(&self, key: &Key<'_>) -> Vec<Cow<'event, BStr>> {
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
                    partial_value = Some(v.as_ref().to_owned());
                }
                Event::ValueDone(v) if found_key => {
                    found_key = false;
                    let mut value = partial_value
                        .take()
                        .expect("ValueDone event called before ValueNotDone");
                    value.push_str(v.as_ref());
                    values.push(normalize_bstring(value));
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
    pub fn values_as<T: TryFrom<Cow<'event, BStr>>>(&self, key: &Key<'_>) -> Result<Vec<T>, lookup::Error<T::Error>> {
        self.values(key)
            .into_iter()
            .map(T::try_from)
            .collect::<Result<Vec<T>, _>>()
            .map_err(lookup::Error::FailedConversion)
    }

    /// Returns an iterator visiting all keys in order.
    pub fn keys(&self) -> impl Iterator<Item = &Key<'event>> {
        self.0
            .iter()
            .filter_map(|e| if let Event::Key(k) = e { Some(k) } else { None })
    }

    /// Checks if the section contains the provided key.
    #[must_use]
    pub fn contains_key(&self, key: &Key<'_>) -> bool {
        self.0.iter().any(|e| {
            matches!(e,
                Event::Key(k) if k == key
            )
        })
    }

    /// Returns the number of values in the section.
    #[must_use]
    pub fn num_values(&self) -> usize {
        self.0.iter().filter(|e| matches!(e, Event::Key(_))).count()
    }

    /// Returns if the section is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the the range containing the value events for the section.
    /// If the value is not found, then this returns an empty range.
    fn value_range_by_key(&self, key: &Key<'event>) -> Range<usize> {
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
    type Item = (Key<'event>, Cow<'event, BStr>);

    type IntoIter = SectionBodyIter<'event>;

    fn into_iter(self) -> Self::IntoIter {
        SectionBodyIter(self.0.into())
    }
}

/// An owning iterator of a section body. Created by [`SectionBody::into_iter`].
#[allow(clippy::module_name_repetitions)]
pub struct SectionBodyIter<'event>(VecDeque<Event<'event>>);

impl<'event> Iterator for SectionBodyIter<'event> {
    type Item = (Key<'event>, Cow<'event, BStr>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut key = None;
        let mut partial_value = BString::default();
        let mut value = None;

        while let Some(event) = self.0.pop_front() {
            match event {
                Event::Key(k) => key = Some(k),
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

        key.zip(value.map(normalize_cow))
    }
}

impl FusedIterator for SectionBodyIter<'_> {}

impl<'event> From<Vec<Event<'event>>> for SectionBody<'event> {
    fn from(e: Vec<Event<'event>>) -> Self {
        Self(e)
    }
}
