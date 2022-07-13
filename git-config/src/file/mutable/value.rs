use std::{borrow::Cow, collections::HashMap, ops::DerefMut};

use bstr::{BStr, BString, ByteVec};

use crate::file::mutable::section::{MutableSection, SectionBody};
use crate::{
    file::{Index, SectionBodyId, Size},
    lookup,
    parse::{section, Event},
    value::{normalize_bstr, normalize_bstring},
};

/// An intermediate representation of a mutable value obtained from a [`File`][crate::File].
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MutableValue<'borrow, 'lookup, 'event> {
    pub(crate) section: MutableSection<'borrow, 'event>,
    pub(crate) key: section::Key<'lookup>,
    pub(crate) index: Index,
    pub(crate) size: Size,
}

impl<'borrow, 'lookup, 'event> MutableValue<'borrow, 'lookup, 'event> {
    /// Returns the actual value. This is computed each time this is called
    /// requiring an allocation for multi-line values.
    pub fn get(&self) -> Result<Cow<'_, BStr>, lookup::existing::Error> {
        self.section.get(&self.key, self.index, self.index + self.size)
    }

    /// Update the value to the provided one. This modifies the value such that
    /// the Value event(s) are replaced with a single new event containing the
    /// new value.
    pub fn set_string(&mut self, input: impl Into<String>) {
        self.set_bytes(input.into().into());
    }

    /// Update the value to the provided one. This modifies the value such that
    /// the Value event(s) are replaced with a single new event containing the
    /// new value.
    pub fn set_bytes(&mut self, input: BString) {
        if self.size.0 > 0 {
            self.section.delete(self.index, self.index + self.size);
        }
        self.size = self.section.set_internal(self.index, self.key.to_owned(), input);
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
    pub(crate) section_id: SectionBodyId,
    pub(crate) offset_index: usize,
}

/// An intermediate representation of a mutable multivar obtained from a [`File`][crate::File].
#[derive(PartialEq, Eq, Debug)]
pub struct MutableMultiValue<'borrow, 'lookup, 'event> {
    pub(crate) section: &'borrow mut HashMap<SectionBodyId, SectionBody<'event>>,
    pub(crate) key: section::Key<'lookup>,
    /// Each entry data struct provides sufficient information to index into
    /// [`Self::offsets`]. This layer of indirection is used for users to index
    /// into the offsets rather than leaking the internal data structures.
    pub(crate) indices_and_sizes: Vec<EntryData>,
    /// Each offset represents the size of a event slice and whether or not the
    /// event slice is significant or not. This is used to index into the
    /// actual section.
    pub(crate) offsets: HashMap<SectionBodyId, Vec<usize>>,
}

impl<'borrow, 'lookup, 'event> MutableMultiValue<'borrow, 'lookup, 'event> {
    /// Returns the actual values.
    pub fn get(&self) -> Result<Vec<Cow<'_, BStr>>, lookup::existing::Error> {
        let mut expect_value = false;
        let mut values = Vec::new();
        let mut concatenated_value = BString::default();

