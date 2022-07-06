use bstr::{BStr, BString};
use std::{borrow::Cow, collections::HashMap, ops::DerefMut};

use crate::{
    file::{
        section::{MutableSection, SectionBody},
        Index, SectionId, Size,
    },
    lookup,
    parse::{Event, Key},
    value::{normalize_bstr, normalize_bstring},
};

/// An intermediate representation of a mutable value obtained from
/// [`File`].
///
/// This holds a mutable reference to the underlying data structure of
/// [`File`], and thus guarantees through Rust's borrower checker that
/// multiple mutable references to [`File`] cannot be owned at the same
/// time.
///
/// [`File`]: crate::File
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MutableValue<'borrow, 'lookup, 'event> {
    section: MutableSection<'borrow, 'event>,
    key: Key<'lookup>,
    index: Index,
    size: Size,
}

impl<'borrow, 'lookup, 'event> MutableValue<'borrow, 'lookup, 'event> {
    pub(crate) const fn new(
        section: MutableSection<'borrow, 'event>,
        key: Key<'lookup>,
        index: Index,
        size: Size,
    ) -> Self {
        Self {
            section,
            key,
            index,
            size,
        }
    }

    /// Returns the actual value. This is computed each time this is called, so
    /// it's best to reuse this value or own it if an allocation is acceptable.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup failed.
    pub fn get(&self) -> Result<Cow<'_, BStr>, lookup::existing::Error> {
        self.section.get(&self.key, self.index, self.index + self.size)
    }

    /// Update the value to the provided one. This modifies the value such that
    /// the Value event(s) are replaced with a single new event containing the
    /// new value.
    pub fn set_string(&mut self, input: String) {
        self.set_bytes(input.into());
    }

    /// Update the value to the provided one. This modifies the value such that
    /// the Value event(s) are replaced with a single new event containing the
    /// new value.
    pub fn set_bytes(&mut self, input: BString) {
        if self.size.0 > 0 {
            self.section.delete(self.index, self.index + self.size);
        }
        self.size = Size(3);
        self.section.set_internal(self.index, self.key.to_owned(), input);
    }

    /// Removes the value. Does nothing when called multiple times in
    /// succession.
    pub fn delete(&mut self) {
        if self.size.0 > 0 {
            self.section.delete(self.index, self.index + self.size);
            self.size = Size(0);
        }
    }
}

/// Internal data structure for [`MutableMultiValue`]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct EntryData {
    section_id: SectionId,
    offset_index: usize,
}

impl EntryData {
    pub(crate) const fn new(section_id: SectionId, offset_index: usize) -> Self {
        Self {
            section_id,
            offset_index,
        }
    }
}

/// An intermediate representation of a mutable multivar obtained from
/// [`File`].
///
/// This holds a mutable reference to the underlying data structure of
/// [`File`], and thus guarantees through Rust's borrower checker that
/// multiple mutable references to [`File`] cannot be owned at the same
/// time.
///
/// [`File`]: crate::File
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Debug)]
pub struct MutableMultiValue<'borrow, 'lookup, 'event> {
    section: &'borrow mut HashMap<SectionId, SectionBody<'event>>,
    key: Key<'lookup>,
    /// Each entry data struct provides sufficient information to index into
    /// [`Self::offsets`]. This layer of indirection is used for users to index
    /// into the offsets rather than leaking the internal data structures.
    indices_and_sizes: Vec<EntryData>,
    /// Each offset represents the size of a event slice and whether or not the
    /// event slice is significant or not. This is used to index into the
    /// actual section.
    offsets: HashMap<SectionId, Vec<usize>>,
}

impl<'borrow, 'lookup, 'event> MutableMultiValue<'borrow, 'lookup, 'event> {
    pub(crate) fn new(
        section: &'borrow mut HashMap<SectionId, SectionBody<'event>>,
        key: Key<'lookup>,
        indices_and_sizes: Vec<EntryData>,
        offsets: HashMap<SectionId, Vec<usize>>,
    ) -> Self {
        Self {
            section,
            key,
            indices_and_sizes,
            offsets,
        }
    }

