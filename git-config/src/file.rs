//! This module provides a high level wrapper around a single `git-config` file.

use crate::parser::{
    parse_from_bytes, parse_from_str, Error, Event, Key, ParsedSectionHeader, Parser, SectionHeaderName,
};
use crate::values::{normalize_bytes, normalize_cow, normalize_vec};
use std::borrow::{Borrow, Cow};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::{Deref, DerefMut, Range};

/// All possible error types that may occur from interacting with [`GitConfig`].
#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Debug)]
pub enum GitConfigError<'a> {
    /// The requested section does not exist.
    SectionDoesNotExist(SectionHeaderName<'a>),
    /// The requested subsection does not exist.
    SubSectionDoesNotExist(Option<&'a str>),
    /// The key does not exist in the requested section.
    KeyDoesNotExist,
    /// The conversion into the provided type for methods such as
    /// [`GitConfig::value`] failed.
    FailedConversion,
}

impl Display for GitConfigError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SectionDoesNotExist(s) => write!(f, "Section '{}' does not exist.", s),
            Self::SubSectionDoesNotExist(s) => match s {
                Some(s) => write!(f, "Subsection '{}' does not exist.", s),
                None => write!(f, "Top level section does not exist."),
            },
            Self::KeyDoesNotExist => write!(f, "The name for a value provided does not exist."),
            Self::FailedConversion => write!(f, "Failed to convert to specified type."),
        }
    }
}

impl std::error::Error for GitConfigError<'_> {}

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
    fn new(section: &'borrow mut SectionBody<'event>) -> Self {
        Self {
            section,
            implicit_newline: true,
            whitespace: 2,
        }
    }

    fn get<'key>(&self, key: &Key<'key>, start: usize, end: usize) -> Result<Cow<'_, [u8]>, GitConfigError<'key>> {
        let mut found_key = false;
        let mut latest_value = None;
        let mut partial_value = None;
        // section_id is guaranteed to exist in self.sections, else we have a
        // violated invariant.

        for event in &self.section.0[start..=end] {
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
    fn delete(&mut self, start: usize, end: usize) {
        self.section.0.drain(start..=end);
    }

    fn set_internal(&mut self, index: usize, key: Key<'event>, value: Vec<u8>) {
        self.section.0.insert(index, Event::Value(Cow::Owned(value)));
        self.section.0.insert(index, Event::KeyValueSeparator);
        self.section.0.insert(index, Event::Key(key));
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
    /// Constructs a new empty section body.
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    /// Retrieves the last matching value in a section with the given key.
    /// Returns None if the key was not found.
    // We hit this lint because of the unreachable!() call may panic, but this
    // is a clippy bug (rust-clippy#6699), so we allow this lint for this
    // function.
    #[allow(clippy::clippy::missing_panics_doc)]
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
        for e in &self.0 {
            if let Event::Key(k) = e {
                if k == key {
                    return true;
                }
            }
        }
        false
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

impl<'event> From<Vec<Event<'event>>> for SectionBody<'event> {
    #[inline]
    fn from(e: Vec<Event<'event>>) -> Self {
        Self(e)
    }
}

/// The section ID is a monotonically increasing ID used to refer to sections.
/// This value does not imply any ordering between sections, as new sections
/// with higher section IDs may be in between lower ID sections.
///
/// We need to use a section id because `git-config` permits sections with
/// identical names. As a result, we can't simply use the section name as a key
/// in a map.
///
/// This id guaranteed to be unique, but not guaranteed to be compact. In other
/// words, it's possible that a section may have an ID of 3 but the next section
/// has an ID of 5.
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord, Debug)]
struct SectionId(usize);

/// Internal data structure for the section id lookup tree used by
/// [`GitConfig`]. Note that order in Vec matters as it represents the order
/// of section ids with the matched section and name, and is used for precedence
/// management.
#[derive(PartialEq, Eq, Clone, Debug)]
enum LookupTreeNode<'a> {
    Terminal(Vec<SectionId>),
    NonTerminal(HashMap<Cow<'a, str>, Vec<SectionId>>),
}

/// High level `git-config` reader and writer.
///
/// Internally, this uses various acceleration data structures to improve
/// performance of the typical usage behavior of many lookups and relatively
/// fewer insertions.
///
/// # Multivar behavior
///
/// `git` is flexible enough to allow users to set a key multiple times in
/// any number of identically named sections. When this is the case, the key
/// is known as a "multivar". In this case, `get_raw_value` follows the
/// "last one wins" approach that `git-config` internally uses for multivar
/// resolution.
///
/// Concretely, the following config has a multivar, `a`, with the values
/// of `b`, `c`, and `d`, while `e` is a single variable with the value
/// `f g h`.
///
/// ```text
/// [core]
///     a = b
///     a = c
/// [core]
///     a = d
///     e = f g h
/// ```
///
/// Calling methods that fetch or set only one value (such as [`get_raw_value`])
/// key `a` with the above config will fetch `d` or replace `d`, since the last
/// valid config key/value pair is `a = d`:
///
/// ```
/// # use git_config::file::GitConfig;
/// # use std::borrow::Cow;
/// # use std::convert::TryFrom;
/// # let git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
/// assert_eq!(git_config.get_raw_value("core", None, "a"), Ok(Cow::Borrowed("d".as_bytes())));
/// ```
///
/// Consider the `multi` variants of the methods instead, if you want to work
/// with all values instead.
///
/// [`get_raw_value`]: Self::get_raw_value
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct GitConfig<'event> {
    /// The list of events that occur before an actual section. Since a
    /// `git-config` file prohibits global values, this vec is limited to only
    /// comment, newline, and whitespace events.
    frontmatter_events: SectionBody<'event>,
    /// Section name and subsection name to section id lookup tree. This is
    /// effectively a n-tree (opposed to a binary tree) that can have a height
    /// of at most three (including an implicit root node).
    section_lookup_tree: HashMap<SectionHeaderName<'event>, Vec<LookupTreeNode<'event>>>,
    /// SectionId to section mapping. The value of this HashMap contains actual
    /// events.
    ///
    /// This indirection with the SectionId as the key is critical to flexibly
    /// supporting `git-config` sections, as duplicated keys are permitted.
    sections: HashMap<SectionId, SectionBody<'event>>,
    section_headers: HashMap<SectionId, ParsedSectionHeader<'event>>,
    /// Internal monotonically increasing counter for section ids.
    section_id_counter: usize,
    /// Section order for output ordering.
    section_order: VecDeque<SectionId>,
}