        for EntryData {
            section_id,
            offset_index,
        } in &self.indices_and_sizes
        {
            let (offset, size) = MutableMultiValue::index_and_size(&self.offsets, *section_id, *offset_index);
            for event in &self.section.get(section_id).expect("known section id").as_ref()[offset..offset + size] {
                match event {
                    Event::SectionKey(section_key) if *section_key == self.key => expect_value = true,
                    Event::Value(v) if expect_value => {
                        expect_value = false;
                        values.push(normalize_bstr(v.as_ref()));
                    }
                    Event::ValueNotDone(v) if expect_value => concatenated_value.push_str(v.as_ref()),
                    Event::ValueDone(v) if expect_value => {
                        expect_value = false;
                        concatenated_value.push_str(v.as_ref());
                        values.push(normalize_bstring(std::mem::take(&mut concatenated_value)));
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

    /// Returns the amount of values within this multivar.
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices_and_sizes.len()
    }

    /// Returns true if the multivar does not have any values.
    /// This might occur if the value was deleted but wasn't yet set with a new value.
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
    pub fn set_value(&mut self, index: usize, input: Cow<'event, BStr>) {
        let EntryData {
            section_id,
            offset_index,
        } = self.indices_and_sizes[index];
        MutableMultiValue::set_value_inner(
            &self.key,
            &mut self.offsets,
            self.section.get_mut(&section_id).expect("known section id"),
            section_id,
            offset_index,
            input,
        );
    }

    /// Sets all values to the provided ones. Note that this follows [`zip`]
    /// logic: if the number of values in the input is less than the number of
    /// values currently existing, then only the first `n` values are modified.
    /// If more values are provided than there currently are, then the
    /// remaining values are ignored.
    ///
    /// [`zip`]: std::iter::Iterator::zip
    pub fn set_values(&mut self, input: impl IntoIterator<Item = Cow<'event, BStr>>) {
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
                self.section.get_mut(section_id).expect("known section id"),
                *section_id,
                *offset_index,
                value,
            );
        }
    }

    /// Sets all values in this multivar to the provided one by copying the
    /// `input` string to all values.
    pub fn set_str_all(&mut self, input: &str) {
        self.set_owned_values_all(input);
    }

    /// Sets all values in this multivar to the provided one by copying the
    /// `input` bytes to all values.
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
                self.section.get_mut(section_id).expect("known section id"),
                *section_id,
                *offset_index,
                Cow::Owned(input.to_owned()),
            );
        }
    }

    /// Sets all values in this multivar to the provided one without owning the
    /// provided input. Consider using [`Self::set_owned_values_all`] or
    /// [`Self::set_str_all`] unless you have a strict performance or memory
    /// need for a more ergonomic interface.
    ///
    /// [`File`]: crate::File
    pub fn set_values_all(&mut self, input: &'event BStr) {
        for EntryData {
            section_id,
            offset_index,
        } in &self.indices_and_sizes
        {
            Self::set_value_inner(
                &self.key,
                &mut self.offsets,
                self.section.get_mut(section_id).expect("known section id"),
                *section_id,
                *offset_index,
                Cow::Borrowed(input),
            );
        }
    }

    fn set_value_inner<'a: 'event>(
        key: &section::Key<'lookup>,
        offsets: &mut HashMap<SectionBodyId, Vec<usize>>,
        section: &mut SectionBody<'event>,
        section_id: SectionBodyId,
        offset_index: usize,
        input: Cow<'a, BStr>,
    ) {
        let (offset, size) = MutableMultiValue::index_and_size(offsets, section_id, offset_index);
        let section = section.as_mut();
        section.drain(offset..offset + size);

        MutableMultiValue::set_offset(offsets, section_id, offset_index, 3);
        section.insert(offset, Event::Value(input));
        section.insert(offset, Event::KeyValueSeparator);
        section.insert(offset, Event::SectionKey(key.to_owned()));
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
        if size == 0 {
            return;
        }
        self.section
            .get_mut(section_id)
            .expect("known section id")
            .as_mut()
            .drain(offset..offset + size);

        Self::set_offset(&mut self.offsets, *section_id, *offset_index, 0);
        self.indices_and_sizes.remove(index);
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
            if size == 0 {
                continue;
            }
            self.section
                .get_mut(section_id)
                .expect("known section id")
                .as_mut()
                .drain(offset..offset + size);
            Self::set_offset(&mut self.offsets, *section_id, *offset_index, 0);
        }
        self.indices_and_sizes.clear();
    }

    fn index_and_size(
        offsets: &'lookup HashMap<SectionBodyId, Vec<usize>>,
        section_id: SectionBodyId,
        offset_index: usize,
    ) -> (usize, usize) {
        offsets
            .get(&section_id)
            .expect("known section id")
            .iter()
            .take(offset_index + 1)
            .fold((0, 0), |(total_ofs, ofs), size| (total_ofs + ofs, *size))
    }

    // This must be an associated function rather than a method to allow Rust
    // to split mutable borrows.
    fn set_offset(
        offsets: &mut HashMap<SectionBodyId, Vec<usize>>,
        section_id: SectionBodyId,
        offset_index: usize,
        value: usize,
    ) {
        *offsets
            .get_mut(&section_id)
            .expect("known section id")
            .get_mut(offset_index)
            .unwrap()
            .deref_mut() = value;
    }
}