    /// Returns the actual values. This is computed each time this is called.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup failed.
    pub fn get(&self) -> Result<Vec<Cow<'_, BStr>>, lookup::existing::Error> {
        let mut found_key = false;
        let mut values = vec![];
        let mut partial_value = None;
        // section_id is guaranteed to exist in self.sections, else we have a
        // violated invariant.
        for EntryData {
            section_id,
            offset_index,
        } in &self.indices_and_sizes
        {
            let (offset, size) = MutableMultiValue::index_and_size(&self.offsets, *section_id, *offset_index);
            for event in &self
                .section
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .as_ref()[offset..offset + size]
            {
                match event {
                    Event::Key(event_key) if *event_key == self.key => found_key = true,
                    Event::Value(v) if found_key => {
                        found_key = false;
                        values.push(normalize_bstr(v.as_ref()));
                    }
                    Event::ValueNotDone(v) if found_key => {
                        partial_value = Some((*v).to_vec());
                    }
                    Event::ValueDone(v) if found_key => {
                        found_key = false;
                        let mut value = partial_value
                            .take()
                            .expect("Somehow got ValueDone before ValueNotDone event");
                        value.extend(&***v);
                        values.push(normalize_bstring(value));
                    }
                    _ => (),
                }
            }
        }

        if values.is_empty() {
            return Err(lookup::existing::Error::KeyMissing);
        }