impl<'event> GitConfig<'event> {
    /// Constructs an empty `git-config` file.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an interpreted value given a section, an optional subsection and
    /// key.
    ///
    /// It's recommended to use one of the values in the [`values`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::multi_value`] if you want to get all values of a
    /// multivar instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Value, Boolean};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = 10k
    ///         c
    /// "#;
    /// let git_config = GitConfig::try_from(config)?;
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.value::<Integer>("core", None, "a")?;
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Boolean = git_config.value("core", None, "c")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`values`]: crate::values
    /// [`TryFrom`]: std::convert::TryFrom
    #[inline]
    pub fn value<'lookup, T: TryFrom<Cow<'event, [u8]>>>(
        &'event self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<T, GitConfigError<'lookup>> {
        T::try_from(self.get_raw_value(section_name, subsection_name, key)?)
            .map_err(|_| GitConfigError::FailedConversion)
    }

    /// Returns all interpreted values given a section, an optional subsection
    /// and key.
    ///
    /// It's recommended to use one of the values in the [`values`] module as
    /// the conversion is already implemented, but this function is flexible and
    /// will accept any type that implements [`TryFrom<&[u8]>`][`TryFrom`].
    ///
    /// Consider [`Self::value`] if you want to get a single value
    /// (following last-one-wins resolution) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Value, Boolean, TrueVariant};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = true
    ///         c = g
    ///     [core]
    ///         a
    ///         a = false
    /// "#;
    /// let git_config = GitConfig::try_from(config).unwrap();
    /// // You can either use the turbofish to determine the type...
    /// let a_value = git_config.multi_value::<Boolean>("core", None, "a")?;
    /// assert_eq!(
    ///     a_value,
    ///     vec![
    ///         Boolean::True(TrueVariant::Explicit(Cow::Borrowed("true"))),
    ///         Boolean::True(TrueVariant::Implicit),
    ///         Boolean::False(Cow::Borrowed("false")),
    ///     ]
    /// );
    /// // ... or explicitly declare the type to avoid the turbofish
    /// let c_value: Vec<Value> = git_config.multi_value("core", None, "c")?;
    /// assert_eq!(c_value, vec![Value::Other(Cow::Borrowed(b"g"))]);
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, if the section and subsection do not exist, or
    /// if there was an issue converting the type into the requested variant.
    ///
    /// [`values`]: crate::values
    /// [`TryFrom`]: std::convert::TryFrom
    #[inline]
    pub fn multi_value<'lookup, T: TryFrom<Cow<'event, [u8]>>>(
        &'event self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Vec<T>, GitConfigError<'lookup>> {
        self.get_raw_multi_value(section_name, subsection_name, key)?
            .into_iter()
            .map(T::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| GitConfigError::FailedConversion)
    }

    /// Returns an immutable section reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section and optional
    /// subsection do not exist.
    pub fn section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
    ) -> Result<&SectionBody<'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let id = section_ids
            .last()
            .expect("Section lookup vec was empty, internal invariant violated");
        Ok(self
            .sections
            .get(id)
            .expect("Section did not have id from lookup, internal invariant violated"))
    }

    /// Returns an mutable section reference.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section and optional
    /// subsection do not exist.
    pub fn section_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
    ) -> Result<MutableSection<'_, 'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let id = section_ids
            .last()
            .expect("Section lookup vec was empty, internal invariant violated");
        Ok(MutableSection::new(self.sections.get_mut(id).expect(
            "Section did not have id from lookup, internal invariant violated",
        )))
    }

    /// Gets all sections that match the provided name, ignoring any subsections.
    ///
    /// # Examples
    ///
    /// Provided the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core ""]
    ///     c = d
    /// [core "apple"]
    ///     e = f
    /// ```
    ///
    /// Calling this method will yield all sections:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use git_config::values::{Integer, Value, Boolean, TrueVariant};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// let config = r#"
    ///     [core]
    ///         a = b
    ///     [core ""]
    ///         c = d
    ///     [core "apple"]
    ///         e = f
    /// "#;
    /// let git_config = GitConfig::try_from(config).unwrap();
    /// assert_eq!(git_config.sections_by_name("core").len(), 3);
    /// ```
    #[must_use]
    pub fn sections_by_name<'lookup>(&self, section_name: &'lookup str) -> Vec<&SectionBody<'event>> {
        self.get_section_ids_by_name(section_name)
            .unwrap_or_default()
            .into_iter()
            .map(|id| {
                self.sections
                    .get(&id)
                    .expect("section doesn't have id from from lookup")
            })
            .collect()
    }

    /// Adds a new section to config. If a subsection name was provided, then
    /// the generated header will use the modern subsection syntax. Returns a
    /// reference to the new section for immediate editing.
    ///
    /// # Examples
    ///
    /// Creating a new empty section:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::new();
    /// let _section = git_config.new_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n");
    /// ```
    ///
    /// Creating a new empty section and adding values to it:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::new();
    /// let mut section = git_config.new_section("hello", Some("world".into()));
    /// section.push("a".into(), "b".as_bytes().into());
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n  a=b\n");
    /// let _section = git_config.new_section("core", None);
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n  a=b\n[core]\n");
    /// ```
    pub fn new_section(
        &mut self,
        section_name: impl Into<Cow<'event, str>>,
        subsection_name: impl Into<Option<Cow<'event, str>>>,
    ) -> MutableSection<'_, 'event> {
        let mut section = self.push_section(section_name, subsection_name, SectionBody::new());
        section.push_newline();
        section
    }

    /// Removes the section, returning the events it had, if any. If multiple
    /// sections have the same name, then the last one is returned. Note that
    /// later sections with the same name have precedent over earlier ones.
    ///
    /// # Examples
    ///
    /// Creating and removing a section:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::try_from(
    /// r#"[hello "world"]
    ///     some-value = 4
    /// "#).unwrap();
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "");
    /// ```
    ///
    /// Precedence example for removing sections with the same name:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::convert::TryFrom;
    /// let mut git_config = GitConfig::try_from(
    /// r#"[hello "world"]
    ///     some-value = 4
    /// [hello "world"]
    ///     some-value = 5
    /// "#).unwrap();
    ///
    /// let events = git_config.remove_section("hello", Some("world".into()));
    /// assert_eq!(git_config.to_string(), "[hello \"world\"]\n    some-value = 4\n");
    /// ```
    pub fn remove_section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: impl Into<Option<&'lookup str>>,
    ) -> Option<SectionBody> {
        let id = self
            .get_section_ids_by_name_and_subname(section_name, subsection_name.into())
            .ok()?
            .pop()?;
        self.section_order.remove(
            self.section_order
                .iter()
                .position(|v| *v == id)
                .expect("Section order does not contain section that we were trying to remove"),
        );
        self.sections.remove(&id)
    }

    /// Adds the provided section to the config, returning a mutable reference
    /// to it.
    pub fn push_section(
        &mut self,
        section_name: impl Into<Cow<'event, str>>,
        subsection_name: impl Into<Option<Cow<'event, str>>>,
        section: SectionBody<'event>,
    ) -> MutableSection<'_, 'event> {
        let subsection_name = subsection_name.into();
        if subsection_name.is_some() {
            self.push_section_internal(
                ParsedSectionHeader {
                    name: SectionHeaderName(section_name.into()),
                    separator: Some(" ".into()),
                    subsection_name,
                },
                section,
            )
        } else {
            self.push_section_internal(
                ParsedSectionHeader {
                    name: SectionHeaderName(section_name.into()),
                    separator: None,
                    subsection_name: None,
                },
                section,
            )
        }
    }

    /// Renames a section, modifying the last matching section.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup doesn't exist
    pub fn rename_section<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: impl Into<Option<&'lookup str>>,
        new_section_name: impl Into<SectionHeaderName<'event>>,
        new_subsection_name: impl Into<Option<Cow<'event, str>>>,
    ) -> Result<(), GitConfigError<'lookup>> {
        let id = self.get_section_ids_by_name_and_subname(section_name, subsection_name.into())?;
        let id = id
            .last()
            .expect("list of sections were empty, which violates invariant");
        let header = self
            .section_headers
            .get_mut(id)
            .expect("sections does not have section id from section ids");
        header.name = new_section_name.into();
        header.subsection_name = new_subsection_name.into();

        Ok(())
    }
}

