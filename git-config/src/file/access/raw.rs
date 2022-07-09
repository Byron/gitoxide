use bstr::{BStr, BString};
use std::{borrow::Cow, collections::HashMap};

use crate::{
    file::{EntryData, Index, MutableMultiValue, MutableSection, MutableValue, Size},
    lookup,
    parse::{section, Event},
    File,
};

/// # Raw value API
///
/// These functions are the raw value API. Instead of returning Rust structures,
/// these functions return bytes which may or may not be owned.
impl<'event> File<'event> {
    /// Returns an uninterpreted value given a section, an optional subsection
    /// and key.
    ///
    /// Consider [`Self::raw_values()`] if you want to get all values of
    /// a multivar instead.
    pub fn raw_value(
        &self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Result<Cow<'_, BStr>, lookup::existing::Error> {
        // Note: cannot wrap around the raw_multi_value method because we need
        // to guarantee that the highest section id is used (so that we follow
        // the "last one wins" resolution strategy by `git-config`).
        let key = section::Key(Cow::<BStr>::Borrowed(key.into()));
        for section_id in self
            .section_ids_by_name_and_subname(section_name, subsection_name)?
            .iter()
            .rev()
        {
            if let Some(v) = self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .value(&key)
            {
                return Ok(v);
            }
        }

        Err(lookup::existing::Error::KeyMissing)
    }

    /// Returns a mutable reference to an uninterpreted value given a section,
    /// an optional subsection and key.
    ///
    /// Consider [`Self::raw_values_mut`] if you want to get mutable
    /// references to all values of a multivar instead.
    pub fn raw_value_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<MutableValue<'_, 'lookup, 'event>, lookup::existing::Error> {
        let section_ids = self.section_ids_by_name_and_subname(section_name, subsection_name)?;
        let key = section::Key(Cow::<BStr>::Borrowed(key.into()));

