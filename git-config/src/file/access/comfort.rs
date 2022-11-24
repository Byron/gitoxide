use std::{borrow::Cow, convert::TryFrom};

use bstr::BStr;

use crate::{file::MetadataFilter, value, File};

/// Comfortable API for accessing values
impl<'event> File<'event> {
    /// Like [`value()`][File::value()], but returning `None` if the string wasn't found.
    ///
    /// As strings perform no conversions, this will never fail.
    pub fn string(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<Cow<'_, BStr>> {
        self.string_filter(section_name, subsection_name, key, &mut |_| true)
    }

    /// Like [`string()`][File::string()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn string_by_key<'a>(&self, key: impl Into<&'a BStr>) -> Option<Cow<'_, BStr>> {
        self.string_filter_by_key(key, &mut |_| true)
    }

    /// Like [`string()`][File::string()], but the section containing the returned value must pass `filter` as well.
    pub fn string_filter(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
        filter: &mut MetadataFilter,
    ) -> Option<Cow<'_, BStr>> {
        self.raw_value_filter(section_name, subsection_name, key, filter).ok()
    }

    /// Like [`string_filter()`][File::string_filter()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn string_filter_by_key<'a>(
        &self,
        key: impl Into<&'a BStr>,
        filter: &mut MetadataFilter,
    ) -> Option<Cow<'_, BStr>> {
        let key = crate::parse::key(key)?;
        self.raw_value_filter(key.section_name, key.subsection_name, key.value_name, filter)
            .ok()
    }

    /// Like [`value()`][File::value()], but returning `None` if the path wasn't found.
    ///
    /// Note that this path is not vetted and should only point to resources which can't be used
    /// to pose a security risk. Prefer using [`path_filter()`][File::path_filter()] instead.
    ///
    /// As paths perform no conversions, this will never fail.
    pub fn path(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<crate::Path<'_>> {
        self.path_filter(section_name, subsection_name, key, &mut |_| true)
    }

    /// Like [`path()`][File::path()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn path_by_key<'a>(&self, key: impl Into<&'a BStr>) -> Option<crate::Path<'_>> {
        self.path_filter_by_key(key, &mut |_| true)
    }

    /// Like [`path()`][File::path()], but the section containing the returned value must pass `filter` as well.
    ///
    /// This should be the preferred way of accessing paths as those from untrusted
    /// locations can be
    ///
    /// As paths perform no conversions, this will never fail.
    pub fn path_filter(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
        filter: &mut MetadataFilter,
    ) -> Option<crate::Path<'_>> {
        self.raw_value_filter(section_name, subsection_name, key, filter)
            .ok()
            .map(crate::Path::from)
    }

    /// Like [`path_filter()`][File::path_filter()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn path_filter_by_key<'a>(
        &self,
        key: impl Into<&'a BStr>,
        filter: &mut MetadataFilter,
    ) -> Option<crate::Path<'_>> {
        let key = crate::parse::key(key)?;
        self.path_filter(key.section_name, key.subsection_name, key.value_name, filter)
    }

    /// Like [`value()`][File::value()], but returning `None` if the boolean value wasn't found.
    pub fn boolean(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<Result<bool, value::Error>> {
        self.boolean_filter(section_name, subsection_name, key, &mut |_| true)
    }

    /// Like [`boolean()`][File::boolean()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn boolean_by_key<'a>(&self, key: impl Into<&'a BStr>) -> Option<Result<bool, value::Error>> {
        self.boolean_filter_by_key(key, &mut |_| true)
    }

    /// Like [`boolean()`][File::boolean()], but the section containing the returned value must pass `filter` as well.
    pub fn boolean_filter(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
        filter: &mut MetadataFilter,
    ) -> Option<Result<bool, value::Error>> {
        let section_name = section_name.as_ref();
        let section_ids = self
            .section_ids_by_name_and_subname(section_name, subsection_name)
            .ok()?;
        let key = key.as_ref();
        for section_id in section_ids.rev() {
            let section = self.sections.get(&section_id).expect("known section id");
            if !filter(section.meta()) {
                continue;
            }
            match section.value_implicit(key) {
                Some(Some(v)) => return Some(crate::Boolean::try_from(v).map(|b| b.into())),
                Some(None) => return Some(Ok(true)),
                None => continue,
            }
        }
        None
    }

    /// Like [`boolean_filter()`][File::boolean_filter()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn boolean_filter_by_key<'a>(
        &self,
        key: impl Into<&'a BStr>,
        filter: &mut MetadataFilter,
    ) -> Option<Result<bool, value::Error>> {
        let key = crate::parse::key(key)?;
        self.boolean_filter(key.section_name, key.subsection_name, key.value_name, filter)
    }

    /// Like [`value()`][File::value()], but returning an `Option` if the integer wasn't found.
    pub fn integer(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<Result<i64, value::Error>> {
        self.integer_filter(section_name, subsection_name, key, &mut |_| true)
    }

    /// Like [`integer()`][File::integer()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn integer_by_key<'a>(&self, key: impl Into<&'a BStr>) -> Option<Result<i64, value::Error>> {
        self.integer_filter_by_key(key, &mut |_| true)
    }

    /// Like [`integer()`][File::integer()], but the section containing the returned value must pass `filter` as well.
    pub fn integer_filter(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
        filter: &mut MetadataFilter,
    ) -> Option<Result<i64, value::Error>> {
        let int = self.raw_value_filter(section_name, subsection_name, key, filter).ok()?;
        Some(crate::Integer::try_from(int.as_ref()).and_then(|b| {
            b.to_decimal()
                .ok_or_else(|| value::Error::new("Integer overflow", int.into_owned()))
        }))
    }

    /// Like [`integer_filter()`][File::integer_filter()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn integer_filter_by_key<'a>(
        &self,
        key: impl Into<&'a BStr>,
        filter: &mut MetadataFilter,
    ) -> Option<Result<i64, value::Error>> {
        let key = crate::parse::key(key)?;
        self.integer_filter(key.section_name, key.subsection_name, key.value_name, filter)
    }

    /// Similar to [`values(…)`][File::values()] but returning strings if at least one of them was found.
    pub fn strings(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<Vec<Cow<'_, BStr>>> {
        self.raw_values(section_name, subsection_name, key).ok()
    }

    /// Like [`strings()`][File::strings()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn strings_by_key<'a>(&self, key: impl Into<&'a BStr>) -> Option<Vec<Cow<'_, BStr>>> {
        let key = crate::parse::key(key)?;
        self.strings(key.section_name, key.subsection_name, key.value_name)
    }

    /// Similar to [`strings(…)`][File::strings()], but all values are in sections that passed `filter`.
    pub fn strings_filter(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
        filter: &mut MetadataFilter,
    ) -> Option<Vec<Cow<'_, BStr>>> {
        self.raw_values_filter(section_name, subsection_name, key, filter).ok()
    }

    /// Like [`strings_filter()`][File::strings_filter()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn strings_filter_by_key<'a>(
        &self,
        key: impl Into<&'a BStr>,
        filter: &mut MetadataFilter,
    ) -> Option<Vec<Cow<'_, BStr>>> {
        let key = crate::parse::key(key)?;
        self.strings_filter(key.section_name, key.subsection_name, key.value_name, filter)
    }

    /// Similar to [`values(…)`][File::values()] but returning integers if at least one of them was found
    /// and if none of them overflows.
    pub fn integers(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
    ) -> Option<Result<Vec<i64>, value::Error>> {
        self.integers_filter(section_name, subsection_name, key, &mut |_| true)
    }

    /// Like [`integers()`][File::integers()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn integers_by_key<'a>(&self, key: impl Into<&'a BStr>) -> Option<Result<Vec<i64>, value::Error>> {
        self.integers_filter_by_key(key, &mut |_| true)
    }

    /// Similar to [`integers(…)`][File::integers()] but all integers are in sections that passed `filter`
    /// and that are not overflowing.
    pub fn integers_filter(
        &self,
        section_name: impl AsRef<str>,
        subsection_name: Option<&BStr>,
        key: impl AsRef<str>,
        filter: &mut MetadataFilter,
    ) -> Option<Result<Vec<i64>, value::Error>> {
        self.raw_values_filter(section_name, subsection_name, key, filter)
            .ok()
            .map(|values| {
                values
                    .into_iter()
                    .map(|v| {
                        crate::Integer::try_from(v.as_ref()).and_then(|int| {
                            int.to_decimal()
                                .ok_or_else(|| value::Error::new("Integer overflow", v.into_owned()))
                        })
                    })
                    .collect()
            })
    }

    /// Like [`integers_filter()`][File::integers_filter()], but suitable for statically known `key`s like `remote.origin.url`.
    pub fn integers_filter_by_key<'a>(
        &self,
        key: impl Into<&'a BStr>,
        filter: &mut MetadataFilter,
    ) -> Option<Result<Vec<i64>, value::Error>> {
        let key = crate::parse::key(key)?;
        self.integers_filter(key.section_name, key.subsection_name, key.value_name, filter)
    }
}