/// An intermediate representation of a mutable value obtained from
/// [`GitConfig`].
///
/// This holds a mutable reference to the underlying data structure of
/// [`GitConfig`], and thus guarantees through Rust's borrower checker that
/// multiple mutable references to [`GitConfig`] cannot be owned at the same
/// time.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MutableValue<'borrow, 'lookup, 'event> {
    section: MutableSection<'borrow, 'event>,
    key: Key<'lookup>,
    index: usize,
    size: usize,
}

impl MutableValue<'_, '_, '_> {
    /// Returns the actual value. This is computed each time this is called, so
    /// it's best to reuse this value or own it if an allocation is acceptable.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup failed.
    #[inline]
    pub fn get(&self) -> Result<Cow<'_, [u8]>, GitConfigError> {
        self.section.get(&self.key, self.index, self.index + self.size)
    }

    /// Update the value to the provided one. This modifies the value such that
    /// the Value event(s) are replaced with a single new event containing the
    /// new value.
    #[inline]
    pub fn set_string(&mut self, input: String) {
        self.set_bytes(input.into_bytes());
    }

    /// Update the value to the provided one. This modifies the value such that
    /// the Value event(s) are replaced with a single new event containing the
    /// new value.
    pub fn set_bytes(&mut self, input: Vec<u8>) {
        if self.size > 0 {
            self.section.delete(self.index, self.index + self.size);
        }
        self.size = 3;
        self.section
            .set_internal(self.index, Key(Cow::Owned(self.key.to_string())), input);
    }

    /// Removes the value. Does nothing when called multiple times in
    /// succession.
    pub fn delete(&mut self) {
        if self.size > 0 {
            self.section.delete(self.index, self.index + self.size);
            self.size = 0;
        }
    }
}

/// Internal data structure for [`MutableMultiValue`]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct EntryData {
    section_id: SectionId,
    offset_index: usize,
}

/// An intermediate representation of a mutable multivar obtained from
/// [`GitConfig`].
///
/// This holds a mutable reference to the underlying data structure of
/// [`GitConfig`], and thus guarantees through Rust's borrower checker that
/// multiple mutable references to [`GitConfig`] cannot be owned at the same
/// time.
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