        Ok(values)
    }

    /// Returns the size of values the multivar has.
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices_and_sizes.len()
    }

    /// Returns if the multivar has any values. This might occur if the value
    /// was deleted but not set with a new value.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices_and_sizes.is_empty()
    }

    /// Sets the value at the given index.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    pub fn set_string(&mut self, index: usize, input: String) {
        self.set_bytes(index, input);
    }

    /// Sets the value at the given index.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    pub fn set_bytes(&mut self, index: usize, input: impl Into<BString>) {
        self.set_value(index, Cow::Owned(input.into()));
    }

    /// Sets the value at the given index.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    pub fn set_value<'a: 'event>(&mut self, index: usize, input: Cow<'a, BStr>) {
        let EntryData {
            section_id,
            offset_index,
        } = self.indices_and_sizes[index];
        MutableMultiValue::set_value_inner(
            &self.key,
            &mut self.offsets,
            self.section
                .get_mut(&section_id)
                .expect("sections does not have section id from section ids"),
            section_id,
            offset_index,
            input,
        );
    }

    /// Sets all values to the provided values. Note that this follows [`zip`]
    /// logic: if the number of values in the input is less than the number of
    /// values currently existing, then only the first `n` values are modified.
    /// If more values are provided than there currently are, then the
    /// remaining values are ignored.
    ///
    /// [`zip`]: std::iter::Iterator::zip
    pub fn set_values<'a: 'event>(&mut self, input: impl Iterator<Item = Cow<'a, BStr>>) {
        for (
            EntryData {
                section_id,
                offset_index,
            },
            value,
        ) in self.indices_and_sizes.iter().zip(input)
        {
            Self::set_value_inner(
                &self.key,
                &mut self.offsets,
                self.section
                    .get_mut(section_id)
                    .expect("sections does not have section id from section ids"),
                *section_id,
                *offset_index,
                value,
            );
        }
    }

    /// Sets all values in this multivar to the provided one by copying the
    /// input for all values.
    pub fn set_str_all(&mut self, input: &str) {
        self.set_owned_values_all(input);
    }

    /// Sets all values in this multivar to the provided one by copying the
    /// input bytes for all values.
    pub fn set_owned_values_all<'a>(&mut self, input: impl Into<&'a BStr>) {
        let input = input.into();
        for EntryData {
            section_id,
            offset_index,
        } in &self.indices_and_sizes
        {
            Self::set_value_inner(
                &self.key,
                &mut self.offsets,
                self.section
                    .get_mut(section_id)
                    .expect("sections does not have section id from section ids"),
                *section_id,
                *offset_index,
                Cow::Owned(input.to_owned()),
            );
        }
    }

    /// Sets all values in this multivar to the provided one without owning the
    /// provided input. Note that this requires `input` to last longer than
    /// [`File`]. Consider using [`Self::set_owned_values_all`] or
    /// [`Self::set_str_all`] unless you have a strict performance or memory
    /// need for a more ergonomic interface.
    ///
    /// [`File`]: crate::File
    pub fn set_values_all<'a: 'event>(&mut self, input: &'a BStr) {
        for EntryData {
            section_id,
            offset_index,
        } in &self.indices_and_sizes
        {
            Self::set_value_inner(
                &self.key,
                &mut self.offsets,
                self.section
                    .get_mut(section_id)
                    .expect("sections does not have section id from section ids"),
                *section_id,
                *offset_index,
                Cow::Borrowed(input),
            );
        }
    }

    fn set_value_inner<'a: 'event>(
        key: &Key<'lookup>,
        offsets: &mut HashMap<SectionId, Vec<usize>>,
        section: &mut SectionBody<'event>,
        section_id: SectionId,
        offset_index: usize,
        input: Cow<'a, BStr>,
    ) {
        let (offset, size) = MutableMultiValue::index_and_size(offsets, section_id, offset_index);
        section.as_mut().drain(offset..offset + size);

        MutableMultiValue::set_offset(offsets, section_id, offset_index, 3);
        section.as_mut().insert(offset, Event::Value(input));
        section.as_mut().insert(offset, Event::KeyValueSeparator);
        section.as_mut().insert(offset, Event::Key(key.to_owned()));
    }

    /// Removes the value at the given index. Does nothing when called multiple
    /// times in succession.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    pub fn delete(&mut self, index: usize) {
        let EntryData {
            section_id,
            offset_index,
        } = &self.indices_and_sizes[index];
        let (offset, size) = MutableMultiValue::index_and_size(&self.offsets, *section_id, *offset_index);
        if size > 0 {
            self.section
                .get_mut(section_id)
                .expect("sections does not have section id from section ids")
                .as_mut()
                .drain(offset..offset + size);

            Self::set_offset(&mut self.offsets, *section_id, *offset_index, 0);
            self.indices_and_sizes.remove(index);
        }
    }

    /// Removes all values. Does nothing when called multiple times in
    /// succession.
    pub fn delete_all(&mut self) {
        for EntryData {
            section_id,
            offset_index,
        } in &self.indices_and_sizes
        {
            let (offset, size) = MutableMultiValue::index_and_size(&self.offsets, *section_id, *offset_index);
            if size > 0 {
                self.section
                    .get_mut(section_id)
                    .expect("sections does not have section id from section ids")
                    .as_mut()
                    .drain(offset..offset + size);
                Self::set_offset(&mut self.offsets, *section_id, *offset_index, 0);
            }
        }
        self.indices_and_sizes.clear();
    }

    // SectionId is the same size as a reference, which means it's just as
    // efficient passing in a value instead of a reference.
    fn index_and_size(
        offsets: &'lookup HashMap<SectionId, Vec<usize>>,
        section_id: SectionId,
        offset_index: usize,
    ) -> (usize, usize) {
        offsets
            .get(&section_id)
            .expect("sections does not have section id from section ids")
            .iter()
            .take(offset_index + 1)
            .fold((0, 0), |(old, new), offset| (old + new, *offset))
    }

    // This must be an associated function rather than a method to allow Rust
    // to split mutable borrows.
    //
    // SectionId is the same size as a reference, which means it's just as
    // efficient passing in a value instead of a reference.
    fn set_offset(
        offsets: &mut HashMap<SectionId, Vec<usize>>,
        section_id: SectionId,
        offset_index: usize,
        value: usize,
    ) {
        *offsets
            .get_mut(&section_id)
            .expect("sections does not have section id from section ids")
            .get_mut(offset_index)
            .unwrap()
            .deref_mut() = value;
    }
}