        for section_id in section_ids.iter().rev() {
            let mut size = Size(0);
            let mut index = Index(0);
            let mut found_key = false;
            // todo: iter backwards
            for (i, event) in self
                .sections
                .get(section_id)
                .expect("sections does not have section id from section ids")
                .as_ref()
                .iter()
                .enumerate()
            {
                match event {
                    Event::SectionKey(event_key) if *event_key == key => {
                        found_key = true;
                        size = Size(1);
                        index = Index(i);
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

            if size.0 == 0 {
                continue;
            }

            return Ok(MutableValue::new(
                MutableSection::new(
                    self.sections
                        .get_mut(section_id)
                        .expect("sections does not have section id from section ids"),
                ),
                key,
                index,
                size,
            ));
        }

        Err(lookup::existing::Error::KeyMissing)
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
    /// # use git_config::File;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.raw_values("core", None, "a").unwrap(),
    ///     vec![
    ///         Cow::<[u8]>::Borrowed(b"b"),
    ///         Cow::<[u8]>::Borrowed(b"c"),
    ///         Cow::<[u8]>::Borrowed(b"d"),
    ///     ],
    /// );
    /// ```
    ///
    /// Consider [`Self::raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    pub fn raw_values(
        &self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
    ) -> Result<Vec<Cow<'_, BStr>>, lookup::existing::Error> {
        let mut values = vec![];
        for section_id in self.section_ids_by_name_and_subname(section_name, subsection_name)? {
            values.extend(
                self.sections
                    .get(&section_id)
                    .expect("sections does not have section id from section ids")
                    .values(&section::Key(Cow::<BStr>::Borrowed(key.into())))
                    .into_iter(),
            );
        }

        if values.is_empty() {
            Err(lookup::existing::Error::KeyMissing)
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
    /// # use git_config::File;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # use bstr::BStr;
    /// # let mut git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(
    ///     git_config.raw_values("core", None, "a")?,
    ///     vec![
    ///         Cow::<BStr>::Borrowed("b".into()),
    ///         Cow::<BStr>::Borrowed("c".into()),
    ///         Cow::<BStr>::Borrowed("d".into())
    ///     ]
    /// );
    ///
    /// git_config.raw_values_mut("core", None, "a")?.set_str_all("g");
    ///
    /// assert_eq!(
    ///     git_config.raw_values("core", None, "a")?,
    ///     vec![
    ///         Cow::<BStr>::Borrowed("g".into()),
    ///         Cow::<BStr>::Borrowed("g".into()),
    ///         Cow::<BStr>::Borrowed("g".into())
    ///     ],
    /// );
    /// # Ok::<(), git_config::lookup::existing::Error>(())
    /// ```
    ///
    /// Consider [`Self::raw_value`] if you want to get the resolved single
    /// value for a given key, if your key does not support multi-valued values.
    ///
    /// Note that this operation is relatively expensive, requiring a full
    /// traversal of the config.
    pub fn raw_values_mut<'lookup>(
        &mut self,
        section_name: &'lookup str,
        subsection_name: Option<&'lookup str>,
        key: &'lookup str,
    ) -> Result<MutableMultiValue<'_, 'lookup, 'event>, lookup::existing::Error> {
        let section_ids = self.section_ids_by_name_and_subname(section_name, subsection_name)?;
        let key = section::Key(Cow::<BStr>::Borrowed(key.into()));

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
                .as_ref()
                .iter()
                .enumerate()
            {
                match event {
                    Event::SectionKey(event_key) if *event_key == key => {
                        found_key = true;
                        offset_list.push(i - last_boundary);
                        offset_index += 1;
                        last_boundary = i;
                    }
                    Event::Value(_) | Event::ValueDone(_) if found_key => {
                        found_key = false;
                        entries.push(EntryData::new(*section_id, offset_index));
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
            Err(lookup::existing::Error::KeyMissing)
        } else {
            Ok(MutableMultiValue::new(&mut self.sections, key, entries, offsets))
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
    /// # use git_config::File;
    /// # use std::borrow::Cow;
    /// # use bstr::BStr;
    /// # use std::convert::TryFrom;
    /// # let mut git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// git_config.set_raw_value("core", None, "a", "e".into())?;
    /// assert_eq!(git_config.raw_value("core", None, "a")?, Cow::<BStr>::Borrowed("e".into()));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn set_raw_value(
        &mut self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
        new_value: BString,
    ) -> Result<(), lookup::existing::Error> {
        self.raw_value_mut(section_name, subsection_name, key)
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
    /// consider using [`raw_multi_value_mut`], which will let you iterate
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
    /// # use git_config::File;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # use bstr::BStr;
    /// # let mut git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values = vec![
    ///     Cow::<BStr>::Borrowed("x".into()),
    ///     Cow::<BStr>::Borrowed("y".into()),
    ///     Cow::<BStr>::Borrowed("z".into()),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// let fetched_config = git_config.raw_values("core", None, "a")?;
    /// assert!(fetched_config.contains(&Cow::<BStr>::Borrowed("x".into())));
    /// assert!(fetched_config.contains(&Cow::<BStr>::Borrowed("y".into())));
    /// assert!(fetched_config.contains(&Cow::<BStr>::Borrowed("z".into())));
    /// # Ok::<(), git_config::lookup::existing::Error>(())
    /// ```
    ///
    /// Setting less than the number of present values sets the first ones found:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # use bstr::BStr;
    /// # let mut git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values = vec![
    ///     Cow::<BStr>::Borrowed("x".into()),
    ///     Cow::<BStr>::Borrowed("y".into()),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// let fetched_config = git_config.raw_values("core", None, "a")?;
    /// assert!(fetched_config.contains(&Cow::<BStr>::Borrowed("x".into())));
    /// assert!(fetched_config.contains(&Cow::<BStr>::Borrowed("y".into())));
    /// # Ok::<(), git_config::lookup::existing::Error>(())
    /// ```
    ///
    /// Setting more than the number of present values discards the rest:
    ///
    /// ```
    /// # use git_config::File;
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # use bstr::BStr;
    /// # let mut git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// let new_values = vec![
    ///     Cow::<BStr>::Borrowed("x".into()),
    ///     Cow::<BStr>::Borrowed("y".into()),
    ///     Cow::<BStr>::Borrowed("z".into()),
    ///     Cow::<BStr>::Borrowed("discarded".into()),
    /// ];
    /// git_config.set_raw_multi_value("core", None, "a", new_values.into_iter())?;
    /// assert!(!git_config.raw_values("core", None, "a")?.contains(&Cow::<BStr>::Borrowed("discarded".into())));
    /// # Ok::<(), git_config::lookup::existing::Error>(())
    /// ```
    ///
    /// [`raw_multi_value_mut`]: Self::raw_values_mut
    pub fn set_raw_multi_value(
        &mut self,
        section_name: &str,
        subsection_name: Option<&str>,
        key: &str,
        new_values: impl Iterator<Item = Cow<'event, BStr>>,
    ) -> Result<(), lookup::existing::Error> {
        self.raw_values_mut(section_name, subsection_name, key)
            .map(|mut v| v.set_values(new_values))
    }
}