impl<'lookup, 'event> MutableMultiValue<'_, 'lookup, 'event> {
    /// Returns the actual values. This is computed each time this is called.
    ///
    /// # Errors
    ///
    /// Returns an error if the lookup failed.
    pub fn get(&self) -> Result<Vec<Cow<'_, [u8]>>, GitConfigError> {
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
            let (offset, size) = MutableMultiValue::get_index_and_size(&self.offsets, *section_id, *offset_index);
            for event in &self
                .section
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .0[offset..offset + size]
            {
                match event {
                    Event::Key(event_key) if *event_key == self.key => found_key = true,
                    Event::Value(v) if found_key => {
                        found_key = false;
                        values.push(normalize_bytes(v.borrow()));
                    }
                    Event::ValueNotDone(v) if found_key => {
                        partial_value = Some((*v).to_vec());
                    }
                    Event::ValueDone(v) if found_key => {
                        found_key = false;
                        let mut value = partial_value
                            .take()
                            .expect("Somehow got ValueDone before ValueNotDone event");
                        value.extend(&**v);
                        values.push(normalize_vec(value));
                    }
                    _ => (),
                }
            }
        }

        if values.is_empty() {
            return Err(GitConfigError::KeyDoesNotExist);
        }

        Ok(values)
    }

    /// Returns the size of values the multivar has.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices_and_sizes.len()
    }

    /// Returns if the multivar has any values. This might occur if the value
    /// was deleted but not set with a new value.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices_and_sizes.is_empty()
    }

    /// Sets the value at the given index.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    #[inline]
    pub fn set_string(&mut self, index: usize, input: String) {
        self.set_bytes(index, input.into_bytes())
    }

    /// Sets the value at the given index.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    #[inline]
    pub fn set_bytes(&mut self, index: usize, input: Vec<u8>) {
        self.set_value(index, Cow::Owned(input))
    }

    /// Sets the value at the given index.
    ///
    /// # Safety
    ///
    /// This will panic if the index is out of range.
    pub fn set_value<'a: 'event>(&mut self, index: usize, input: Cow<'a, [u8]>) {
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
    #[inline]
    pub fn set_values<'a: 'event>(&mut self, input: impl Iterator<Item = Cow<'a, [u8]>>) {
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
    #[inline]
    pub fn set_str_all(&mut self, input: &str) {
        self.set_owned_values_all(input.as_bytes())
    }

    /// Sets all values in this multivar to the provided one by copying the
    /// input bytes for all values.
    #[inline]
    pub fn set_owned_values_all(&mut self, input: &[u8]) {
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
                Cow::Owned(input.to_vec()),
            );
        }
    }

    /// Sets all values in this multivar to the provided one without owning the
    /// provided input. Note that this requires `input` to last longer than
    /// [`GitConfig`]. Consider using [`Self::set_owned_values_all`] or
    /// [`Self::set_str_all`] unless you have a strict performance or memory
    /// need for a more ergonomic interface.
    #[inline]
    pub fn set_values_all<'a: 'event>(&mut self, input: &'a [u8]) {
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
        input: Cow<'a, [u8]>,
    ) {
        let (offset, size) = MutableMultiValue::get_index_and_size(offsets, section_id, offset_index);
        section.0.drain(offset..offset + size);

        MutableMultiValue::set_offset(offsets, section_id, offset_index, 3);
        section.0.insert(offset, Event::Value(input));
        section.0.insert(offset, Event::KeyValueSeparator);
        section.0.insert(offset, Event::Key(Key(Cow::Owned(key.0.to_string()))));
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
        let (offset, size) = MutableMultiValue::get_index_and_size(&self.offsets, *section_id, *offset_index);
        if size > 0 {
            self.section
                .get_mut(section_id)
                .expect("sections does not have section id from section ids")
                .0
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
            let (offset, size) = MutableMultiValue::get_index_and_size(&self.offsets, *section_id, *offset_index);
            if size > 0 {
                self.section
                    .get_mut(section_id)
                    .expect("sections does not have section id from section ids")
                    .0
                    .drain(offset..offset + size);
                Self::set_offset(&mut self.offsets, *section_id, *offset_index, 0);
            }
        }
        self.indices_and_sizes.clear();
    }

    // SectionId is the same size as a reference, which means it's just as
    // efficient passing in a value instead of a reference.
    #[inline]
    fn get_index_and_size(
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
    #[inline]
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

/// # Raw value API
///
/// These functions are the raw value API. Instead of returning Rust structures,
/// these functions return bytes which may or may not be owned.
impl<'event> GitConfig<'event> {
    /// Returns an uninterpreted value given a section, an optional subsection
    /// and key.
    ///
    /// Consider [`Self::get_raw_multi_value`] if you want to get all values of
    /// a multivar instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, or if the section and subsection do not exist.
    pub fn get_raw_value<'lookup>(
        &self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Cow<'_, [u8]>, GitConfigError<'lookup>> {
        // Note: cannot wrap around the raw_multi_value method because we need
        // to guarantee that the highest section id is used (so that we follow
        // the "last one wins" resolution strategy by `git-config`).
        let key = Key(key.into());
        for section_id in self
            .get_section_ids_by_name_and_subname(section_name, subsection_name)?
            .iter()
            .rev()
        {
            if let Some(v) = self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .value(&key)
            {
                return Ok(v.to_vec().into());
            }
        }

        Err(GitConfigError::KeyDoesNotExist)
    }

    /// Returns a mutable reference to an uninterpreted value given a section,
    /// an optional subsection and key.
    ///
    /// Consider [`Self::get_raw_multi_value_mut`] if you want to get mutable
    /// references to all values of a multivar instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in the requested
    /// section and subsection, or if the section and subsection do not exist.
    pub fn get_raw_value_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<MutableValue<'_, 'lookup, 'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let key = Key(key.into());

        for section_id in section_ids.iter().rev() {
            let mut size = 0;
            let mut index = 0;
            let mut found_key = false;
            // todo: iter backwards
            for (i, event) in self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .0
                .iter()
                .enumerate()
            {
                match event {
                    Event::Key(event_key) if *event_key == key => {
                        found_key = true;
                        size = 1;
                        index = i;
                    }
                    Event::Newline(_) | Event::Whitespace(_) | Event::ValueNotDone(_) if found_key => {
                        size += 1;
                    }
                    Event::ValueDone(_) | Event::Value(_) if found_key => {
                        found_key = false;
                        size += 1;
                    }
                    _ => (),
                }
            }

            if size == 0 {
                continue;
            }

            return Ok(MutableValue {
                section: MutableSection::new(
                    self.sections
                        .get_mut(section_id)
                        .expect("sections does not have section id from section ids"),
                ),
                key,
                size,
                index,
            });
        }

        Err(GitConfigError::KeyDoesNotExist)
    }

    /// Returns all uninterpreted values given a section, an optional subsection
    /// and key.
    ///
    /// # Examples
    ///
    /// If you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Attempting to get all values of `a` yields the following:
    ///
    /// ```
    /// # use git_config::file::GitConfig;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a"),
    ///     Ok(vec![
    ///         Cow::<[u8]>::Borrowed(b"b"),
    ///         Cow::<[u8]>::Borrowed(b"c"),
    ///         Cow::<[u8]>::Borrowed(b"d"),
    ///     ]),
    /// );
    /// ```
    ///
    /// Consider [`Self::get_raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in any requested
    /// section and subsection, or if no instance of the section and subsections
    /// exist.
    pub fn get_raw_multi_value<'lookup>(
        &self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<Vec<Cow<'_, [u8]>>, GitConfigError<'lookup>> {
        let mut values = vec![];
        for section_id in self.get_section_ids_by_name_and_subname(section_name, subsection_name)? {
            values.extend(
                self.sections
                    .get(&section_id)
                    .expect("sections does not have section id from section ids")
                    .values(&Key(key.into()))
                    .iter()
                    .map(|v| Cow::Owned(v.to_vec())),
            );
        }

        if values.is_empty() {
            Err(GitConfigError::KeyDoesNotExist)
        } else {
            Ok(values)
        }
    }

    /// Returns mutable references to all uninterpreted values given a section,
    /// an optional subsection and key.
    ///
    /// # Examples
    ///
    /// If you have the following config:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Attempting to get all values of `a` yields the following:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![
    ///         Cow::Borrowed(b"b"),
    ///         Cow::Borrowed(b"c"),
    ///         Cow::Borrowed(b"d")
    ///     ]
    /// );
    ///
    /// git_config.get_raw_multi_value_mut("core", None, "a")?.set_str_all("g");
    ///
    /// assert_eq!(
    ///     git_config.get_raw_multi_value("core", None, "a")?,
    ///     vec![
    ///         Cow::Borrowed(b"g"),
    ///         Cow::Borrowed(b"g"),
    ///         Cow::Borrowed(b"g")
    ///     ],
    /// );
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Consider [`Self::get_raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// Note that this operation is relatively expensive, requiring a full
    /// traversal of the config.
    ///
    /// # Errors
    ///
    /// This function will return an error if the key is not in any requested
    /// section and subsection, or if no instance of the section and subsections
    /// exist.
    pub fn get_raw_multi_value_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<MutableMultiValue<'_, 'lookup, 'event>, GitConfigError<'lookup>> {
        let section_ids = self.get_section_ids_by_name_and_subname(section_name, subsection_name)?;
        let key = Key(key.into());

        let mut offsets = HashMap::new();
        let mut entries = vec![];
        for section_id in section_ids.iter().rev() {
            let mut last_boundary = 0;
            let mut found_key = false;
            let mut offset_list = vec![];
            let mut offset_index = 0;
            for (i, event) in self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .0
                .iter()
                .enumerate()
            {
                match event {
                    Event::Key(event_key) if *event_key == key => {
                        found_key = true;
                        offset_list.push(i - last_boundary);
                        offset_index += 1;
                        last_boundary = i;
                    }
                    Event::Value(_) | Event::ValueDone(_) if found_key => {
                        found_key = false;
                        entries.push(EntryData {
                            section_id: *section_id,
                            offset_index,
                        });
                        offset_list.push(i - last_boundary + 1);
                        offset_index += 1;
                        last_boundary = i + 1;
                    }
                    _ => (),
                }
            }
            offsets.insert(*section_id, offset_list);
        }

        entries.sort();

        if entries.is_empty() {
            Err(GitConfigError::KeyDoesNotExist)
        } else {
            Ok(MutableMultiValue {
                section: &mut self.sections,
                key,
                indices_and_sizes: entries,
                offsets,
            })
        }
    }

    /// Sets a value in a given section, optional subsection, and key value.
    ///
    /// # Examples
    ///
    /// Given the config,
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Setting a new value to the key `core.a` will yield the following:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// git_config.set_raw_value("core", None, "a", vec![b'e'])?;
    /// assert_eq!(git_config.get_raw_value("core", None, "a")?, Cow::Borrowed(b"e"));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This errors if any lookup input (section, subsection, and key value) fails.
    pub fn set_raw_value<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
        new_value: Vec<u8>,
    ) -> Result<(), GitConfigError<'lookup>> {
        self.get_raw_value_mut(section_name, subsection_name, key)
            .map(|mut entry| entry.set_bytes(new_value))
    }

    /// Sets a multivar in a given section, optional subsection, and key value.
    ///
    /// This internally zips together the new values and the existing values.
    /// As a result, if more new values are provided than the current amount of
    /// multivars, then the latter values are not applied. If there are less
    /// new values than old ones then the remaining old values are unmodified.
    ///
    /// **Note**: Mutation order is _not_ guaranteed and is non-deterministic.
    /// If you need finer control over which values of the multivar are set,
    /// consider using [`get_raw_multi_value_mut`], which will let you iterate
    /// and check over the values instead. This is best used as a convenience
    /// function for setting multivars whose values should be treated as an
    /// unordered set.
    ///
    /// # Examples
    ///
    /// Let us use the follow config for all examples:
    ///
    /// ```text
    /// [core]
    ///     a = b
    /// [core]
    ///     a = c
    ///     a = d
    /// ```
    ///
    /// Setting an equal number of values:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values: Vec<Cow<'_, [u8]>> = vec![
    ///     Cow::Borrowed(b"x"),
    ///     Cow::Borrowed(b"y"),
    ///     Cow::Borrowed(b"z"),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// let fetched_config = git_config.get_raw_multi_value("core", None, "a")?;
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"x")));
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"y")));
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"z")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Setting less than the number of present values sets the first ones found:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values: Vec<Cow<'_, [u8]>> = vec![
    ///     Cow::Borrowed(b"x"),
    ///     Cow::Borrowed(b"y"),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// let fetched_config = git_config.get_raw_multi_value("core", None, "a")?;
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"x")));
    /// assert!(fetched_config.contains(&Cow::Borrowed(b"y")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// Setting more than the number of present values discards the rest:
    ///
    /// ```
    /// # use git_config::file::{GitConfig, GitConfigError};
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = GitConfig::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values: Vec<Cow<'_, [u8]>> = vec![
    ///     Cow::Borrowed(b"x"),
    ///     Cow::Borrowed(b"y"),
    ///     Cow::Borrowed(b"z"),
    ///     Cow::Borrowed(b"discarded"),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// assert!(!git_config.get_raw_multi_value("core", None, "a")?.contains(&Cow::Borrowed(b"discarded")));
    /// # Ok::<(), GitConfigError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This errors if any lookup input (section, subsection, and key value) fails.
    ///
    /// [`get_raw_multi_value_mut`]: Self::get_raw_multi_value_mut
    pub fn set_raw_multi_value<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
        new_values: impl Iterator<Item = Cow<'event, [u8]>>,
    ) -> Result<(), GitConfigError<'lookup>> {
        self.get_raw_multi_value_mut(section_name, subsection_name, key)
            .map(|mut v| v.set_values(new_values))
    }
}

/// Private helper functions
impl<'event> GitConfig<'event> {
    /// Adds a new section to the config file.
    fn push_section_internal(
        &mut self,
        // current_section_name: Option<SectionHeaderName<'event>>,
        // current_subsection_name: Option<Cow<'event, str>>,
        header: ParsedSectionHeader<'event>,
        section: SectionBody<'event>,
    ) -> MutableSection<'_, 'event> {
        let new_section_id = SectionId(self.section_id_counter);
        self.section_headers.insert(new_section_id, header.clone());
        self.sections.insert(new_section_id, section);
        let lookup = self.section_lookup_tree.entry(header.name).or_default();

        let mut found_node = false;
        if let Some(subsection_name) = header.subsection_name {
            for node in lookup.iter_mut() {
                if let LookupTreeNode::NonTerminal(subsection) = node {
                    found_node = true;
                    subsection
                        // Clones the cow, not the inner borrowed str.
                        .entry(subsection_name.clone())
                        .or_default()
                        .push(new_section_id);
                    break;
                }
            }
            if !found_node {
                let mut map = HashMap::new();
                map.insert(subsection_name, vec![new_section_id]);
                lookup.push(LookupTreeNode::NonTerminal(map));
            }
        } else {
            for node in lookup.iter_mut() {
                if let LookupTreeNode::Terminal(vec) = node {
                    found_node = true;
                    vec.push(new_section_id);
                    break;
                }
            }
            if !found_node {
                lookup.push(LookupTreeNode::Terminal(vec![new_section_id]))
            }
        }
        self.section_order.push_back(new_section_id);
        self.section_id_counter += 1;
        self.sections.get_mut(&new_section_id).map(MutableSection::new).unwrap()
    }

    /// Returns the mapping between section and subsection name to section ids.
    fn get_section_ids_by_name_and_subname<'lookup>(
        &self,
        section_name: impl Into<SectionHeaderName<'lookup>>,
        subsection_name: Option<&'lookup str>,
    ) -> Result<Vec<SectionId>, GitConfigError<'lookup>> {
        let section_name = section_name.into();
        let section_ids = self
            .section_lookup_tree
            .get(&section_name)
            .ok_or(GitConfigError::SectionDoesNotExist(section_name))?;
        let mut maybe_ids = None;
        // Don't simplify if and matches here -- the for loop currently needs
        // `n + 1` checks, while the if and matches will result in the for loop
        // needing `2n` checks.
        if let Some(subsection_name) = subsection_name {
            for node in section_ids {
                if let LookupTreeNode::NonTerminal(subsection_lookup) = node {
                    maybe_ids = subsection_lookup.get(subsection_name);
                    break;
                }
            }
        } else {
            for node in section_ids {
                if let LookupTreeNode::Terminal(subsection_lookup) = node {
                    maybe_ids = Some(subsection_lookup);
                    break;
                }
            }
        }
        maybe_ids
            .map(Vec::to_owned)
            .ok_or(GitConfigError::SubSectionDoesNotExist(subsection_name))
    }

    fn get_section_ids_by_name<'lookup>(
        &self,
        section_name: impl Into<SectionHeaderName<'lookup>>,
    ) -> Result<Vec<SectionId>, GitConfigError<'lookup>> {
        let section_name = section_name.into();
        self.section_lookup_tree
            .get(&section_name)
            .map(|lookup| {
                lookup
                    .iter()
                    .flat_map(|node| match node {
                        LookupTreeNode::Terminal(v) => v.clone(),
                        LookupTreeNode::NonTerminal(v) => v.values().flatten().copied().collect(),
                    })
                    .collect()
            })
            .ok_or(GitConfigError::SectionDoesNotExist(section_name))
    }
}

impl<'a> TryFrom<&'a str> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`GitConfig`]. See [`parse_from_str`] for more information.
    ///
    /// [`parse_from_str`]: crate::parser::parse_from_str
    #[inline]
    fn try_from(s: &'a str) -> Result<GitConfig<'a>, Self::Error> {
        parse_from_str(s).map(Self::from)
    }
}

impl<'a> TryFrom<&'a [u8]> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`GitConfig`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    #[inline]
    fn try_from(value: &'a [u8]) -> Result<GitConfig<'a>, Self::Error> {
        parse_from_bytes(value).map(GitConfig::from)
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for GitConfig<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`GitConfig`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    #[inline]
    fn try_from(value: &'a Vec<u8>) -> Result<GitConfig<'a>, Self::Error> {
        parse_from_bytes(value).map(GitConfig::from)
    }
}

impl<'a> From<Parser<'a>> for GitConfig<'a> {
    fn from(parser: Parser<'a>) -> Self {
        let mut new_self = Self::default();

        // Current section that we're building
        let mut prev_section_header = None;
        let mut section_events = SectionBody::new();

        #[allow(clippy::explicit_into_iter_loop)] // it's not really an iterator (yet), needs streaming iterator support
        for event in parser.into_iter() {
            match event {
                Event::SectionHeader(header) => {
                    if let Some(prev_header) = prev_section_header.take() {
                        new_self.push_section_internal(prev_header, section_events);
                    } else {
                        new_self.frontmatter_events = section_events;
                    }
                    prev_section_header = Some(header);
                    section_events = SectionBody::new();
                }
                e @ Event::Key(_)
                | e @ Event::Value(_)
                | e @ Event::ValueNotDone(_)
                | e @ Event::ValueDone(_)
                | e @ Event::KeyValueSeparator => section_events.0.push(e),
                e @ Event::Comment(_) | e @ Event::Newline(_) | e @ Event::Whitespace(_) => {
                    section_events.0.push(e);
                }
            }
        }

        // The last section doesn't get pushed since we only push if there's a
        // new section header, so we need to call push one more time.
        if let Some(header) = prev_section_header {
            new_self.push_section_internal(header, section_events);
        } else {
            new_self.frontmatter_events = section_events;
        }

        new_self
    }
}

impl From<GitConfig<'_>> for Vec<u8> {
    #[inline]
    fn from(c: GitConfig) -> Self {
        c.into()
    }
}

impl From<&GitConfig<'_>> for Vec<u8> {
    fn from(config: &GitConfig) -> Self {
        let mut value = Self::new();

        for events in &config.frontmatter_events.0 {
            value.extend(events.to_vec());
        }

        for section_id in &config.section_order {
            value.extend(
                config
                    .section_headers
                    .get(section_id)
                    .expect("section_header does not contain section id from section_order")
                    .to_vec(),
            );

            for event in &config
                .sections
                .get(section_id)
                .expect("sections does not contain section id from section_order")
                .0
            {
                value.extend(event.to_vec());
            }
        }

        value
    }
}

impl Display for GitConfig<'_> {
    /// Note that this is a best-effort attempt at printing a `GitConfig`. If
    /// there are non UTF-8 values in your config, this will _NOT_ render as
    /// read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for front_matter in &self.frontmatter_events.0 {
            front_matter.fmt(f)?;
        }

        for section_id in &self.section_order {
            self.section_headers.get(section_id).unwrap().fmt(f)?;
            for event in &self.sections.get(section_id).unwrap().0 {
                event.fmt(f)?;
            }
        }

        Ok(())
    }
}

// todo impl serialize

#[cfg(test)]
mod mutable_value {
    use super::GitConfig;
    use std::convert::TryFrom;

    fn init_config() -> GitConfig<'static> {
        GitConfig::try_from(
            r#"[core]
                a=b"100"
            [core]
                c=d
                e=f"#,
        )
        .unwrap()
    }

    #[test]
    fn value_is_correct() {
        let mut git_config = init_config();

        let value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        assert_eq!(&*value.get().unwrap(), b"b100");
    }

    #[test]
    fn set_string_cleanly_updates() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.set_string("hello world".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e=f"#,
        );

        let mut value = git_config.get_raw_value_mut("core", None, "e").unwrap();
        value.set_string(String::new());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e="#,
        );
    }

    #[test]
    fn delete_value() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );

        let mut value = git_config.get_raw_value_mut("core", None, "c").unwrap();
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]\n                \n                e=f",
        );
    }

    #[test]
    fn get_value_after_deleted() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        assert!(value.get().is_err());
    }

    #[test]
    fn set_string_after_deleted() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        value.delete();
        value.set_string("hello world".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=hello world
            [core]
                c=d
                e=f"#,
        );
    }

    #[test]
    fn subsequent_delete_calls_are_noop() {
        let mut git_config = init_config();

        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        for _ in 0..10 {
            value.delete();
        }
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );
    }

    #[test]
    fn partial_values_are_supported() {
        let mut git_config = GitConfig::try_from(
            r#"[core]
                a=b"100"\
b
            [core]
                c=d
                e=f"#,
        )
        .unwrap();
        let mut value = git_config.get_raw_value_mut("core", None, "a").unwrap();
        assert_eq!(&*value.get().unwrap(), b"b100b");
        value.delete();
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                c=d
                e=f",
        );
    }
}

#[cfg(test)]
mod mutable_multi_value {
    use super::GitConfig;
    use std::{borrow::Cow, convert::TryFrom};

    fn init_config() -> GitConfig<'static> {
        GitConfig::try_from(
            r#"[core]
                a=b"100"
            [core]
                a=d
                a=f"#,
        )
        .unwrap()
    }

    #[test]
    fn value_is_correct() {
        let mut git_config = init_config();

        let value = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        assert_eq!(
            &*value.get().unwrap(),
            vec![
                Cow::<[u8]>::Owned(b"b100".to_vec()),
                Cow::<[u8]>::Borrowed(b"d"),
                Cow::<[u8]>::Borrowed(b"f"),
            ]
        );
    }

    #[test]
    fn non_empty_sizes_are_correct() {
        let mut git_config = init_config();
        assert_eq!(git_config.get_raw_multi_value_mut("core", None, "a").unwrap().len(), 3);
        assert!(!git_config
            .get_raw_multi_value_mut("core", None, "a")
            .unwrap()
            .is_empty());
    }

    #[test]
    fn set_value_at_start() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_string(0, "Hello".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=Hello
            [core]
                a=d
                a=f"#,
        );
    }

    #[test]
    fn set_value_at_end() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_string(2, "Hello".to_string());
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=b"100"
            [core]
                a=d
                a=Hello"#,
        );
    }

    #[test]
    fn set_values_all() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.set_owned_values_all(b"Hello");
        assert_eq!(
            git_config.to_string(),
            r#"[core]
                a=Hello
            [core]
                a=Hello
                a=Hello"#,
        );
    }

    #[test]
    fn delete() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.delete(0);
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]
                a=d
                a=f",
        );
    }

    #[test]
    fn delete_all() {
        let mut git_config = init_config();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();
        values.delete_all();
        assert!(values.get().is_err());
        assert_eq!(
            git_config.to_string(),
            "[core]\n                \n            [core]\n                \n                ",
        );
    }

    #[test]
    fn partial_values_are_supported() {
        let mut git_config = GitConfig::try_from(
            r#"[core]
                a=b\
"100"
            [core]
                a=d\
b
                a=f\
a"#,
        )
        .unwrap();
        let mut values = git_config.get_raw_multi_value_mut("core", None, "a").unwrap();

        assert_eq!(
            &*values.get().unwrap(),
            vec![
                Cow::<[u8]>::Owned(b"b100".to_vec()),
                Cow::<[u8]>::Borrowed(b"db"),
                Cow::<[u8]>::Borrowed(b"fa"),
            ]
        );

        values.delete_all();
        assert!(values.get().is_err());
    }
}

#[cfg(test)]
mod from_parser {
    use super::{Cow, Event, GitConfig, HashMap, LookupTreeNode, SectionBody, SectionId, TryFrom};
    use crate::parser::SectionHeaderName;
    use crate::test_util::{name_event, newline_event, section_header, value_event};

    #[test]
    fn parse_empty() {
        let config = GitConfig::try_from("").unwrap();
        assert!(config.section_headers.is_empty());
        assert_eq!(config.section_id_counter, 0);
        assert!(config.section_lookup_tree.is_empty());
        assert!(config.sections.is_empty());
        assert!(config.section_order.is_empty());
    }

    #[test]
    fn parse_single_section() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                ]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_single_subsection() {
        let mut config = GitConfig::try_from("[core.sub]\na=b\nc=d").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", (".", "sub")));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 1);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            let mut inner_tree = HashMap::new();
            inner_tree.insert(Cow::Borrowed("sub"), vec![SectionId(0)]);
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::NonTerminal(inner_tree)],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                ]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0)]);
    }

    #[test]
    fn parse_multiple_sections() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d\n[other]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map.insert(SectionId(1), section_header("other", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::Terminal(vec![SectionId(0)])],
            );
            tree.insert(
                SectionHeaderName(Cow::Borrowed("other")),
                vec![LookupTreeNode::Terminal(vec![SectionId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                    newline_event(),
                ]),
            );
            sections.insert(
                SectionId(1),
                SectionBody::from(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0), SectionId(1)]);
    }

    #[test]
    fn parse_multiple_duplicate_sections() {
        let mut config = GitConfig::try_from("[core]\na=b\nc=d\n[core]e=f").unwrap();
        let expected_separators = {
            let mut map = HashMap::new();
            map.insert(SectionId(0), section_header("core", None));
            map.insert(SectionId(1), section_header("core", None));
            map
        };
        assert_eq!(config.section_headers, expected_separators);
        assert_eq!(config.section_id_counter, 2);
        let expected_lookup_tree = {
            let mut tree = HashMap::new();
            tree.insert(
                SectionHeaderName(Cow::Borrowed("core")),
                vec![LookupTreeNode::Terminal(vec![SectionId(0), SectionId(1)])],
            );
            tree
        };
        assert_eq!(config.section_lookup_tree, expected_lookup_tree);
        let expected_sections = {
            let mut sections = HashMap::new();
            sections.insert(
                SectionId(0),
                SectionBody::from(vec![
                    newline_event(),
                    name_event("a"),
                    Event::KeyValueSeparator,
                    value_event("b"),
                    newline_event(),
                    name_event("c"),
                    Event::KeyValueSeparator,
                    value_event("d"),
                    newline_event(),
                ]),
            );
            sections.insert(
                SectionId(1),
                SectionBody::from(vec![name_event("e"), Event::KeyValueSeparator, value_event("f")]),
            );
            sections
        };
        assert_eq!(config.sections, expected_sections);
        assert_eq!(config.section_order.make_contiguous(), &[SectionId(0), SectionId(1)]);
    }
}

#[cfg(test)]
mod get_raw_value {
    use super::{Cow, GitConfig, GitConfigError, TryFrom};
    use crate::parser::SectionHeaderName;

    #[test]
    fn single_section() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(config.get_raw_value("core", None, "c"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn last_one_wins_respected_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn last_one_wins_respected_across_section() {
        let config = GitConfig::try_from("[core]\na=b\n[core]\na=d").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"d")));
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist(SectionHeaderName("foo".into())))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist)
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(config.get_raw_value("core", None, "a"), Ok(Cow::<[u8]>::Borrowed(b"b")));
        assert_eq!(
            config.get_raw_value("core", Some("a"), "a"),
            Ok(Cow::<[u8]>::Borrowed(b"c"))
        );
    }
}

#[cfg(test)]
mod get_value {
    use super::{Cow, GitConfig, TryFrom};
    use crate::values::{Boolean, TrueVariant, Value};
    use std::error::Error;

    #[test]
    fn single_section() -> Result<(), Box<dyn Error>> {
        let config = GitConfig::try_from("[core]\na=b\nc").unwrap();
        let first_value: Value = config.value("core", None, "a")?;
        let second_value: Boolean = config.value("core", None, "c")?;

        assert_eq!(first_value, Value::Other(Cow::Borrowed(b"b")));
        assert_eq!(second_value, Boolean::True(TrueVariant::Implicit));

        Ok(())
    }

    #[test]
    fn sections_by_name() {
        let config = r#"
        [core]
            repositoryformatversion = 0
            filemode = true
            bare = false
            logallrefupdates = true
        [remote "origin"]
            url = git@github.com:Byron/gitoxide.git
            fetch = +refs/heads/*:refs/remotes/origin/*
        "#;

        let config = GitConfig::try_from(config).unwrap();
        let value = config.value::<Value>("remote", Some("origin"), "url").unwrap();
        assert_eq!(value, Value::Other(Cow::Borrowed(b"git@github.com:Byron/gitoxide.git")));
    }
}

#[cfg(test)]
mod get_raw_multi_value {
    use super::{Cow, GitConfig, GitConfigError, TryFrom};
    use crate::parser::SectionHeaderName;

    #[test]
    fn single_value_is_identical_to_single_value_query() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            vec![config.get_raw_value("core", None, "a").unwrap()],
            config.get_raw_multi_value("core", None, "a").unwrap()
        );
    }

    #[test]
    fn multi_value_in_section() {
        let config = GitConfig::try_from("[core]\na=b\na=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn multi_value_across_sections() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
        );
    }

    #[test]
    fn section_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("foo", None, "a"),
            Err(GitConfigError::SectionDoesNotExist(SectionHeaderName("foo".into())))
        );
    }

    #[test]
    fn subsection_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a"),
            Err(GitConfigError::SubSectionDoesNotExist(Some("a")))
        );
    }

    #[test]
    fn key_not_found() {
        let config = GitConfig::try_from("[core]\na=b\nc=d").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "aaaaaa"),
            Err(GitConfigError::KeyDoesNotExist)
        );
    }

    #[test]
    fn subsection_must_be_respected() {
        let config = GitConfig::try_from("[core]a=b\n[core.a]a=c").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b")]
        );
        assert_eq!(
            config.get_raw_multi_value("core", Some("a"), "a").unwrap(),
            vec![Cow::Borrowed(b"c")]
        );
    }

    #[test]
    fn non_relevant_subsection_is_ignored() {
        let config = GitConfig::try_from("[core]\na=b\na=c\n[core]a=d\n[core]g=g").unwrap();
        assert_eq!(
            config.get_raw_multi_value("core", None, "a").unwrap(),
            vec![Cow::Borrowed(b"b"), Cow::Borrowed(b"c"), Cow::Borrowed(b"d")]
        );
    }
}

#[cfg(test)]
mod display {
    use super::{GitConfig, TryFrom};

    #[test]
    fn can_reconstruct_empty_config() {
        let config = r#"

        "#;
        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_non_empty_config() {
        let config = r#"
            [user]
                email = code@eddie.sh
            [core]
                autocrlf = input
            [push]
                default = simple
            [commit]
                gpgsign = true
            [gpg]
                program = gpg
            [url "ssh://git@github.com/"]
                insteadOf = "github://"
            [url "ssh://git@git.eddie.sh/edward/"]
                insteadOf = "gitea://"
            [pull]
                ff = only
            [init]
                defaultBranch = master
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_with_implicits() {
        let config = r#"
            [user]
                email
                name
            [core]
                autocrlf
            [push]
                default
            [commit]
                gpgsign
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }

    #[test]
    fn can_reconstruct_configs_without_whitespace_in_middle() {
        let config = r#"
            [core]
                autocrlf=input
            [push]
                default=simple
            [commit]
                gpgsign=true
            [pull]
                ff = only
            [init]
                defaultBranch = master
        "#;

        assert_eq!(GitConfig::try_from(config).unwrap().to_string(), config);
    }
}
